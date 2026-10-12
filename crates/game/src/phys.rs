//! The world's physics (`sv_phys.c`): each entity moved by its kind of motion once a frame.
//!
//! Doors and lifts are pushers, `MOVETYPE_PUSH`; things that fall are tossed; walking monsters
//! step; the player walks. A function an entity carries (`think`, `touch`, `blocked`) is called
//! where the original calls it, with `self`, `other` and `time` set as it set them.

use crate::world::Trace;
use crate::{
    CONTENT_EMPTY, CONTENT_WATER, Ent, FL_FLY, FL_ONGROUND, FL_SWIM, FL_WATERJUMP, Game, MAX_CLIENTS, MOVE_MISSILE, MOVE_NOMONSTERS, MOVE_NORMAL, MOVETYPE_BOUNCE, MOVETYPE_FLY,
    MOVETYPE_FLYMISSILE, MOVETYPE_NOCLIP, MOVETYPE_NONE, MOVETYPE_PUSH, MOVETYPE_STEP, MOVETYPE_TOSS, MOVETYPE_WALK, SOLID_BSP, SOLID_NOT, SOLID_TRIGGER,
};
use glam::Vec3;

const STOP_EPSILON: f32 = 0.1;
const MAX_CLIP_PLANES: usize = 5;
const STEPSIZE: f32 = 18.0;

/// `ClipVelocity`: the velocity slid off a surface; the flags say what was hit (1 a floor,
/// 2 a wall or step).
pub fn clip_velocity(input: Vec3, normal: Vec3, overbounce: f32) -> (Vec3, i32) {
    let mut blocked = 0;
    if normal.z > 0.0 {
        blocked |= 1;
    }
    if normal.z == 0.0 {
        blocked |= 2;
    }
    let backoff = input.dot(normal) * overbounce;
    let mut out = input - normal * backoff;
    for i in 0..3 {
        if out[i] > -STOP_EPSILON && out[i] < STOP_EPSILON {
            out[i] = 0.0;
        }
    }
    (out, blocked)
}

impl Game {
    /// The seconds one frame of the world lasts (`host_frametime`).
    pub fn host_frametime(&self) -> f32 {
        self.frame_seconds
    }

    /// `SV_CheckVelocity`: no number not a number, no speed over `sv_maxvelocity` on any axis.
    pub fn check_velocity(&mut self, e: Ent) {
        let max = self.cvars.value("sv_maxvelocity");
        let v = &mut self.ents[e].v;
        for i in 0..3 {
            if v.velocity[i].is_nan() {
                v.velocity[i] = 0.0;
            }
            if v.origin[i].is_nan() {
                v.origin[i] = 0.0;
            }
            v.velocity[i] = v.velocity[i].clamp(-max, max);
        }
    }

    /// `SV_RunThink`: the entity's `think`, if its time has come within this frame; false if it
    /// removed itself.
    pub fn run_think(&mut self, e: Ent) -> bool {
        let mut thinktime = self.ents[e].v.nextthink;
        if thinktime <= 0.0 || thinktime > (self.sv_time + self.frame_seconds as f64) as f32 {
            return true;
        }
        if thinktime < self.sv_time as f32 {
            thinktime = self.sv_time as f32; // never in the past
        }
        self.ents[e].v.nextthink = 0.0;
        self.time = thinktime;
        self.self_ = e;
        self.other = 0;
        let f = self.ents[e].v.think;
        self.call(f);
        !self.ents[e].free
    }

    /// `SV_Impact`: two entities touched; each one's `touch` runs.
    pub fn impact(&mut self, e1: Ent, e2: Ent) {
        let (old_self, old_other) = (self.self_, self.other);
        self.time = self.sv_time as f32;
        if let Some(f) = self.ents[e1].v.touch
            && self.ents[e1].v.solid != SOLID_NOT {
                self.self_ = e1;
                self.other = e2;
                f(self);
            }
        if let Some(f) = self.ents[e2].v.touch
            && self.ents[e2].v.solid != SOLID_NOT {
                self.self_ = e2;
                self.other = e1;
                f(self);
            }
        self.self_ = old_self;
        self.other = old_other;
    }

