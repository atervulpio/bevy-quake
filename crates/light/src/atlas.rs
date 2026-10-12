//! The faces' samples laid side by side in one picture for the card, each face's block with a
//! one-sample border copied from its edge, so the card's blending never reaches a neighbour.

use crate::Lightmaps;
use glam::Vec3;

/// Where each face's block of samples stands in the atlas: its corner's sample, inside the
/// border. `None` for a face with no light of its own (sky, liquids).
pub struct Atlas {
    pub width: u32,
    pub height: u32,
    pub places: Vec<Option<(u32, u32)>>,
    /// One layer for each of a face's four style slots, `width × height` each, packed as
    /// `Rgb9e5` (the card's shared-exponent format: three 9-bit mantissas, one 5-bit exponent).
    pub layers: [Vec<u32>; 4],
    /// The first slot's light's direction, `Rgba8`: the direction's three axes from −1 to 1
    /// brought to 0–255, and its share.
    pub directions: Vec<u32>,
}

/// A colour in the card's `Rgb9e5Ufloat`: nine bits for each channel and an exponent they share,
/// so light from a thousandth to sixty thousand times a texture's brightness fits in four bytes.
pub fn rgb9e5(c: Vec3) -> u32 {
    const N: i32 = 9;
    const B: i32 = 15;
    const MAX: f32 = ((1 << N) - 1) as f32 / (1 << N) as f32 * 65536.0;
    let c = c.clamp(Vec3::ZERO, Vec3::splat(MAX));
    let m = c.max_element();
    if m <= 0.0 {
        return 0;
    }
    let mut e = (m.log2().floor() as i32 + 1).max(-B - 1) + B;
    let mut denom = 2f32.powi(e - B - N);
    if (m / denom + 0.5).floor() as i32 == 1 << N {
        e += 1;
        denom *= 2.0;
    }
    let q = |x: f32| ((x / denom + 0.5).floor() as u32).min((1 << N) - 1);
    q(c.x) | (q(c.y) << 9) | (q(c.z) << 18) | ((e as u32) << 27)
}

pub fn pack(lm: &Lightmaps, width: u32) -> Atlas {
    let mut order: Vec<usize> = (0..lm.faces.len()).filter(|&i| lm.faces[i].width > 0).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(lm.faces[i].height));
    let mut places = vec![None; lm.faces.len()];
    let (mut x, mut y, mut shelf) = (0u32, 0u32, 0u32);
    for &i in &order {
        let f = &lm.faces[i];
        let (w, h) = (f.width + 2, f.height + 2);
        assert!(w <= width, "a face wider than the atlas");
        if x + w > width {
            x = 0;
            y += shelf;
            shelf = 0;
        }
        places[i] = Some((x + 1, y + 1));
        x += w;
        shelf = shelf.max(h);
    }
    let height = (y + shelf).max(1);
    let mut layers: [Vec<u32>; 4] = std::array::from_fn(|_| vec![0u32; (width * height) as usize]);
    let mut directions = vec![0x0080_8080u32; (width * height) as usize];
    for (i, f) in lm.faces.iter().enumerate() {
        let Some((px, py)) = places[i] else { continue };
        for (slot, layer) in layers.iter_mut().enumerate() {
            if f.styles[slot] == 255 {
                continue;
            }
            let base = f.offsets[slot] as usize;
            // the block and its border: a border sample repeats the nearest edge sample
            for j in -1..=f.height as i32 {
                for k in -1..=f.width as i32 {
                    let sj = j.clamp(0, f.height as i32 - 1) as usize;
                    let sk = k.clamp(0, f.width as i32 - 1) as usize;
                    let at = base + sj * f.width as usize + sk;
                    let ax = (px as i32 + k) as u32;
                    let ay = (py as i32 + j) as u32;
                    layer[(ay * width + ax) as usize] = rgb9e5(lm.samples[at]);
                    if slot == 0 {
                        let d = lm.directions[at];
                        let q = |x: f32| ((x * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0).round() as u32;
                        directions[(ay * width + ax) as usize] = q(d.x) | (q(d.y) << 8) | (q(d.z) << 16) | (((d.w.clamp(0.0, 1.0) * 255.0).round() as u32) << 24);
                    }
                }
            }
        }
    }
    Atlas { width, height, places, layers, directions }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unpack(v: u32) -> Vec3 {
        let e = (v >> 27) as i32 - 15 - 9;
        let s = 2f32.powi(e);
        Vec3::new((v & 511) as f32 * s, ((v >> 9) & 511) as f32 * s, ((v >> 18) & 511) as f32 * s)
    }

    #[test]
    fn shared_exponent_keeps_three_figures() {
        for c in [Vec3::new(1.0, 0.5, 0.25), Vec3::new(0.003, 0.002, 0.001), Vec3::new(40.0, 20.0, 1.0)] {
            let back = unpack(rgb9e5(c));
            assert!((back - c).abs().max_element() <= c.max_element() / 256.0, "{c} {back}");
        }
    }
}
