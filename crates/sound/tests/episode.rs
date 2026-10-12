//! Every sound of the episode made ready, and what the cleaning takes out of each. With no
//! pack, says why and stops.

use data::pak::Packs;
use std::path::PathBuf;
use std::time::Instant;

#[test]
fn every_sound_of_the_episode_is_made_ready() {
    let dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("run by cargo")).join("../../foreign/shareware/id1");
    let Ok(p) = Packs::open_folder(&dir) else {
        eprintln!("not run: no shareware pack");
        return;
    };
    let t = Instant::now();
    let mut n = 0;
    let mut seconds = 0.0;
    for name in p.names().filter(|n| n.ends_with(".wav")) {
        let w = data::wav::Wav::parse(p.get(name).unwrap()).unwrap();
        let s = sound::clean::prepare(&w, 48000);
        assert!(s.samples.iter().all(|v| v.is_finite()), "{name}");
        let expected = (w.samples.len() as f64 * 48000.0 / w.rate as f64).ceil() as usize;
        assert_eq!(s.samples.len(), expected, "{name}");
        n += 1;
        seconds += s.samples.len() as f32 / 48000.0;
    }
    eprintln!("{n} sounds, {seconds:.0} s of sound, made ready in {:.1} s here", t.elapsed().as_secs_f32());
}
