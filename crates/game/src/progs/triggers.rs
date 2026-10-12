//! `triggers.qc`, id's QuakeC 1.06, in Rust. Its global `entity s` is `g.s`.

use super::*;

pub fn trigger_reactivate(g: &mut Game) {
    let s = g.self_;
    g.v(s).solid = SOLID_TRIGGER;
}

// ---- multiple and once ------------------------------------------------------------------------

const SPAWNFLAG_NOMESSAGE: i32 = 1;
const SPAWNFLAG_NOTOUCH: i32 = 1;

/// The wait time has passed, so set back up for another activation.
pub fn multi_wait(g: &mut Game) {
    let s = g.self_;
    if g.r(s).max_health != 0.0 {
        g.v(s).health = g.r(s).max_health;
        g.v(s).takedamage = DAMAGE_YES;
        g.v(s).solid = SOLID_BBOX;
    }
}

/// The trigger was just touched, killed or used. `self.enemy` is the activator, so it can be
/// held through a delay.
pub fn multi_trigger(g: &mut Game) {
    let s = g.self_;
    if g.r(s).nextthink > g.time {
        return; // already been triggered
    }

    if g.r(s).classname == "trigger_secret" {
        let enemy = g.r(s).enemy;
        if g.r(enemy).classname != "player" {
            return;
        }

        g.found_secrets += 1.0;
        g.msg_found_secret();
    }

    if !g.r(s).noise.is_empty() {
        g.sound(s, CHAN_VOICE, g.r(s).noise, 1.0, ATTN_NORM);
    }

    // don't trigger again until reset
    g.v(s).takedamage = DAMAGE_NO;

    g.activator = g.r(s).enemy;

    SUB_UseTargets(g);

    let s = g.self_;
    if g.r(s).wait > 0.0 {
        g.v(s).think = Some(multi_wait);
        g.v(s).nextthink = g.time + g.r(s).wait;
    } else {
        // we can't just remove(self) here, because this is a touch function called while C
        // code is looping through area links...
        g.v(s).touch = Some(SUB_Null);
        g.v(s).nextthink = g.time + 0.1;
        g.v(s).think = Some(SUB_Remove);
    }
}

pub fn multi_killed(g: &mut Game) {
    let s = g.self_;
    g.v(s).enemy = g.damage_attacker;
    multi_trigger(g);
}

pub fn multi_use(g: &mut Game) {
    let s = g.self_;
    g.v(s).enemy = g.activator;
    multi_trigger(g);
}

pub fn multi_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname != "player" {
        return;
    }

    // if the trigger has an angles field, check player's facing direction
    if g.r(s).movedir != Vec3::ZERO {
        g.makevectors(g.r(o).angles);
        // v_forward * movedir is the dot product
        if g.v_forward.dot(g.r(s).movedir) < 0.0 {
            return; // not facing the right way
        }
    }

    g.v(s).enemy = o;
    multi_trigger(g);
}

/// QUAKED trigger_multiple (.5 .5 .5) ? notouch
///
/// Variable sized repeatable trigger; must target one or more entities. With "health" it must
/// be killed each time. "delay" waits before firing. "wait" is the time between triggerings
/// (default .2). "notouch" fires only by other entities (obsoleted by trigger_relay).
/// "sounds": 1 secret, 2 beep beep, 3 large switch. "message" is a text string.
pub fn trigger_multiple(g: &mut Game) {
    let s = g.self_;
    let sounds = g.r(s).sounds;
    if sounds == 1.0 {
        g.precache_sound("misc/secret.wav");
        g.v(s).noise = "misc/secret.wav";
    } else if sounds == 2.0 {
        g.precache_sound("misc/talk.wav");
        g.v(s).noise = "misc/talk.wav";
    } else if sounds == 3.0 {
        g.precache_sound("misc/trigger1.wav");
        g.v(s).noise = "misc/trigger1.wav";
    }

    if g.r(s).wait == 0.0 {
        g.v(s).wait = 0.2;
    }
    g.v(s).use_ = Some(multi_use);

    InitTrigger(g);

    if g.r(s).health != 0.0 {
        if (g.r(s).spawnflags & SPAWNFLAG_NOTOUCH) != 0 {
            g.objerror("health and notouch don't make sense\n");
        }

        g.v(s).max_health = g.r(s).health;
        g.v(s).th_die = Some(multi_killed);
        g.v(s).takedamage = DAMAGE_YES;
        g.v(s).solid = SOLID_BBOX;
        g.setorigin(s, g.r(s).origin); // make sure it links into the world
    } else if (g.r(s).spawnflags & SPAWNFLAG_NOTOUCH) == 0 {
        g.v(s).touch = Some(multi_touch);
    }
}

