//! A level begun and played (`sv_main.c`, `pr_edict.c`, `host_cmd.c`, `host.c`): the map's
//! entities spawned from its text, the player put in, each frame of the server, and what the
//! player carries to the next level.

use crate::{
    Edict, Event, Game, MAX_CLIENTS, MOVETYPE_PUSH, SOLID_BSP, SPAWNFLAG_NOT_DEATHMATCH, SPAWNFLAG_NOT_EASY, SPAWNFLAG_NOT_HARD, SPAWNFLAG_NOT_MEDIUM, UserCmd, intern, world::World,
};
use data::bsp::Bsp;
use data::pak::Packs;
use std::sync::Arc;

/// The server's own seconds a frame: at most 72 a second, as the original host ran
/// (`Host_FilterTime` refuses frames closer than 1/72 s apart).
pub const TICK: f32 = 1.0 / 72.0;

/// What outlives a level (`server_static_t`): the settings, the episode's sigils and what the
/// player carries.
pub struct Carried {
    pub cvars: crate::Cvars,
    pub serverflags: i32,
    pub spawn_parms: [f32; 16],
    /// Whether the player has had `SetNewParms` yet (`SV_ConnectClient`).
    pub connected: bool,
}

impl Default for Carried {
    fn default() -> Self {
        Carried { cvars: crate::Cvars::new(), serverflags: 0, spawn_parms: [0.0; 16], connected: false }
    }
}

impl Game {
    /// `SV_SpawnServer`: a new level, its entities spawned, the world run two tenths of a second
    /// to settle, and the player connected and put in (`SV_ConnectClient`, `Host_Spawn_f`).
    pub fn spawn_server(files: Arc<Packs>, mapname: &str, carried: Carried) -> Result<Game, String> {
        let path = format!("maps/{mapname}.bsp");
        let bytes = files.get(&path).ok_or_else(|| format!("Couldn't spawn server {path}"))?;
        let bsp = Bsp::parse(bytes).map_err(|e| format!("{path}: {e}"))?;
        let mut g = Game::new(World::from_bsp(&bsp));
        g.files = Some(files);
        g.cvars = carried.cvars;
        if g.cvars.value("coop") != 0.0 {
            g.cvars.0.insert("deathmatch".into(), "0".into());
        }
        let skill = ((g.cvars.value("skill") + 0.5) as i32).clamp(0, 3);
        g.cvars.0.insert("skill".into(), skill.to_string());
        for _ in 0..=MAX_CLIENTS {
            g.ents.push(Edict::new());
        }
        g.sv_time = 1.0;
        g.loading = true;
        let modelname = intern(&path);
        g.sounds.push("");
        g.models.push(crate::Model { name: "", mins: glam::Vec3::ZERO, maxs: glam::Vec3::ZERO, brush: None });
        g.models.push(crate::Model { name: modelname, mins: g.map.submodels[0].mins, maxs: g.map.submodels[0].maxs, brush: Some(0) });
        for i in 1..g.map.submodels.len() {
            let m = g.load_model(intern(&format!("*{i}")));
            g.models.push(m);
        }
        let w = &mut g.ents[0].v;
        w.model = modelname;
        w.modelindex = 1.0;
        w.solid = SOLID_BSP;
        w.movetype = MOVETYPE_PUSH;
        if g.cvars.value("coop") != 0.0 {
            g.coop = g.cvars.value("coop");
        } else {
            g.deathmatch = g.cvars.value("deathmatch");
        }
        g.mapname = intern(mapname);
        g.serverflags = carried.serverflags;
        g.load_from_file(&bsp.entities, skill);
        g.loading = false;
        g.frame_seconds = 0.1;
        g.physics();
        g.physics();

        // SV_ConnectClient: a player new to the game gets the game's starting things.
        g.client_active = true;
        g.spawn_parms = carried.spawn_parms;
        if !carried.connected {
            g.self_ = 1;
            crate::progs::SetNewParms(&mut g);
            g.spawn_parms = g.parm;
        }
        g.spawn_client();
        Ok(g)
    }

