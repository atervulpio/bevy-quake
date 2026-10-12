//! What light reaches the walls near E1M1's start: a check of the bake's numbers against the
//! map's own lamps. With no pack, says why and stops.

use data::bsp::Bsp;
use data::pak::Packs;
use glam::Vec3;
use std::path::PathBuf;

#[test]
fn the_lamps_near_e1m1s_start() {
    let dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("run by cargo")).join("../../foreign/shareware/id1");
    let Ok(p) = Packs::open_folder(&dir) else {
        eprintln!("not run: no shareware pack");
        return;
    };
    let bsp = Bsp::parse(p.need("maps/e1m1.bsp").unwrap()).unwrap();
    let start = Vec3::new(480.0, -352.0, 88.0);
    for l in light::lamps(&bsp.entities) {
        let d = l.origin.distance(start);
        if d < 500.0 {
            eprintln!("lamp at {:?}, {:.0} away, intensity {:.0}, E at 100: {:.3}, style {}, spot {:?}", l.origin, d, l.intensity, l.intensity / 1e4, l.style, l.spot.is_some());
        }
    }
    // the original's own light near the start, for comparison: the faces' lightmap bytes
    let mut n = 0;
    for f in &bsp.faces {
        let pts = bsp.face_points(f);
        let c = pts.iter().fold(Vec3::ZERO, |a, &b| a + b) / pts.len() as f32;
        if c.distance(start) < 200.0 && f.light_ofs >= 0 && n < 12 {
            let v = bsp.lighting[f.light_ofs as usize];
            eprintln!("face at {:?}: id's light {} (style {})", c, v, f.styles[0]);
            n += 1;
        }
    }
}

#[test]
fn the_kept_bake_near_e1m1s_start() {
    let dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("run by cargo")).join("../../foreign/shareware/id1");
    let Ok(p) = Packs::open_folder(&dir) else { return };
    let bsp = Bsp::parse(p.need("maps/e1m1.bsp").unwrap()).unwrap();
    let kept = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("run by cargo")).join("../../cache/light");
    let Some(file) = std::fs::read_dir(&kept).ok().and_then(|mut d| d.find_map(|e| e.ok().filter(|e| e.file_name().to_string_lossy().starts_with("e1m1-")))) else {
        eprintln!("not run: no kept bake of e1m1");
        return;
    };
    let lm = light::Lightmaps::from_bytes(&std::fs::read(file.path()).unwrap()).unwrap();
    let start = Vec3::new(480.0, -352.0, 88.0);
    let mut n = 0;
    for (fi, f) in bsp.faces.iter().enumerate() {
        let pts = bsp.face_points(f);
        let c = pts.iter().fold(Vec3::ZERO, |a, &b| a + b) / pts.len() as f32;
        if c.distance(start) < 200.0 && n < 12 {
            let fl = &lm.faces[fi];
            let k = (fl.width * fl.height) as usize;
            let mean = if k > 0 { lm.samples[fl.offsets[0] as usize..fl.offsets[0] as usize + k].iter().fold(Vec3::ZERO, |a, &b| a + b) / k as f32 } else { Vec3::ZERO };
            eprintln!("face {fi} at {c:?}: {}x{} styles {:?} mean {mean:?}; id {}", fl.width, fl.height, fl.styles, if f.light_ofs >= 0 { bsp.lighting[f.light_ofs as usize] } else { 0 });
            n += 1;
        }
    }
}