    /// `SV_FlyMove`: a body moved for `time`, sliding along up to five surfaces; returns what
    /// was hit (1 floor, 2 wall or step, 4 a dead stop), and the last wall's trace.
    pub fn fly_move(&mut self, e: Ent, time: f32) -> (i32, Option<Trace>) {
        let mut steptrace = None;
        let mut blocked = 0;
        let mut original_velocity = self.ents[e].v.velocity;
        let primal_velocity = original_velocity;
        let mut planes = [Vec3::ZERO; MAX_CLIP_PLANES];
        let mut numplanes = 0;
        let mut time_left = time;
        for _ in 0..4 {
            let v = &self.ents[e].v;
            if v.velocity == Vec3::ZERO {
                break;
            }
            let end = v.origin + time_left * v.velocity;
            let (o, mins, maxs) = (v.origin, v.mins, v.maxs);
            let trace = self.trace_move(o, mins, maxs, end, MOVE_NORMAL, e);
            if trace.allsolid {
                self.ents[e].v.velocity = Vec3::ZERO; // trapped in another solid
                return (3, steptrace);
            }
            if trace.fraction > 0.0 {
                self.ents[e].v.origin = trace.endpos;
                original_velocity = self.ents[e].v.velocity;
                numplanes = 0;
            }
            if trace.fraction == 1.0 {
                break;
            }
            let hit = trace.ent.expect("SV_FlyMove: !trace.ent");
            if trace.normal.z > 0.7 {
                blocked |= 1;
                if self.ents[hit].v.solid == SOLID_BSP {
                    self.ents[e].v.flags |= FL_ONGROUND;
                    self.ents[e].v.groundentity = hit;
                }
            }
            if trace.normal.z == 0.0 {
                blocked |= 2;
                steptrace = Some(trace);
            }
            self.impact(e, hit);
            if self.ents[e].free {
                break; // removed by the touch
            }
            time_left -= time_left * trace.fraction;
            if numplanes >= MAX_CLIP_PLANES {
                self.ents[e].v.velocity = Vec3::ZERO;
                return (3, steptrace);
            }
            planes[numplanes] = trace.normal;
            numplanes += 1;
            let mut new_velocity = Vec3::ZERO;
            let mut i = 0;
            while i < numplanes {
                new_velocity = clip_velocity(original_velocity, planes[i], 1.0).0;
                let ok = (0..numplanes).all(|j| j == i || new_velocity.dot(planes[j]) >= 0.0);
                if ok {
                    break;
                }
                i += 1;
            }
            if i != numplanes {
                self.ents[e].v.velocity = new_velocity;
            } else {
                if numplanes != 2 {
                    self.ents[e].v.velocity = Vec3::ZERO;
                    return (7, steptrace);
                }
                let dir = planes[0].cross(planes[1]);
                let d = dir.dot(self.ents[e].v.velocity);
                self.ents[e].v.velocity = dir * d; // along the crease
            }
            if self.ents[e].v.velocity.dot(primal_velocity) <= 0.0 {
                self.ents[e].v.velocity = Vec3::ZERO; // never turn back against the first push
                return (blocked, steptrace);
            }
        }
        (blocked, steptrace)
    }

    /// `SV_AddGravity`. id's game has no `gravity` field, so every body falls alike.
    pub fn add_gravity(&mut self, e: Ent) {
        let g = self.cvars.value("sv_gravity");
        self.ents[e].v.velocity.z -= g * self.frame_seconds;
    }

    /// `SV_PushEntity`: moved by `push` without its velocity changing, touching what it meets.
    pub fn push_entity(&mut self, e: Ent, push: Vec3) -> Trace {
        let v = &self.ents[e].v;
        let end = v.origin + push;
        let kind = if v.movetype == MOVETYPE_FLYMISSILE {
            MOVE_MISSILE
        } else if v.solid == SOLID_TRIGGER || v.solid == SOLID_NOT {
            MOVE_NOMONSTERS
        } else {
            MOVE_NORMAL
        };
        let (o, mins, maxs) = (v.origin, v.mins, v.maxs);
        let trace = self.trace_move(o, mins, maxs, end, kind, e);
        self.ents[e].v.origin = trace.endpos;
        self.link_edict(e, true);
        if let Some(hit) = trace.ent {
            self.impact(e, hit);
        }
        trace
    }

