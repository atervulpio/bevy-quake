//! The player's side (`cl_*.c`, `view.c`): what the keys and the mouse ask for, the eye's place
//! and turn (`V_CalcRefdef`), and what the world shows this frame, moved smoothly between the
//! server's frames.

use crate::effects::Effects;
use crate::meshes::{Built, MeshRef, Pose};
use game::{Event, Game, MOVETYPE_STEP, UserCmd};
use glam::{Mat4, Vec3};
use render::{Draw, View, angle_vectors};

/// `cl_bob`, `cl_bobcycle`, `cl_bobup`: the eye rises and falls with the step.
const BOB: f32 = 0.02;
const BOB_CYCLE: f32 = 0.6;
const BOB_UP: f32 = 0.5;
/// `v_kicktime`, `v_kickroll`, `v_kickpitch`: a hit tilts the view for half a second.
const KICK_TIME: f32 = 0.5;
const KICK_ROLL: f32 = 0.6;
const KICK_PITCH: f32 = 0.6;

/// How a thing moves on the screen between two places the server gave it.
#[derive(Clone, Copy, Default)]
struct Motion {
    from: Vec3,
    to: Vec3,
    from_angles: Vec3,
    to_angles: Vec3,
    start: f32,
    seconds: f32,
    /// The poses it shows: the one it leaves, the one it reaches, and when it began to.
    pose_from: usize,
    pose_to: usize,
    pose_start: f32,
    frame: f32,
    model: f32,
}

/// A thing frozen into the scenery (`makestatic`).
pub struct Static {
    pub model: usize,
    pub frame: usize,
    pub skin: usize,
    pub origin: Vec3,
    pub angles: Vec3,
}

/// A moving light (`dlight_t`): where, how far it reaches in the original's measure, until when,
/// how fast it shrinks, its colour, and whose it is (a thing's light is replaced, not added).
#[derive(Clone, Copy)]
pub struct DLight {
    pub origin: Vec3,
    pub radius: f32,
    pub die: f32,
    pub decay: f32,
    pub colour: Vec3,
    pub key: usize,
}

/// Fire's colour, for flashes and explosions: the original's moving lights were white; light
/// from burning powder is not.
const FIRE: Vec3 = Vec3::new(1.0, 0.72, 0.42);

/// A colour laid over the whole screen (`cshift_t`): a percent of it, from 0 to 255.
#[derive(Clone, Copy, Default)]
pub struct Shift {
    pub colour: Vec3,
    pub percent: f32,
}

#[derive(Default)]
pub struct Client {
    pub viewangles: Vec3,
    /// The client's clock, `cl.time`.
    pub time: f32,
    motions: Vec<Motion>,
    pub statics: Vec<Static>,
    dmg_time: f32,
    dmg_roll: f32,
    dmg_pitch: f32,
    /// Contents, damage, bonus and power-up (`CSHIFT_*`).
    pub shifts: [Shift; 4],
    oldz: f32,
    pub intermission: bool,
    pub centre: Option<(String, f32)>,
    pub lines: Vec<(String, f32)>,
    /// The 64 light styles' patterns, a letter a tenth of a second (`cl_lightstyle`).
    pub lightstyles: Vec<String>,
    pub dlights: Vec<DLight>,
    rng: u32,
    pub effects: Effects,
    /// Sounds the player's side starts itself (`CL_ParseTEnt`), for the host to play.
    pub sounds: Vec<(&'static str, Vec3)>,
    pub palette: Option<data::Palette>,
    /// A line printed in pieces, until its end of line (`Con_Print`).
    partial: String,
    /// When the level was done (`cl.completed_time`), and the finale's text and when it began.
    pub completed: Option<f32>,
    pub finale: Option<(String, f32)>,
}

/// The sounds the player's side plays by itself (`CL_InitTEnts`).
pub const CLIENT_SOUNDS: [&str; 7] = ["wizard/hit.wav", "hknight/hit.wav", "weapons/tink1.wav", "weapons/ric1.wav", "weapons/ric2.wav", "weapons/ric3.wav", "weapons/r_exp3.wav"];

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let mut d = b - a;
    if d > 180.0 {
        d -= 360.0;
    } else if d < -180.0 {
        d += 360.0;
    }
    a + d * t
}

