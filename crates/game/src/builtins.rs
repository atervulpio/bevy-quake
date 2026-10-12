//! The engine's functions the game calls (`pr_cmds.c`, QuakeC's `#1` to `#78`), each under its
//! QuakeC name. The message writes (`WriteByte` and the rest) are not here: what each of id's
//! write sequences says is one call below (`temp_entity`, `finale`, …), so the game's code
//! names what it sends.

use crate::world::Trace;
use crate::{Ent, EntVars, Event, FL_FLY, FL_NOTARGET, FL_ONGROUND, FL_SWIM, Game, MAX_CLIENTS, Model, SOLID_NOT, Str, TempEntity, intern};
use glam::Vec3;

/// `normalize`: the vector made one long, or nought if it is nought.
pub fn normalize(v: Vec3) -> Vec3 {
    let l = v.length();
    if l == 0.0 { Vec3::ZERO } else { v * (1.0 / l) }
}

/// `vlen`.
pub fn vlen(v: Vec3) -> f32 {
    v.length()
}

/// `vectoyaw`: the direction's yaw in whole degrees, 0 to 359.
pub fn vectoyaw(v: Vec3) -> f32 {
    if v.y == 0.0 && v.x == 0.0 {
        return 0.0;
    }
    let mut yaw = ((v.y.atan2(v.x) * 180.0 / std::f32::consts::PI) as i32) as f32;
    if yaw < 0.0 {
        yaw += 360.0;
    }
    yaw
}

/// `vectoangles`: pitch and yaw in whole degrees, roll nought.
pub fn vectoangles(v: Vec3) -> Vec3 {
    let (pitch, yaw);
    if v.y == 0.0 && v.x == 0.0 {
        yaw = 0.0;
        pitch = if v.z > 0.0 { 90.0 } else { 270.0 };
    } else {
        let mut y = ((v.y.atan2(v.x) * 180.0 / std::f32::consts::PI) as i32) as f32;
        if y < 0.0 {
            y += 360.0;
        }
        yaw = y;
        let forward = (v.x * v.x + v.y * v.y).sqrt();
        let mut p = ((v.z.atan2(forward) * 180.0 / std::f32::consts::PI) as i32) as f32;
        if p < 0.0 {
            p += 360.0;
        }
        pitch = p;
    }
    Vec3::new(pitch, yaw, 0.0)
}

/// `rint`: rounded half away from nought.
pub fn rint(f: f32) -> f32 {
    if f > 0.0 { ((f + 0.5) as i32) as f32 } else { ((f - 0.5) as i32) as f32 }
}

pub fn floor(f: f32) -> f32 {
    f.floor()
}

pub fn ceil(f: f32) -> f32 {
    f.ceil()
}

pub fn fabs(f: f32) -> f32 {
    f.abs()
}

/// `ftos`: a whole number as itself, any other with one decimal in a field of five.
pub fn ftos(v: f32) -> String {
    if v == (v as i32) as f32 { format!("{}", v as i32) } else { format!("{v:5.1}") }
}

/// `vtos`.
pub fn vtos(v: Vec3) -> String {
    format!("'{:5.1} {:5.1} {:5.1}'", v.x, v.y, v.z)
}

/// `AngleVectors` (`mathlib.c`): forward, right and up from pitch, yaw and roll in degrees.
pub fn angle_vectors(a: Vec3) -> (Vec3, Vec3, Vec3) {
    let (sp, cp) = (a.x * std::f32::consts::PI * 2.0 / 360.0).sin_cos();
    let (sy, cy) = (a.y * std::f32::consts::PI * 2.0 / 360.0).sin_cos();
    let (sr, cr) = (a.z * std::f32::consts::PI * 2.0 / 360.0).sin_cos();
    let forward = Vec3::new(cp * cy, cp * sy, -sp);
    let right = Vec3::new(-sr * sp * cy + -cr * -sy, -sr * sp * sy + -cr * cy, -sr * cp);
    let up = Vec3::new(cr * sp * cy + -sr * -sy, cr * sp * sy + -sr * cy, cr * cp);
    (forward, right, up)
}

