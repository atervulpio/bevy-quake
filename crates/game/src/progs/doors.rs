//! `doors.qc`, id's QuakeC 1.06, in Rust.
//!
//! Doors are like buttons, but can spawn a fat trigger field around them to open without a
//! touch, and they link together into double or quad doors. `owner` is the master door (itself
//! when alone); `enemy` chains from the master through every door of the set.

use super::*;

const DOOR_START_OPEN: i32 = 1;
const DOOR_DONT_LINK: i32 = 4;
const DOOR_GOLD_KEY: i32 = 8;
const DOOR_SILVER_KEY: i32 = 16;
const DOOR_TOGGLE: i32 = 32;

// ---- think functions --------------------------------------------------------------------------

pub fn door_blocked(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    let dmg = g.r(s).dmg;
    T_Damage(g, o, s, s, dmg);

    // a door with a negative wait would never come back if blocked, so it just squashes the
    // object to death real fast
    let s = g.self_;
    if g.r(s).wait >= 0.0 {
        if g.r(s).state == STATE_DOWN {
            door_go_up(g);
        } else {
            door_go_down(g);
        }
    }
}

pub fn door_hit_top(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise1, 1.0, ATTN_NORM);
    g.v(s).state = STATE_TOP;
    if (g.r(s).spawnflags & DOOR_TOGGLE) != 0 {
        return; // don't come down automatically
    }

    g.v(s).think = Some(door_go_down);
    g.v(s).nextthink = g.r(s).ltime + g.r(s).wait;
}

pub fn door_hit_bottom(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise1, 1.0, ATTN_NORM);
    g.v(s).state = STATE_BOTTOM;
}

pub fn door_go_down(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise2, 1.0, ATTN_NORM);
    if g.r(s).max_health != 0.0 {
        g.v(s).takedamage = DAMAGE_YES;
        g.v(s).health = g.r(s).max_health;
    }

    g.v(s).state = STATE_DOWN;
    let (dest, speed) = (g.r(s).pos1, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, door_hit_bottom);
}

pub fn door_go_up(g: &mut Game) {
    let s = g.self_;
    if g.r(s).state == STATE_UP {
        return; // already going up
    }

    if g.r(s).state == STATE_TOP {
        // reset top wait time
        g.v(s).nextthink = g.r(s).ltime + g.r(s).wait;
        return;
    }

    g.sound(s, CHAN_VOICE, g.r(s).noise2, 1.0, ATTN_NORM);
    g.v(s).state = STATE_UP;
    let (dest, speed) = (g.r(s).pos2, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, door_hit_top);

    SUB_UseTargets(g);
}

// ---- activation functions ---------------------------------------------------------------------

pub fn door_fire(g: &mut Game) {
    let s = g.self_;
    if g.r(s).owner != s {
        g.objerror("door_fire: self.owner != self");
    }

    // play use key sound
    if g.r(s).items != 0 {
        g.sound(s, CHAN_VOICE, g.r(s).noise4, 1.0, ATTN_NORM);
    }

    g.v(s).message = g.string_null; // no more message
    let oself = s;

    if (g.r(s).spawnflags & DOOR_TOGGLE) != 0
        && (g.r(s).state == STATE_UP || g.r(s).state == STATE_TOP) {
            let starte = g.self_;
            loop {
                door_go_down(g);
                g.self_ = g.r(g.self_).enemy;
                if !(g.self_ != starte && g.self_ != world) {
                    break;
                }
            }
            g.self_ = oself;
            return;
        }

    // trigger all paired doors
    let starte = g.self_;
    loop {
        door_go_up(g);
        g.self_ = g.r(g.self_).enemy;
        if !(g.self_ != starte && g.self_ != world) {
            break;
        }
    }
    g.self_ = oself;
}

pub fn door_use(g: &mut Game) {
    let s = g.self_;
    g.v(s).message = ""; // door messages are for touch only
    let owner = g.r(s).owner;
    g.v(owner).message = "";
    let enemy = g.r(s).enemy;
    g.v(enemy).message = "";
    let oself = s;
    g.self_ = g.r(s).owner;
    door_fire(g);
    g.self_ = oself;
}