/// The transform of a thing at a place with its angles, as `R_RotateForEntity` turns it: yaw, then
/// the pitch negated about the side axis, then roll. A model's pitch lifts its nose (QuakeC's
/// `vectoangles` gives pitch up, and the weapon in hand is given the view's pitch negated); a
/// brush model's angle is negated first (`R_DrawBrushModel`'s "stupid quake bug"), so its pitch
/// lowers it, as the view's does.
fn transform(origin: Vec3, angles: Vec3, model_pitch: bool) -> Mat4 {
    let pitch = if model_pitch { angles.x } else { -angles.x };
    Mat4::from_translation(origin) * Mat4::from_rotation_z(angles.y.to_radians()) * Mat4::from_rotation_y((-pitch).to_radians()) * Mat4::from_rotation_x(angles.z.to_radians())
}

impl Client {
    /// The command for this frame from what is held: always running (`cl_forwardspeed` 400,
    /// `cl_backspeed` 400, `cl_sidespeed` 350, `cl_upspeed` 200; the server caps a walk at 320).
    pub fn command(&self, forward: f32, side: f32, up: f32, attack: bool, jump: bool, impulse: i32) -> UserCmd {
        UserCmd {
            viewangles: self.viewangles,
            forwardmove: forward * 400.0,
            sidemove: side * 350.0,
            upmove: up * 200.0,
            buttons: attack as i32 | ((jump as i32) << 1),
            impulse,
        }
    }

    /// The mouse turns the view: `sensitivity` 3 times `m_yaw` and `m_pitch` 0.022 degrees a count,
    /// the pitch held within the original's 80 degrees each way (`cl_pitchdriftspeed` aside).
    pub fn turn(&mut self, dx: f32, dy: f32) {
        let s = 3.0 * 0.022;
        self.viewangles.y -= dx * s;
        self.viewangles.x = (self.viewangles.x + dy * s).clamp(-70.0, 80.0);
    }

    /// What the server said this frame, for the player's side.
    pub fn take_events(&mut self, g: &mut Game) {
        for ev in g.events.drain(..) {
            match ev {
                Event::SetAngle(a) => self.viewangles = a,
                Event::Damage { armor, blood, from } => {
                    // V_ParseDamage
                    let count = (blood * 0.5 + armor * 0.5).max(10.0);
                    let s = &mut self.shifts[1];
                    s.percent = (s.percent + 3.0 * count).clamp(0.0, 150.0);
                    s.colour = if armor > blood {
                        Vec3::new(200.0, 100.0, 100.0)
                    } else if armor != 0.0 {
                        Vec3::new(220.0, 50.0, 50.0)
                    } else {
                        Vec3::new(255.0, 0.0, 0.0)
                    };
                    let p = &g.ents[1].v;
                    let dir = (from - p.origin).normalize_or_zero();
                    let (forward, right, _) = angle_vectors(p.angles);
                    self.dmg_roll = count * dir.dot(right) * KICK_ROLL;
                    self.dmg_pitch = count * dir.dot(forward) * KICK_PITCH;
                    self.dmg_time = KICK_TIME;
                }
                Event::BonusFlash => self.shifts[2] = Shift { colour: Vec3::new(215.0, 186.0, 69.0), percent: 50.0 },
                Event::Static { model, frame, skin, origin, angles } => {
                    if let Some(i) = g.models.iter().position(|m| m.name == model) {
                        self.statics.push(Static { model: i, frame: frame as usize, skin: skin.max(0.0) as usize, origin, angles });
                    }
                }
                Event::Intermission => {
                    self.intermission = true;
                    self.completed = Some(self.time);
                }
                Event::Finale(text) => {
                    self.intermission = true;
                    self.completed.get_or_insert(self.time);
                    self.finale = Some((text, self.time));
                }
                Event::CenterPrint(s) => self.centre = Some((s, self.time + crate::status::CENTRE_SECONDS)),
                Event::Print(s) => {
                    self.partial.push_str(&s);
                    while let Some(i) = self.partial.find('\n') {
                        let line: String = self.partial.drain(..=i).collect();
                        self.lines.push((line, self.time));
                    }
                    if self.lines.len() > 32 {
                        self.lines.drain(..self.lines.len() - 32);
                    }
                }
                Event::MuzzleFlash(i) => {
                    // CL_RelinkEntities: 16 up and 18 ahead of the thing, 200 to 231 out, a tenth
                    let v = &g.ents[i].v;
                    let (f, _, _) = angle_vectors(v.angles);
                    let r = 200.0 + self.rand31();
                    self.dlight(DLight { origin: v.origin + Vec3::new(0.0, 0.0, 16.0) + f * 18.0, radius: r, die: self.time + 0.1, decay: 0.0, colour: FIRE, key: i });
                }
                Event::TempEntity(t) => self.temp_entity(t),
                Event::Particle { origin, dir, colour, count } => self.effects.run(origin, dir, colour, count, self.time),
                Event::LightStyle(i, s) => {
                    if self.lightstyles.len() < 64 {
                        self.lightstyles.resize(64, String::new());
                    }
                    self.lightstyles[i] = s.to_string();
                }
                _ => {}
            }
        }
    }