    /// `SV_PushMove`: a door or lift moved, carrying what stands on it and pushing what is in
    /// its way; if something cannot be pushed, everything goes back and `blocked` runs.
    fn push_move(&mut self, pusher: Ent, movetime: f32) {
        if self.ents[pusher].v.velocity == Vec3::ZERO {
            self.ents[pusher].v.ltime += movetime;
            return;
        }
        let mv = self.ents[pusher].v.velocity * movetime;
        let mins = self.ents[pusher].v.absmin + mv;
        let maxs = self.ents[pusher].v.absmax + mv;
        let pushorig = self.ents[pusher].v.origin;
        self.ents[pusher].v.origin += mv;
        self.ents[pusher].v.ltime += movetime;
        self.link_edict(pusher, false);
        let mut moved: Vec<(Ent, Vec3)> = Vec::new();
        for check in 1..self.ents.len() {
            if self.ents[check].free {
                continue;
            }
            let c = &self.ents[check].v;
            if c.movetype == MOVETYPE_PUSH || c.movetype == MOVETYPE_NONE || c.movetype == MOVETYPE_NOCLIP {
                continue;
            }
            // if the entity is standing on the pusher, it will definitely be moved
            if !(c.flags & FL_ONGROUND != 0 && c.groundentity == pusher) {
                if c.absmin.x >= maxs.x || c.absmin.y >= maxs.y || c.absmin.z >= maxs.z || c.absmax.x <= mins.x || c.absmax.y <= mins.y || c.absmax.z <= mins.z {
                    continue;
                }
                if !self.test_entity_position(check) {
                    continue; // not in the pusher's way
                }
            }
            if self.ents[check].v.movetype != MOVETYPE_WALK {
                self.ents[check].v.flags &= !FL_ONGROUND;
            }
            let entorig = self.ents[check].v.origin;
            moved.push((check, entorig));
            self.ents[pusher].v.solid = SOLID_NOT;
            self.push_entity(check, mv);
            self.ents[pusher].v.solid = SOLID_BSP;
            if self.test_entity_position(check) {
                let c = &mut self.ents[check].v;
                if c.mins.x == c.maxs.x {
                    continue; // a point: it may stay inside
                }
                if c.solid == SOLID_NOT || c.solid == SOLID_TRIGGER {
                    // a corpse: flattened to a point rather than blocking
                    c.mins.x = 0.0;
                    c.mins.y = 0.0;
                    c.maxs = c.mins;
                    continue;
                }
                c.origin = entorig;
                self.link_edict(check, true);
                self.ents[pusher].v.origin = pushorig;
                self.link_edict(pusher, false);
                self.ents[pusher].v.ltime -= movetime;
                if let Some(f) = self.ents[pusher].v.blocked {
                    self.self_ = pusher;
                    self.other = check;
                    f(self);
                }
                for &(m, from) in &moved {
                    self.ents[m].v.origin = from;
                    self.link_edict(m, false);
                }
                return;
            }
        }
    }

    /// `SV_Physics_Pusher`: moved for the frame or until its `think` is due, then the think.
    fn physics_pusher(&mut self, e: Ent) {
        let oldltime = self.ents[e].v.ltime;
        let thinktime = self.ents[e].v.nextthink;
        let ft = self.frame_seconds;
        let movetime = if thinktime < self.ents[e].v.ltime + ft { (thinktime - self.ents[e].v.ltime).max(0.0) } else { ft };
        if movetime != 0.0 {
            self.push_move(e, movetime); // advances ltime if not blocked
        }
        if thinktime > oldltime && thinktime <= self.ents[e].v.ltime {
            self.ents[e].v.nextthink = 0.0;
            self.time = self.sv_time as f32;
            self.self_ = e;
            self.other = 0;
            let f = self.ents[e].v.think;
            self.call(f);
        }
    }

