//! The sound: the game's own sounds, cleaner (`clean.rs`), placed around the listener's head for
//! two ears, and mixed at the output's rate in floating point.
//!
//! **Where a sound is heard from** (row 40). As the original (`SND_Spatialize`): each ear takes a
//! share of the loudness by the cosine of the sound's direction to its side, `1 ± cos`, so a sound
//! to the right is loud in the right ear and gone from the left. Besides, the far ear hears it later
//! by the path round the head (Woodworth's `ITD = a/c · (θ + sin θ)`, up to 0.66 ms for a head of
//! radius 8.75 cm), and a sound from behind is heard duller in both ears, the outer ear shading its
//! highs (a first-order shelf, down to −6 dB straight behind). The head's model alone, the same
//! loudness in both ears (row 15), put every sound in the middle.
//!
//! **How loud.** The game's own law, as it was designed against: a sound fades in a straight line
//! to nothing at `1000 / attenuation` units (`dist_mult`), its volume set by the game.
//!
//! **What follows a thing.** The original kept a sound where it began; here a sound of a moving
//! thing goes with it.

pub mod clean;

use clean::Sound;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use glam::Vec3;
use std::sync::{Arc, Mutex};

/// A head's radius in world units: 8.75 cm, with the game's player 56 units for 1.8 m.
const HEAD_RADIUS: f32 = 0.0875 * 56.0 / 1.8;
/// The speed of sound in world units a second (343 m/s).
const SOUND_SPEED: f32 = 343.0 * 56.0 / 1.8;
/// `sound_nominal_clip_dist`: the distance an attenuation of 1 fades to nothing over.
const CLIP_DIST: f32 = 1000.0;
/// The original's `volume` 0.7 (`default.cfg`).
const VOLUME: f32 = 0.7;
/// Samples of a voice kept for its far ear's delay: more than 0.66 ms at 192 kHz.
const HISTORY: usize = 256;

/// The listener: where the ears are and which way is right.
#[derive(Clone, Copy, Default)]
pub struct Listener {
    pub origin: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    /// The entity the listener is (`cl.viewentity`): its own sounds are heard inside the head.
    pub ent: usize,
}

/// One ear's state for one voice: its share of the loudness, its delay now, and the shading
/// filter.
#[derive(Clone, Copy, Default)]
struct Ear {
    level: f32,
    delay: f32,
    b0: f32,
    b1: f32,
    a1: f32,
    x1: f32,
    y1: f32,
}

impl Ear {
    /// The shelf for a given `α` (Brown and Duda's form, `H(s) = (1 + α s / 2ω₀) / (1 + s / 2ω₀)`,
    /// `ω₀ = c / a`, by the bilinear transform at `rate`): 1 passes the sound unchanged, below 1
    /// dulls its highs.
    fn coefficients(&mut self, alpha: f32, rate: f32) {
        let w0 = SOUND_SPEED / HEAD_RADIUS;
        let k = 2.0 * rate;
        let d = 2.0 * w0 + k;
        self.b0 = (2.0 * w0 + alpha * k) / d;
        self.b1 = (2.0 * w0 - alpha * k) / d;
        self.a1 = (2.0 * w0 - k) / d;
    }

    fn filter(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 - self.a1 * self.y1;
        self.x1 = x;
        self.y1 = y;
        y
    }
}

/// A sound playing.
struct Voice {
    sound: usize,
    pos: usize,
    ent: usize,
    channel: i32,
    origin: Vec3,
    volume: f32,
    dist_mult: f32,
    /// Plays for ever (a looped static or ambient sound) rather than once.
    looping: bool,
    history: [f32; HISTORY],
    at: usize,
    ears: [Ear; 2],
    gain: f32,
    /// For an ambient channel: the level it is fading towards.
    ambient_target: Option<f32>,
}

impl Voice {
    fn new(sound: usize, ent: usize, channel: i32, origin: Vec3, volume: f32, dist_mult: f32, looping: bool) -> Voice {
        Voice { sound, pos: 0, ent, channel, origin, volume, dist_mult, looping, history: [0.0; HISTORY], at: 0, ears: [Ear::default(); 2], gain: 0.0, ambient_target: None }
    }
}

pub struct Mixer {
    rate: f32,
    sounds: Vec<Option<Arc<Sound>>>,
    voices: Vec<Voice>,
    listener: Listener,
    /// The limiter's follower of the mix's peak.
    envelope: f32,
}

