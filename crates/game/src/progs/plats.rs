//! `plats.qc`, id's QuakeC 1.06, in Rust: platforms and trains.

use super::*;

const PLAT_LOW_TRIGGER: i32 = 1;

pub fn plat_spawn_inside_trigger(g: &mut Game) {
    let s = g.self_;

    // middle trigger
    let trigger = g.spawn();
    g.v(trigger).touch = Some(plat_center_touch);
    g.v(trigger).movetype = MOVETYPE_NONE;
    g.v(trigger).solid = SOLID_TRIGGER;
    g.v(trigger).enemy = s;

    let me = g.r(s);
    let mut tmin = me.mins + Vec3::new(25.0, 25.0, 0.0);
    let mut tmax = me.maxs - Vec3::new(25.0, 25.0, -8.0);
    tmin.z = tmax.z - (me.pos1.z - me.pos2.z + 8.0);
    if (me.spawnflags & PLAT_LOW_TRIGGER) != 0 {
        tmax.z = tmin.z + 8.0;
    }

    if me.size.x <= 50.0 {
        tmin.x = (me.mins.x + me.maxs.x) / 2.0;
        tmax.x = tmin.x + 1.0;
    }
    if me.size.y <= 50.0 {
        tmin.y = (me.mins.y + me.maxs.y) / 2.0;
        tmax.y = tmin.y + 1.0;
    }

    g.setsize(trigger, tmin, tmax);
}

pub fn plat_hit_top(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise1, 1.0, ATTN_NORM);
    g.v(s).state = STATE_TOP;
    g.v(s).think = Some(plat_go_down);
    g.v(s).nextthink = g.r(s).ltime + 3.0;
}

pub fn plat_hit_bottom(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise1, 1.0, ATTN_NORM);
    g.v(s).state = STATE_BOTTOM;
}

pub fn plat_go_down(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise, 1.0, ATTN_NORM);
    g.v(s).state = STATE_DOWN;
    let (dest, speed) = (g.r(s).pos2, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, plat_hit_bottom);
}

pub fn plat_go_up(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, g.r(s).noise, 1.0, ATTN_NORM);
    g.v(s).state = STATE_UP;
    let (dest, speed) = (g.r(s).pos1, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, plat_hit_top);
}

pub fn plat_center_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname != "player" {
        return;
    }
    if g.r(o).health <= 0.0 {
        return;
    }

    // self is not put back here; the engine does it after a touch
    g.self_ = g.r(s).enemy;
    let s = g.self_;
    if g.r(s).state == STATE_BOTTOM {
        plat_go_up(g);
    } else if g.r(s).state == STATE_TOP {
        g.v(s).nextthink = g.r(s).ltime + 1.0; // delay going down
    }
}

pub fn plat_outside_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname != "player" {
        return;
    }
    if g.r(o).health <= 0.0 {
        return;
    }

    // self is not put back here; the engine does it after a touch
    g.self_ = g.r(s).enemy;
    let s = g.self_;
    if g.r(s).state == STATE_TOP {
        plat_go_down(g);
    }
}

pub fn plat_trigger_use(g: &mut Game) {
    let s = g.self_;
    if g.r(s).think.is_some() {
        return; // already activated
    }

    plat_go_down(g);
}

pub fn plat_crush(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    T_Damage(g, o, s, s, 1.0);

    let s = g.self_;
    if g.r(s).state == STATE_UP {
        plat_go_down(g);
    } else if g.r(s).state == STATE_DOWN {
        plat_go_up(g);
    } else {
        g.objerror("plat_crush: bad self.state\n");
    }
}

pub fn plat_use(g: &mut Game) {
    let s = g.self_;
    g.v(s).use_ = Some(SUB_Null);
    if g.r(s).state != STATE_UP {
        g.objerror("plat_use: not in up state");
    }

    plat_go_down(g);
}