impl Game {
    /// `makevectors`: `v_forward`, `v_right` and `v_up` from the angles.
    pub fn makevectors(&mut self, ang: Vec3) {
        let (f, r, u) = angle_vectors(ang);
        self.v_forward = f;
        self.v_right = r;
        self.v_up = u;
    }

    /// `setorigin`: the one way to move an entity other than by the world's physics.
    pub fn setorigin(&mut self, e: Ent, o: Vec3) {
        self.ents[e].v.origin = o;
        self.link_edict(e, false);
    }

    /// `SetMinMaxSize`.
    fn set_min_max_size(&mut self, e: Ent, min: Vec3, max: Vec3) {
        assert!(min.cmple(max).all(), "backwards mins/maxs");
        let v = &mut self.ents[e].v;
        v.mins = min;
        v.maxs = max;
        v.size = max - min;
        self.link_edict(e, false);
    }

    /// `setsize`.
    pub fn setsize(&mut self, e: Ent, min: Vec3, max: Vec3) {
        self.set_min_max_size(e, min, max);
    }

    /// `setmodel`: the model, its number in the precache list, and its size.
    pub fn setmodel(&mut self, e: Ent, m: Str) {
        let i = self.models.iter().position(|x| x.name == m).unwrap_or_else(|| panic!("no precache: {m}"));
        self.ents[e].v.model = m;
        self.ents[e].v.modelindex = i as f32;
        let (mins, maxs) = (self.models[i].mins, self.models[i].maxs);
        self.set_min_max_size(e, mins, maxs);
    }

    /// `random`: a number from 0 to 1, both ends possible (`(rand() & 0x7fff) / 0x7fff`).
    pub fn random(&mut self) -> f32 {
        self.rand() as f32 / 32767.0
    }

    /// `random() + random() + random()` as id's compiler built it, and so as the game shipped
    /// played it. qcc returns every call's value in one shared slot (`def_ret`,
    /// `PR_ParseFunctionCall`) and does not copy it out, so the first sum adds the second roll to
    /// itself: the three rolls are taken, and the answer is twice the second plus the third.
    pub fn random_sum3_as_compiled(&mut self) -> f32 {
        let _first = self.random();
        let second = self.random();
        let third = self.random();
        2.0 * second + third
    }

    /// `random() + random()` as id's compiler built it: both rolls taken, twice the second
    /// answered (see `random_sum3_as_compiled`).
    pub fn random_sum2_as_compiled(&mut self) -> f32 {
        let _first = self.random();
        2.0 * self.random()
    }

    /// `sound`: a sound from the entity's middle on one of its channels.
    pub fn sound(&mut self, e: Ent, chan: i32, samp: Str, vol: f32, atten: f32) {
        let volume = (vol * 255.0) as i32;
        assert!((0..=255).contains(&volume), "SV_StartSound: volume = {volume}");
        assert!((0.0..=4.0).contains(&atten), "SV_StartSound: attenuation = {atten}");
        assert!((0..=7).contains(&chan), "SV_StartSound: channel = {chan}");
        if !self.sounds.contains(&samp) {
            self.events.push(Event::Print(format!("SV_StartSound: {samp} not precacheed\n")));
            return;
        }
        let v = &self.ents[e].v;
        let origin = v.origin + (v.mins + v.maxs) * 0.5;
        self.events.push(Event::Sound { ent: e, channel: chan, sample: samp, volume: volume as f32 / 255.0, attenuation: atten, origin });
    }

    /// `error`: a fault in the game's logic; the original ends the game with "Program error".
    pub fn error(&mut self, s: &str) -> ! {
        panic!("SERVER ERROR: {s} (self is entity {})", self.self_)
    }

    /// `objerror`: a fault of one entity; the original frees it and ends the game.
    pub fn objerror(&mut self, s: &str) -> ! {
        let me = self.self_;
        let classname = self.ents[me].v.classname;
        self.remove(me);
        panic!("OBJECT ERROR: {s} (a {classname}, entity {me})")
    }

    /// `spawn` (`ED_Alloc`): the first free entity freed over half a second ago (or since the
    /// level began), else a new one.
    pub fn spawn(&mut self) -> Ent {
        for i in (MAX_CLIENTS + 1)..self.ents.len() {
            let e = &self.ents[i];
            if e.free && (e.freetime < 2.0 || self.time - e.freetime > 0.5) {
                self.ents[i] = crate::Edict::new();
                return i;
            }
        }
        assert!(self.ents.len() < crate::MAX_EDICTS, "ED_Alloc: no free edicts");
        self.ents.push(crate::Edict::new());
        self.ents.len() - 1
    }