    /// `SV_CheckStuck`: a player inside a wall put back where it last stood, or nudged out.
    fn check_stuck(&mut self, e: Ent) {
        if !self.test_entity_position(e) {
            self.ents[e].v.oldorigin = self.ents[e].v.origin;
            return;
        }
        let org = self.ents[e].v.origin;
        self.ents[e].v.origin = self.ents[e].v.oldorigin;
        if !self.test_entity_position(e) {
            self.link_edict(e, true);
            return;
        }
        for z in 0..18 {
            for i in -1..=1 {
                for j in -1..=1 {
                    self.ents[e].v.origin = org + Vec3::new(i as f32, j as f32, z as f32);
                    if !self.test_entity_position(e) {
                        self.link_edict(e, true);
                        return;
                    }
                }
            }
        }
        self.ents[e].v.origin = org; // stuck
    }

    /// `SV_CheckWater`: how deep the player stands in a liquid (1 feet, 2 waist, 3 eyes);
    /// true from the waist down.
    pub fn check_water(&mut self, e: Ent) -> bool {
        let v = &self.ents[e].v;
        let mut point = Vec3::new(v.origin.x, v.origin.y, v.origin.z + v.mins.z + 1.0);
        let (mid, eye) = (v.origin.z + (v.mins.z + v.maxs.z) * 0.5, v.origin.z + v.view_ofs.z);
        self.ents[e].v.waterlevel = 0;
        self.ents[e].v.watertype = CONTENT_EMPTY;
        let cont = self.point_contents(point);
        if cont <= CONTENT_WATER {
            self.ents[e].v.watertype = cont;
            self.ents[e].v.waterlevel = 1;
            point.z = mid;
            if self.point_contents(point) <= CONTENT_WATER {
                self.ents[e].v.waterlevel = 2;
                point.z = eye;
                if self.point_contents(point) <= CONTENT_WATER {
                    self.ents[e].v.waterlevel = 3;
                }
            }
        }
        self.ents[e].v.waterlevel > 1
    }

    /// `SV_WallFriction`: running into a wall slows the slide along it.
    fn wall_friction(&mut self, e: Ent, trace: &Trace) {
        let (forward, _, _) = crate::builtins::angle_vectors(self.ents[e].v.v_angle);
        let d = trace.normal.dot(forward) + 0.5;
        if d >= 0.0 {
            return;
        }
        let v = &mut self.ents[e].v;
        let i = trace.normal.dot(v.velocity);
        let side = v.velocity - trace.normal * i;
        v.velocity.x = side.x * (1.0 + d);
        v.velocity.y = side.y * (1.0 + d);
    }

    /// `SV_TryUnstick`: a player stopped dead nudged two units each way in turn.
    fn try_unstick(&mut self, e: Ent, oldvel: Vec3) -> i32 {
        let oldorg = self.ents[e].v.origin;
        let dirs = [(2.0, 0.0), (0.0, 2.0), (-2.0, 0.0), (0.0, -2.0), (2.0, 2.0), (-2.0, 2.0), (2.0, -2.0), (-2.0, -2.0)];
        for (x, y) in dirs {
            self.push_entity(e, Vec3::new(x, y, 0.0));
            self.ents[e].v.velocity = Vec3::new(oldvel.x, oldvel.y, 0.0);
            let (clip, _) = self.fly_move(e, 0.1);
            let o = self.ents[e].v.origin;
            if (oldorg.y - o.y).abs() > 4.0 || (oldorg.x - o.x).abs() > 4.0 {
                return clip;
            }
            self.ents[e].v.origin = oldorg;
        }
        self.ents[e].v.velocity = Vec3::ZERO;
        7 // still not moving
    }