/// QUAKED func_plat (0 .5 .8) ? PLAT_LOW_TRIGGER
///
/// "speed" (150). Plats are drawn in the extended position, so they light correctly. A plat
/// that is the target of a trigger or button starts disabled in the extended position until
/// fired, then lowers and becomes a normal plat. "height" sets how far it moves, instead of
/// the model's height. "sounds": 1 base fast, 2 chain slow.
pub fn func_plat(g: &mut Game) {
    let s = g.self_;
    if g.r(s).t_length == 0.0 {
        g.v(s).t_length = 80.0;
    }
    if g.r(s).t_width == 0.0 {
        g.v(s).t_width = 10.0;
    }

    if g.r(s).sounds == 0.0 {
        g.v(s).sounds = 2.0;
        // FIX THIS TO LOAD A GENERIC PLAT SOUND
    }

    let sounds = g.r(s).sounds;
    if sounds == 1.0 {
        g.precache_sound("plats/plat1.wav");
        g.precache_sound("plats/plat2.wav");
        g.v(s).noise = "plats/plat1.wav";
        g.v(s).noise1 = "plats/plat2.wav";
    } else if sounds == 2.0 {
        g.precache_sound("plats/medplat1.wav");
        g.precache_sound("plats/medplat2.wav");
        g.v(s).noise = "plats/medplat1.wav";
        g.v(s).noise1 = "plats/medplat2.wav";
    }

    g.v(s).mangle = g.r(s).angles;
    g.v(s).angles = Vec3::ZERO;

    g.v(s).classname = "plat";
    g.v(s).solid = SOLID_BSP;
    g.v(s).movetype = MOVETYPE_PUSH;
    g.setorigin(s, g.r(s).origin);
    g.setmodel(s, g.r(s).model);
    g.setsize(s, g.r(s).mins, g.r(s).maxs);

    g.v(s).blocked = Some(plat_crush);
    if g.r(s).speed == 0.0 {
        g.v(s).speed = 150.0;
    }

    // pos1 is the top position, pos2 is the bottom
    g.v(s).pos1 = g.r(s).origin;
    g.v(s).pos2 = g.r(s).origin;
    if g.r(s).height != 0.0 {
        g.v(s).pos2.z = g.r(s).origin.z - g.r(s).height;
    } else {
        g.v(s).pos2.z = g.r(s).origin.z - g.r(s).size.z + 8.0;
    }

    g.v(s).use_ = Some(plat_trigger_use);

    plat_spawn_inside_trigger(g); // the "start moving" trigger

    if !g.r(s).targetname.is_empty() {
        g.v(s).state = STATE_UP;
        g.v(s).use_ = Some(plat_use);
    } else {
        g.setorigin(s, g.r(s).pos2);
        g.v(s).state = STATE_BOTTOM;
    }
}

// ---- trains -----------------------------------------------------------------------------------

pub fn train_blocked(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.time < g.r(s).attack_finished {
        return;
    }

    g.v(s).attack_finished = g.time + 0.5;
    let dmg = g.r(s).dmg;
    T_Damage(g, o, s, s, dmg);
}

// QuakeC compares the function itself; each function here has its own address.
#[allow(unpredictable_function_pointer_comparisons)]
pub fn train_use(g: &mut Game) {
    let s = g.self_;
    if g.r(s).think != Some(func_train_find as fn(&mut Game)) {
        return; // already activated
    }

    train_next(g);
}

pub fn train_wait(g: &mut Game) {
    let s = g.self_;
    if g.r(s).wait != 0.0 {
        g.v(s).nextthink = g.r(s).ltime + g.r(s).wait;
        g.sound(s, CHAN_VOICE, g.r(s).noise, 1.0, ATTN_NORM);
    } else {
        g.v(s).nextthink = g.r(s).ltime + 0.1;
    }

    g.v(s).think = Some(train_next);
}

pub fn train_next(g: &mut Game) {
    let s = g.self_;
    let targ = g.find(world, |v: &EntVars| v.targetname, g.r(s).target);
    g.v(s).target = g.r(targ).target;
    if g.r(s).target.is_empty() {
        g.objerror("train_next: no next target");
    }

    if g.r(targ).wait != 0.0 {
        g.v(s).wait = g.r(targ).wait;
    } else {
        g.v(s).wait = 0.0;
    }
    g.sound(s, CHAN_VOICE, g.r(s).noise1, 1.0, ATTN_NORM);
    let (dest, speed) = (g.r(targ).origin - g.r(s).mins, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, train_wait);
}