pub fn door_trigger_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).health <= 0.0 {
        return;
    }

    if g.time < g.r(s).attack_finished {
        return;
    }

    g.v(s).attack_finished = g.time + 1.0;

    g.activator = o;

    // self is not put back here; the engine does it after a touch
    g.self_ = g.r(s).owner;
    door_use(g);
}

pub fn door_killed(g: &mut Game) {
    let oself = g.self_;
    g.self_ = g.r(oself).owner;
    let s = g.self_;
    g.v(s).health = g.r(s).max_health;
    g.v(s).takedamage = DAMAGE_NO; // will be reset upon return
    door_use(g);
    g.self_ = oself;
}

/// Prints messages and opens key doors.
pub fn door_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname != "player" {
        return;
    }

    let owner = g.r(s).owner;
    if g.r(owner).attack_finished > g.time {
        return;
    }

    g.v(owner).attack_finished = g.time + 2.0;

    if !g.r(owner).message.is_empty() {
        g.centerprint(o, g.r(owner).message);
        g.sound(o, CHAN_VOICE, "misc/talk.wav", 1.0, ATTN_NORM);
    }

    // key door stuff
    if g.r(s).items == 0 {
        return;
    }

    // FIXME: blink key on player's status bar
    if (g.r(s).items & g.r(o).items) != g.r(s).items {
        let worldtype = g.r(world).worldtype;
        if g.r(owner).items == IT_KEY1 {
            if worldtype == 2.0 {
                g.centerprint(o, "You need the silver keycard");
                g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
            } else if worldtype == 1.0 {
                g.centerprint(o, "You need the silver runekey");
                g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
            } else if worldtype == 0.0 {
                g.centerprint(o, "You need the silver key");
                g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
            }
        } else if worldtype == 2.0 {
            g.centerprint(o, "You need the gold keycard");
            g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
        } else if worldtype == 1.0 {
            g.centerprint(o, "You need the gold runekey");
            g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
        } else if worldtype == 0.0 {
            g.centerprint(o, "You need the gold key");
            g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
        }
        return;
    }

    g.v(o).items = g.r(o).items - g.r(s).items;
    g.v(s).touch = Some(SUB_Null);
    let enemy = g.r(s).enemy;
    if enemy != world {
        g.v(enemy).touch = Some(SUB_Null); // get paired door
    }
    door_use(g);
}

// ---- spawning functions -----------------------------------------------------------------------

pub fn spawn_field(g: &mut Game, fmins: Vec3, fmaxs: Vec3) -> Ent {
    let trigger = g.spawn();
    g.v(trigger).movetype = MOVETYPE_NONE;
    g.v(trigger).solid = SOLID_TRIGGER;
    g.v(trigger).owner = g.self_;
    g.v(trigger).touch = Some(door_trigger_touch);

    let t1 = fmins;
    let t2 = fmaxs;
    g.setsize(trigger, t1 - Vec3::new(60.0, 60.0, 8.0), t2 + Vec3::new(60.0, 60.0, 8.0));
    trigger
}

pub fn EntitiesTouching(g: &mut Game, e1: Ent, e2: Ent) -> f32 {
    let (a, b) = (g.r(e1), g.r(e2));
    if a.mins.x > b.maxs.x {
        return FALSE;
    }
    if a.mins.y > b.maxs.y {
        return FALSE;
    }
    if a.mins.z > b.maxs.z {
        return FALSE;
    }
    if a.maxs.x < b.mins.x {
        return FALSE;
    }
    if a.maxs.y < b.mins.y {
        return FALSE;
    }
    if a.maxs.z < b.mins.z {
        return FALSE;
    }
    TRUE
}