/// At most this many sounds at once, the ambient and static ones besides; the original had 8.
const MAX_VOICES: usize = 64;

impl Mixer {
    fn new(rate: f32) -> Mixer {
        Mixer { rate, sounds: Vec::new(), voices: Vec::new(), listener: Listener::default(), envelope: 0.0 }
    }

    /// A voice's gain and its ears' delays and filters for where it is now (`SND_Spatialize`).
    fn place(&self, v: &mut Voice) {
        let l = &self.listener;
        if v.ent == l.ent && v.ent != 0 || v.ambient_target.is_some() {
            v.gain = v.volume;
            for e in &mut v.ears {
                e.level = 1.0;
                e.delay = 0.0;
                e.coefficients(1.0, self.rate);
            }
            return;
        }
        let to = v.origin - l.origin;
        let dist = to.length();
        let dir = if dist > 0.0 { to / dist } else { l.forward };
        v.gain = v.volume * (1.0 - dist * v.dist_mult).max(0.0);
        // each ear's share by the cosine to its side, as the original; the far ear later by the
        // path round the head; a sound behind duller in both
        let side = dir.dot(l.right).clamp(-1.0, 1.0);
        v.ears[0].level = 1.0 - side;
        v.ears[1].level = 1.0 + side;
        let lateral = side.asin(); // −π/2 (left) … π/2 (right)
        let itd = HEAD_RADIUS / SOUND_SPEED * (lateral.abs() + lateral.abs().sin()) * self.rate;
        let (far, near) = if side >= 0.0 { (0, 1) } else { (1, 0) };
        v.ears[far].delay = itd;
        v.ears[near].delay = 0.0;
        let behind = (-dir.dot(l.forward)).max(0.0);
        for e in &mut v.ears {
            e.coefficients(1.0 - 0.5 * behind, self.rate);
        }
    }

    /// `SND_PickChannel`: a sound from the same thing on the same channel takes its place
    /// (channel 0 never does); otherwise a free voice, or the one nearest its end, the
    /// listener's own sounds spared.
    fn pick(&mut self, ent: usize, channel: i32) -> Option<usize> {
        let dynamic: Vec<usize> = (0..self.voices.len()).filter(|&i| self.voices[i].ambient_target.is_none() && !self.voices[i].looping).collect();
        if channel != 0
            && let Some(&i) = dynamic.iter().find(|&&i| self.voices[i].ent == ent && self.voices[i].channel == channel) {
                return Some(i);
            }
        if dynamic.len() < MAX_VOICES {
            return None; // room for a new voice
        }
        dynamic
            .iter()
            .copied()
            .filter(|&i| !(self.voices[i].ent == self.listener.ent && ent != self.listener.ent))
            .min_by_key(|&i| self.sounds[self.voices[i].sound].as_ref().map(|s| s.samples.len().saturating_sub(self.voices[i].pos)).unwrap_or(0))
    }

    fn mix(&mut self, out: &mut [f32], channels: usize) {
        for s in out.iter_mut() {
            *s = 0.0;
        }
        let frames = out.len() / channels;
        let mut voices = std::mem::take(&mut self.voices);
        for v in &mut voices {
            self.place(v);
        }
        for v in &mut voices {
            let Some(sound) = self.sounds.get(v.sound).and_then(|s| s.clone()) else {
                v.pos = usize::MAX;
                continue;
            };
            if let Some(target) = v.ambient_target {
                // S_UpdateAmbientSounds: towards its level at `ambient_fade` 100 a second (of 255)
                let step = 100.0 / 255.0 * frames as f32 / self.rate;
                v.volume = if v.volume < target { (v.volume + step).min(target) } else { (v.volume - step).max(target) };
                v.gain = v.volume;
            }
            for f in 0..frames {
                if v.pos >= sound.samples.len() {
                    match (v.looping, sound.loop_start) {
                        (true, Some(l)) => v.pos = l,
                        _ => break,
                    }
                }
                let x = sound.samples[v.pos];
                v.pos += 1;
                v.history[v.at] = x;
                for (ear, e) in v.ears.iter_mut().enumerate() {
                    // the delayed sample, between two kept ones
                    let d = e.delay.min((HISTORY - 2) as f32);
                    let i = d.floor() as usize;
                    let fr = d - i as f32;
                    let a = v.history[(v.at + HISTORY - i) % HISTORY];
                    let b = v.history[(v.at + HISTORY - i - 1) % HISTORY];
                    let s = e.filter(a + (b - a) * fr) * v.gain * e.level * VOLUME;
                    if channels >= 2 {
                        out[f * channels + ear] += s;
                    } else {
                        out[f * channels] += s * 0.5;
                    }
                }
                v.at = (v.at + 1) % HISTORY;
            }
        }
        voices.retain(|v| v.pos != usize::MAX && (v.looping || self.sounds.get(v.sound).and_then(|s| s.as_ref()).is_some_and(|s| v.pos < s.samples.len())));
        self.voices = voices;
        // a limiter: the mix's peak followed (at once up, 200 ms down) and the mix brought under
        // 0.95 when it goes over, so many sounds together never clip
        let release = (-1.0 / (0.2 * self.rate)).exp();
        for f in 0..frames {
            let peak = (0..channels.min(2)).map(|c| out[f * channels + c].abs()).fold(0.0f32, f32::max);
            self.envelope = if peak > self.envelope { peak } else { self.envelope * release };
            let g = if self.envelope > 0.95 { 0.95 / self.envelope } else { 1.0 };
            for c in 0..channels {
                out[f * channels + c] *= g;
            }
        }
    }
}

