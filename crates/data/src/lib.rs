//! The game's files as id wrote them: the pack, maps (BSP version 29), models, sprites, the
//! picture pack, the palette, sounds and the entity text of a map. Read only; nothing here draws,
//! sounds or decides. Each format is read from the original source's own headers (`bspfile.h`,
//! `modelgen.h`, `spritegn.h`, `wad.h`, `common.c`, `snd_mem.c`).

pub mod bsp;
pub mod entities;
pub mod mdl;
pub mod pak;
pub mod spr;
pub mod wad;
pub mod wav;

mod read;

pub use read::Error;

/// The 256 colours every picture of the game indexes, from `gfx/palette.lmp`.
#[derive(Clone)]
pub struct Palette(pub [[u8; 3]; 256]);

impl Palette {
    pub fn parse(bytes: &[u8]) -> Result<Palette, Error> {
        if bytes.len() < 768 {
            return Err(Error::new("palette shorter than 768 bytes"));
        }
        let mut p = [[0u8; 3]; 256];
        for (i, c) in p.iter_mut().enumerate() {
            c.copy_from_slice(&bytes[i * 3..i * 3 + 3]);
        }
        Ok(Palette(p))
    }

    /// Colour `i` in light's own linear values, 0 to 1.
    pub fn linear(&self, i: u8) -> glam::Vec3 {
        let [r, g, b] = self.0[i as usize];
        glam::Vec3::new(srgb_to_linear(r as f32 / 255.0), srgb_to_linear(g as f32 / 255.0), srgb_to_linear(b as f32 / 255.0))
    }
}

/// A colour channel from sRGB's 0–1, as the palette and every screen hold it, to light's own
/// linear 0–1.
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// A channel from light's own linear 0–1 back to sRGB's 0–1.
pub fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.003_130_8 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
}
