//! Every file of the shareware pack read by its own reader. With no pack, says why and stops.

use data::{Palette, bsp::Bsp, mdl::Mdl, pak::Packs, spr::Spr, wad::Wad, wav::Wav};
use std::path::PathBuf;

fn packs() -> Option<Packs> {
    let dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("run by cargo")).join("../../foreign/shareware/id1");
    match Packs::open_folder(&dir) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("not run: {e}");
            None
        }
    }
}

#[test]
fn every_file_of_the_pack_reads() {
    let Some(p) = packs() else { return };
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for name in p.names() {
        let b = p.get(name).unwrap();
        let ext = name.rsplit('.').next().unwrap();
        match ext {
            "bsp" => {
                let m = Bsp::parse(b).unwrap_or_else(|e| panic!("{name}: {e}"));
                assert!(!m.faces.is_empty(), "{name} has no faces");
                for f in &m.faces {
                    assert!(f.num_edges >= 3, "{name}: a face of {} edges", f.num_edges);
                }
            }
            "mdl" => {
                let m = Mdl::parse(b).unwrap_or_else(|e| panic!("{name}: {e}"));
                assert!(!m.frames.is_empty(), "{name} has no frames");
            }
            "spr" => {
                Spr::parse(b).unwrap_or_else(|e| panic!("{name}: {e}"));
            }
            "wav" => {
                let w = Wav::parse(b).unwrap_or_else(|e| panic!("{name}: {e}"));
                assert!(!w.samples.is_empty() && w.rate >= 11025, "{name}: {} samples at {} Hz", w.samples.len(), w.rate);
            }
            "wad" => {
                Wad::parse(b).unwrap_or_else(|e| panic!("{name}: {e}"));
            }
            "lmp" if name == "gfx/palette.lmp" => {
                Palette::parse(b).unwrap();
            }
            _ => {}
        }
        *counts.entry(ext).or_default() += 1;
    }
    eprintln!("{counts:?}");
    assert!(counts["bsp"] >= 8);
}

#[test]
fn e1m1_has_its_start() {
    let Some(p) = packs() else { return };
    let m = Bsp::parse(p.need("maps/e1m1.bsp").unwrap()).unwrap();
    let ents = data::entities::parse(&m.entities);
    assert_eq!(data::entities::get(&ents[0], "classname"), Some("worldspawn"));
    assert!(ents.iter().any(|e| data::entities::get(e, "classname") == Some("info_player_start")));
}