/// QUAKED trigger_once (.5 .5 .5) ? notouch
///
/// Variable sized trigger. Triggers once, then removes itself. "target", "health",
/// "killtarget", "angle" (only fires when someone faces it; "360" for 0), "sounds" and
/// "message" as trigger_multiple.
pub fn trigger_once(g: &mut Game) {
    let s = g.self_;
    g.v(s).wait = -1.0;
    trigger_multiple(g);
}

// ---- relay ------------------------------------------------------------------------------------

/// QUAKED trigger_relay (.5 .5 .5) (-8 -8 -8) (8 8 8)
///
/// Cannot be touched, only fired by other events. Can hold killtargets, targets, delays and
/// messages.
pub fn trigger_relay(g: &mut Game) {
    let s = g.self_;
    g.v(s).use_ = Some(SUB_UseTargets);
}

// ---- secret -----------------------------------------------------------------------------------

/// QUAKED trigger_secret (.5 .5 .5) ?
///
/// Secret counter trigger. "sounds": 1 secret, 2 beep beep. "message" is a text string.
pub fn trigger_secret(g: &mut Game) {
    let s = g.self_;
    g.total_secrets += 1.0;
    g.v(s).wait = -1.0;
    if g.r(s).message.is_empty() {
        g.v(s).message = "You found a secret area!";
    }
    if g.r(s).sounds == 0.0 {
        g.v(s).sounds = 1.0;
    }

    let sounds = g.r(s).sounds;
    if sounds == 1.0 {
        g.precache_sound("misc/secret.wav");
        g.v(s).noise = "misc/secret.wav";
    } else if sounds == 2.0 {
        g.precache_sound("misc/talk.wav");
        g.v(s).noise = "misc/talk.wav";
    }

    trigger_multiple(g);
}

// ---- counter ----------------------------------------------------------------------------------

pub fn counter_use(g: &mut Game) {
    let s = g.self_;
    let a = g.activator;
    g.v(s).count = g.r(s).count - 1.0;
    if g.r(s).count < 0.0 {
        return;
    }

    if g.r(s).count != 0.0 {
        if g.r(a).classname == "player" && (g.r(s).spawnflags & SPAWNFLAG_NOMESSAGE) == 0 {
            let count = g.r(s).count;
            if count >= 4.0 {
                g.centerprint(a, "There are more to go...");
            } else if count == 3.0 {
                g.centerprint(a, "Only 3 more to go...");
            } else if count == 2.0 {
                g.centerprint(a, "Only 2 more to go...");
            } else {
                g.centerprint(a, "Only 1 more to go...");
            }
        }
        return;
    }

    if g.r(a).classname == "player" && (g.r(s).spawnflags & SPAWNFLAG_NOMESSAGE) == 0 {
        g.centerprint(a, "Sequence completed!");
    }

    g.v(s).enemy = a;
    multi_trigger(g);
}

