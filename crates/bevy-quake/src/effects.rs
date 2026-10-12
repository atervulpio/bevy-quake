//! The effects the player's side makes by itself (`r_part.c`, `cl_tent.c`): particles of
//! explosions, blood, sparks and trails, and the lightning's beams, as the original made and
//! moved them; drawn as small soft discs lit by the level's light (fire glowing by itself).

use glam::Vec3;

/// `MAX_PARTICLES`'s default: the most particles at once.
const MAX_PARTICLES: usize = 2048;

const RAMP1: [u8; 8] = [0x6f, 0x6d, 0x6b, 0x69, 0x67, 0x65, 0x63, 0x61];
const RAMP2: [u8; 8] = [0x6f, 0x6e, 0x6d, 0x6c, 0x6b, 0x6a, 0x68, 0x66];
const RAMP3: [u8; 6] = [0x6d, 0x6b, 6, 5, 4, 3];

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Static,
    Fire,
    Explode,
    Explode2,
    Blob,
    Blob2,
    SlowGrav,
}

#[derive(Clone, Copy)]
struct Particle {
    org: Vec3,
    vel: Vec3,
    colour: u8,
    ramp: f32,
    die: f32,
    kind: Kind,
}

/// A lightning beam (`beam_t`): from a thing to where it struck, for a fifth of a second.
#[derive(Clone, Copy)]
pub struct Beam {
    pub ent: usize,
    pub model: &'static str,
    pub start: Vec3,
    pub end: Vec3,
    pub end_time: f32,
}

/// A particle as the card draws it: where, how large, its colour (linear) and whether it gives
/// off its own light.
#[derive(Clone, Copy)]
pub struct Dot {
    pub origin: Vec3,
    pub size: f32,
    pub colour: Vec3,
    pub glow: f32,
}

#[derive(Default)]
pub struct Effects {
    particles: Vec<Particle>,
    pub beams: Vec<Beam>,
    rng: u32,
    tracers: u32,
}