    /// `remove` (`ED_Free`): unlinked and its number kept free for a while. Only these fields
    /// are cleared, as in the original; the rest (its classname, its health) stay readable by
    /// whoever still holds its number.
    pub fn remove(&mut self, e: Ent) {
        self.unlink_edict(e);
        let ed = &mut self.ents[e];
        ed.free = true;
        let v = &mut ed.v;
        v.model = "";
        v.takedamage = 0;
        v.modelindex = 0.0;
        v.colormap = 0.0;
        v.skin = 0.0;
        v.frame = 0.0;
        v.origin = Vec3::ZERO;
        v.angles = Vec3::ZERO;
        v.nextthink = -1.0;
        v.solid = 0;
        ed.freetime = self.time;
    }

    fn set_trace_globals(&mut self, t: &Trace) {
        self.trace_allsolid = t.allsolid;
        self.trace_startsolid = t.startsolid;
        self.trace_fraction = t.fraction;
        self.trace_inwater = t.inwater;
        self.trace_inopen = t.inopen;
        self.trace_endpos = t.endpos;
        self.trace_plane_normal = t.normal;
        self.trace_plane_dist = t.dist;
        self.trace_ent = t.ent.unwrap_or(0);
    }

    /// `traceline`: a point traced from `v1` to `v2`; `nomonsters` 1 passes through all but the
    /// map and its brush models. The answer is in the `trace_*` globals.
    pub fn traceline(&mut self, v1: Vec3, v2: Vec3, nomonsters: f32, forent: Ent) {
        let t = self.trace_move(v1, Vec3::ZERO, Vec3::ZERO, v2, nomonsters as i32, forent);
        self.set_trace_globals(&t);
    }

    /// `PF_newcheckclient`: the next living, visible player, and what it can see.
    fn newcheckclient(&mut self, check: usize) -> usize {
        let check = check.clamp(1, MAX_CLIENTS);
        let mut i = if check == MAX_CLIENTS { 1 } else { check + 1 };
        loop {
            if i == MAX_CLIENTS + 1 {
                i = 1;
            }
            let ent = &self.ents[i];
            if i == check {
                break;
            }
            if ent.free || ent.v.health <= 0.0 || ent.v.flags & FL_NOTARGET != 0 {
                i += 1;
                continue;
            }
            break;
        }
        let org = self.ents[i].v.origin + self.ents[i].v.view_ofs;
        let leaf = self.map.point_in_leaf(org);
        self.checkpvs = self.map.leaf_pvs[leaf].clone();
        i
    }

    /// `checkclient`: the player, if `self` could see them; `world` otherwise.
    pub fn checkclient(&mut self) -> Ent {
        if self.time - self.lastchecktime >= 0.1 {
            self.lastcheck = self.newcheckclient(self.lastcheck);
            self.lastchecktime = self.time;
        }
        let ent = self.lastcheck;
        if ent >= self.ents.len() || self.ents[ent].free || self.ents[ent].v.health <= 0.0 {
            return 0;
        }
        let me = self.self_;
        let view = self.ents[me].v.origin + self.ents[me].v.view_ofs;
        let leaf = self.map.point_in_leaf(view) as i32 - 1;
        if leaf < 0 || self.checkpvs.get((leaf >> 3) as usize).is_none_or(|b| b & (1 << (leaf & 7)) == 0) {
            return 0;
        }
        ent
    }

    /// `find`: the next entity after `start` whose string field equals `m`, or `world`.
    pub fn find(&self, start: Ent, field: fn(&EntVars) -> Str, m: &str) -> Ent {
        for e in start + 1..self.ents.len() {
            let ed = &self.ents[e];
            if !ed.free && field(&ed.v) == m {
                return e;
            }
        }
        0
    }

