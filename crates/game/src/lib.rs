//! The game: id's server (`sv_*.c`, `world.c`, `pr_cmds.c`) and id's game logic (QuakeC 1.06,
//! `progs/`), written again in Rust. Nothing here draws or sounds; what the player should see and
//! hear leaves as `Event`s, and what they press comes in as a `UserCmd`.
//!
//! The game logic keeps QuakeC's shape so that each function can be read beside its original:
//! every function, field and global keeps its QuakeC name; `self` and `other` are globals
//! (`g.self_`, `g.other`); an entity is its number, `world` is 0.

#![allow(non_snake_case, non_upper_case_globals, clippy::too_many_arguments)]

pub mod builtins;
pub mod fields;
pub mod movement;
pub mod phys;
pub mod progs;
pub mod spawn;
pub mod user;
pub mod world;

pub use fields::EntVars;
use glam::Vec3;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

/// An entity, by its number; `world` is 0.
pub type Ent = usize;
pub const world: Ent = 0;

/// A string of the game: names, messages, sounds. Interned, so it copies as a pointer.
pub type Str = &'static str;

/// A function an entity calls by itself: `.void() think` and the rest.
pub type Think = Option<fn(&mut Game)>;
/// `.void(entity attacker, float damage) th_pain`.
pub type Pain = Option<fn(&mut Game, Ent, f32)>;

/// Every string the game has made, kept once (`intern`), so equal strings are one pointer and
/// none outlives the program's need: the set only grows with names never seen before.
pub fn intern(s: &str) -> Str {
    static SET: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();
    let mut set = SET.get_or_init(Default::default).lock().unwrap();
    if let Some(&k) = set.get(s) {
        return k;
    }
    let k: &'static str = Box::leak(s.to_owned().into_boxed_str());
    set.insert(k);
    k
}

/// C's `atof`: the longest number at the head of the text, nought if there is none.
pub fn atof(s: &str) -> f32 {
    let s = s.trim_start();
    let b = s.as_bytes();
    let mut end = 0;
    if end < b.len() && (b[end] == b'-' || b[end] == b'+') {
        end += 1;
    }
    while end < b.len() && b[end].is_ascii_digit() {
        end += 1;
    }
    if end < b.len() && b[end] == b'.' {
        end += 1;
        while end < b.len() && b[end].is_ascii_digit() {
            end += 1;
        }
    }
    if end < b.len() && (b[end] == b'e' || b[end] == b'E') {
        let mut e = end + 1;
        if e < b.len() && (b[e] == b'-' || b[e] == b'+') {
            e += 1;
        }
        if e < b.len() && b[e].is_ascii_digit() {
            while e < b.len() && b[e].is_ascii_digit() {
                e += 1;
            }
            end = e;
        }
    }
    s[..end].parse().unwrap_or(0.0)
}

// ---- defs.qc: the constants -------------------------------------------------------------------

pub const FALSE: f32 = 0.0;
pub const TRUE: f32 = 1.0;

pub const FL_FLY: i32 = 1;
pub const FL_SWIM: i32 = 2;
pub const FL_CONVEYOR: i32 = 4;
pub const FL_CLIENT: i32 = 8;
pub const FL_INWATER: i32 = 16;
pub const FL_MONSTER: i32 = 32;
pub const FL_GODMODE: i32 = 64;
pub const FL_NOTARGET: i32 = 128;
pub const FL_ITEM: i32 = 256;
pub const FL_ONGROUND: i32 = 512;
pub const FL_PARTIALGROUND: i32 = 1024;
pub const FL_WATERJUMP: i32 = 2048;
pub const FL_JUMPRELEASED: i32 = 4096;