pub fn LinkDoors(g: &mut Game) {
    let s = g.self_;
    if g.r(s).enemy != world {
        return; // already linked by another door
    }

    if (g.r(s).spawnflags & DOOR_DONT_LINK) != 0 {
        // self.owner = self.enemy = self;
        g.v(s).enemy = s;
        g.v(s).owner = s;
        return; // don't want to link this door
    }

    let mut cmins = g.r(s).mins;
    let mut cmaxs = g.r(s).maxs;

    let starte = s;
    let mut t = s;

    loop {
        let me = g.self_;
        g.v(me).owner = starte; // master door

        if g.r(me).health != 0.0 {
            g.v(starte).health = g.r(me).health;
        }
        if !g.r(me).targetname.is_empty() {
            g.v(starte).targetname = g.r(me).targetname;
        }
        if !g.r(me).message.is_empty() {
            g.v(starte).message = g.r(me).message;
        }

        t = g.find(t, |v: &EntVars| v.classname, g.r(me).classname);
        if t == world {
            g.v(me).enemy = starte; // make the chain a loop

            // shootable, fired, or key doors just needed the owner/enemy links, they don't
            // spawn a field
            g.self_ = g.r(me).owner;
            let s = g.self_;
            if g.r(s).health != 0.0 {
                return;
            }
            if !g.r(s).targetname.is_empty() {
                return;
            }
            if g.r(s).items != 0 {
                return;
            }

            let owner = g.r(s).owner;
            let field = spawn_field(g, cmins, cmaxs);
            g.v(owner).trigger_field = field;

            return;
        }

        if EntitiesTouching(g, me, t) != 0.0 {
            if g.r(t).enemy != world {
                g.objerror("cross connected doors");
            }

            g.v(me).enemy = t;
            g.self_ = t;

            let (tmins, tmaxs) = (g.r(t).mins, g.r(t).maxs);
            if tmins.x < cmins.x {
                cmins.x = tmins.x;
            }
            if tmins.y < cmins.y {
                cmins.y = tmins.y;
            }
            if tmins.z < cmins.z {
                cmins.z = tmins.z;
            }
            if tmaxs.x > cmaxs.x {
                cmaxs.x = tmaxs.x;
            }
            if tmaxs.y > cmaxs.y {
                cmaxs.y = tmaxs.y;
            }
            if tmaxs.z > cmaxs.z {
                cmaxs.z = tmaxs.z;
            }
        }
    }
}

