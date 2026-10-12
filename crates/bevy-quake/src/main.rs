//! bevy-quake: the first Quake, written again in Rust, drawn by Bevy (row 41).
//!
//!     bevy-quake [map]                          play, from the map's start (the start map by
//!                                                 default, where the skill is chosen)
//!     bevy-quake [map] --shot picture.png [s]   play s seconds (2 by default), then show the
//!                                                 frame in a window of its own and keep it as a
//!                                                 picture
//!         --eye x y z pitch yaw                   …seen from this place and turn
//!         --look pitch                            …the player looking so far down (up if less than
//!                                                 nought), the weapon in hand with them
//!         --fire                                  …holding the trigger
//!         --time                                  …and say how long a frame takes here
//!         --size WIDTHxHEIGHT                     …for a screen of that size (1920x1080)
//!         --flat                                  …shown as it is, not on the monitor
//!         --impulses 9,5                          …the game's commands given one a frame from the
//!                                                 start (`impulse 9` every weapon, then the fifth)
//!
//! In play, F2 or the menu's MONITOR switches between the monitor (row 24) and the picture as it
//! is.

mod client;
mod effects;
mod meshes;
mod status;

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::{CursorGrabMode, CursorOptions, MonitorSelection, PresentMode, PrimaryWindow, WindowMode, WindowResolution};
use client::Client;
use data::bsp::Bsp;
use data::pak::Packs;
use game::Game;
use game::spawn::{Carried, TICK};
use meshes::Built;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// A level being played: the server, the player's side and what Bevy draws for it.
struct Level {
    game: Game,
    client: Client,
    built: Built,
    world: Bsp,
    /// Seconds not yet given to the server: it runs whole frames of 1/72 s.
    owed: f32,
    /// The ambient channels' sounds (water, sky; the original had none for slime and lava).
    ambient: [Option<usize>; 4],
}

/// The ambient sounds by channel (`S_Init`): water and the sky's wind.
const AMBIENT_SOUNDS: [Option<&str>; 4] = [Some("ambience/water1.wav"), Some("ambience/wind2.wav"), None, None];

/// Sounds made ready once, by name, kept across levels.
type SoundStore = std::collections::HashMap<String, Arc<sound::clean::Sound>>;

fn ready_sounds(files: &Packs, names: &[&str], rate: u32, store: &mut SoundStore) -> Vec<Option<sound::clean::Sound>> {
    use rayon::prelude::*;
    let missing: Vec<&str> = names.iter().copied().filter(|n| !n.is_empty() && !store.contains_key(*n)).collect();
    let made: Vec<(String, sound::clean::Sound)> = missing
        .par_iter()
        .filter_map(|n| files.get(&format!("sound/{n}")).and_then(|b| data::wav::Wav::parse(b).ok()).map(|w| (n.to_string(), sound::clean::prepare(&w, rate))))
        .collect();
    for (n, s) in made {
        store.insert(n, Arc::new(s));
    }
    names.iter().map(|n| store.get(*n).map(|s| sound::clean::Sound { samples: s.samples.clone(), loop_start: s.loop_start })).collect()
}

impl Level {
    fn start(files: &Arc<Packs>, map: &str, carried: Carried, palette: &data::Palette) -> Result<Level, String> {
        let game = Game::spawn_server(files.clone(), map, carried)?;
        let bytes = files.need(&format!("maps/{map}.bsp")).map_err(|e| e.to_string())?;
        let world = Bsp::parse(bytes).map_err(|e| e.to_string())?;
        let maps = meshes::light(bytes, &world, palette, map);
        let atlas = light::atlas::pack(&maps, 4096);
        let built = meshes::build(files, &world, &meshes::Lit { maps: &maps, atlas: &atlas }, &game.models, palette);
        let n = game.sounds.len();
        let ambient = std::array::from_fn(|a| AMBIENT_SOUNDS[a].map(|_| n + a));
        let mut client = Client::default();
        client.palette = Some(palette.clone());
        let mut level = Level { game, client, built, world, owed: 0.0, ambient };
        level.client.after_tick(&level.game, &level.built, TICK);
        Ok(level)
    }

