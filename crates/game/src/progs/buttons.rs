//! `buttons.qc`, id's QuakeC 1.06, in Rust: the button and the multiple button.

use super::*;

pub fn button_wait(g: &mut Game) {
    let s = g.self_;
    g.v(s).state = STATE_TOP;
    g.v(s).nextthink = g.r(s).ltime + g.r(s).wait;
    g.v(s).think = Some(button_return);
    g.activator = g.r(s).enemy;
    SUB_UseTargets(g);
    let s = g.self_;
    g.v(s).frame = 1.0; // use alternate textures
}

pub fn button_done(g: &mut Game) {
    let s = g.self_;
    g.v(s).state = STATE_BOTTOM;
}

pub fn button_return(g: &mut Game) {
    let s = g.self_;
    g.v(s).state = STATE_DOWN;
    let (dest, speed) = (g.r(s).pos1, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, button_done);
    g.v(s).frame = 0.0; // use normal textures
    if g.r(s).health != 0.0 {
        g.v(s).takedamage = DAMAGE_YES; // can be shot again
    }
}

pub fn button_blocked(_g: &mut Game) {
    // do nothing, just don't come all the way back out
}

pub fn button_fire(g: &mut Game) {
    let s = g.self_;
    if g.r(s).state == STATE_UP || g.r(s).state == STATE_TOP {
        return;
    }

    g.sound(s, CHAN_VOICE, g.r(s).noise, 1.0, ATTN_NORM);

    g.v(s).state = STATE_UP;
    let (dest, speed) = (g.r(s).pos2, g.r(s).speed);
    SUB_CalcMove(g, dest, speed, button_wait);
}

pub fn button_use(g: &mut Game) {
    let s = g.self_;
    g.v(s).enemy = g.activator;
    button_fire(g);
}

pub fn button_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    if g.r(o).classname != "player" {
        return;
    }

    g.v(s).enemy = o;
    button_fire(g);
}

pub fn button_killed(g: &mut Game) {
    let s = g.self_;
    g.v(s).enemy = g.damage_attacker;
    g.v(s).health = g.r(s).max_health;
    g.v(s).takedamage = DAMAGE_NO; // will be reset upon return
    button_fire(g);
}

/// QUAKED func_button (0 .5 .8) ?
///
/// When touched, it moves some distance along its angle, fires all its targets, waits, then
/// returns and can be fired again. "angle", "target", "speed" (40), "wait" (1, -1 never
/// returns), "lip" (4), "health" (killed instead of touched), "sounds" (0 steam metal, 1
/// wooden clunk, 2 metallic click, 3 in-out).
pub fn func_button(g: &mut Game) {
    let s = g.self_;
    let sounds = g.r(s).sounds;
    if sounds == 0.0 {
        g.precache_sound("buttons/airbut1.wav");
        g.v(s).noise = "buttons/airbut1.wav";
    } else if sounds == 1.0 {
        g.precache_sound("buttons/switch21.wav");
        g.v(s).noise = "buttons/switch21.wav";
    } else if sounds == 2.0 {
        g.precache_sound("buttons/switch02.wav");
        g.v(s).noise = "buttons/switch02.wav";
    } else if sounds == 3.0 {
        g.precache_sound("buttons/switch04.wav");
        g.v(s).noise = "buttons/switch04.wav";
    }

    SetMovedir(g);

    g.v(s).movetype = MOVETYPE_PUSH;
    g.v(s).solid = SOLID_BSP;
    g.setmodel(s, g.r(s).model);

    g.v(s).blocked = Some(button_blocked);
    g.v(s).use_ = Some(button_use);

    if g.r(s).health != 0.0 {
        g.v(s).max_health = g.r(s).health;
        g.v(s).th_die = Some(button_killed);
        g.v(s).takedamage = DAMAGE_YES;
    } else {
        g.v(s).touch = Some(button_touch);
    }

    if g.r(s).speed == 0.0 {
        g.v(s).speed = 40.0;
    }
    if g.r(s).wait == 0.0 {
        g.v(s).wait = 1.0;
    }
    if g.r(s).lip == 0.0 {
        g.v(s).lip = 4.0;
    }

    g.v(s).state = STATE_BOTTOM;

    g.v(s).pos1 = g.r(s).origin;
    let (pos1, movedir, size, lip) = (g.r(s).pos1, g.r(s).movedir, g.r(s).size, g.r(s).lip);
    // movedir * size is the dot product
    g.v(s).pos2 = pos1 + movedir * (fabs(movedir.dot(size)) - lip);
}