/// QUAKED func_door (0 .5 .8) ? START_OPEN x DOOR_DONT_LINK GOLD_KEY SILVER_KEY TOGGLE
///
/// Doors that touch are one unit. TOGGLE waits in both states for a trigger. START_OPEN moves
/// to its destination when spawned and works in reverse. Key doors are always wait -1.
/// "message" (touch, until fired), "angle", "targetname" (no touch field), "health" (shot
/// open), "speed" (100), "wait" (3, -1 never returns), "lip" (8), "dmg" (2), "sounds" (0 none,
/// 1 stone, 2 base, 3 stone chain, 4 screechy metal).
pub fn func_door(g: &mut Game) {
    let s = g.self_;
    let worldtype = g.r(world).worldtype;
    if worldtype == 0.0 {
        g.precache_sound("doors/medtry.wav");
        g.precache_sound("doors/meduse.wav");
        g.v(s).noise3 = "doors/medtry.wav";
        g.v(s).noise4 = "doors/meduse.wav";
    } else if worldtype == 1.0 {
        g.precache_sound("doors/runetry.wav");
        g.precache_sound("doors/runeuse.wav");
        g.v(s).noise3 = "doors/runetry.wav";
        g.v(s).noise4 = "doors/runeuse.wav";
    } else if worldtype == 2.0 {
        g.precache_sound("doors/basetry.wav");
        g.precache_sound("doors/baseuse.wav");
        g.v(s).noise3 = "doors/basetry.wav";
        g.v(s).noise4 = "doors/baseuse.wav";
    } else {
        g.dprint("no worldtype set!\n");
    }

    let sounds = g.r(s).sounds;
    if sounds == 0.0 {
        g.precache_sound("misc/null.wav");
        g.precache_sound("misc/null.wav");
        g.v(s).noise1 = "misc/null.wav";
        g.v(s).noise2 = "misc/null.wav";
    } else if sounds == 1.0 {
        g.precache_sound("doors/drclos4.wav");
        g.precache_sound("doors/doormv1.wav");
        g.v(s).noise1 = "doors/drclos4.wav";
        g.v(s).noise2 = "doors/doormv1.wav";
    } else if sounds == 2.0 {
        g.precache_sound("doors/hydro1.wav");
        g.precache_sound("doors/hydro2.wav");
        g.v(s).noise2 = "doors/hydro1.wav";
        g.v(s).noise1 = "doors/hydro2.wav";
    } else if sounds == 3.0 {
        g.precache_sound("doors/stndr1.wav");
        g.precache_sound("doors/stndr2.wav");
        g.v(s).noise2 = "doors/stndr1.wav";
        g.v(s).noise1 = "doors/stndr2.wav";
    } else if sounds == 4.0 {
        g.precache_sound("doors/ddoor1.wav");
        g.precache_sound("doors/ddoor2.wav");
        g.v(s).noise1 = "doors/ddoor2.wav";
        g.v(s).noise2 = "doors/ddoor1.wav";
    }

    SetMovedir(g);

    g.v(s).max_health = g.r(s).health;
    g.v(s).solid = SOLID_BSP;
    g.v(s).movetype = MOVETYPE_PUSH;
    g.setorigin(s, g.r(s).origin);
    g.setmodel(s, g.r(s).model);
    g.v(s).classname = "door";

    g.v(s).blocked = Some(door_blocked);
    g.v(s).use_ = Some(door_use);

    if (g.r(s).spawnflags & DOOR_SILVER_KEY) != 0 {
        g.v(s).items = IT_KEY1;
    }
    if (g.r(s).spawnflags & DOOR_GOLD_KEY) != 0 {
        g.v(s).items = IT_KEY2;
    }

    if g.r(s).speed == 0.0 {
        g.v(s).speed = 100.0;
    }
    if g.r(s).wait == 0.0 {
        g.v(s).wait = 3.0;
    }
    if g.r(s).lip == 0.0 {
        g.v(s).lip = 8.0;
    }
    if g.r(s).dmg == 0.0 {
        g.v(s).dmg = 2.0;
    }

    g.v(s).pos1 = g.r(s).origin;
    let (pos1, movedir, size, lip) = (g.r(s).pos1, g.r(s).movedir, g.r(s).size, g.r(s).lip);
    // movedir * size is the dot product
    g.v(s).pos2 = pos1 + movedir * (fabs(movedir.dot(size)) - lip);

    // DOOR_START_OPEN is to allow an entity to be lighted in the closed position but spawn in
    // the open position
    if (g.r(s).spawnflags & DOOR_START_OPEN) != 0 {
        g.setorigin(s, g.r(s).pos2);
        g.v(s).pos2 = g.r(s).pos1;
        g.v(s).pos1 = g.r(s).origin;
    }

    g.v(s).state = STATE_BOTTOM;

    if g.r(s).health != 0.0 {
        g.v(s).takedamage = DAMAGE_YES;
        g.v(s).th_die = Some(door_killed);
    }

    if g.r(s).items != 0 {
        g.v(s).wait = -1.0;
    }

    g.v(s).touch = Some(door_touch);

    // LinkDoors can't be done until all of the doors have been spawned, so the sizes can be
    // detected properly.
    g.v(s).think = Some(LinkDoors);
    g.v(s).nextthink = g.r(s).ltime + 0.1;
}

// ---- secret doors -----------------------------------------------------------------------------

const SECRET_OPEN_ONCE: i32 = 1; // stays open
const SECRET_1ST_LEFT: i32 = 2; // 1st move is left of arrow
const SECRET_1ST_DOWN: i32 = 4; // 1st move is down from arrow
const SECRET_NO_SHOOT: i32 = 8; // only opened by trigger
const SECRET_YES_SHOOT: i32 = 16; // shootable even if targeted