pub const MOVETYPE_NONE: i32 = 0;
pub const MOVETYPE_ANGLENOCLIP: i32 = 1;
pub const MOVETYPE_ANGLECLIP: i32 = 2;
pub const MOVETYPE_WALK: i32 = 3;
pub const MOVETYPE_STEP: i32 = 4;
pub const MOVETYPE_FLY: i32 = 5;
pub const MOVETYPE_TOSS: i32 = 6;
pub const MOVETYPE_PUSH: i32 = 7;
pub const MOVETYPE_NOCLIP: i32 = 8;
pub const MOVETYPE_FLYMISSILE: i32 = 9;
pub const MOVETYPE_BOUNCE: i32 = 10;
pub const MOVETYPE_BOUNCEMISSILE: i32 = 11;

pub const SOLID_NOT: i32 = 0;
pub const SOLID_TRIGGER: i32 = 1;
pub const SOLID_BBOX: i32 = 2;
pub const SOLID_SLIDEBOX: i32 = 3;
pub const SOLID_BSP: i32 = 4;

pub const RANGE_MELEE: i32 = 0;
pub const RANGE_NEAR: i32 = 1;
pub const RANGE_MID: i32 = 2;
pub const RANGE_FAR: i32 = 3;

pub const DEAD_NO: i32 = 0;
pub const DEAD_DYING: i32 = 1;
pub const DEAD_DEAD: i32 = 2;
pub const DEAD_RESPAWNABLE: i32 = 3;

pub const DAMAGE_NO: i32 = 0;
pub const DAMAGE_YES: i32 = 1;
pub const DAMAGE_AIM: i32 = 2;

pub const IT_AXE: i32 = 4096;
pub const IT_SHOTGUN: i32 = 1;
pub const IT_SUPER_SHOTGUN: i32 = 2;
pub const IT_NAILGUN: i32 = 4;
pub const IT_SUPER_NAILGUN: i32 = 8;
pub const IT_GRENADE_LAUNCHER: i32 = 16;
pub const IT_ROCKET_LAUNCHER: i32 = 32;
pub const IT_LIGHTNING: i32 = 64;
pub const IT_EXTRA_WEAPON: i32 = 128;
pub const IT_SHELLS: i32 = 256;
pub const IT_NAILS: i32 = 512;
pub const IT_ROCKETS: i32 = 1024;
pub const IT_CELLS: i32 = 2048;
pub const IT_ARMOR1: i32 = 8192;
pub const IT_ARMOR2: i32 = 16384;
pub const IT_ARMOR3: i32 = 32768;
pub const IT_SUPERHEALTH: i32 = 65536;
pub const IT_KEY1: i32 = 131072;
pub const IT_KEY2: i32 = 262144;
pub const IT_INVISIBILITY: i32 = 524288;
pub const IT_INVULNERABILITY: i32 = 1048576;
pub const IT_SUIT: i32 = 2097152;
pub const IT_QUAD: i32 = 4194304;

pub const CONTENT_EMPTY: i32 = -1;
pub const CONTENT_SOLID: i32 = -2;
pub const CONTENT_WATER: i32 = -3;
pub const CONTENT_SLIME: i32 = -4;
pub const CONTENT_LAVA: i32 = -5;
pub const CONTENT_SKY: i32 = -6;

pub const STATE_TOP: i32 = 0;
pub const STATE_BOTTOM: i32 = 1;
pub const STATE_UP: i32 = 2;
pub const STATE_DOWN: i32 = 3;

pub const VEC_ORIGIN: Vec3 = Vec3::ZERO;
pub const VEC_HULL_MIN: Vec3 = Vec3::new(-16.0, -16.0, -24.0);
pub const VEC_HULL_MAX: Vec3 = Vec3::new(16.0, 16.0, 32.0);
pub const VEC_HULL2_MIN: Vec3 = Vec3::new(-32.0, -32.0, -24.0);
pub const VEC_HULL2_MAX: Vec3 = Vec3::new(32.0, 32.0, 64.0);

pub const CHAN_AUTO: i32 = 0;
pub const CHAN_WEAPON: i32 = 1;
pub const CHAN_VOICE: i32 = 2;
pub const CHAN_ITEM: i32 = 3;
pub const CHAN_BODY: i32 = 4;