/// The game's side of the sound: what to play, from where, and the listener.
pub struct Audio {
    mixer: Arc<Mutex<Mixer>>,
    _stream: cpal::Stream,
    pub rate: u32,
}

impl Audio {
    /// The output opened on the default device, floating-point stereo at its own rate.
    pub fn start() -> Result<Audio, String> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or("no sound device")?;
        let config = device.default_output_config().map_err(|e| e.to_string())?;
        let rate = config.sample_rate();
        let channels = config.channels() as usize;
        let mixer = Arc::new(Mutex::new(Mixer::new(rate as f32)));
        let m = mixer.clone();
        let err = |e: cpal::Error| eprintln!("bevy-quake: sound: {e}");
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(config.into(), move |data: &mut [f32], _: &cpal::OutputCallbackInfo| m.lock().unwrap().mix(data, channels), err, None),
            cpal::SampleFormat::I16 => {
                let mut buf = Vec::new();
                device.build_output_stream(
                    config.into(),
                    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                        buf.resize(data.len(), 0.0);
                        m.lock().unwrap().mix(&mut buf, channels);
                        for (o, s) in data.iter_mut().zip(&buf) {
                            *o = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
                        }
                    },
                    err,
                    None,
                )
            }
            f => return Err(format!("sample format {f:?} not handled")),
        }
        .map_err(|e| e.to_string())?;
        stream.play().map_err(|e| e.to_string())?;
        Ok(Audio { mixer, _stream: stream, rate })
    }

    /// The level's sounds, by the game's sound numbers; the ambient ones (water, wind, slime,
    /// lava) after them in `ambient`.
    pub fn load(&self, sounds: Vec<Option<Sound>>) {
        let mut m = self.mixer.lock().unwrap();
        m.voices.clear();
        m.sounds = sounds.into_iter().map(|s| s.map(Arc::new)).collect();
    }

    /// `S_StartSound`.
    pub fn play(&self, sound: usize, ent: usize, channel: i32, origin: Vec3, volume: f32, attenuation: f32) {
        let mut m = self.mixer.lock().unwrap();
        let v = Voice::new(sound, ent, channel, origin, volume, attenuation / CLIP_DIST, false);
        match m.pick(ent, channel) {
            Some(i) => m.voices[i] = v,
            None => m.voices.push(v),
        }
    }

    /// `S_StaticSound`: a sound looping at a place for the level; its attenuation reaches 64 times
    /// further than a moving sound's.
    pub fn static_sound(&self, sound: usize, origin: Vec3, volume: f32, attenuation: f32) {
        let mut m = self.mixer.lock().unwrap();
        m.voices.push(Voice::new(sound, 0, 0, origin, volume, attenuation / 64.0 / CLIP_DIST, true));
    }

    /// The four ambient channels' levels where the listener stands (`ambient_level` 0.3 times the
    /// leaf's own, nought under 8 of 255), each fading towards its level.
    pub fn ambients(&self, sounds: [Option<usize>; 4], levels: [f32; 4]) {
        let mut m = self.mixer.lock().unwrap();
        for (a, (sound, level)) in sounds.iter().zip(levels).enumerate() {
            let Some(sound) = *sound else { continue };
            let target = if level * 255.0 < 8.0 { 0.0 } else { level };
            let key = usize::MAX - a;
            if let Some(v) = m.voices.iter_mut().find(|v| v.ent == key) {
                v.ambient_target = Some(target);
            } else {
                let mut v = Voice::new(sound, key, 0, Vec3::ZERO, 0.0, 0.0, true);
                v.ambient_target = Some(target);
                m.voices.push(v);
            }
        }
    }

    /// Where the listener is, and where each thing now is, so its sounds go with it.
    pub fn listen(&self, listener: Listener, place_of: impl Fn(usize) -> Option<Vec3>) {
        let mut m = self.mixer.lock().unwrap();
        m.listener = listener;
        for v in &mut m.voices {
            if v.ent != 0 && v.ambient_target.is_none() && !v.looping
                && let Some(p) = place_of(v.ent) {
                    v.origin = p;
                }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sound_on_the_right_is_loud_in_the_right_ear_and_reaches_it_first() {
        let mut m = Mixer::new(48000.0);
        // a click: one sample, then silence
        let mut s = vec![0.0f32; 4800];
        s[0] = 1.0;
        m.sounds = vec![Some(Arc::new(Sound { samples: s, loop_start: None }))];
        m.listener = Listener { origin: Vec3::ZERO, forward: Vec3::X, right: Vec3::NEG_Y, ent: 1 };
        // straight to the right: all of it in the right ear, none in the left (`1 ± cos`)
        m.voices.push(Voice::new(0, 5, 1, Vec3::new(0.0, -100.0, 0.0), 1.0, 0.0, false));
        let mut out = vec![0.0f32; 2 * 256];
        m.mix(&mut out, 2);
        let energy = |out: &[f32], ear: usize| (0..256).map(|f| out[f * 2 + ear].powi(2)).sum::<f32>();
        assert!(energy(&out, 0) < energy(&out, 1) * 1e-6, "left {} right {}", energy(&out, 0), energy(&out, 1));
        // half way round to the right: the left ear hears it quieter and later, by the path round
        // the head (Woodworth: (π/4 + sin π/4) · a/c, 18 samples at 48 kHz)
        m.voices.clear();
        m.voices.push(Voice::new(0, 5, 1, Vec3::new(100.0, -100.0, 0.0), 1.0, 0.0, false));
        let mut out = vec![0.0f32; 2 * 256];
        m.mix(&mut out, 2);
        let first = |ear: usize| (0..256).find(|&f| out[f * 2 + ear].abs() > 1e-3).unwrap();
        assert!((first(0) as i32 - first(1) as i32 - 18).abs() <= 2, "{} {}", first(0), first(1));
        assert!(energy(&out, 0) < energy(&out, 1));
    }

    #[test]
    fn a_sound_ahead_is_as_loud_in_both_ears_and_one_behind_duller() {
        let mut m = Mixer::new(48000.0);
        // white noise
        let mut seed = 7u32;
        let s: Vec<f32> = (0..4800).map(|_| { seed = seed.wrapping_mul(1103515245).wrapping_add(12345); (seed >> 16) as f32 / 32768.0 - 1.0 }).collect();
        m.sounds = vec![Some(Arc::new(Sound { samples: s, loop_start: None }))];
        m.listener = Listener { origin: Vec3::ZERO, forward: Vec3::X, right: Vec3::NEG_Y, ent: 1 };
        let hiss = |m: &mut Mixer, at: Vec3| {
            m.voices.clear();
            m.voices.push(Voice::new(0, 5, 1, at, 1.0, 0.0, false));
            let mut out = vec![0.0f32; 2 * 2048];
            m.mix(&mut out, 2);
            // the highs: each sample less the one before
            let high = |ear: usize| (1..2048).map(|f| (out[f * 2 + ear] - out[(f - 1) * 2 + ear]).powi(2)).sum::<f32>();
            (high(0), high(1))
        };
        let (l, r) = hiss(&mut m, Vec3::new(100.0, 0.0, 0.0));
        assert!((l / r - 1.0).abs() < 1e-3, "ahead: {l} {r}");
        let (bl, _) = hiss(&mut m, Vec3::new(-100.0, 0.0, 0.0));
        assert!(bl < l * 0.6, "behind {bl} ahead {l}");
    }
}