/// QUAKED trigger_counter (.5 .5 .5) ? nomessage
///
/// Acts as an intermediary for an action that takes multiple inputs. Unless nomessage is set,
/// it prints "1 more..." and so on when triggered and "sequence complete" when finished.
/// After "count" triggerings (default 2) it fires all its targets and removes itself.
pub fn trigger_counter(g: &mut Game) {
    let s = g.self_;
    g.v(s).wait = -1.0;
    if g.r(s).count == 0.0 {
        g.v(s).count = 2.0;
    }

    g.v(s).use_ = Some(counter_use);
}

// ---- teleport triggers ------------------------------------------------------------------------

const PLAYER_ONLY: i32 = 1;
const SILENT: i32 = 2;

pub fn play_teleport(g: &mut Game) {
    let s = g.self_;
    let v = g.random() * 5.0;
    let tmpstr = if v < 1.0 {
        "misc/r_tele1.wav"
    } else if v < 2.0 {
        "misc/r_tele2.wav"
    } else if v < 3.0 {
        "misc/r_tele3.wav"
    } else if v < 4.0 {
        "misc/r_tele4.wav"
    } else {
        "misc/r_tele5.wav"
    };

    g.sound(s, CHAN_VOICE, tmpstr, 1.0, ATTN_NORM);
    g.remove(s);
}

pub fn spawn_tfog(g: &mut Game, org: Vec3) {
    g.s = g.spawn();
    let s = g.s;
    g.v(s).origin = org;
    g.v(s).nextthink = g.time + 0.2;
    g.v(s).think = Some(play_teleport);

    g.temp_entity(TempEntity::Teleport(org));
}

pub fn tdeath_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if o == g.r(s).owner {
        return;
    }

    // frag anyone who teleports in on top of an invincible player
    if g.r(o).classname == "player" {
        if g.r(o).invincible_finished > g.time {
            g.v(s).classname = "teledeath2";
        }
        let owner = g.r(s).owner;
        if g.r(owner).classname != "player" {
            // other monsters explode themselves
            T_Damage(g, owner, s, s, 50000.0);
            return;
        }
    }

    if g.r(o).health != 0.0 {
        T_Damage(g, o, s, s, 50000.0);
    }
}

pub fn spawn_tdeath(g: &mut Game, org: Vec3, death_owner: Ent) {
    let death = g.spawn();
    g.v(death).classname = "teledeath";
    g.v(death).movetype = MOVETYPE_NONE;
    g.v(death).solid = SOLID_TRIGGER;
    g.v(death).angles = Vec3::ZERO;
    let (mins, maxs) = (g.r(death_owner).mins, g.r(death_owner).maxs);
    g.setsize(death, mins - Vec3::ONE, maxs + Vec3::ONE);
    g.setorigin(death, org);
    g.v(death).touch = Some(tdeath_touch);
    g.v(death).nextthink = g.time + 0.2;
    g.v(death).think = Some(SUB_Remove);
    g.v(death).owner = death_owner;

    g.force_retouch = 2.0; // make sure even still objects get hit
}

pub fn teleport_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if !g.r(s).targetname.is_empty()
        && g.r(s).nextthink < g.time {
            return; // not fired yet
        }

    if (g.r(s).spawnflags & PLAYER_ONLY) != 0
        && g.r(o).classname != "player" {
            return;
        }

    // only teleport living creatures
    if g.r(o).health <= 0.0 || g.r(o).solid != SOLID_SLIDEBOX {
        return;
    }

    SUB_UseTargets(g);

    // put a tfog where the player was
    let s = g.self_;
    let o_origin = g.r(o).origin;
    spawn_tfog(g, o_origin);

    let t = g.find(world, |v: &EntVars| v.targetname, g.r(s).target);
    if t == world {
        g.objerror("couldn't find target");
    }

    // spawn a tfog flash in front of the destination
    g.makevectors(g.r(t).mangle);
    let org = g.r(t).origin + 32.0 * g.v_forward;

    spawn_tfog(g, org);
    let t_origin = g.r(t).origin;
    spawn_tdeath(g, t_origin, o);

    // move the player and lock him down for a little while
    if g.r(o).health == 0.0 {
        g.v(o).origin = g.r(t).origin;
        let vel = g.r(o).velocity;
        g.v(o).velocity = (g.v_forward * vel.x) + (g.v_forward * vel.y);
        return;
    }

    g.setorigin(o, g.r(t).origin);
    g.v(o).angles = g.r(t).mangle;
    if g.r(o).classname == "player" {
        g.v(o).fixangle = 1; // turn this way immediately
        g.v(o).teleport_time = g.time + 0.7;
        if (g.r(o).flags & FL_ONGROUND) != 0 {
            g.v(o).flags = g.r(o).flags - FL_ONGROUND;
        }
        g.v(o).velocity = g.v_forward * 300.0;
    }
    // `other.flags - other.flags & FL_ONGROUND`: & binds tighter than - in QuakeC
    g.v(o).flags = g.r(o).flags - (g.r(o).flags & FL_ONGROUND);
}