    /// `ED_LoadFromFile`: every entity of the map's text spawned by its classname's function;
    /// those of another skill or of deathmatch left out.
    fn load_from_file(&mut self, text: &str, skill: i32) {
        self.time = self.sv_time as f32;
        let deathmatch = self.cvars.value("deathmatch") != 0.0;
        for (n, fields) in data::entities::parse(text).into_iter().enumerate() {
            let e = if n == 0 { 0 } else { self.spawn() };
            if e != 0 {
                self.ents[e].v = Default::default();
            }
            for (key, value) in &fields {
                // the map's "angle" is the yaw; its "light" is the light's level (`light_lev`)
                let (key, value) = match key.trim_end_matches(' ') {
                    "angle" => ("angles", format!("0 {value} 0")),
                    "light" => ("light_lev", value.clone()),
                    k => (k, value.clone()),
                };
                if key.starts_with('_') {
                    continue;
                }
                if !self.ents[e].v.set_from_map(key, &value) {
                    self.events.push(Event::Print(format!("'{key}' is not a field\n")));
                }
            }
            let flags = self.ents[e].v.spawnflags;
            let inhibited = if deathmatch {
                flags & SPAWNFLAG_NOT_DEATHMATCH != 0
            } else {
                (skill == 0 && flags & SPAWNFLAG_NOT_EASY != 0) || (skill == 1 && flags & SPAWNFLAG_NOT_MEDIUM != 0) || (skill >= 2 && flags & SPAWNFLAG_NOT_HARD != 0)
            };
            if inhibited {
                self.remove(e);
                continue;
            }
            let classname = self.ents[e].v.classname;
            if classname.is_empty() {
                self.events.push(Event::Print("No classname\n".into()));
                self.remove(e);
                continue;
            }
            let Some(f) = crate::progs::spawn_function(classname) else {
                self.events.push(Event::Print(format!("No spawn function for {classname}\n")));
                self.remove(e);
                continue;
            };
            self.self_ = e;
            f(self);
        }
    }

    /// `Host_Spawn_f`: the player's entity cleared and filled by the game (`ClientConnect`,
    /// `PutClientInServer`), with what they carried in `parm1`…`parm16`.
    fn spawn_client(&mut self) {
        let p = 1;
        self.ents[p] = Edict::new();
        let v = &mut self.ents[p].v;
        v.colormap = p as f32;
        v.team = 1.0; // colours 0: (0 & 15) + 1
        v.netname = intern("player");
        self.parm = self.spawn_parms;
        self.time = self.sv_time as f32;
        self.self_ = p;
        crate::progs::ClientConnect(self);
        crate::progs::PutClientInServer(self);
        self.client_spawned = true;
        let a = self.ents[p].v.angles;
        self.events.push(Event::SetAngle(glam::Vec3::new(a.x, a.y, 0.0)));
        for (i, s) in self.lightstyles.clone().into_iter().enumerate() {
            self.events.push(Event::LightStyle(i, s));
        }
    }

    /// `SV_SaveSpawnparms`: before the next level, what the player carries (`SetChangeParms`).
    pub fn save_spawn_parms(&mut self) -> Carried {
        self.self_ = 1;
        crate::progs::SetChangeParms(self);
        Carried { cvars: std::mem::take(&mut self.cvars), serverflags: self.serverflags, spawn_parms: self.parm, connected: true }
    }

    /// One frame of the server (`Host_ServerFrame`): the player's command, the world's physics,
    /// and what the player's screen must know (`SV_WriteClientdataToMessage`). `frame_seconds`
    /// is the frame's length; the host calls this at most 72 times a second.
    pub fn server_frame(&mut self, seconds: f32, cmd: UserCmd, in_game: bool) {
        self.frame_seconds = seconds;
        self.frametime = seconds;
        self.run_client(cmd);
        if !self.paused && in_game {
            self.physics();
        }
        self.write_client_data();
        // SV_CleanupEnts: a muzzle flash lasts the one frame it was sent in.
        for (i, e) in self.ents.iter_mut().enumerate() {
            if !e.free && e.v.effects & crate::EF_MUZZLEFLASH != 0 {
                self.events.push(Event::MuzzleFlash(i));
                e.v.effects &= !crate::EF_MUZZLEFLASH;
            }
        }
    }

    /// `SV_WriteClientdataToMessage`'s part that changes the player: the damage taken is told
    /// once and cleared, the view's pitch follows the slope, and a turned view is told once.
    fn write_client_data(&mut self) {
        let p = 1;
        let v = &self.ents[p].v;
        if v.dmg_take != 0.0 || v.dmg_save != 0.0 {
            let o = &self.ents[v.dmg_inflictor].v;
            let from = o.origin + 0.5 * (o.mins + o.maxs);
            self.events.push(Event::Damage { armor: v.dmg_save, blood: v.dmg_take, from });
            let v = &mut self.ents[p].v;
            v.dmg_take = 0.0;
            v.dmg_save = 0.0;
        }
        self.set_ideal_pitch(p);
        if self.ents[p].v.fixangle != 0 {
            let a = self.ents[p].v.angles;
            self.events.push(Event::SetAngle(a));
            self.ents[p].v.fixangle = 0;
        }
    }
}