    /// The level's first events (its light styles, its frozen scenery, its looping sounds) heard
    /// and taken by the player's side, once the sound has the level's sounds.
    fn begin(&mut self, audio: Option<&sound::Audio>) {
        if let Some(a) = audio {
            let p = &self.game.ents[1].v;
            self.hear(a, &self.game.events, p.origin + p.view_ofs, self.client.viewangles);
        }
        self.client.take_events(&mut self.game);
    }

    /// The level's sounds given to the output, the game's precached ones then the ambient ones.
    fn load_sounds(&self, files: &Packs, audio: &sound::Audio, store: &mut SoundStore) {
        let mut names: Vec<&str> = self.game.sounds.to_vec();
        names.extend(AMBIENT_SOUNDS.iter().map(|a| a.unwrap_or("")));
        names.extend(client::CLIENT_SOUNDS);
        audio.load(ready_sounds(files, &names, audio.rate, store));
    }

    /// What the server asked to be heard this frame, and the listener where the eye is.
    fn hear(&self, audio: &sound::Audio, events: &[game::Event], eye: glam::Vec3, angles: glam::Vec3) {
        for ev in events {
            match ev {
                game::Event::Sound { ent, channel, sample, volume, attenuation, origin } => {
                    if let Some(i) = self.game.sounds.iter().position(|s| s == sample) {
                        audio.play(i, *ent, *channel, *origin, *volume, *attenuation);
                    }
                }
                game::Event::AmbientSound { origin, sample, volume, attenuation } => {
                    if let Some(i) = self.game.sounds.iter().position(|s| s == sample) {
                        audio.static_sound(i, *origin, *volume, *attenuation);
                    }
                }
                _ => {}
            }
        }
        let (forward, right, _) = render::angle_vectors(angles);
        let g = &self.game;
        audio.listen(sound::Listener { origin: eye, forward, right, ent: 1 }, |e| {
            g.ents.get(e).filter(|x| !x.free).map(|x| x.v.origin + (x.v.mins + x.v.maxs) * 0.5)
        });
        // S_UpdateAmbientSounds: the leaf the ear is in says how loud the water and wind are
        let leaf = g.map.point_in_leaf(eye);
        let a = self.world.leafs[leaf].ambient;
        audio.ambients(self.ambient, std::array::from_fn(|i| 0.3 * a[i] as f32 / 255.0));
    }

    /// Real seconds passed: the server's whole frames run, then the frame to draw.
    fn advance(&mut self, seconds: f32, cmd: impl Fn(&Client) -> game::UserCmd, audio: Option<&sound::Audio>, in_game: bool) -> Option<String> {
        self.client.time += seconds;
        self.owed = (self.owed + seconds).min(0.25);
        let mut next = None;
        while self.owed >= TICK {
            self.owed -= TICK;
            let c = cmd(&self.client);
            self.game.server_frame(TICK, c, in_game);
            for ev in &self.game.events {
                if let game::Event::Command(c) = ev
                    && let Some(map) = c.strip_prefix("changelevel ") {
                        next = Some(map.trim().to_string());
                    }
            }
            if let Some(a) = audio {
                let p = &self.game.ents[1].v;
                self.hear(a, &self.game.events, p.origin + p.view_ofs, self.client.viewangles);
            }
            self.client.take_events(&mut self.game);
            self.client.after_tick(&self.game, &self.built, TICK);
            // the sounds the player's side started itself, after the game's and the ambient ones
            let base = self.game.sounds.len() + AMBIENT_SOUNDS.len();
            for (name, origin) in self.client.sounds.drain(..) {
                if let (Some(a), Some(k)) = (audio, client::CLIENT_SOUNDS.iter().position(|s| *s == name)) {
                    a.play(base + k, usize::MAX / 2, 0, origin, 1.0, 1.0);
                }
            }
        }
        next
    }

    /// What Bevy draws for this level from now: its meshes, taken from it once.
    fn hand_over(&mut self) -> render::NewLevel {
        render::NewLevel(Some(std::mem::take(&mut self.built.meshes)))
    }
}