impl Effects {
    /// C's `rand()`, fifteen bits.
    fn rand(&mut self) -> i32 {
        self.rng = self.rng.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.rng >> 16) & 0x7fff) as i32
    }

    fn add(&mut self, p: Particle) {
        if self.particles.len() < MAX_PARTICLES {
            self.particles.push(p);
        }
    }

    fn jitter(&mut self, half: i32) -> Vec3 {
        Vec3::new((self.rand() % (2 * half) - half) as f32, (self.rand() % (2 * half) - half) as f32, (self.rand() % (2 * half) - half) as f32)
    }

    /// `R_ParticleExplosion`: 1,024 particles of fire flung out, half slowing, half spreading.
    pub fn explosion(&mut self, org: Vec3, now: f32) {
        for i in 0..1024 {
            let ramp = (self.rand() & 3) as f32;
            let o = org + self.jitter(16);
            let v = Vec3::new((self.rand() % 512 - 256) as f32, (self.rand() % 512 - 256) as f32, (self.rand() % 512 - 256) as f32);
            self.add(Particle { org: o, vel: v, colour: RAMP1[0], ramp, die: now + 5.0, kind: if i & 1 != 0 { Kind::Explode } else { Kind::Explode2 } });
        }
    }

    /// `R_BlobExplosion`: the spawn's burst, purple and blue.
    pub fn blob_explosion(&mut self, org: Vec3, now: f32) {
        for i in 0..1024 {
            let die = now + 1.0 + (self.rand() & 8) as f32 * 0.05;
            let (kind, colour) = if i & 1 != 0 { (Kind::Blob, 66 + self.rand() % 6) } else { (Kind::Blob2, 150 + self.rand() % 6) };
            let o = org + self.jitter(16);
            let v = Vec3::new((self.rand() % 512 - 256) as f32, (self.rand() % 512 - 256) as f32, (self.rand() % 512 - 256) as f32);
            self.add(Particle { org: o, vel: v, colour: colour as u8, ramp: 0.0, die, kind });
        }
    }

    /// `R_RunParticleEffect`: `count` particles of one colour's eight shades, drifting along `dir`.
    pub fn run(&mut self, org: Vec3, dir: Vec3, colour: i32, count: i32, now: f32) {
        if count == 1024 {
            self.explosion(org, now);
            return;
        }
        for _ in 0..count {
            let die = now + 0.1 * (self.rand() % 5) as f32;
            let c = ((colour & !7) + (self.rand() & 7)) as u8;
            let o = org + Vec3::new(((self.rand() & 15) - 8) as f32, ((self.rand() & 15) - 8) as f32, ((self.rand() & 15) - 8) as f32);
            self.add(Particle { org: o, vel: dir * 15.0, colour: c, ramp: 0.0, die, kind: Kind::SlowGrav });
        }
    }

    /// `R_LavaSplash`: a ring of lava's glow thrown up.
    pub fn lava_splash(&mut self, org: Vec3, now: f32) {
        for i in -16..16 {
            for j in -16..16 {
                let die = now + 2.0 + (self.rand() & 31) as f32 * 0.02;
                let colour = (224 + (self.rand() & 7)) as u8;
                let dir = Vec3::new((j * 8 + (self.rand() & 7)) as f32, (i * 8 + (self.rand() & 7)) as f32, 256.0);
                let o = org + Vec3::new(dir.x, dir.y, (self.rand() & 63) as f32);
                let vel = 50.0 + (self.rand() & 63) as f32;
                self.add(Particle { org: o, vel: dir.normalize() * vel, colour, ramp: 0.0, die, kind: Kind::SlowGrav });
            }
        }
    }

    /// `R_TeleportSplash`: a column of grey sparks bursting from the arrival.
    pub fn teleport_splash(&mut self, org: Vec3, now: f32) {
        for i in (-16..16).step_by(4) {
            for j in (-16..16).step_by(4) {
                for k in (-24..32).step_by(4) {
                    let die = now + 0.2 + (self.rand() & 7) as f32 * 0.02;
                    let colour = (7 + (self.rand() & 7)) as u8;
                    let dir = Vec3::new((j * 8) as f32, (i * 8) as f32, (k * 8) as f32);
                    let o = org + Vec3::new((i + (self.rand() & 3)) as f32, (j + (self.rand() & 3)) as f32, (k + (self.rand() & 3)) as f32);
                    let vel = 50.0 + (self.rand() & 63) as f32;
                    self.add(Particle { org: o, vel: dir.normalize_or_zero() * vel, colour, ramp: 0.0, die, kind: Kind::SlowGrav });
                }
            }
        }
    }

    /// `R_RocketTrail`: a trail from where a thing was to where it is: fire (0), smoke (1), blood
    /// (2), a tracer (3, 5), a little blood (4), the vore's trail (6).
    pub fn trail(&mut self, mut start: Vec3, end: Vec3, kind: i32, now: f32) {
        let mut vec = end - start;
        let mut len = vec.length();
        if len == 0.0 {
            return;
        }
        vec /= len;
        let dec = 3.0;
        while len > 0.0 {
            len -= dec;
            let mut p = Particle { org: start, vel: Vec3::ZERO, colour: 0, ramp: 0.0, die: now + 2.0, kind: Kind::Static };
            let j = |s: &mut Self| Vec3::new((s.rand() % 6 - 3) as f32, (s.rand() % 6 - 3) as f32, (s.rand() % 6 - 3) as f32);
            match kind {
                0 | 1 => {
                    p.ramp = (self.rand() & 3) as f32 + if kind == 1 { 2.0 } else { 0.0 };
                    p.colour = RAMP3[p.ramp as usize];
                    p.kind = Kind::Fire;
                    p.org = start + j(self);
                }
                2 | 4 => {
                    p.kind = Kind::SlowGrav; // pt_grav falls as pt_slowgrav in Quake 1
                    p.colour = (67 + (self.rand() & 3)) as u8;
                    p.org = start + j(self);
                    if kind == 4 {
                        len -= 3.0;
                    }
                }
                3 | 5 => {
                    p.die = now + 0.5;
                    p.colour = if kind == 3 { 52 + ((self.tracers & 4) << 1) as u8 } else { 230 + ((self.tracers & 4) << 1) as u8 };
                    self.tracers += 1;
                    p.vel = if self.tracers & 1 != 0 { Vec3::new(30.0 * vec.y, 30.0 * -vec.x, 0.0) } else { Vec3::new(30.0 * -vec.y, 30.0 * vec.x, 0.0) };
                }
                _ => {
                    p.colour = (9 * 16 + 8 + (self.rand() & 3)) as u8;
                    p.die = now + 0.3;
                    p.org = start + Vec3::new(((self.rand() & 15) - 8) as f32, ((self.rand() & 15) - 8) as f32, ((self.rand() & 15) - 8) as f32);
                }
            }
            self.add(p);
            start += vec;
        }
    }

    /// `CL_ParseBeam`: a beam from the same thing replaces its last.
    pub fn beam(&mut self, ent: usize, model: &'static str, start: Vec3, end: Vec3, now: f32) {
        let b = Beam { ent, model, start, end, end_time: now + 0.2 };
        if let Some(old) = self.beams.iter_mut().find(|x| x.ent == ent) {
            *old = b;
        } else {
            self.beams.push(b);
        }
    }

    /// The beams this frame (`CL_UpdateTEnts`), each whole from its start to where it struck (row
    /// 26: the card draws the lightning along it, where the original laid its model every 30
    /// units). A beam from the player starts where the player now is.
    pub fn beam_ends(&mut self, now: f32, player: Vec3) -> Vec<(&'static str, Vec3, Vec3)> {
        self.beams.retain(|b| b.end_time >= now);
        self.beams.iter().map(|b| (b.model, if b.ent == 1 { player } else { b.start }, b.end)).collect()
    }

    /// `R_DrawParticles`' motion: each particle moved, its colour stepped along its ramp, fire
    /// rising and the rest falling at a twentieth of gravity.
    pub fn step(&mut self, seconds: f32, now: f32) {
        let time3 = seconds * 15.0;
        let time2 = seconds * 10.0;
        let time1 = seconds * 5.0;
        let grav = seconds * 800.0 * 0.05;
        let dvel = 4.0 * seconds;
        for p in &mut self.particles {
            p.org += p.vel * seconds;
            match p.kind {
                Kind::Static => {}
                Kind::Fire => {
                    p.ramp += time1;
                    if p.ramp >= 6.0 {
                        p.die = -1.0;
                    } else {
                        p.colour = RAMP3[p.ramp as usize];
                    }
                    p.vel.z += grav;
                }
                Kind::Explode => {
                    p.ramp += time2;
                    if p.ramp >= 8.0 {
                        p.die = -1.0;
                    } else {
                        p.colour = RAMP1[p.ramp as usize];
                    }
                    p.vel += p.vel * dvel;
                    p.vel.z -= grav;
                }
                Kind::Explode2 => {
                    p.ramp += time3;
                    if p.ramp >= 8.0 {
                        p.die = -1.0;
                    } else {
                        p.colour = RAMP2[p.ramp as usize];
                    }
                    p.vel -= p.vel * seconds;
                    p.vel.z -= grav;
                }
                Kind::Blob => {
                    p.vel += p.vel * dvel;
                    p.vel.z -= grav;
                }
                Kind::Blob2 => {
                    p.vel.x -= p.vel.x * dvel;
                    p.vel.y -= p.vel.y * dvel;
                    p.vel.z -= grav;
                }
                Kind::SlowGrav => p.vel.z -= grav,
            }
        }
        self.particles.retain(|p| p.die >= now);
    }

    /// The particles as the card draws them, coloured from the palette. Fire, explosions and the
    /// palette's own bright colours (224 and up) give off light; the others are lit. A particle
    /// is about 1.5 units across, larger with distance as the original made it, so far ones do
    /// not vanish between pixels.
    pub fn dots(&self, palette: &data::Palette, eye: Vec3, forward: Vec3) -> Vec<Dot> {
        self.particles
            .iter()
            .map(|p| {
                let colour = palette.linear(p.colour);
                let glow = if matches!(p.kind, Kind::Fire | Kind::Explode | Kind::Explode2) || p.colour >= 224 { 1.0 } else { 0.0 };
                let depth = (p.org - eye).dot(forward);
                let size = 1.5 * if depth < 20.0 { 1.0 } else { 1.0 + depth * 0.004 };
                Dot { origin: p.org, size, colour, glow }
            })
            .collect()
    }
}