pub const ATTN_NONE: f32 = 0.0;
pub const ATTN_NORM: f32 = 1.0;
pub const ATTN_IDLE: f32 = 2.0;
pub const ATTN_STATIC: f32 = 3.0;

pub const EF_BRIGHTFIELD: i32 = 1;
pub const EF_MUZZLEFLASH: i32 = 2;
pub const EF_BRIGHTLIGHT: i32 = 4;
pub const EF_DIMLIGHT: i32 = 8;

pub const AS_STRAIGHT: i32 = 1;
pub const AS_SLIDING: i32 = 2;
pub const AS_MELEE: i32 = 3;
pub const AS_MISSILE: i32 = 4;

/// The spawn flags that keep an entity out of a skill or of deathmatch (`server.h`).
pub const SPAWNFLAG_NOT_EASY: i32 = 256;
pub const SPAWNFLAG_NOT_MEDIUM: i32 = 512;
pub const SPAWNFLAG_NOT_HARD: i32 = 1024;
pub const SPAWNFLAG_NOT_DEATHMATCH: i32 = 2048;

/// The traceline's `nomonsters` argument (`MOVE_*` in `world.h`).
pub const MOVE_NORMAL: i32 = 0;
pub const MOVE_NOMONSTERS: i32 = 1;
pub const MOVE_MISSILE: i32 = 2;

/// What the game writes for the player's screen and ears: id's `WriteByte` sequences and its
/// server's own messages, each kept as what it means. The client reads them after each frame.
#[derive(Clone, Debug)]
pub enum Event {
    /// `SV_StartSound`: a sound from an entity on one of its eight channels.
    Sound { ent: Ent, channel: i32, sample: Str, volume: f32, attenuation: f32, origin: Vec3 },
    /// `ambientsound`: a sound looping at a place for the whole level.
    AmbientSound { origin: Vec3, sample: Str, volume: f32, attenuation: f32 },
    /// `SV_StartParticle`: `count` particles of palette colour `colour` thrown along `dir`.
    Particle { origin: Vec3, dir: Vec3, colour: i32, count: i32 },
    TempEntity(TempEntity),
    /// `bprint` and `sprint`: a line in the corner.
    Print(String),
    /// `centerprint`: a message in the middle of the screen.
    CenterPrint(String),
    /// `stuffcmd(…, "bf\n")`: the screen flashes for a picked-up thing.
    BonusFlash,
    /// `SVC_KILLEDMONSTER` and `SVC_FOUNDSECRET`: the client's own counters move by one.
    KilledMonster,
    FoundSecret,
    /// `SVC_INTERMISSION`: the level's end, the tally shown.
    Intermission,
    /// `SVC_FINALE`: an episode's end, the text typed out.
    Finale(String),
    /// `SVC_CDTRACK`: the music track and the one to loop after it.
    CdTrack(i32, i32),
    /// `SVC_SELLSCREEN`: the shareware's order screen.
    SellScreen,
    /// `lightstyle`: one of the 64 brightness patterns changed.
    LightStyle(usize, Str),
    /// `makestatic`: an entity frozen into the level's scenery and freed.
    Static { model: Str, frame: f32, skin: f32, origin: Vec3, angles: Vec3 },
    /// `fixangle`: the player's view turned to the entity's angles.
    SetAngle(Vec3),
    /// A muzzle flash this frame (`EF_MUZZLEFLASH`, sent before `SV_CleanupEnts` clears it).
    MuzzleFlash(Ent),
    /// `svc_damage`: the player was hurt; how much the armour took, how much the body, and where
    /// from, for the screen's flash and the view's kick.
    Damage { armor: f32, blood: f32, from: Vec3 },
    /// A console command the game asked for: `changelevel`, `restart` and the like.
    Command(String),
}

