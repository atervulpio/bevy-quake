//! The player's own movement (`sv_user.c`): what the keys ask for turned into a velocity, by
//! friction, acceleration, swimming and the jump out of water; and the view's pitch set to the
//! slope ahead (`SV_SetIdealPitch`).

use crate::builtins::angle_vectors;
use crate::{Ent, FL_ONGROUND, FL_WATERJUMP, Game, MOVETYPE_NOCLIP, MOVETYPE_NONE, MOVETYPE_WALK, UserCmd};
use glam::Vec3;

const ON_EPSILON: f32 = 0.1;
/// `cl_rollangle` and `cl_rollspeed`: the view leans up to two degrees when strafing at 200.
const ROLL_ANGLE: f32 = 2.0;
const ROLL_SPEED: f32 = 200.0;

/// `V_CalcRoll`: how far the view leans with the sideways speed.
pub fn calc_roll(angles: Vec3, velocity: Vec3) -> f32 {
    let (_, right, _) = angle_vectors(angles);
    let side = velocity.dot(right);
    let sign = if side < 0.0 { -1.0 } else { 1.0 };
    let side = side.abs();
    let side = if side < ROLL_SPEED { side * ROLL_ANGLE / ROLL_SPEED } else { ROLL_ANGLE };
    side * sign
}

impl Game {
    /// `SV_SetIdealPitch`: on a slope that climbs or falls evenly ahead, the view tilts with it.
    pub fn set_ideal_pitch(&mut self, p: Ent) {
        if self.ents[p].v.flags & FL_ONGROUND == 0 {
            return;
        }
        let angleval = self.ents[p].v.angles.y * std::f32::consts::PI * 2.0 / 360.0;
        let (sinval, cosval) = angleval.sin_cos();
        let mut z = [0.0f32; 6];
        for (i, zi) in z.iter_mut().enumerate() {
            let v = &self.ents[p].v;
            let top = Vec3::new(v.origin.x + cosval * (i as f32 + 3.0) * 12.0, v.origin.y + sinval * (i as f32 + 3.0) * 12.0, v.origin.z + v.view_ofs.z);
            let bottom = top - Vec3::new(0.0, 0.0, 160.0);
            let tr = self.trace_move(top, Vec3::ZERO, Vec3::ZERO, bottom, 1, p);
            if tr.allsolid || tr.fraction == 1.0 {
                return; // a wall ahead, or a drop
            }
            *zi = top.z + tr.fraction * (bottom.z - top.z);
        }
        let mut dir = 0.0f32;
        let mut steps = 0;
        for j in 1..6 {
            // `step` is an int in the original: the rise between two samples, cut to whole units.
            let step = (z[j] - z[j - 1]) as i32 as f32;
            if step > -ON_EPSILON && step < ON_EPSILON {
                continue;
            }
            if dir != 0.0 && (step - dir > ON_EPSILON || step - dir < -ON_EPSILON) {
                return; // mixed changes
            }
            steps += 1;
            dir = step;
        }
        if dir == 0.0 {
            self.ents[p].v.idealpitch = 0.0;
            return;
        }
        if steps < 2 {
            return;
        }
        self.ents[p].v.idealpitch = -dir * self.cvars.value("sv_idealpitchscale");
    }

    /// `SV_UserFriction`: walking slows by friction, twice over at an edge.
    fn user_friction(&mut self, p: Ent) {
        let v = &self.ents[p].v;
        let vel = v.velocity;
        let speed = (vel.x * vel.x + vel.y * vel.y).sqrt();
        if speed == 0.0 {
            return;
        }
        let start = Vec3::new(v.origin.x + vel.x / speed * 16.0, v.origin.y + vel.y / speed * 16.0, v.origin.z + v.mins.z);
        let stop = start - Vec3::new(0.0, 0.0, 34.0);
        let trace = self.trace_move(start, Vec3::ZERO, Vec3::ZERO, stop, 1, p);
        let friction = if trace.fraction == 1.0 { self.cvars.value("sv_friction") * self.cvars.value("sv_edgefriction") } else { self.cvars.value("sv_friction") };
        let stopspeed = self.cvars.value("sv_stopspeed");
        let control = if speed < stopspeed { stopspeed } else { speed };
        let newspeed = (speed - self.frame_seconds * control * friction).max(0.0) / speed;
        self.ents[p].v.velocity *= newspeed;
    }

    /// `SV_Accelerate`.
    fn accelerate(&mut self, p: Ent, wishdir: Vec3, wishspeed: f32) {
        let v = &mut self.ents[p].v;
        let currentspeed = v.velocity.dot(wishdir);
        let addspeed = wishspeed - currentspeed;
        if addspeed <= 0.0 {
            return;
        }
        let accelspeed = (self.cvars.value("sv_accelerate") * self.frame_seconds * wishspeed).min(addspeed);
        v.velocity += accelspeed * wishdir;
    }

    /// `SV_AirAccelerate`: in the air the wish counts for at most 30 units a second.
    fn air_accelerate(&mut self, p: Ent, wishveloc: Vec3, wishspeed: f32) {
        let len = wishveloc.length();
        let dir = if len == 0.0 { wishveloc } else { wishveloc / len };
        let wishspd = len.min(30.0);
        let v = &mut self.ents[p].v;
        let currentspeed = v.velocity.dot(dir);
        let addspeed = wishspd - currentspeed;
        if addspeed <= 0.0 {
            return;
        }
        let accelspeed = (self.cvars.value("sv_accelerate") * wishspeed * self.frame_seconds).min(addspeed);
        v.velocity += accelspeed * dir;
    }