    /// `precache_sound`: a sound the level may play; only while it is being spawned.
    pub fn precache_sound(&mut self, s: Str) -> Str {
        assert!(self.loading, "PF_Precache_*: Precache can only be done in spawn functions");
        assert!(s.as_bytes().first().is_some_and(|&c| c > b' '), "Bad string");
        if !self.sounds.contains(&s) {
            assert!(self.sounds.len() < 256, "PF_precache_sound: overflow");
            self.sounds.push(s);
        }
        s
    }

    /// `precache_model`: a model the level may show, loaded for its size.
    pub fn precache_model(&mut self, s: Str) -> Str {
        assert!(self.loading, "PF_Precache_*: Precache can only be done in spawn functions");
        assert!(s.as_bytes().first().is_some_and(|&c| c > b' '), "Bad string");
        if !self.models.iter().any(|m| m.name == s) {
            assert!(self.models.len() < 256, "PF_precache_model: overflow");
            let m = self.load_model(s);
            self.models.push(m);
        }
        s
    }

    /// What a model's name loads to, as far as the server needs: its box, and for a map's own
    /// brush model (`*1`, `*2`, …) its number.
    pub fn load_model(&self, s: Str) -> Model {
        if let Some(n) = s.strip_prefix('*') {
            let i: usize = n.parse().expect("a brush model's number");
            let sub = &self.map.submodels[i];
            return Model { name: s, mins: sub.mins, maxs: sub.maxs, brush: Some(i) };
        }
        let bytes = self.files.as_ref().and_then(|f| f.get(s));
        let (mins, maxs) = match (s.rsplit('.').next(), bytes) {
            (Some("spr"), Some(b)) => {
                let spr = data::spr::Spr::parse(b).expect("a sprite");
                let (w, h) = ((spr.width / 2) as f32, (spr.height / 2) as f32);
                (Vec3::new(-w, -w, -h), Vec3::new(w, w, h))
            }
            (Some("bsp"), Some(b)) => {
                let bsp = data::bsp::Bsp::parse(b).expect("a brush model");
                (bsp.models[0].mins - Vec3::ONE, bsp.models[0].maxs + Vec3::ONE)
            }
            _ => (Vec3::splat(-16.0), Vec3::splat(16.0)),
        };
        Model { name: s, mins, maxs, brush: None }
    }

    /// `precache_file`: only for id's copying tools; nothing at play.
    pub fn precache_file(&mut self, s: Str) -> Str {
        s
    }

    pub fn precache_model2(&mut self, s: Str) -> Str {
        self.precache_model(s)
    }

    pub fn precache_sound2(&mut self, s: Str) -> Str {
        self.precache_sound(s)
    }

    pub fn precache_file2(&mut self, s: Str) -> Str {
        s
    }

    /// `stuffcmd`: a command run on the player's side. The game sends only "bf\n".
    pub fn stuffcmd(&mut self, client: Ent, s: &str) {
        assert!((1..=MAX_CLIENTS).contains(&client), "Parm 0 not a client");
        for line in s.lines() {
            match line.trim() {
                "bf" => self.events.push(Event::BonusFlash),
                "" => {}
                other => self.events.push(Event::Command(other.to_string())),
            }
        }
    }

    /// `findradius`: every entity that can be touched whose middle is within `rad`, as a chain
    /// through `.chain`, the last found first.
    pub fn findradius(&mut self, org: Vec3, rad: f32) -> Ent {
        let mut chain = 0;
        for e in 1..self.ents.len() {
            let ed = &mut self.ents[e];
            if ed.free || ed.v.solid == SOLID_NOT {
                continue;
            }
            let eorg = org - (ed.v.origin + (ed.v.mins + ed.v.maxs) * 0.5);
            if eorg.length() > rad {
                continue;
            }
            ed.v.chain = chain;
            chain = e;
        }
        chain
    }

    pub fn bprint(&mut self, s: &str) {
        self.events.push(Event::Print(s.to_string()));
    }

    pub fn sprint(&mut self, client: Ent, s: &str) {
        if (1..=MAX_CLIENTS).contains(&client) {
            self.events.push(Event::Print(s.to_string()));
        }
    }

    /// `dprint`: shown only with `developer` set; kept out of the player's way.
    pub fn dprint(&mut self, _s: &str) {}