    /// `SV_WalkMove`: the player's move, with stairs climbed: if the move hit a step, it is
    /// tried again from 18 units up and set back down.
    fn walk_move(&mut self, e: Ent) {
        let oldonground = self.ents[e].v.flags & FL_ONGROUND;
        self.ents[e].v.flags &= !FL_ONGROUND;
        let oldorg = self.ents[e].v.origin;
        let oldvel = self.ents[e].v.velocity;
        let ft = self.frame_seconds;
        let (clip, _) = self.fly_move(e, ft);
        if clip & 2 == 0 {
            return; // did not block on a step
        }
        if oldonground == 0 && self.ents[e].v.waterlevel == 0 {
            return; // no climbing while jumping
        }
        if self.ents[e].v.movetype != MOVETYPE_WALK {
            return; // gibbed by a trigger
        }
        if self.cvars.value("sv_nostep") != 0.0 {
            return;
        }
        if self.ents[e].v.flags & FL_WATERJUMP != 0 {
            return;
        }
        let nosteporg = self.ents[e].v.origin;
        let nostepvel = self.ents[e].v.velocity;
        self.ents[e].v.origin = oldorg; // back to the start
        let upmove = Vec3::new(0.0, 0.0, STEPSIZE);
        let downmove = Vec3::new(0.0, 0.0, -STEPSIZE + oldvel.z * ft);
        self.push_entity(e, upmove);
        self.ents[e].v.velocity = Vec3::new(oldvel.x, oldvel.y, 0.0);
        let (mut clip, steptrace) = self.fly_move(e, ft);
        if clip != 0 {
            let o = self.ents[e].v.origin;
            if (oldorg.y - o.y).abs() < 0.03125 && (oldorg.x - o.x).abs() < 0.03125 {
                clip = self.try_unstick(e, oldvel); // stepping up made no progress
            }
        }
        if clip & 2 != 0
            && let Some(t) = steptrace {
                self.wall_friction(e, &t);
            }
        let downtrace = self.push_entity(e, downmove);
        if downtrace.normal.z > 0.7 {
            if self.ents[e].v.solid == SOLID_BSP {
                self.ents[e].v.flags |= FL_ONGROUND;
                self.ents[e].v.groundentity = downtrace.ent.unwrap_or(0);
            }
        } else {
            // the step up led nowhere: keep the move made without it
            self.ents[e].v.origin = nosteporg;
            self.ents[e].v.velocity = nostepvel;
        }
    }

    /// `SV_Physics_Client`: the player's frame, between the game's `PlayerPreThink` and
    /// `PlayerPostThink`.
    fn physics_client(&mut self, e: Ent) {
        if !self.client_active {
            return;
        }
        self.time = self.sv_time as f32;
        self.self_ = e;
        crate::progs::PlayerPreThink(self);
        self.check_velocity(e);
        let ft = self.frame_seconds;
        match self.ents[e].v.movetype {
            MOVETYPE_NONE => {
                if !self.run_think(e) {
                    return;
                }
            }
            MOVETYPE_WALK => {
                if !self.run_think(e) {
                    return;
                }
                if !self.check_water(e) && self.ents[e].v.flags & FL_WATERJUMP == 0 {
                    self.add_gravity(e);
                }
                self.check_stuck(e);
                self.walk_move(e);
            }
            MOVETYPE_TOSS | MOVETYPE_BOUNCE => self.physics_toss(e),
            MOVETYPE_FLY => {
                if !self.run_think(e) {
                    return;
                }
                self.fly_move(e, ft);
            }
            MOVETYPE_NOCLIP => {
                if !self.run_think(e) {
                    return;
                }
                let v = &mut self.ents[e].v;
                v.origin += ft * v.velocity;
            }
            m => panic!("SV_Physics_client: bad movetype {m}"),
        }
        self.link_edict(e, true);
        self.time = self.sv_time as f32;
        self.self_ = e;
        crate::progs::PlayerPostThink(self);
    }

    /// `SV_Physics_Noclip`: moved by its velocity, through everything.
    fn physics_noclip(&mut self, e: Ent) {
        if !self.run_think(e) {
            return;
        }
        let ft = self.frame_seconds;
        let v = &mut self.ents[e].v;
        v.angles += ft * v.avelocity;
        v.origin += ft * v.velocity;
        self.link_edict(e, false);
    }

    /// `SV_CheckWaterTransition`: the splash when a thrown thing crosses into or out of water.
    fn check_water_transition(&mut self, e: Ent) {
        let cont = self.point_contents(self.ents[e].v.origin);
        if self.ents[e].v.watertype == 0 {
            self.ents[e].v.watertype = cont; // just spawned here
            self.ents[e].v.waterlevel = 1;
            return;
        }
        if cont <= CONTENT_WATER {
            if self.ents[e].v.watertype == CONTENT_EMPTY {
                self.engine_sound(e, "misc/h2ohit1.wav");
            }
            self.ents[e].v.watertype = cont;
            self.ents[e].v.waterlevel = 1;
        } else {
            if self.ents[e].v.watertype != CONTENT_EMPTY {
                self.engine_sound(e, "misc/h2ohit1.wav");
            }
            self.ents[e].v.watertype = CONTENT_EMPTY;
            // As in the original: the contents value itself, not a depth.
            self.ents[e].v.waterlevel = cont;
        }
    }

