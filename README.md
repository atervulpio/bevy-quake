# bevy-quake

The engine of the first Quake (id Software, 1996), written again in Rust, for fun.

- **The game's logic is id's QuakeC 1.06, translated by hand into Rust**, every function under its
  own name (`crates/game/src/progs`); no bytecode is run.
- **The light is computed again from the map's own lamps** (`crates/light`), not taken from the
  light baked into the map, and drawn by [Bevy](https://bevyengine.org): lightmaps for the world, an
  irradiance volume for what moves, Bevy's shadows for the moving lights.
- **The sound is the original sound, cleaned and placed in space for two ears** (`crates/sound`).
- **Textures, models, skins and sprites are id's own**, drawn as id drew them: the two drifting
  sky layers, the warped liquids, the lightning bolt.
- **The picture is seen through a sharp CRT monitor** (scanlines, aperture grille, a little glass
  halo). `F2` turns it off and on.
- **Every line is Rust**, the programs on the graphics card included: they are written in Rust and
  compiled to SPIR-V by [rust-gpu](https://github.com/Rust-GPU/rust-gpu) (`crates/shaders`).

It is a hobby project. It is not affiliated with id Software, Bethesda or ZeniMax. Quake is their
trademark.

## The game's data is not included

You need `pak0.pak` from Quake. The free shareware episode is enough; a bought copy's
`id1/pak0.pak` works too. Put it at

    foreign/shareware/id1/pak0.pak

(the folder is ignored by git). The shareware pack's sha256 is
`35a9c55e5e5a284a159ad2a62e0e8def23d829561fe2f54eb402dbc0a9a946af`. One way to get it, from the
project's folder:

    mkdir -p foreign/shareware && cd foreign/shareware
    wget https://idgames.cyberd.org/idstuff/quake/quake106.zip
    unzip -L quake106.zip && bsdtar -xf resource.1
    mv ID1 id1 && mv id1/PAK0.PAK id1/pak0.pak

## Building and playing

Built and played on Linux, on a Vulkan graphics card. You need:

- Rust through [rustup](https://rustup.rs). The first build installs the nightly toolchain rust-gpu
  needs (`nightly-2026-07-03`, with `rust-src`, `rustc-dev` and `llvm-tools`) beside your stable one,
  and builds rust-gpu's compiler into `cache/`. It takes several minutes, once.
- A Vulkan driver, and the usual libraries for a window and sound (Wayland or X11, ALSA).

Then:

    cargo run --release

It begins on the start map, where the skill and the episode are chosen, as the original does. Each
map computes its light the first time it is entered and keeps it in `cache/light/`: that takes a few
minutes per map, once.

| key | |
| --- | --- |
| mouse | look |
| `W` `A` `S` `D` | move |
| `Space` | jump, swim up |
| `C` | swim down |
| left button | fire |
| `1` … `8`, wheel | choose a weapon |
| `Esc` | the menu |
| `F2` | the CRT monitor off and on |

A map's name starts on that map: `cargo run --release -- e1m1`.

## Checks

    cargo test --workspace

With no `pak0.pak`, the checks that need it print `not run` and check nothing, though cargo still
lists them as `ok`: run them with the pack in place.

## How it is laid out

| crate | what |
| --- | --- |
| `crates/data` | the game's files read as id wrote them: pack, BSP 29 maps, models, sprites, WAD, palette, sounds, entity text |
| `crates/game` | the game's logic (QuakeC translated by hand), the server's physics and movement |
| `crates/light` | the light computed from the map's lamps, and probes for what moves |
| `crates/sound` | the sounds cleaned and placed for two ears |
| `crates/letters` | the game's letters, drawn from strokes |
| `crates/shaders` | the programs for the card, in Rust, compiled to SPIR-V by rust-gpu |
| `crates/render` | the picture, drawn by Bevy |
| `crates/bevy-quake` | the game itself, a Bevy app |

Comments that cite `row N` mean a row of the author's own decision log, which is not published.

## License

GPL-2.0-or-later, the same as id's Quake source and QuakeC, from which this is translated. See
`LICENSE`.

Quake's engine source: [id-Software/Quake](https://github.com/id-Software/Quake). QuakeC 1.06:
[maddes-b/QuakeC-releases](https://github.com/maddes-b/QuakeC-releases).