    /// After each server frame: where each thing now is, so the frames between move it there
    /// smoothly. A walking monster steps ten times a second, so its step is spread over a tenth
    /// of a second; a jump over 100 units (a teleport) is not smoothed.
    pub fn after_tick(&mut self, g: &Game, built: &Built, tick: f32) {
        if self.motions.len() < g.ents.len() {
            self.motions.resize(g.ents.len(), Motion::default());
        }
        let now = self.time;
        for (i, e) in g.ents.iter().enumerate() {
            let m = &mut self.motions[i];
            let v = &e.v;
            if e.free || v.modelindex != m.model {
                *m = Motion { from: v.origin, to: v.origin, from_angles: v.angles, to_angles: v.angles, start: now, seconds: tick, model: v.modelindex, frame: v.frame, ..Default::default() };
                if let Some(MeshRef::Alias(a)) = built.refs.get(v.modelindex as usize) {
                    let p = built.frames[*a].pose_of(v.frame as usize, now, i);
                    m.pose_from = p;
                    m.pose_to = p;
                }
                continue;
            }
            if v.origin != m.to {
                // CL_RelinkEntities: the trail a model's flags ask for, from where it was
                if let Some(MeshRef::Alias(a)) = built.refs.get(v.modelindex as usize) {
                    let f = built.frames[*a].flags;
                    let kind = if f & 4 != 0 {
                        Some(2) // EF_GIB
                    } else if f & 32 != 0 {
                        Some(4) // EF_ZOMGIB
                    } else if f & 16 != 0 {
                        Some(3) // EF_TRACER
                    } else if f & 64 != 0 {
                        Some(5) // EF_TRACER2
                    } else if f & 1 != 0 {
                        Some(0) // EF_ROCKET
                    } else if f & 2 != 0 {
                        Some(1) // EF_GRENADE
                    } else if f & 128 != 0 {
                        Some(6) // EF_TRACER3
                    } else {
                        None
                    };
                    if let Some(k) = kind
                        && (v.origin - m.to).length() < 100.0 {
                            self.effects.trail(m.to, v.origin, k, now);
                        }
                }
            }
            let m = &mut self.motions[i];
            if v.origin != m.to || v.angles != m.to_angles {
                let t = ((now - m.start) / m.seconds).clamp(0.0, 1.0);
                let shown = m.from.lerp(m.to, t);
                let jump = (v.origin - m.to).length() > 100.0;
                m.from = if jump { v.origin } else { shown };
                m.from_angles = Vec3::new(lerp_angle(m.from_angles.x, m.to_angles.x, t), lerp_angle(m.from_angles.y, m.to_angles.y, t), lerp_angle(m.from_angles.z, m.to_angles.z, t));
                m.to = v.origin;
                m.to_angles = v.angles;
                m.start = now;
                m.seconds = if v.movetype == MOVETYPE_STEP { 0.1 } else { tick };
            }
            if v.frame != m.frame {
                if let Some(MeshRef::Alias(a)) = built.refs.get(v.modelindex as usize) {
                    m.pose_from = m.pose_to;
                    m.pose_to = built.frames[*a].pose_of(v.frame as usize, now, i);
                    m.pose_start = now;
                }
                m.frame = v.frame;
            }
        }
    }