/// QuakeC stores the plain `void()` functions `SUB_Null` and `fd_secret_use` in `th_pain`, a
/// `.void(entity, float)` field; called with the two arguments, they read neither. These two
/// carry them in `th_pain`'s type.
fn SUB_Null_as_pain(g: &mut Game, _attacker: Ent, _damage: f32) {
    SUB_Null(g)
}

fn fd_secret_use_as_pain(g: &mut Game, _attacker: Ent, _damage: f32) {
    fd_secret_use(g)
}

pub fn fd_secret_use(g: &mut Game) {
    let s = g.self_;
    g.v(s).health = 10000.0;

    // exit if still moving around...
    if g.r(s).origin != g.r(s).oldorigin {
        return;
    }

    g.v(s).message = g.string_null; // no more message

    SUB_UseTargets(g); // fire all targets / killtargets

    let s = g.self_;
    if (g.r(s).spawnflags & SECRET_NO_SHOOT) == 0 {
        g.v(s).th_pain = Some(SUB_Null_as_pain); // will be reset upon return
        g.v(s).takedamage = DAMAGE_NO; // will be reset upon return
    }
    g.v(s).velocity = Vec3::ZERO;

    // Make a sound, wait a little...
    g.sound(s, CHAN_VOICE, g.r(s).noise1, 1.0, ATTN_NORM);
    g.v(s).nextthink = g.r(s).ltime + 0.1;

    let temp = 1.0 - (g.r(s).spawnflags & SECRET_1ST_LEFT) as f32; // 1 or -1
    g.makevectors(g.r(s).mangle);

    if g.r(s).t_width == 0.0 {
        if (g.r(s).spawnflags & SECRET_1ST_DOWN) != 0 {
            g.v(s).t_width = fabs(g.v_up.dot(g.r(s).size));
        } else {
            g.v(s).t_width = fabs(g.v_right.dot(g.r(s).size));
        }
    }

    if g.r(s).t_length == 0.0 {
        g.v(s).t_length = fabs(g.v_forward.dot(g.r(s).size));
    }

    if (g.r(s).spawnflags & SECRET_1ST_DOWN) != 0 {
        g.v(s).dest1 = g.r(s).origin - g.v_up * g.r(s).t_width;
    } else {
        g.v(s).dest1 = g.r(s).origin + g.v_right * (g.r(s).t_width * temp);
    }

    g.v(s).dest2 = g.r(s).dest1 + g.v_forward * g.r(s).t_length;
    let (dest, speed) = (g.r(s).dest1, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, fd_secret_move1);
    g.sound(s, CHAN_VOICE, g.r(s).noise2, 1.0, ATTN_NORM);
}

/// Wait after first movement...
pub fn fd_secret_move1(g: &mut Game) {
    let s = g.self_;
    g.v(s).nextthink = g.r(s).ltime + 1.0;
    g.v(s).think = Some(fd_secret_move2);
    g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
}

/// Start moving sideways w/ sound...
pub fn fd_secret_move2(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise2, 1.0, ATTN_NORM);
    let (dest, speed) = (g.r(s).dest2, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, fd_secret_move3);
}

/// Wait here until time to go back...
pub fn fd_secret_move3(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
    if (g.r(s).spawnflags & SECRET_OPEN_ONCE) == 0 {
        g.v(s).nextthink = g.r(s).ltime + g.r(s).wait;
        g.v(s).think = Some(fd_secret_move4);
    }
}

/// Move backward...
pub fn fd_secret_move4(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise2, 1.0, ATTN_NORM);
    let (dest, speed) = (g.r(s).dest1, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, fd_secret_move5);
}

/// Wait 1 second...
pub fn fd_secret_move5(g: &mut Game) {
    let s = g.self_;
    g.v(s).nextthink = g.r(s).ltime + 1.0;
    g.v(s).think = Some(fd_secret_move6);
    g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
}

pub fn fd_secret_move6(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise2, 1.0, ATTN_NORM);
    let (dest, speed) = (g.r(s).oldorigin, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, fd_secret_done);
}