    /// `DropPunchAngle`: the kick of a shot or a hit dies away at ten degrees a second.
    fn drop_punch_angle(&mut self, p: Ent) {
        let v = &mut self.ents[p].v;
        let len = v.punchangle.length();
        let dir = if len == 0.0 { Vec3::ZERO } else { v.punchangle / len };
        let len = (len - 10.0 * self.frame_seconds).max(0.0);
        v.punchangle = dir * len;
    }

    /// `SV_WaterMove`: swimming, with a drift to the bottom when nothing is pressed.
    fn water_move(&mut self, p: Ent, cmd: &UserCmd) {
        let (forward, right, _) = angle_vectors(self.ents[p].v.v_angle);
        let mut wishvel = forward * cmd.forwardmove + right * cmd.sidemove;
        if cmd.forwardmove == 0.0 && cmd.sidemove == 0.0 && cmd.upmove == 0.0 {
            wishvel.z -= 60.0;
        } else {
            wishvel.z += cmd.upmove;
        }
        let maxspeed = self.cvars.value("sv_maxspeed");
        let mut wishspeed = wishvel.length();
        if wishspeed > maxspeed {
            wishvel *= maxspeed / wishspeed;
            wishspeed = maxspeed;
        }
        wishspeed *= 0.7;
        let friction = self.cvars.value("sv_friction");
        let ft = self.frame_seconds;
        let v = &mut self.ents[p].v;
        let speed = v.velocity.length();
        let newspeed = if speed != 0.0 {
            let n = (speed - ft * speed * friction).max(0.0);
            v.velocity *= n / speed;
            n
        } else {
            0.0
        };
        if wishspeed == 0.0 {
            return;
        }
        let addspeed = wishspeed - newspeed;
        if addspeed <= 0.0 {
            return;
        }
        let l = wishvel.length();
        if l != 0.0 {
            wishvel /= l;
        }
        let accelspeed = (self.cvars.value("sv_accelerate") * wishspeed * ft).min(addspeed);
        self.ents[p].v.velocity += accelspeed * wishvel;
    }

    /// `SV_WaterJump`: the leap out of water carries on until it lands or times out.
    fn water_jump(&mut self, p: Ent) {
        let t = self.sv_time as f32;
        let v = &mut self.ents[p].v;
        if t > v.teleport_time || v.waterlevel == 0 {
            v.flags &= !FL_WATERJUMP;
            v.teleport_time = 0.0;
        }
        v.velocity.x = v.movedir.x;
        v.velocity.y = v.movedir.y;
    }

    /// `SV_AirMove`: walking, flying with no clip, or steering in the air.
    fn air_move(&mut self, p: Ent, cmd: &UserCmd) {
        let (forward, right, _) = angle_vectors(self.ents[p].v.angles);
        let mut fmove = cmd.forwardmove;
        let smove = cmd.sidemove;
        if (self.sv_time as f32) < self.ents[p].v.teleport_time && fmove < 0.0 {
            fmove = 0.0; // hold still after a teleport
        }
        let mut wishvel = forward * fmove + right * smove;
        wishvel.z = if self.ents[p].v.movetype != MOVETYPE_WALK { cmd.upmove } else { 0.0 };
        let mut wishspeed = wishvel.length();
        let wishdir = if wishspeed == 0.0 { wishvel } else { wishvel / wishspeed };
        let maxspeed = self.cvars.value("sv_maxspeed");
        if wishspeed > maxspeed {
            wishvel *= maxspeed / wishspeed;
            wishspeed = maxspeed;
        }
        if self.ents[p].v.movetype == MOVETYPE_NOCLIP {
            self.ents[p].v.velocity = wishvel;
        } else if self.ents[p].v.flags & FL_ONGROUND != 0 {
            self.user_friction(p);
            self.accelerate(p, wishdir, wishspeed);
        } else {
            self.air_accelerate(p, wishvel, wishspeed);
        }
    }

    /// `SV_ClientThink`: the player's body turned and moved by this frame's command.
    pub fn client_think(&mut self, p: Ent) {
        if self.ents[p].v.movetype == MOVETYPE_NONE {
            return;
        }
        self.drop_punch_angle(p);
        if self.ents[p].v.health <= 0.0 {
            return;
        }
        let cmd = self.cmd;
        let v = &mut self.ents[p].v;
        let v_angle = v.v_angle + v.punchangle;
        v.angles.z = calc_roll(v.angles, v.velocity) * 4.0;
        if v.fixangle == 0 {
            v.angles.x = -v_angle.x / 3.0;
            v.angles.y = v_angle.y;
        }
        if v.flags & FL_WATERJUMP != 0 {
            self.water_jump(p);
            return;
        }
        if self.ents[p].v.waterlevel >= 2 && self.ents[p].v.movetype != MOVETYPE_NOCLIP {
            self.water_move(p, &cmd);
            return;
        }
        self.air_move(p, &cmd);
    }

    /// `SV_ReadClientMove` and `SV_RunClients`: the command's angles and buttons set on the
    /// player, and its move applied.
    pub fn run_client(&mut self, cmd: UserCmd) {
        let p = 1;
        if !self.client_active {
            return;
        }
        if !self.client_spawned {
            self.cmd = UserCmd::default();
            return;
        }
        self.cmd = cmd;
        let v = &mut self.ents[p].v;
        v.v_angle = cmd.viewangles;
        v.button0 = cmd.buttons & 1;
        v.button2 = (cmd.buttons & 2) >> 1;
        if cmd.impulse != 0 {
            v.impulse = cmd.impulse;
        }
        if !self.paused {
            self.client_think(p);
        }
    }
}
