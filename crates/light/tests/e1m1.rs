//! E1M1 lit, and how long it takes here. With no pack, says why and stops.

use data::bsp::Bsp;
use data::pak::Packs;
use glam::Vec3;
use light::{Settings, Surface, bake};
use std::path::PathBuf;
use std::time::Instant;

fn linear(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// Each texture's mean colour as its albedo; the sky and lava glowing with it.
pub fn surfaces(bsp: &Bsp, palette: &[u8]) -> Vec<Surface> {
    bsp.textures
        .iter()
        .map(|t| match t {
            Some(t) => {
                let mut c = Vec3::ZERO;
                for &p in &t.pixels {
                    let i = p as usize * 3;
                    c += Vec3::new(linear(palette[i]), linear(palette[i + 1]), linear(palette[i + 2]));
                }
                c /= t.pixels.len().max(1) as f32;
                let sky = t.name.starts_with("sky");
                let lava = t.name.contains("lava");
                Surface { albedo: c, emission: if sky || lava { c * 4.0 } else { Vec3::ZERO }, sky, liquid: t.name.starts_with('*') }
            }
            None => Surface { albedo: Vec3::splat(0.3), emission: Vec3::ZERO, sky: false, liquid: false },
        })
        .collect()
}

#[test]
fn e1m1_is_lit() {
    let dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("run by cargo")).join("../../foreign/shareware/id1");
    let Ok(p) = Packs::open_folder(&dir) else {
        eprintln!("not run: no shareware pack");
        return;
    };
    let bsp = Bsp::parse(p.need("maps/e1m1.bsp").unwrap()).unwrap();
    let surf = surfaces(&bsp, p.need("gfx/palette.lmp").unwrap());
    let t = Instant::now();
    let lm = bake(&bsp, &surf, &Settings::default());
    let lit = lm.samples.iter().filter(|v| v.max_element() > 0.01).count();
    eprintln!("E1M1: {} samples, {} lit, in {:.1} s here", lm.samples.len(), lit, t.elapsed().as_secs_f32());
    assert!(lit * 2 > lm.samples.len());
}