    /// A spike's spark and its sound: a tink four times in five, else one of three ricochets
    /// (`CL_ParseTEnt`).
    fn spike_sound(&mut self, pos: Vec3) {
        if self.rand31() as i32 % 5 != 0 {
            self.sounds.push(("weapons/tink1.wav", pos));
        } else {
            let r = self.rand31() as i32 & 3;
            let s = match r {
                1 => "weapons/ric1.wav",
                2 => "weapons/ric2.wav",
                _ => "weapons/ric3.wav",
            };
            self.sounds.push((s, pos));
        }
    }

    /// `CL_ParseTEnt`: each kind's particles, light and sound.
    fn temp_entity(&mut self, t: game::TempEntity) {
        use game::TempEntity as T;
        let now = self.time;
        match t {
            T::WizSpike(p) => {
                self.effects.run(p, Vec3::ZERO, 20, 30, now);
                self.sounds.push(("wizard/hit.wav", p));
            }
            T::KnightSpike(p) => {
                self.effects.run(p, Vec3::ZERO, 226, 20, now);
                self.sounds.push(("hknight/hit.wav", p));
            }
            T::Spike(p) => {
                self.effects.run(p, Vec3::ZERO, 0, 10, now);
                self.spike_sound(p);
            }
            T::SuperSpike(p) => {
                self.effects.run(p, Vec3::ZERO, 0, 20, now);
                self.spike_sound(p);
            }
            T::Gunshot(p) => self.effects.run(p, Vec3::ZERO, 0, 20, now),
            T::Explosion(p) => {
                self.effects.explosion(p, now);
                // its light: 350 out, half a second, shrinking at 300 a second
                self.dlight(DLight { origin: p, radius: 350.0, die: now + 0.5, decay: 300.0, colour: FIRE, key: 0 });
                self.sounds.push(("weapons/r_exp3.wav", p));
            }
            T::TarExplosion(p) => {
                self.effects.blob_explosion(p, now);
                self.sounds.push(("weapons/r_exp3.wav", p));
            }
            T::Lightning1 { ent, start, end } => self.effects.beam(ent, "progs/bolt.mdl", start, end, now),
            T::Lightning2 { ent, start, end } => self.effects.beam(ent, "progs/bolt2.mdl", start, end, now),
            T::Lightning3 { ent, start, end } => self.effects.beam(ent, "progs/bolt3.mdl", start, end, now),
            T::LavaSplash(p) => self.effects.lava_splash(p, now),
            T::Teleport(p) => self.effects.teleport_splash(p, now),
        }
    }

    /// `CL_AllocDlight`: a thing's light replaces its last; a light of no thing takes a free
    /// place, or the oldest's.
    fn dlight(&mut self, d: DLight) {
        if d.key != 0
            && let Some(slot) = self.dlights.iter_mut().find(|x| x.key == d.key) {
                *slot = d;
                return;
            }
        if self.dlights.len() < render::MAX_LIGHTS {
            self.dlights.push(d);
        } else if let Some(slot) = self.dlights.iter_mut().min_by(|a, b| a.die.total_cmp(&b.die)) {
            *slot = d;
        }
    }