    pub fn centerprint(&mut self, client: Ent, s: &str) {
        if (1..=MAX_CLIENTS).contains(&client) {
            self.events.push(Event::CenterPrint(s.to_string()));
        }
    }

    /// `walkmove`: `self` steps `dist` along `yaw` if it stands, flies or swims.
    pub fn walkmove(&mut self, yaw: f32, dist: f32) -> bool {
        let ent = self.self_;
        if self.ents[ent].v.flags & (FL_ONGROUND | FL_FLY | FL_SWIM) == 0 {
            return false;
        }
        let r = yaw * std::f32::consts::PI * 2.0 / 360.0;
        let mv = Vec3::new(r.cos() * dist, r.sin() * dist, 0.0);
        let me = self.self_;
        let ok = self.movestep(ent, mv, true);
        self.self_ = me;
        ok
    }

    /// `droptofloor`: `self` dropped up to 256 units onto what is below it.
    pub fn droptofloor(&mut self) -> bool {
        let ent = self.self_;
        let v = &self.ents[ent].v;
        let (o, mins, maxs) = (v.origin, v.mins, v.maxs);
        let t = self.trace_move(o, mins, maxs, o - Vec3::new(0.0, 0.0, 256.0), 0, ent);
        if t.fraction == 1.0 || t.allsolid {
            return false;
        }
        self.ents[ent].v.origin = t.endpos;
        self.link_edict(ent, false);
        self.ents[ent].v.flags |= FL_ONGROUND;
        self.ents[ent].v.groundentity = t.ent.unwrap_or(0);
        true
    }

    /// `lightstyle`: one of the 64 brightness patterns, as letters from `a` (dark) to `z`.
    pub fn lightstyle(&mut self, style: f32, val: Str) {
        let s = style as usize;
        self.lightstyles[s] = val;
        self.events.push(Event::LightStyle(s, val));
    }

    /// `checkbottom`.
    pub fn checkbottom(&mut self, e: Ent) -> bool {
        self.check_bottom(e)
    }

    /// `pointcontents`.
    pub fn pointcontents(&self, v: Vec3) -> i32 {
        self.point_contents(v)
    }

    /// `aim`: the direction to shoot along, bent towards the target nearest the line of sight
    /// within `sv_aim`'s cosine, as long as nothing stands between.
    pub fn aim(&mut self, ent: Ent, _speed: f32) -> Vec3 {
        let mut start = self.ents[ent].v.origin;
        start.z += 20.0;
        let fwd = self.v_forward;
        let end = start + fwd * 2048.0;
        let tr = self.trace_move(start, Vec3::ZERO, Vec3::ZERO, end, 0, ent);
        let teamplay = self.cvars.value("teamplay") != 0.0;
        if let Some(t) = tr.ent {
            let tv = &self.ents[t].v;
            let me = &self.ents[ent].v;
            if tv.takedamage == crate::DAMAGE_AIM && (!teamplay || me.team <= 0.0 || me.team != tv.team) {
                return fwd;
            }
        }
        let mut bestdist = self.cvars.value("sv_aim");
        let mut bestent = None;
        for check in 1..self.ents.len() {
            let c = &self.ents[check].v;
            if self.ents[check].free || c.takedamage != crate::DAMAGE_AIM || check == ent {
                continue;
            }
            let me = &self.ents[ent].v;
            if teamplay && me.team > 0.0 && me.team == c.team {
                continue;
            }
            let end = c.origin + 0.5 * (c.mins + c.maxs);
            let dir = normalize(end - start);
            let dist = dir.dot(fwd);
            if dist < bestdist {
                continue;
            }
            let tr = self.trace_move(start, Vec3::ZERO, Vec3::ZERO, end, 0, ent);
            if tr.ent == Some(check) {
                bestdist = dist;
                bestent = Some(check);
            }
        }
        match bestent {
            Some(b) => {
                let dir = self.ents[b].v.origin - self.ents[ent].v.origin;
                let dist = dir.dot(fwd);
                let mut end = fwd * dist;
                end.z = dir.z;
                normalize(end)
            }
            None => fwd,
        }
    }

    /// `cvar`: a setting's value.
    pub fn cvar(&self, name: &str) -> f32 {
        self.cvars.value(name)
    }