/// `SVC_TEMPENTITY`'s kinds (`cl_tent.c`).
#[derive(Clone, Debug)]
pub enum TempEntity {
    Spike(Vec3),
    SuperSpike(Vec3),
    Gunshot(Vec3),
    Explosion(Vec3),
    TarExplosion(Vec3),
    Lightning1 { ent: Ent, start: Vec3, end: Vec3 },
    Lightning2 { ent: Ent, start: Vec3, end: Vec3 },
    Lightning3 { ent: Ent, start: Vec3, end: Vec3 },
    WizSpike(Vec3),
    KnightSpike(Vec3),
    LavaSplash(Vec3),
    Teleport(Vec3),
}

/// What the player asked for in one frame (`usercmd_t`, `cl_input.c`).
#[derive(Clone, Copy, Default, Debug)]
pub struct UserCmd {
    pub viewangles: Vec3,
    pub forwardmove: f32,
    pub sidemove: f32,
    pub upmove: f32,
    /// Bit 0 fire, bit 1 jump (`in_attack`, `in_jump`).
    pub buttons: i32,
    pub impulse: i32,
}

pub struct Edict {
    pub free: bool,
    /// When it was freed: a number is reused only half a second after (`ED_Alloc`).
    pub freetime: f32,
    pub v: EntVars,
    /// The leafs of the world it stands in (`SV_FindTouchedLeafs`), at most 16.
    pub leafs: Vec<u16>,
    /// The area node it is linked into and whether as a trigger (`SV_LinkEdict`).
    pub area: Option<(usize, bool)>,
}

impl Edict {
    fn new() -> Edict {
        Edict { free: false, freetime: 0.0, v: EntVars::default(), leafs: Vec::new(), area: None }
    }
}

/// A model in the precache list: what its name loads to, as far as the server needs it.
#[derive(Clone)]
pub struct Model {
    pub name: Str,
    pub mins: Vec3,
    pub maxs: Vec3,
    /// A brush model: its number among the map's models (`*1` is 1, the world 0).
    pub brush: Option<usize>,
}

/// The server's settings it reads by name (`cvar`), with their defaults.
pub struct Cvars(pub HashMap<String, String>);

impl Cvars {
    pub fn new() -> Cvars {
        let mut m = HashMap::new();
        for (k, v) in [
            ("skill", "1"),
            ("deathmatch", "0"),
            ("coop", "0"),
            ("teamplay", "0"),
            ("fraglimit", "0"),
            ("timelimit", "0"),
            ("samelevel", "0"),
            ("noexit", "0"),
            ("registered", "0"),
            ("temp1", "0"),
            ("sv_gravity", "800"),
            ("sv_friction", "4"),
            ("sv_edgefriction", "2"),
            ("sv_stopspeed", "100"),
            ("sv_maxspeed", "320"),
            ("sv_accelerate", "10"),
            ("sv_maxvelocity", "2000"),
            ("sv_idealpitchscale", "0.8"),
            ("sv_aim", "0.93"),
            ("sv_nostep", "0"),
            ("saved1", "0"),
            ("saved2", "0"),
            ("saved3", "0"),
            ("saved4", "0"),
            ("scratch1", "0"),
            ("scratch2", "0"),
            ("scratch3", "0"),
            ("scratch4", "0"),
        ] {
            m.insert(k.to_string(), v.to_string());
        }
        Cvars(m)
    }
    pub fn value(&self, name: &str) -> f32 {
        self.0.get(name).map(|s| atof(s)).unwrap_or(0.0)
    }
}

impl Default for Cvars {
    fn default() -> Self {
        Cvars::new()
    }
}

/// The server and the game's globals (`globalvars_t` and the rest of QuakeC's).
pub struct Game {
    pub ents: Vec<Edict>,