    fn rand31(&mut self) -> f32 {
        self.rng = self.rng.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.rng >> 16) & 31) as f32
    }

    /// The lights things carry this frame (`CL_RelinkEntities`): a muzzle's flash, a bright or dim
    /// glow, a rocket's flame.
    fn things_lights(&mut self, g: &Game, built: &Built) {
        for (i, e) in g.ents.iter().enumerate().skip(1) {
            // a thing with no model's name is not drawn, as `SV_WriteEntitiesToClient` sends no
            // such thing (an item taken keeps its model's number, its name cleared)
            if e.free || e.v.modelindex == 0.0 || e.v.model.is_empty() || i >= self.motions.len() {
                continue;
            }
            let (o, _, _) = self.placed(i);
            let fx = e.v.effects;
            if fx & game::EF_BRIGHTLIGHT != 0 {
                let r = 400.0 + self.rand31();
                self.dlight(DLight { origin: o + Vec3::new(0.0, 0.0, 16.0), radius: r, die: self.time + 0.001, decay: 0.0, colour: Vec3::ONE, key: i });
            }
            if fx & game::EF_DIMLIGHT != 0 {
                let r = 200.0 + self.rand31();
                self.dlight(DLight { origin: o, radius: r, die: self.time + 0.001, decay: 0.0, colour: Vec3::ONE, key: i });
            }
            if let Some(MeshRef::Alias(m)) = built.refs.get(e.v.modelindex as usize)
                && built.frames[*m].flags & 1 != 0 {
                    // EF_ROCKET: the rocket's own flame
                    self.dlight(DLight { origin: o, radius: 200.0, die: self.time + 0.01, decay: 0.0, colour: FIRE, key: i });
                }
        }
    }

    fn placed(&self, i: usize) -> (Vec3, Vec3, f32) {
        let m = &self.motions[i];
        let t = ((self.time - m.start) / m.seconds).clamp(0.0, 1.0);
        let a = Vec3::new(lerp_angle(m.from_angles.x, m.to_angles.x, t), lerp_angle(m.from_angles.y, m.to_angles.y, t), lerp_angle(m.from_angles.z, m.to_angles.z, t));
        (m.from.lerp(m.to, t), a, ((self.time - m.pose_start) / 0.1).clamp(0.0, 1.0))
    }

    /// `V_CalcBob`.
    fn bob(&self, velocity: Vec3) -> f32 {
        let mut cycle = self.time - (self.time / BOB_CYCLE).floor() * BOB_CYCLE;
        cycle /= BOB_CYCLE;
        cycle = if cycle < BOB_UP { std::f32::consts::PI * cycle / BOB_UP } else { std::f32::consts::PI + std::f32::consts::PI * (cycle - BOB_UP) / (1.0 - BOB_UP) };
        let bob = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt() * BOB;
        (bob * 0.3 + bob * 0.7 * cycle.sin()).clamp(-7.0, 4.0)
    }

    /// The eye (`V_CalcRefdef`) and everything to draw.
    pub fn frame(&mut self, g: &Game, built: &Built, frame_seconds: f32, screen: (u32, u32)) -> (View, Vec<Draw>) {
        let p = &g.ents[1].v;
        let (origin, _, _) = if self.motions.len() > 1 { self.placed(1) } else { (p.origin, p.angles, 0.0) };
        let mut draws = vec![Draw::Brush { mesh: 0, transform: Mat4::IDENTITY, alternate: false }];
        let mut view = View { origin, angles: self.viewangles, fov: 90.0, time: self.time, styles: self.style_values().to_vec(), exposure: 1.0, crt: true, ..Default::default() };
        if self.intermission {
            view.origin = p.origin;
            view.angles = p.angles;
        } else {
            let bob = self.bob(p.velocity);
            view.origin.z += p.view_ofs.z + bob;
            view.origin += Vec3::splat(1.0 / 32.0);
            // V_CalcViewRoll: the lean when strafing, a hit's kick, and the dead's tilt
            view.angles.z += game::user::calc_roll(p.angles, p.velocity);
            if self.dmg_time > 0.0 {
                view.angles.z += self.dmg_time / KICK_TIME * self.dmg_roll;
                view.angles.x += self.dmg_time / KICK_TIME * self.dmg_pitch;
                self.dmg_time -= frame_seconds;
            }
            if p.health <= 0.0 {
                view.angles.z = 80.0;
            }
            // V_BoundOffsets
            view.origin.x = view.origin.x.clamp(origin.x - 14.0, origin.x + 14.0);
            view.origin.y = view.origin.y.clamp(origin.y - 14.0, origin.y + 14.0);
            view.origin.z = view.origin.z.clamp(origin.z - 22.0, origin.z + 30.0);
            // the weapon in hand, bobbing with the step, two units up at the full screen
            let (forward, _, _) = angle_vectors(Vec3::new(self.viewangles.x, self.viewangles.y, 0.0));
            let mut gun = origin + Vec3::new(0.0, 0.0, p.view_ofs.z) + forward * bob * 0.4;
            gun.z += bob + 2.0;
            view.angles += p.punchangle;
            // smooth the climb up a stair: the eye rises at most 80 units a second behind the body
            if p.flags & game::FL_ONGROUND != 0 && origin.z - self.oldz > 0.0 {
                self.oldz = (self.oldz + frame_seconds * 80.0).min(origin.z).max(origin.z - 12.0);
                let lag = self.oldz - origin.z;
                view.origin.z += lag;
                gun.z += lag;
            } else {
                self.oldz = origin.z;
            }
            if let Some(&wm) = built.by_name.get(p.weaponmodel)
                && let Some(MeshRef::Alias(a)) = built.refs.get(wm) {
                    let pose = built.frames[*a].pose(p.weaponframe as usize, self.time);
                    let ga = Vec3::new(-self.viewangles.x, self.viewangles.y, 0.0);
                    let first = draws.len();
                    draws.push(built.model(*a, transform(gun, ga, true), Pose::still(pose), 0));
                    view.in_hand = first..draws.len();
                }
        }
        // EF_ROTATE: pick-ups turn a hundred degrees a second, all together (`bobjrotate`)
        let spin = (100.0 * self.time) % 360.0;
        for (i, e) in g.ents.iter().enumerate().skip(1) {
            if e.free || e.v.modelindex == 0.0 || e.v.model.is_empty() || i == 1 || i >= self.motions.len() {
                continue;
            }
            let (o, mut a, t) = self.placed(i);
            match built.refs.get(e.v.modelindex as usize) {
                Some(MeshRef::Brush(b)) => draws.push(Draw::Brush { mesh: *b, transform: transform(o, a, false), alternate: e.v.frame != 0.0 }),
                Some(MeshRef::Alias(m)) => {
                    let mo = &self.motions[i];
                    if built.frames[*m].flags & 8 != 0 {
                        a.y = spin;
                    }
                    draws.push(built.model(*m, transform(o, a, true), Pose { from: mo.pose_from, to: mo.pose_to, t }, e.v.skin.max(0.0) as usize));
                }
                Some(r @ MeshRef::Sprite { .. }) => view.sprites.extend(r.sprite(o, e.v.frame)),
                _ => {}
            }
        }
        for (n, s) in self.statics.iter().enumerate() {
            match built.refs.get(s.model) {
                Some(MeshRef::Alias(m)) => {
                    let pose = built.frames[*m].pose_of(s.frame, self.time, 1000 + n);
                    draws.push(built.model(*m, transform(s.origin, s.angles, true), Pose::still(pose), s.skin));
                }
                Some(r @ MeshRef::Sprite { .. }) => view.sprites.extend(r.sprite(s.origin, s.frame as f32)),
                _ => {}
            }
        }
        // the lightning as id draws it (`CL_UpdateTEnts`, row 39): its bolt model laid end to end
        // from the beam's start every thirty units, each turned along the beam and rolled at random
        let player = g.ents[1].v.origin;
        for (model, a, b) in self.effects.beam_ends(self.time, player) {
            let Some(MeshRef::Alias(m)) = built.by_name.get(model).and_then(|&i| built.refs.get(i)) else { continue };
            let dist = b - a;
            let (yaw, pitch) = if dist.x == 0.0 && dist.y == 0.0 {
                (0.0, if dist.z > 0.0 { 90.0 } else { 270.0 })
            } else {
                let yaw = dist.y.atan2(dist.x).to_degrees().rem_euclid(360.0);
                let pitch = dist.z.atan2((dist.x * dist.x + dist.y * dist.y).sqrt()).to_degrees().rem_euclid(360.0);
                (yaw, pitch)
            };
            let length = dist.length();
            let along = dist / length.max(1.0e-6);
            let mut d = length;
            let mut org = a;
            while d > 0.0 {
                self.rng = self.rng.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                let roll = ((self.rng >> 16) % 360) as f32;
                draws.push(built.model(*m, transform(org, Vec3::new(pitch, yaw, roll), true), Pose::still(0), 0));
                org += along * 30.0;
                d -= 30.0;
            }
        }
        self.effects.step(frame_seconds, self.time);
        if let Some(pal) = &self.palette {
            let (f, _, _) = angle_vectors(view.angles);
            view.dots = self
                .effects
                .dots(pal, view.origin, f)
                .into_iter()
                .map(|d| render::Dot { place: d.origin, size: d.size, colour: d.colour, glows: d.glow > 0.0 })
                .collect();
        }
        view.blend = self.blend(g, view.origin, frame_seconds);
        // what the screen says
        let mut st = crate::status::Status { w: screen.0 as f32, h: screen.1 as f32, texts: &mut view.texts };
        if let Some((text, since)) = &self.finale {
            st.finale(text, self.time - since);
        } else if self.intermission {
            st.tally(g, self.completed.unwrap_or(self.time));
        } else {
            st.bar(g);
            if g.ents[1].v.health > 0.0 {
                st.sight();
            }
            if let Some((text, until)) = &self.centre
                && self.time < *until {
                    st.centre(text);
                }
        }
        st.notify(&self.lines, self.time);
        // CL_DecayLights: each light shrinks by its decay and goes when it dies
        self.things_lights(g, built);
        let now = self.time;
        self.dlights.retain_mut(|d| {
            d.radius -= frame_seconds * d.decay;
            d.die >= now && d.radius > 0.0
        });
        view.lights = self.dlights.iter().map(|d| (d.origin, d.radius.powi(3) / 2048.0, d.colour)).collect();
        (view, draws)
    }

    /// Each style's brightness now (`R_AnimateLight`): its pattern's letter at ten a second, `a`
    /// dark, `m` the normal light, `z` twice that (`(letter − 'a') × 22 / 256`); an empty
    /// pattern is the normal light.
    fn style_values(&self) -> [f32; 64] {
        let step = (self.time * 10.0) as usize;
        std::array::from_fn(|i| match self.lightstyles.get(i).map(|s| s.as_bytes()) {
            Some(p) if !p.is_empty() => (p[step % p.len()] as f32 - b'a' as f32) * 22.0 / 256.0,
            _ => 1.0,
        })
    }

    /// The colour laid over the screen (`V_UpdatePalette`, `V_CalcBlend`): the liquid the eye is
    /// in, a hit's red and a pick-up's gold dying away, a power-up's tint.
    fn blend(&mut self, g: &Game, eye: Vec3, seconds: f32) -> glam::Vec4 {
        let c = g.point_contents(eye);
        self.shifts[0] = match c {
            game::CONTENT_LAVA => Shift { colour: Vec3::new(255.0, 80.0, 0.0), percent: 150.0 },
            game::CONTENT_SLIME => Shift { colour: Vec3::new(0.0, 25.0, 5.0), percent: 150.0 },
            game::CONTENT_WATER => Shift { colour: Vec3::new(130.0, 80.0, 50.0), percent: 128.0 },
            _ => Shift { colour: Vec3::new(130.0, 80.0, 50.0), percent: 0.0 },
        };
        let items = g.ents[1].v.items;
        self.shifts[3] = if items & game::IT_QUAD != 0 {
            Shift { colour: Vec3::new(0.0, 0.0, 255.0), percent: 30.0 }
        } else if items & game::IT_SUIT != 0 {
            Shift { colour: Vec3::new(0.0, 255.0, 0.0), percent: 20.0 }
        } else if items & game::IT_INVISIBILITY != 0 {
            Shift { colour: Vec3::new(100.0, 100.0, 100.0), percent: 100.0 }
        } else if items & game::IT_INVULNERABILITY != 0 {
            Shift { colour: Vec3::new(255.0, 255.0, 0.0), percent: 30.0 }
        } else {
            Shift { colour: Vec3::ZERO, percent: 0.0 }
        };
        self.shifts[1].percent = (self.shifts[1].percent - seconds * 150.0).max(0.0);
        self.shifts[2].percent = (self.shifts[2].percent - seconds * 100.0).max(0.0);
        let (mut rgb, mut a) = (Vec3::ZERO, 0.0f32);
        for s in &self.shifts {
            let a2 = s.percent / 255.0;
            if a2 == 0.0 {
                continue;
            }
            a += a2 * (1.0 - a);
            let w = a2 / a;
            rgb = rgb * (1.0 - w) + s.colour * w;
        }
        // the palette's colours are the screen's, so the blend is laid in the screen's own values
        let linear = (rgb / 255.0).powf(2.2);
        linear.extend(a.clamp(0.0, 1.0))
    }
}