/// QUAKED info_teleport_destination (.5 .5 .5) (-8 -8 -8) (8 8 32)
///
/// The destination marker for a teleporter. Its "targetname" matches a teleporter's "target".
pub fn info_teleport_destination(g: &mut Game) {
    let s = g.self_;
    // this does nothing, just serves as a target spot
    g.v(s).mangle = g.r(s).angles;
    g.v(s).angles = Vec3::ZERO;
    g.v(s).model = "";
    g.v(s).origin = g.r(s).origin + Vec3::new(0.0, 0.0, 27.0);
    if g.r(s).targetname.is_empty() {
        g.objerror("no targetname");
    }
}

pub fn teleport_use(g: &mut Game) {
    let s = g.self_;
    g.v(s).nextthink = g.time + 0.2;
    g.force_retouch = 2.0; // make sure even still objects get hit
    g.v(s).think = Some(SUB_Null);
}

/// QUAKED trigger_teleport (.5 .5 .5) ? PLAYER_ONLY SILENT
///
/// Anything touching it is moved to the info_teleport_destination whose "targetname" matches
/// its "target". With a targetname of its own, it teleports only when fired.
pub fn trigger_teleport(g: &mut Game) {
    let s = g.self_;
    InitTrigger(g);
    g.v(s).touch = Some(teleport_touch);

    // find the destination
    if g.r(s).target.is_empty() {
        g.objerror("no target");
    }

    g.v(s).use_ = Some(teleport_use);

    if (g.r(s).spawnflags & SILENT) == 0 {
        g.precache_sound("ambience/hum1.wav");
        let o = (g.r(s).mins + g.r(s).maxs) * 0.5;
        g.ambientsound(o, "ambience/hum1.wav", 0.5, ATTN_STATIC);
    }
}

// ---- trigger_setskill -------------------------------------------------------------------------

pub fn trigger_skill_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname != "player" {
        return;
    }

    g.cvar_set("skill", g.r(s).message);
}

/// QUAKED trigger_setskill (.5 .5 .5) ?
///
/// Sets the skill level to the value of "message". Only used on the start map.
pub fn trigger_setskill(g: &mut Game) {
    let s = g.self_;
    InitTrigger(g);
    g.v(s).touch = Some(trigger_skill_touch);
}

// ---- only registered triggers -----------------------------------------------------------------

pub fn trigger_onlyregistered_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname != "player" {
        return;
    }

    if g.r(s).attack_finished > g.time {
        return;
    }

    g.v(s).attack_finished = g.time + 2.0;

    if g.cvar("registered") != 0.0 {
        g.v(s).message = "";
        SUB_UseTargets(g);
        let s = g.self_;
        g.remove(s);
    } else if !g.r(s).message.is_empty() {
        g.centerprint(o, g.r(s).message);
        g.sound(o, CHAN_BODY, "misc/talk.wav", 1.0, ATTN_NORM);
    }
}