/// A picture kept of one frame (`--shot`), and what is asked of it.
struct Shot {
    path: String,
    eye: Option<(glam::Vec3, glam::Vec3)>,
    look: Option<f32>,
    flat: bool,
    /// The moment shown, worked out once: the frames before the picture is kept only wait for the
    /// card, and a moment worked out again each of them would move on (the particles die).
    moment: Option<(render::View, Vec<render::Draw>)>,
    /// How many frames in a row have been whole (every pipeline made), before it is taken.
    whole: u32,
    /// Frames timed so far and their seconds, when `--time` asks.
    timing: Option<(u32, f32)>,
    taken: bool,
    saved: Arc<AtomicBool>,
}

/// Frames that must come whole one after another before the picture is kept: a pipeline asked
/// for in a frame is made over the frames after, and the copies a frame first draws are given
/// their materials the frame after that.
const WHOLE_FRAMES: u32 = 10;
/// Frames timed for `--time`.
const TIMED_FRAMES: u32 = 120;

/// The game: everything the window's frames move along.
struct Play {
    files: Arc<Packs>,
    audio: Option<sound::Audio>,
    sounds: SoundStore,
    palette: data::Palette,
    level: Level,
    impulse: i32,
    grabbed: bool,
    /// The menu, when it is open: the item chosen. The world stands still meanwhile, as the
    /// original paused a game for one under its menu.
    menu: Option<usize>,
    /// The skill a new game or the next level is played at (`skill`: 0 easy to 3 nightmare).
    skill: i32,
    /// The picture shown on the sharpest monitor the screen can show, or as it is (row 24).
    crt: bool,
    /// A level being made on a thread of its own (row 27): the window keeps going meanwhile.
    loading: Option<std::thread::JoinHandle<Result<Level, String>>>,
    focused: bool,
    shot: Option<Shot>,
}

const SKILLS: [&str; 4] = ["EASY", "NORMAL", "HARD", "NIGHTMARE"];

/// The menu's items, top to bottom.
fn menu_items(skill: i32, crt: bool) -> [String; MENU_ITEMS] {
    ["RESUME".into(), "NEW GAME".into(), format!("SKILL  < {} >", SKILLS[skill as usize]), format!("MONITOR  < {} >", if crt { "CRT" } else { "FLAT" }), "QUIT".into()]
}

const MENU_ITEMS: usize = 5;

/// Where the menu's item `i` stands on a screen `h` high: its base line and size.
fn menu_place(i: usize, h: f32) -> (f32, f32) {
    let size = 30.0 * h / 1080.0;
    (h * 0.38 + i as f32 * size * 2.2, size)
}

impl Play {
    /// A new game: the start map at the chosen skill, nothing carried.
    fn new_game(&mut self) {
        let mut carried = Carried::default();
        carried.cvars.0.insert("skill".into(), self.skill.to_string());
        self.load("start".into(), carried);
    }

    /// Start making a level on a thread of its own; the one being played stands still under
    /// "LOADING" until it is ready (row 27).
    fn load(&mut self, map: String, carried: Carried) {
        if self.loading.is_some() {
            return;
        }
        let (files, palette) = (self.files.clone(), self.palette.clone());
        self.loading = Some(std::thread::spawn(move || Level::start(&files, &map, carried, &palette)));
    }

    /// The level made on its thread, if it is ready, put in play: its sounds, its first events,
    /// Bevy's copy.
    fn take_loaded(&mut self, new: &mut render::NewLevel) {
        if !self.loading.as_ref().is_some_and(|h| h.is_finished()) {
            return;
        }
        match self.loading.take().map(|h| h.join()) {
            Some(Ok(Ok(l))) => {
                self.level = l;
                if let Some(a) = &self.audio {
                    self.level.load_sounds(&self.files, a, &mut self.sounds);
                }
                self.level.begin(self.audio.as_ref());
                *new = self.level.hand_over();
            }
            Some(Ok(Err(e))) => eprintln!("bevy-quake: {e}"),
            Some(Err(_)) => eprintln!("bevy-quake: the level's making failed (its thread panicked)"),
            None => {}
        }
    }
}

/// The cursor held by the window while playing, free under the menu.
fn grab(play: &mut Play, cursor: &mut CursorOptions, on: bool) {
    cursor.grab_mode = if on { CursorGrabMode::Locked } else { CursorGrabMode::None };
    cursor.visible = !on;
    play.grabbed = on;
}