    /// A sound the engine itself starts (`SV_StartSound (ent, 0, …, 255, 1)`).
    fn engine_sound(&mut self, e: Ent, sample: &'static str) {
        let s = crate::intern(sample);
        self.sound(e, 0, s, 1.0, 1.0);
    }

    /// `SV_Physics_Toss`: thrown, bounced or flown, until it comes to rest on a floor.
    fn physics_toss(&mut self, e: Ent) {
        if !self.run_think(e) {
            return;
        }
        if self.ents[e].v.flags & FL_ONGROUND != 0 {
            return;
        }
        self.check_velocity(e);
        let mt = self.ents[e].v.movetype;
        if mt != MOVETYPE_FLY && mt != MOVETYPE_FLYMISSILE {
            self.add_gravity(e);
        }
        let ft = self.frame_seconds;
        let v = &mut self.ents[e].v;
        v.angles += ft * v.avelocity;
        let mv = v.velocity * ft;
        let trace = self.push_entity(e, mv);
        if trace.fraction == 1.0 || self.ents[e].free {
            return;
        }
        let backoff = if self.ents[e].v.movetype == MOVETYPE_BOUNCE { 1.5 } else { 1.0 };
        let v = &mut self.ents[e].v;
        v.velocity = clip_velocity(v.velocity, trace.normal, backoff).0;
        if trace.normal.z > 0.7 && (v.velocity.z < 60.0 || v.movetype != MOVETYPE_BOUNCE) {
            v.flags |= FL_ONGROUND;
            v.groundentity = trace.ent.unwrap_or(0);
            v.velocity = Vec3::ZERO;
            v.avelocity = Vec3::ZERO;
        }
        self.check_water_transition(e);
    }

    /// `SV_Physics_Step`: a monster falls if nothing holds it up, and otherwise moves only by its
    /// own steps.
    fn physics_step(&mut self, e: Ent) {
        if self.ents[e].v.flags & (FL_ONGROUND | FL_FLY | FL_SWIM) == 0 {
            let hitsound = self.ents[e].v.velocity.z < self.cvars.value("sv_gravity") * -0.1;
            self.add_gravity(e);
            self.check_velocity(e);
            let ft = self.frame_seconds;
            self.fly_move(e, ft);
            self.link_edict(e, true);
            if self.ents[e].v.flags & FL_ONGROUND != 0 && hitsound {
                self.engine_sound(e, "demon/dland2.wav");
            }
        }
        self.run_think(e);
        self.check_water_transition(e);
    }

    /// `SV_Physics`: one frame of the whole world.
    pub fn physics(&mut self) {
        self.self_ = 0;
        self.other = 0;
        self.time = self.sv_time as f32;
        crate::progs::StartFrame(self);
        let mut i = 0;
        while i < self.ents.len() {
            if self.ents[i].free {
                i += 1;
                continue;
            }
            if self.force_retouch != 0.0 {
                self.link_edict(i, true); // touched again even if it stands still
            }
            if i > 0 && i <= MAX_CLIENTS {
                self.physics_client(i);
            } else {
                match self.ents[i].v.movetype {
                    MOVETYPE_PUSH => self.physics_pusher(i),
                    MOVETYPE_NONE => {
                        self.run_think(i);
                    }
                    MOVETYPE_NOCLIP => self.physics_noclip(i),
                    MOVETYPE_STEP => self.physics_step(i),
                    MOVETYPE_TOSS | MOVETYPE_BOUNCE | MOVETYPE_FLY | MOVETYPE_FLYMISSILE => self.physics_toss(i),
                    m => panic!("SV_Physics: bad movetype {m}"),
                }
            }
            i += 1;
        }
        if self.force_retouch != 0.0 {
            self.force_retouch -= 1.0;
        }
        self.sv_time += self.frame_seconds as f64;
    }
}