pub fn fd_secret_done(g: &mut Game) {
    let s = g.self_;
    if g.r(s).targetname.is_empty() || (g.r(s).spawnflags & SECRET_YES_SHOOT) != 0 {
        g.v(s).health = 10000.0;
        g.v(s).takedamage = DAMAGE_YES;
        g.v(s).th_pain = Some(fd_secret_use_as_pain);
    }
    g.sound(s, CHAN_VOICE, g.r(s).noise3, 1.0, ATTN_NORM);
}

pub fn secret_blocked(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.time < g.r(s).attack_finished {
        return;
    }

    g.v(s).attack_finished = g.time + 0.5;
    let dmg = g.r(s).dmg;
    T_Damage(g, o, s, s, dmg);
}

/// Prints messages.
pub fn secret_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname != "player" {
        return;
    }

    if g.r(s).attack_finished > g.time {
        return;
    }

    g.v(s).attack_finished = g.time + 2.0;

    if !g.r(s).message.is_empty() {
        g.centerprint(o, g.r(s).message);
        g.sound(o, CHAN_BODY, "misc/talk.wav", 1.0, ATTN_NORM);
    }
}

/// QUAKED func_door_secret (0 .5 .8) ? open_once 1st_left 1st_down no_shoot always_shoot
///
/// Basic secret door. Slides back, then to the side. Angle determines direction. "wait"
/// (seconds before coming back), "t_width", "t_length", "dmg" (2). With a targetname it opens
/// only by its button or trigger, not by damage. "sounds": 1 medieval, 2 metal, 3 base.
pub fn func_door_secret(g: &mut Game) {
    let s = g.self_;
    if g.r(s).sounds == 0.0 {
        g.v(s).sounds = 3.0;
    }

    let sounds = g.r(s).sounds;
    if sounds == 1.0 {
        g.precache_sound("doors/latch2.wav");
        g.precache_sound("doors/winch2.wav");
        g.precache_sound("doors/drclos4.wav");
        g.v(s).noise1 = "doors/latch2.wav";
        g.v(s).noise2 = "doors/winch2.wav";
        g.v(s).noise3 = "doors/drclos4.wav";
    } else if sounds == 2.0 {
        g.precache_sound("doors/airdoor1.wav");
        g.precache_sound("doors/airdoor2.wav");
        g.v(s).noise2 = "doors/airdoor1.wav";
        g.v(s).noise1 = "doors/airdoor2.wav";
        g.v(s).noise3 = "doors/airdoor2.wav";
    } else if sounds == 3.0 {
        g.precache_sound("doors/basesec1.wav");
        g.precache_sound("doors/basesec2.wav");
        g.v(s).noise2 = "doors/basesec1.wav";
        g.v(s).noise1 = "doors/basesec2.wav";
        g.v(s).noise3 = "doors/basesec2.wav";
    }

    if g.r(s).dmg == 0.0 {
        g.v(s).dmg = 2.0;
    }

    // Magic formula...
    g.v(s).mangle = g.r(s).angles;
    g.v(s).angles = Vec3::ZERO;
    g.v(s).solid = SOLID_BSP;
    g.v(s).movetype = MOVETYPE_PUSH;
    g.v(s).classname = "door";
    g.setmodel(s, g.r(s).model);
    g.setorigin(s, g.r(s).origin);

    g.v(s).touch = Some(secret_touch);
    g.v(s).blocked = Some(secret_blocked);
    g.v(s).speed = 50.0;
    g.v(s).use_ = Some(fd_secret_use);
    if g.r(s).targetname.is_empty() || (g.r(s).spawnflags & SECRET_YES_SHOOT) != 0 {
        g.v(s).health = 10000.0;
        g.v(s).takedamage = DAMAGE_YES;
        g.v(s).th_pain = Some(fd_secret_use_as_pain);
        g.v(s).th_die = Some(fd_secret_use);
    }
    g.v(s).oldorigin = g.r(s).origin;
    if g.r(s).wait == 0.0 {
        g.v(s).wait = 5.0; // 5 seconds before closing
    }
}