/// One frame of play: what the keys and the mouse ask, the world moved on, what Bevy draws.
#[allow(clippy::too_many_arguments)]
fn play(
    mut play: NonSendMut<Play>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
    mut window: Query<(&Window, &mut CursorOptions), With<PrimaryWindow>>,
    mut shown: ResMut<render::Shown>,
    mut new: ResMut<render::NewLevel>,
    mut exit: MessageWriter<AppExit>,
) {
    let play = &mut *play;
    let Ok((window, mut cursor)) = window.single_mut() else { return };
    let size = (window.physical_width(), window.physical_height());
    let h = size.1 as f32;
    // the window's focus lost opens the menu, as leaving the game does
    if play.focused && !window.focused && play.menu.is_none() {
        play.menu = Some(0);
        grab(play, &mut cursor, false);
    }
    play.focused = window.focused;
    if keys.just_pressed(KeyCode::F2) {
        play.crt = !play.crt;
    }
    if keys.just_pressed(KeyCode::Escape) {
        let open = play.menu.is_none();
        play.menu = open.then_some(0);
        grab(play, &mut cursor, !open);
    }
    if let Some(m) = play.menu {
        // the menu: the arrows or the mouse choose, Enter or a click does
        let mut chosen = None;
        if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            play.menu = Some((m + MENU_ITEMS - 1) % MENU_ITEMS);
        }
        if keys.any_just_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            play.menu = Some((m + 1) % MENU_ITEMS);
        }
        let (left, right) = (keys.any_just_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]), keys.any_just_pressed([KeyCode::ArrowRight, KeyCode::KeyD]));
        if m == 2 && left {
            play.skill = (play.skill + 3) % 4;
        }
        if m == 2 && right {
            play.skill = (play.skill + 1) % 4;
        }
        if m == 3 && (left || right) {
            play.crt = !play.crt;
        }
        if let Some(p) = window.physical_cursor_position() {
            for i in 0..MENU_ITEMS {
                let (y, sz) = menu_place(i, h);
                if p.y > y - sz * 1.4 && p.y < y + sz * 0.6 {
                    play.menu = Some(i);
                }
            }
        }
        if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) || buttons.just_pressed(MouseButton::Left) {
            chosen = play.menu;
        }
        match chosen {
            Some(0) => {
                play.menu = None;
                grab(play, &mut cursor, true);
            }
            Some(1) => {
                play.new_game();
                play.menu = None;
                grab(play, &mut cursor, true);
            }
            Some(2) => play.skill = (play.skill + 1) % 4,
            Some(3) => play.crt = !play.crt,
            Some(_) => {
                exit.write(AppExit::Success);
            }
            None => {}
        }
    } else if !play.grabbed && buttons.just_pressed(MouseButton::Left) {
        grab(play, &mut cursor, true);
    }
    // 1 to 8 choose a weapon (`impulse 1` … `impulse 8`); the wheel cycles them (10 and 12)
    let digits = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8];
    if let Some(i) = digits.iter().position(|&d| keys.just_pressed(d)) {
        play.impulse = i as i32 + 1;
    }
    if scroll.delta.y > 0.0 {
        play.impulse = 10;
    } else if scroll.delta.y < 0.0 {
        play.impulse = 12;
    }
    if play.grabbed && play.menu.is_none() {
        play.level.client.turn(motion.delta.x, motion.delta.y);
    }
    let k = |c| keys.pressed(c) as i32 as f32;
    let forward = k(KeyCode::KeyW) - k(KeyCode::KeyS);
    let side = k(KeyCode::KeyD) - k(KeyCode::KeyA);
    let up = k(KeyCode::Space) - k(KeyCode::KeyC);
    let attack = buttons.pressed(MouseButton::Left);
    let jump = keys.pressed(KeyCode::Space);
    let impulse = std::mem::take(&mut play.impulse);
    play.take_loaded(&mut new);
    let loading = play.loading.is_some();
    let playing = play.menu.is_none() && !loading;
    let (forward, side, up, attack, jump) = if playing { (forward, side, up, attack, jump) } else { (0.0, 0.0, 0.0, false, false) };
    let dt = time.delta_secs().min(0.1);
    // while the next level is made the world stands still, as the original's did
    let next = if loading { None } else { play.level.advance(dt, |c| c.command(forward, side, up, attack, jump, impulse), play.audio.as_ref(), playing) };
    if let Some(map) = next {
        let mut carried = play.level.game.save_spawn_parms();
        carried.cvars.0.insert("skill".into(), play.skill.to_string());
        play.load(map, carried);
    }
    let level = &mut play.level;
    let (mut view, draws) = level.client.frame(&level.game, &level.built, dt, size);
    view.crt = play.crt;
    let white = glam::Vec4::new(0.85, 0.83, 0.8, 0.9);
    if play.loading.is_some() {
        view.blend = glam::Vec4::new(0.0, 0.0, 0.0, 0.6);
        let sz = 30.0 * h / 1080.0;
        let x = (size.0 as f32 - render::text_width("LOADING", sz)) * 0.5;
        view.texts.push(render::Text { x, y: h * 0.5, size: sz, colour: white, text: "LOADING".into() });
    } else if let Some(chosen) = play.menu {
        view.blend = glam::Vec4::new(0.0, 0.0, 0.0, 0.55);
        for (i, item) in menu_items(play.skill, play.crt).iter().enumerate() {
            let (y, sz) = menu_place(i, h);
            let x = (size.0 as f32 - render::text_width(item, sz)) * 0.5;
            let colour = if i == chosen { glam::Vec4::new(1.0, 0.78, 0.35, 1.0) } else { white };
            view.texts.push(render::Text { x, y, size: sz, colour, text: item.clone() });
        }
    }
    *shown = render::Shown { view, draws };
}