/// QUAKED trigger_onlyregistered (.5 .5 .5) ?
///
/// Fires only in the registered version; otherwise prints the message.
pub fn trigger_onlyregistered(g: &mut Game) {
    let s = g.self_;
    g.precache_sound("misc/talk.wav");
    InitTrigger(g);
    g.v(s).touch = Some(trigger_onlyregistered_touch);
}

// ---- hurt -------------------------------------------------------------------------------------

pub fn hurt_on(g: &mut Game) {
    let s = g.self_;
    g.v(s).solid = SOLID_TRIGGER;
    g.v(s).nextthink = -1.0;
}

pub fn hurt_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).takedamage != 0 {
        g.v(s).solid = SOLID_NOT;
        let dmg = g.r(s).dmg;
        T_Damage(g, o, s, s, dmg);
        let s = g.self_;
        g.v(s).think = Some(hurt_on);
        g.v(s).nextthink = g.time + 1.0;
    }
}

/// QUAKED trigger_hurt (.5 .5 .5) ?
///
/// Any object touching this will be hurt by "dmg" (default 5).
pub fn trigger_hurt(g: &mut Game) {
    let s = g.self_;
    InitTrigger(g);
    g.v(s).touch = Some(hurt_touch);
    if g.r(s).dmg == 0.0 {
        g.v(s).dmg = 5.0;
    }
}

// ---- push -------------------------------------------------------------------------------------

const PUSH_ONCE: i32 = 1;

pub fn trigger_push_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname == "grenade" {
        g.v(o).velocity = g.r(s).speed * g.r(s).movedir * 10.0;
    } else if g.r(o).health > 0.0 {
        g.v(o).velocity = g.r(s).speed * g.r(s).movedir * 10.0;
        if g.r(o).classname == "player"
            && g.r(o).fly_sound < g.time {
                g.v(o).fly_sound = g.time + 1.5;
                g.sound(o, CHAN_AUTO, "ambience/windfly.wav", 1.0, ATTN_NORM);
            }
    }

    if (g.r(s).spawnflags & PUSH_ONCE) != 0 {
        g.remove(s);
    }
}

/// QUAKED trigger_push (.5 .5 .5) ? PUSH_ONCE
///
/// Pushes the player.
pub fn trigger_push(g: &mut Game) {
    let s = g.self_;
    InitTrigger(g);
    g.precache_sound("ambience/windfly.wav");
    g.v(s).touch = Some(trigger_push_touch);
    if g.r(s).speed == 0.0 {
        g.v(s).speed = 1000.0;
    }
}

// ---- monster jump -----------------------------------------------------------------------------

pub fn trigger_monsterjump_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if (g.r(o).flags & (FL_MONSTER | FL_FLY | FL_SWIM)) != FL_MONSTER {
        return;
    }

    // set XY even if not on ground, so the jump will clear lips
    g.v(o).velocity.x = g.r(s).movedir.x * g.r(s).speed;
    g.v(o).velocity.y = g.r(s).movedir.y * g.r(s).speed;

    if (g.r(o).flags & FL_ONGROUND) == 0 {
        return;
    }

    g.v(o).flags = g.r(o).flags - FL_ONGROUND;

    g.v(o).velocity.z = g.r(s).height;
}

/// QUAKED trigger_monsterjump (.5 .5 .5) ?
///
/// Walking monsters that touch this jump along the trigger's angle. "speed" (200) is the speed
/// thrown forward, "height" (200) the speed thrown upwards.
pub fn trigger_monsterjump(g: &mut Game) {
    let s = g.self_;
    if g.r(s).speed == 0.0 {
        g.v(s).speed = 200.0;
    }
    if g.r(s).height == 0.0 {
        g.v(s).height = 200.0;
    }
    if g.r(s).angles == Vec3::ZERO {
        g.v(s).angles = Vec3::new(0.0, 360.0, 0.0);
    }
    InitTrigger(g);
    g.v(s).touch = Some(trigger_monsterjump_touch);
}
