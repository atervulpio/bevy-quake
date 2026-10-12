//! Every level of the shareware episode spawned and run: the game's logic, translated, must
//! play each map's first half minute with no fault. With no pack, says why and stops.

use data::pak::Packs;
use game::spawn::{Carried, TICK};
use game::{Game, UserCmd};
use std::path::PathBuf;
use std::sync::Arc;

fn packs() -> Option<Arc<Packs>> {
    let dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("run by cargo")).join("../../foreign/shareware/id1");
    match Packs::open_folder(&dir) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            eprintln!("not run: {e}");
            None
        }
    }
}

fn run(g: &mut Game, seconds: f32) {
    let frames = (seconds / TICK) as usize;
    for _ in 0..frames {
        g.server_frame(TICK, UserCmd::default(), true);
        g.events.clear();
    }
}

#[test]
fn every_map_spawns_and_plays_half_a_minute() {
    let Some(files) = packs() else { return };
    let maps: Vec<String> = files.names().filter(|n| n.starts_with("maps/") && n.ends_with(".bsp") && !n.starts_with("maps/b_")).map(|n| n[5..n.len() - 4].to_string()).collect();
    assert!(maps.len() >= 9, "{maps:?}");
    for map in maps {
        let mut g = Game::spawn_server(files.clone(), &map, Carried::default()).unwrap_or_else(|e| panic!("{map}: {e}"));
        let live = g.ents.iter().filter(|e| !e.free).count();
        run(&mut g, 30.0);
        eprintln!("{map}: {live} entities at the start, {} monsters, {} secrets, the player at {:?} with {} health", g.total_monsters, g.total_secrets, g.ents[1].v.origin, g.ents[1].v.health);
        assert_eq!(g.ents[1].v.health, 100.0, "{map}: the player stood still and was hurt");
    }
}

#[test]
fn e1m1_has_its_monsters_and_the_player_falls_to_the_floor() {
    let Some(files) = packs() else { return };
    let mut g = Game::spawn_server(files, "e1m1", Carried::default()).unwrap();
    assert!(g.total_monsters > 0.0);
    let z0 = g.ents[1].v.origin.z;
    run(&mut g, 2.0);
    assert!(g.ents[1].v.flags & game::FL_ONGROUND != 0, "the player stands on the floor");
    assert!((g.ents[1].v.origin.z - z0).abs() < 32.0, "{} to {}", z0, g.ents[1].v.origin.z);
}