/// One frame of `--shot`: the same moment shown until every pipeline is made, then timed if
/// asked, then kept as a picture and the window closed.
fn shoot(mut play: NonSendMut<Play>, offscreen: Res<render::Offscreen>, time: Res<Time>, ready: Res<render::Ready>, mut shown: ResMut<render::Shown>, mut commands: Commands, mut exit: MessageWriter<AppExit>) {
    let play = &mut *play;
    let (Some(size), Some(image)) = (offscreen.size, offscreen.image.clone()) else { return };
    let Some(shot) = play.shot.as_mut() else { return };
    if shot.saved.load(Ordering::Relaxed) {
        exit.write(AppExit::Success);
        return;
    }
    let level = &mut play.level;
    let (view, draws) = shot.moment.get_or_insert_with(|| {
        if let Some(pitch) = shot.look {
            level.client.viewangles.x = pitch;
        }
        let (mut view, draws) = level.client.frame(&level.game, &level.built, 1.0 / 60.0, size);
        view.crt = !shot.flat;
        if let Some((origin, angles)) = shot.eye {
            view.origin = origin;
            view.angles = angles;
        }
        (view, draws)
    });
    *shown = render::Shown { view: view.clone(), draws: draws.clone() };
    shot.whole = if ready.0.load(Ordering::Relaxed) { shot.whole + 1 } else { 0 };
    if shot.whole < WHOLE_FRAMES || shot.taken {
        return;
    }
    if let Some((n, seconds)) = shot.timing.as_mut() {
        if *n < TIMED_FRAMES {
            if *n > 0 {
                *seconds += time.delta_secs();
            }
            *n += 1;
            return;
        }
        let ms = *seconds * 1000.0 / (TIMED_FRAMES - 1) as f32;
        println!("{}×{}, drawn by Bevy into a picture: {ms:.2} ms a frame here ({:.0} a second), the game's own frame included", size.0, size.1, 1000.0 / ms);
    }
    shot.taken = true;
    let saved = shot.saved.clone();
    commands.spawn(Screenshot::image(image)).observe(save_to_disk(shot.path.clone())).observe(move |_: On<bevy::render::view::screenshot::ScreenshotCaptured>| saved.store(true, Ordering::Relaxed));
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../foreign/shareware/id1");
    let files = Arc::new(Packs::open_folder(&root).unwrap_or_else(|e| {
        eprintln!("bevy-quake: {e}");
        std::process::exit(1)
    }));
    let args: Vec<String> = std::env::args().skip(1).collect();
    let after = |flag: &str| args.iter().position(|a| a == flag).map(|i| &args[i + 1..]);
    let shot = after("--shot").map(|rest| (rest[0].clone(), rest.get(1).and_then(|s| s.parse::<f32>().ok()).unwrap_or(2.0)));
    // the original's new game begins on the start map, where the skill and episode are chosen
    let map = args.first().filter(|a| !a.starts_with("--")).cloned().unwrap_or_else(|| "start".into());
    let palette = data::Palette::parse(files.need("gfx/palette.lmp").unwrap()).unwrap();
    let mut level = Level::start(&files, &map, Carried::default(), &palette).unwrap_or_else(|e| {
        eprintln!("bevy-quake: {e}");
        std::process::exit(1)
    });
    let timing = args.iter().any(|a| a == "--time");
    let size = after("--size").map(|rest| {
        let n: Vec<u32> = rest[0].split('x').map(|x| x.parse().expect("WIDTHxHEIGHT")).collect();
        (n[0], n[1])
    });
    // A run on a screen of its own (`A_SCREEN_OF_OUR_OWN` set) is a check, not for anyone's ears:
    // it opens no sound, as it shows nothing on a screen anyone sees; and a picture is no game.
    let audio = if shot.is_some() || std::env::var_os("A_SCREEN_OF_OUR_OWN").is_some() {
        None
    } else {
        sound::Audio::start().map_err(|e| eprintln!("bevy-quake: no sound: {e}")).ok()
    };
    let mut sounds = SoundStore::new();
    if let Some(a) = &audio {
        level.load_sounds(&files, a, &mut sounds);
    }
    level.begin(audio.as_ref());
    let shot = shot.map(|(path, seconds)| {
        let fire = args.iter().any(|a| a == "--fire");
        let impulses: Vec<i32> = after("--impulses").map(|rest| rest[0].split(',').map(|n| n.parse().expect("a number")).collect()).unwrap_or_default();
        for step in 0..(seconds * 60.0) as usize {
            // each command held for a server frame of its own (the game reads `impulse` each frame)
            let impulse = impulses.get(step / 5).copied().filter(|_| step % 5 == 0).unwrap_or(0);
            level.advance(1.0 / 60.0, |c| c.command(0.0, 0.0, 0.0, fire, false, impulse), None, true);
        }
        let eye = after("--eye").map(|rest| {
            let n: Vec<f32> = rest[..5].iter().map(|a| a.parse().expect("a number")).collect();
            (glam::Vec3::new(n[0], n[1], n[2]), glam::Vec3::new(n[3], n[4], 0.0))
        });
        Shot { path, eye, look: after("--look").map(|rest| rest[0].parse().expect("degrees down")), flat: args.iter().any(|a| a == "--flat"), moment: None, whole: 0, timing: timing.then_some((0, 0.0)), taken: false, saved: Arc::default() }
    });
    let skill = level.game.cvar("skill") as i32;
    let first = level.hand_over();
    let (w, h) = size.unwrap_or((1920, 1080));
    // a picture is drawn into a texture of its size, the window a small one beside it; the game
    // fills the screen the window opens on (Bevy finds no "current" screen before the window
    // exists, says so, and leaves the choice to the desktop: `Borderless(None)`, as before Bevy)
    let window = Window {
        title: "bevy-quake".into(),
        mode: if shot.is_some() { WindowMode::Windowed } else { WindowMode::BorderlessFullscreen(MonitorSelection::Current) },
        resolution: if shot.is_some() { WindowResolution::new(64, 64) } else { WindowResolution::default() },
        present_mode: if timing { PresentMode::AutoNoVsync } else { PresentMode::AutoVsync },
        ..default()
    };
    let shooting = shot.is_some();
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin { primary_window: Some(window), primary_cursor_options: Some(CursorOptions { grab_mode: if shooting { CursorGrabMode::None } else { CursorGrabMode::Locked }, visible: shooting, ..default() }), ..default() })
            .set(RenderPlugin { render_creation: RenderCreation::Automatic(Box::new(WgpuSettings { backends: Some(Backends::VULKAN), ..default() })), ..default() }),
    )
    .add_plugins(render::Picture)
    .insert_resource(render::Offscreen { size: shooting.then_some((w, h)), image: None })
    .insert_resource(first)
    .insert_non_send(Play { files, audio, sounds, palette, level, impulse: 0, grabbed: !shooting, menu: None, skill, crt: true, loading: None, focused: true, shot });
    if shooting {
        app.add_systems(Update, shoot);
    } else {
        app.add_systems(Update, play);
    }
    app.run();
}