pub fn func_train_find(g: &mut Game) {
    let s = g.self_;
    let targ = g.find(world, |v: &EntVars| v.targetname, g.r(s).target);
    g.v(s).target = g.r(targ).target;
    g.setorigin(s, g.r(targ).origin - g.r(s).mins);
    if g.r(s).targetname.is_empty() {
        // not triggered, so start immediately
        g.v(s).nextthink = g.r(s).ltime + 0.1;
        g.v(s).think = Some(train_next);
    }
}

/// QUAKED func_train (0 .5 .8) ?
///
/// Trains are moving platforms that players can ride. The targets' origins are the train's
/// min point at each corner; it spawns at the first target it points at. The target of a
/// button or trigger, it waits until fired. "speed" (100), "dmg" (2), "sounds" (1 ratchet
/// metal).
pub fn func_train(g: &mut Game) {
    let s = g.self_;
    if g.r(s).speed == 0.0 {
        g.v(s).speed = 100.0;
    }
    if g.r(s).target.is_empty() {
        g.objerror("func_train without a target");
    }

    if g.r(s).dmg == 0.0 {
        g.v(s).dmg = 2.0;
    }

    let sounds = g.r(s).sounds;
    if sounds == 0.0 {
        g.v(s).noise = "misc/null.wav"; // stop sound
        g.precache_sound("misc/null.wav");
        g.v(s).noise1 = "misc/null.wav"; // move sound
        g.precache_sound("misc/null.wav");
    } else if sounds == 1.0 {
        g.v(s).noise = "plats/train2.wav"; // stop sound
        g.precache_sound("plats/train2.wav");
        g.v(s).noise1 = "plats/train1.wav"; // move sound
        g.precache_sound("plats/train1.wav");
    }

    g.v(s).cnt = 1.0;
    g.v(s).solid = SOLID_BSP;
    g.v(s).movetype = MOVETYPE_PUSH;
    g.v(s).blocked = Some(train_blocked);
    g.v(s).use_ = Some(train_use);
    g.v(s).classname = "train";

    g.setmodel(s, g.r(s).model);
    g.setsize(s, g.r(s).mins, g.r(s).maxs);
    g.setorigin(s, g.r(s).origin);

    // start trains on the second frame, to make sure their targets have had a chance to spawn
    g.v(s).nextthink = g.r(s).ltime + 0.1;
    g.v(s).think = Some(func_train_find);
}

/// QUAKED misc_teleporttrain (0 .5 .8) (-8 -8 -8) (8 8 8)
///
/// This is used for the final boss.
pub fn misc_teleporttrain(g: &mut Game) {
    let s = g.self_;
    if g.r(s).speed == 0.0 {
        g.v(s).speed = 100.0;
    }
    if g.r(s).target.is_empty() {
        g.objerror("func_train without a target");
    }

    g.v(s).cnt = 1.0;
    g.v(s).solid = SOLID_NOT;
    g.v(s).movetype = MOVETYPE_PUSH;
    g.v(s).blocked = Some(train_blocked);
    g.v(s).use_ = Some(train_use);
    g.v(s).avelocity = Vec3::new(100.0, 200.0, 300.0);

    g.v(s).noise = "misc/null.wav";
    g.precache_sound("misc/null.wav");
    g.v(s).noise1 = "misc/null.wav";
    g.precache_sound("misc/null.wav");

    g.precache_model2("progs/teleport.mdl");
    g.setmodel(s, "progs/teleport.mdl");
    g.setsize(s, g.r(s).mins, g.r(s).maxs);
    g.setorigin(s, g.r(s).origin);

    // start trains on the second frame, to make sure their targets have had a chance to spawn
    g.v(s).nextthink = g.r(s).ltime + 0.1;
    g.v(s).think = Some(func_train_find);
}