    // defs.qc's globals
    pub self_: Ent,
    pub other: Ent,
    pub time: f32,
    pub frametime: f32,
    pub force_retouch: f32,
    pub mapname: Str,
    pub deathmatch: f32,
    pub coop: f32,
    pub teamplay: f32,
    pub serverflags: i32,
    pub total_secrets: f32,
    pub total_monsters: f32,
    pub found_secrets: f32,
    pub killed_monsters: f32,
    /// `parm1` … `parm16`: what a player carries from one level to the next.
    pub parm: [f32; 16],
    pub v_forward: Vec3,
    pub v_up: Vec3,
    pub v_right: Vec3,
    pub trace_allsolid: bool,
    pub trace_startsolid: bool,
    pub trace_fraction: f32,
    pub trace_endpos: Vec3,
    pub trace_plane_normal: Vec3,
    pub trace_plane_dist: f32,
    pub trace_ent: Ent,
    pub trace_inopen: bool,
    pub trace_inwater: bool,
    pub movedist: f32,
    pub gameover: f32,
    pub string_null: Str,
    pub newmis: Ent,
    pub activator: Ent,
    pub damage_attacker: Ent,
    pub framecount: f32,
    pub skill: f32,

    // the other files' globals
    pub hknight_type: f32,
    pub le1: Ent,
    pub le2: Ent,
    pub lightning_end: f32,
    pub modelindex_eyes: f32,
    pub modelindex_player: f32,
    pub intermission_running: f32,
    pub intermission_exittime: f32,
    pub nextmap: Str,
    pub sight_entity: Ent,
    pub sight_entity_time: f32,
    pub enemy_vis: f32,
    pub enemy_infront: f32,
    pub enemy_range: i32,
    pub enemy_yaw: f32,
    /// `triggers.qc`'s global `s`, a spare entity.
    pub s: Ent,
    pub shub: Ent,
    pub multi_ent: Ent,
    pub multi_damage: f32,
    pub lastspawn: Ent,
    pub bodyqueue_head: Ent,

    // the server's own state
    pub cvars: Cvars,
    pub models: Vec<Model>,
    pub sounds: Vec<Str>,
    pub lightstyles: Vec<Str>,
    /// While the level's entities are spawned (`ss_loading`): only then may a model or sound be
    /// added to the precache lists.
    pub loading: bool,
    pub paused: bool,
    pub events: Vec<Event>,
    /// The map: its hulls, leafs and area tree.
    pub map: world::World,
    /// The player's carried things at the level's start (`client->spawn_parms`).
    pub spawn_parms: [f32; 16],
    /// The player's command for this frame.
    pub cmd: UserCmd,
    /// `sv.lastcheck`, `sv.lastchecktime` (`PF_checkclient`).
    pub lastcheck: usize,
    pub lastchecktime: f32,
    pub checkpvs: Vec<u8>,
    rng: u32,
    pub changelevel_issued: bool,
    /// The game's files, where `precache_model` finds a model's size; none in tests of logic
    /// alone, where every model is a 32-unit box.
    pub files: Option<std::sync::Arc<data::pak::Packs>>,
    /// The server's clock (`sv.time`), apart from the game's `time`, which is set to each
    /// think's own moment.
    pub sv_time: f64,
    /// The seconds the current frame of the world lasts (`host_frametime`).
    pub frame_seconds: f32,
    /// Whether the player is connected (`client->active`) and in the level (`client->spawned`).
    pub client_active: bool,
    pub client_spawned: bool,
}

/// The most entities a level may hold (`MAX_EDICTS`).
pub const MAX_EDICTS: usize = 600;
/// The one player of a game for one (`svs.maxclients`).
pub const MAX_CLIENTS: usize = 1;