    /// `cvar_set`.
    pub fn cvar_set(&mut self, name: &str, val: &str) {
        self.cvars.0.insert(name.to_string(), val.to_string());
    }

    /// `localcmd`: a console command for the host, run after the frame.
    pub fn localcmd(&mut self, s: &str) {
        for line in s.lines().map(str::trim).filter(|l| !l.is_empty()) {
            self.events.push(Event::Command(line.to_string()));
        }
    }

    /// `nextent`: the next entity in use after `e`, or `world`.
    pub fn nextent(&self, e: Ent) -> Ent {
        (e + 1..self.ents.len()).find(|&i| !self.ents[i].free).unwrap_or(0)
    }

    /// `particle`: `count` particles of palette colour `color` along `dir`.
    pub fn particle(&mut self, o: Vec3, d: Vec3, color: f32, count: f32) {
        self.events.push(Event::Particle { origin: o, dir: d, colour: color as i32, count: count as i32 });
    }

    /// `ChangeYaw`: `self` turned towards its `ideal_yaw` by at most its `yaw_speed`.
    pub fn ChangeYaw(&mut self) {
        let me = self.self_;
        self.change_yaw(me);
    }

    /// `makestatic`: the entity frozen into the level's scenery and freed.
    pub fn makestatic(&mut self, e: Ent) {
        let v = &self.ents[e].v;
        self.events.push(Event::Static { model: v.model, frame: v.frame, skin: v.skin, origin: v.origin, angles: v.angles });
        self.remove(e);
    }

    /// `changelevel`: the host asked once to load the next map.
    pub fn changelevel(&mut self, s: &str) {
        if self.changelevel_issued {
            return;
        }
        self.changelevel_issued = true;
        self.events.push(Event::Command(format!("changelevel {s}")));
    }

    /// `ambientsound`: a sound looping at a place.
    pub fn ambientsound(&mut self, pos: Vec3, samp: Str, vol: f32, atten: f32) {
        if !self.sounds.contains(&samp) {
            self.events.push(Event::Print(format!("no precache: {samp}\n")));
            return;
        }
        self.events.push(Event::AmbientSound { origin: pos, sample: samp, volume: vol, attenuation: atten });
    }

    /// `setspawnparms`: the `parm` globals set to what the player carried at the level's start.
    pub fn setspawnparms(&mut self, e: Ent) {
        assert!((1..=MAX_CLIENTS).contains(&e), "Entity is not a client");
        self.parm = self.spawn_parms;
    }

    // ---- the messages id's game writes byte by byte, each as what it says ---------------------

    /// `WriteByte (MSG_BROADCAST, SVC_TEMPENTITY); WriteByte (…, TE_…); …`.
    pub fn temp_entity(&mut self, t: TempEntity) {
        self.events.push(Event::TempEntity(t));
    }

    /// `WriteByte (MSG_ALL, SVC_KILLEDMONSTER)`.
    pub fn msg_killed_monster(&mut self) {
        self.events.push(Event::KilledMonster);
    }

    /// `WriteByte (MSG_ALL, SVC_FOUNDSECRET)`.
    pub fn msg_found_secret(&mut self) {
        self.events.push(Event::FoundSecret);
    }

    /// `WriteByte (MSG_ALL, SVC_INTERMISSION)`.
    pub fn msg_intermission(&mut self) {
        self.events.push(Event::Intermission);
    }

    /// `WriteByte (MSG_ALL, SVC_FINALE); WriteString (MSG_ALL, text)`.
    pub fn msg_finale(&mut self, text: &str) {
        self.events.push(Event::Finale(text.to_string()));
    }

    /// `WriteByte (MSG_ALL, SVC_CDTRACK); WriteByte (…, track); WriteByte (…, looptrack)`.
    pub fn msg_cdtrack(&mut self, track: f32, looptrack: f32) {
        self.events.push(Event::CdTrack(track as i32, looptrack as i32));
    }

    /// `WriteByte (MSG_ALL, SVC_SELLSCREEN)`.
    pub fn msg_sellscreen(&mut self) {
        self.events.push(Event::SellScreen);
    }

    /// A string made at play (`ftos`'s and the like) kept as one of the game's strings.
    pub fn string(&self, s: &str) -> Str {
        intern(s)
    }
}
