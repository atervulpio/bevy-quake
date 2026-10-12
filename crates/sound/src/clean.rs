//! A sound of the game made ready to play: its offset from nought removed, and its rate brought to
//! the output's by a proper band-limited filter instead of the original's nearest sample (whose
//! mirror images of every sound above its own top were the metal edge of Quake's sound). Its
//! 8-bit hiss is left as id's own (row 40: taken out by frequency, it left the warbling of such
//! filters on every quiet passage).


/// A sound ready for the mixer, at the output's rate.
pub struct Sound {
    pub samples: Vec<f32>,
    /// Where a looping sound starts again (`cue ` in its file).
    pub loop_start: Option<usize>,
}


/// Takes out a constant offset and anything below 20 Hz (a one-pole high-pass): many of the
/// game's 8-bit sounds sit a level or two off the middle.
fn remove_offset(x: &mut [f32], rate: f32) {
    let r = (-2.0 * std::f32::consts::PI * 20.0 / rate).exp();
    let (mut px, mut py) = (0.0f32, 0.0f32);
    for s in x.iter_mut() {
        let y = *s - px + r * py;
        px = *s;
        py = y;
        *s = y;
    }
}

/// The zeroth-order modified Bessel function, for the Kaiser window.
fn bessel_i0(x: f32) -> f32 {
    let mut sum = 1.0f32;
    let mut term = 1.0f32;
    let q = x * x / 4.0;
    for k in 1..30 {
        term *= q / (k * k) as f32;
        sum += term;
        if term < sum * 1e-9 {
            break;
        }
    }
    sum
}

/// A new rate by windowed sinc (Kaiser, β 8.6, 32 taps each side): everything below 95% of the
/// lower of the two rates' tops kept, nothing above passed.
fn resample(x: &[f32], from: f32, to: f32) -> Vec<f32> {
    if (from - to).abs() < 0.5 {
        return x.to_vec();
    }
    const HALF: i64 = 32;
    const BETA: f32 = 8.6;
    let ratio = to as f64 / from as f64;
    let cutoff = 0.95 * (ratio as f32).min(1.0); // as a share of the input's top (its rate / 2)
    let i0b = bessel_i0(BETA);
    // the window tabulated finely over its half-width (it is even): 4,096 steps, a step's error
    // far below the 8-bit sounds' own
    const STEPS: usize = 4096;
    let table: Vec<f32> = (0..=STEPS).map(|i| {
        let w = i as f32 / STEPS as f32;
        bessel_i0(BETA * (1.0 - w * w).max(0.0).sqrt()) / i0b
    }).collect();
    let n_out = (x.len() as f64 * ratio).ceil() as usize;
    (0..n_out)
        .map(|n| {
            let t = n as f64 / ratio;
            let centre = t.floor() as i64;
            let mut acc = 0.0f32;
            for k in centre - HALF + 1..=centre + HALF {
                if k < 0 || k >= x.len() as i64 {
                    continue;
                }
                let d = (t - k as f64) as f32;
                let arg = std::f32::consts::PI * d * cutoff;
                let sinc = if arg.abs() < 1e-6 { 1.0 } else { arg.sin() / arg };
                let w = (d / HALF as f32).abs();
                let win = if w >= 1.0 { 0.0 } else { table[(w * STEPS as f32) as usize] };
                acc += x[k as usize] * cutoff * sinc * win;
            }
            acc
        })
        .collect()
}

/// A game sound made ready for an output running at `rate`.
pub fn prepare(wav: &data::wav::Wav, rate: u32) -> Sound {
    let mut x = wav.samples.clone();
    remove_offset(&mut x, wav.rate as f32);
    let samples = resample(&x, wav.rate as f32, rate as f32);
    let loop_start = wav.loop_start.map(|l| ((l as f64) * rate as f64 / wav.rate as f64).round() as usize).filter(|&l| l < samples.len());
    Sound { samples, loop_start }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tone_keeps_its_level_through_a_new_rate() {
        let from = 11025.0;
        let x: Vec<f32> = (0..11025).map(|i| (2.0 * std::f32::consts::PI * 1000.0 * i as f32 / from).sin() * 0.5).collect();
        let y = resample(&x, from, 48000.0);
        let mid = &y[10000..40000];
        let peak = mid.iter().fold(0.0f32, |m, v| m.max(v.abs()));
        assert!((peak - 0.5).abs() < 0.01, "{peak}");
    }
}