impl Game {
    pub fn new(map: world::World) -> Game {
        Game {
            ents: Vec::new(),
            self_: 0,
            other: 0,
            time: 0.0,
            frametime: 0.0,
            force_retouch: 0.0,
            mapname: "",
            deathmatch: 0.0,
            coop: 0.0,
            teamplay: 0.0,
            serverflags: 0,
            total_secrets: 0.0,
            total_monsters: 0.0,
            found_secrets: 0.0,
            killed_monsters: 0.0,
            parm: [0.0; 16],
            v_forward: Vec3::ZERO,
            v_up: Vec3::ZERO,
            v_right: Vec3::ZERO,
            trace_allsolid: false,
            trace_startsolid: false,
            trace_fraction: 0.0,
            trace_endpos: Vec3::ZERO,
            trace_plane_normal: Vec3::ZERO,
            trace_plane_dist: 0.0,
            trace_ent: 0,
            trace_inopen: false,
            trace_inwater: false,
            movedist: 0.0,
            gameover: 0.0,
            string_null: "",
            newmis: 0,
            activator: 0,
            damage_attacker: 0,
            framecount: 0.0,
            skill: 0.0,
            hknight_type: 0.0,
            le1: 0,
            le2: 0,
            lightning_end: 0.0,
            modelindex_eyes: 0.0,
            modelindex_player: 0.0,
            intermission_running: 0.0,
            intermission_exittime: 0.0,
            nextmap: "",
            sight_entity: 0,
            sight_entity_time: 0.0,
            enemy_vis: 0.0,
            enemy_infront: 0.0,
            enemy_range: 0,
            enemy_yaw: 0.0,
            s: 0,
            shub: 0,
            multi_ent: 0,
            multi_damage: 0.0,
            lastspawn: 0,
            bodyqueue_head: 0,
            cvars: Cvars::new(),
            models: Vec::new(),
            sounds: Vec::new(),
            lightstyles: vec![""; 64],
            loading: false,
            paused: false,
            events: Vec::new(),
            map,
            spawn_parms: [0.0; 16],
            cmd: UserCmd::default(),
            lastcheck: 0,
            lastchecktime: 0.0,
            checkpvs: Vec::new(),
            rng: 0x1234_5678,
            changelevel_issued: false,
            files: None,
            sv_time: 0.0,
            frame_seconds: 0.1,
            client_active: false,
            client_spawned: false,
        }
    }

    /// An entity's fields.
    #[inline]
    pub fn v(&mut self, e: Ent) -> &mut EntVars {
        &mut self.ents[e].v
    }

    /// An entity's fields, to read.
    #[inline]
    pub fn r(&self, e: Ent) -> &EntVars {
        &self.ents[e].v
    }

    /// The number of entity slots in use, free ones among them (`sv.num_edicts`).
    pub fn num_edicts(&self) -> usize {
        self.ents.len()
    }

    /// C's `rand()`: fifteen bits, from the same linear rule as the C library of the time
    /// (`next = next * 1103515245 + 12345`), so the game's dice are an honest sequence.
    pub fn rand(&mut self) -> i32 {
        self.rng = self.rng.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.rng >> 16) & 0x7fff) as i32
    }

    /// QuakeC's frame function, `void() f = [$frame, next] { … }`: `self` shows the frame and
    /// calls `next` a tenth of a second on (`OP_STATE`).
    pub fn frame_state(&mut self, frame: f32, next: fn(&mut Game)) {
        let me = self.self_;
        let t = self.time;
        let v = &mut self.ents[me].v;
        v.nextthink = t + 0.1;
        v.frame = frame;
        v.think = Some(next);
    }

    /// Calls a function field, as QuakeC's `self.th_stand ()`: a field never set is the
    /// original's "NULL function" fault.
    pub fn call(&mut self, f: Think) {
        f.expect("NULL function")(self)
    }

    /// `self.th_pain (attacker, damage)`.
    pub fn call_pain(&mut self, f: Pain, attacker: Ent, damage: f32) {
        f.expect("NULL function")(self, attacker, damage)
    }

    /// Calls a function an entity carries, with `self` set to it, and puts `self` and `other`
    /// back after, as the engine does around every call into the game.
    pub fn call_as(&mut self, me: Ent, other: Ent, f: fn(&mut Game)) {
        let (s, o) = (self.self_, self.other);
        self.self_ = me;
        self.other = other;
        f(self);
        self.self_ = s;
        self.other = o;
    }
}
