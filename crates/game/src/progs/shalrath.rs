//! `shalrath.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;


// The model's frames, numbered as `$frame` numbers them: from nought, in order, in this file.
const F_attack1: f32 = 0.0;
const F_attack2: f32 = 1.0;
const F_attack3: f32 = 2.0;
const F_attack4: f32 = 3.0;
const F_attack5: f32 = 4.0;
const F_attack6: f32 = 5.0;
const F_attack7: f32 = 6.0;
const F_attack8: f32 = 7.0;
const F_attack9: f32 = 8.0;
const F_attack10: f32 = 9.0;
const F_attack11: f32 = 10.0;
const F_pain1: f32 = 11.0;
const F_pain2: f32 = 12.0;
const F_pain3: f32 = 13.0;
const F_pain4: f32 = 14.0;
const F_pain5: f32 = 15.0;
const F_death1: f32 = 16.0;
const F_death2: f32 = 17.0;
const F_death3: f32 = 18.0;
const F_death4: f32 = 19.0;
const F_death5: f32 = 20.0;
const F_death6: f32 = 21.0;
const F_death7: f32 = 22.0;
const F_walk1: f32 = 23.0;
const F_walk2: f32 = 24.0;
const F_walk3: f32 = 25.0;
const F_walk4: f32 = 26.0;
const F_walk5: f32 = 27.0;
const F_walk6: f32 = 28.0;
const F_walk7: f32 = 29.0;
const F_walk8: f32 = 30.0;
const F_walk9: f32 = 31.0;
const F_walk10: f32 = 32.0;
const F_walk11: f32 = 33.0;
const F_walk12: f32 = 34.0;


pub fn shal_stand(g: &mut Game) {
    g.frame_state(F_walk1, shal_stand);
    ai_stand(g);
}

pub fn shal_walk1(g: &mut Game) {
    g.frame_state(F_walk2, shal_walk2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "shalrath/idle.wav", 1.0, ATTN_IDLE);
    }
    ai_walk(g, 6.0);
}

pub fn shal_walk2(g: &mut Game) {
    g.frame_state(F_walk3, shal_walk3);
    ai_walk(g, 4.0);
}

pub fn shal_walk3(g: &mut Game) {
    g.frame_state(F_walk4, shal_walk4);
    ai_walk(g, 0.0);
}

pub fn shal_walk4(g: &mut Game) {
    g.frame_state(F_walk5, shal_walk5);
    ai_walk(g, 0.0);
}

pub fn shal_walk5(g: &mut Game) {
    g.frame_state(F_walk6, shal_walk6);
    ai_walk(g, 0.0);
}

pub fn shal_walk6(g: &mut Game) {
    g.frame_state(F_walk7, shal_walk7);
    ai_walk(g, 0.0);
}

pub fn shal_walk7(g: &mut Game) {
    g.frame_state(F_walk8, shal_walk8);
    ai_walk(g, 5.0);
}

pub fn shal_walk8(g: &mut Game) {
    g.frame_state(F_walk9, shal_walk9);
    ai_walk(g, 6.0);
}

pub fn shal_walk9(g: &mut Game) {
    g.frame_state(F_walk10, shal_walk10);
    ai_walk(g, 5.0);
}

pub fn shal_walk10(g: &mut Game) {
    g.frame_state(F_walk11, shal_walk11);
    ai_walk(g, 0.0);
}

pub fn shal_walk11(g: &mut Game) {
    g.frame_state(F_walk12, shal_walk12);
    ai_walk(g, 4.0);
}

pub fn shal_walk12(g: &mut Game) {
    g.frame_state(F_walk1, shal_walk1);
    ai_walk(g, 5.0);
}

pub fn shal_run1(g: &mut Game) {
    g.frame_state(F_walk2, shal_run2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "shalrath/idle.wav", 1.0, ATTN_IDLE);
    }
    ai_run(g, 6.0);
}

pub fn shal_run2(g: &mut Game) {
    g.frame_state(F_walk3, shal_run3);
    ai_run(g, 4.0);
}

pub fn shal_run3(g: &mut Game) {
    g.frame_state(F_walk4, shal_run4);
    ai_run(g, 0.0);
}

pub fn shal_run4(g: &mut Game) {
    g.frame_state(F_walk5, shal_run5);
    ai_run(g, 0.0);
}

pub fn shal_run5(g: &mut Game) {
    g.frame_state(F_walk6, shal_run6);
    ai_run(g, 0.0);
}

pub fn shal_run6(g: &mut Game) {
    g.frame_state(F_walk7, shal_run7);
    ai_run(g, 0.0);
}

pub fn shal_run7(g: &mut Game) {
    g.frame_state(F_walk8, shal_run8);
    ai_run(g, 5.0);
}

pub fn shal_run8(g: &mut Game) {
    g.frame_state(F_walk9, shal_run9);
    ai_run(g, 6.0);
}

pub fn shal_run9(g: &mut Game) {
    g.frame_state(F_walk10, shal_run10);
    ai_run(g, 5.0);
}

pub fn shal_run10(g: &mut Game) {
    g.frame_state(F_walk11, shal_run11);
    ai_run(g, 0.0);
}

pub fn shal_run11(g: &mut Game) {
    g.frame_state(F_walk12, shal_run12);
    ai_run(g, 4.0);
}

pub fn shal_run12(g: &mut Game) {
    g.frame_state(F_walk1, shal_run1);
    ai_run(g, 5.0);
}

pub fn shal_attack1(g: &mut Game) {
    g.frame_state(F_attack1, shal_attack2);
    g.sound(g.self_, CHAN_VOICE, "shalrath/attack.wav", 1.0, ATTN_NORM);
    ai_face(g);
}

pub fn shal_attack2(g: &mut Game) {
    g.frame_state(F_attack2, shal_attack3);
    ai_face(g);
}

pub fn shal_attack3(g: &mut Game) {
    g.frame_state(F_attack3, shal_attack4);
    ai_face(g);
}

pub fn shal_attack4(g: &mut Game) {
    g.frame_state(F_attack4, shal_attack5);
    ai_face(g);
}

pub fn shal_attack5(g: &mut Game) {
    g.frame_state(F_attack5, shal_attack6);
    ai_face(g);
}

pub fn shal_attack6(g: &mut Game) {
    g.frame_state(F_attack6, shal_attack7);
    ai_face(g);
}

pub fn shal_attack7(g: &mut Game) {
    g.frame_state(F_attack7, shal_attack8);
    ai_face(g);
}

pub fn shal_attack8(g: &mut Game) {
    g.frame_state(F_attack8, shal_attack9);
    ai_face(g);
}

pub fn shal_attack9(g: &mut Game) {
    g.frame_state(F_attack9, shal_attack10);
    ShalMissile(g);
}

pub fn shal_attack10(g: &mut Game) {
    g.frame_state(F_attack10, shal_attack11);
    ai_face(g);
}

pub fn shal_attack11(g: &mut Game) {
    g.frame_state(F_attack11, shal_run1);
}

pub fn shal_pain1(g: &mut Game) {
    g.frame_state(F_pain1, shal_pain2);
}

pub fn shal_pain2(g: &mut Game) {
    g.frame_state(F_pain2, shal_pain3);
}

pub fn shal_pain3(g: &mut Game) {
    g.frame_state(F_pain3, shal_pain4);
}

pub fn shal_pain4(g: &mut Game) {
    g.frame_state(F_pain4, shal_pain5);
}

pub fn shal_pain5(g: &mut Game) {
    g.frame_state(F_pain5, shal_run1);
}

pub fn shal_death1(g: &mut Game) {
    g.frame_state(F_death1, shal_death2);
}

pub fn shal_death2(g: &mut Game) {
    g.frame_state(F_death2, shal_death3);
}

pub fn shal_death3(g: &mut Game) {
    g.frame_state(F_death3, shal_death4);
}

pub fn shal_death4(g: &mut Game) {
    g.frame_state(F_death4, shal_death5);
}

pub fn shal_death5(g: &mut Game) {
    g.frame_state(F_death5, shal_death6);
}

pub fn shal_death6(g: &mut Game) {
    g.frame_state(F_death6, shal_death7);
}

pub fn shal_death7(g: &mut Game) {
    g.frame_state(F_death7, shal_death7);
}

pub fn shalrath_pain(g: &mut Game, attacker: Ent, damage: f32) {
    let s = g.self_;
    if g.r(s).pain_finished > g.time {
        return;
    }

    g.sound(s, CHAN_VOICE, "shalrath/pain.wav", 1.0, ATTN_NORM);
    shal_pain1(g);
    g.v(s).pain_finished = g.time + 3.0;
}

pub fn shalrath_die(g: &mut Game) {
    let s = g.self_;
    // check for gib
    if g.r(s).health < -90.0 {
        g.sound(s, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        ThrowHead(g, "progs/h_shal.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib1.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib2.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib3.mdl", g.r(s).health);
        return;
    }

    g.sound(s, CHAN_VOICE, "shalrath/death.wav", 1.0, ATTN_NORM);
    shal_death1(g);
    g.v(s).solid = SOLID_NOT;
    // insert death sounds here
}

pub fn ShalMissile(g: &mut Game) {
    let s = g.self_;
    let enemy = g.r(s).enemy;
    let dir = normalize((g.r(enemy).origin + Vec3::new(0.0, 0.0, 10.0)) - g.r(s).origin);
    let dist = vlen(g.r(enemy).origin - g.r(s).origin);
    let mut flytime = dist * 0.002;
    if flytime < 0.1 {
        flytime = 0.1;
    }

    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;
    g.sound(s, CHAN_WEAPON, "shalrath/attack2.wav", 1.0, ATTN_NORM);

    let missile = g.spawn();
    g.v(missile).owner = s;

    g.v(missile).solid = SOLID_BBOX;
    g.v(missile).movetype = MOVETYPE_FLYMISSILE;
    g.setmodel(missile, "progs/v_spike.mdl");

    g.setsize(missile, Vec3::ZERO, Vec3::ZERO);

    g.v(missile).origin = g.r(s).origin + Vec3::new(0.0, 0.0, 10.0);
    g.v(missile).velocity = dir * 400.0;
    g.v(missile).avelocity = Vec3::new(300.0, 300.0, 300.0);
    g.v(missile).nextthink = flytime + g.time;
    g.v(missile).think = Some(ShalHome);
    g.v(missile).enemy = g.r(s).enemy;
    g.v(missile).touch = Some(ShalMissileTouch);
}

pub fn ShalHome(g: &mut Game) {
    let s = g.self_;
    let enemy = g.r(s).enemy;
    let vtemp = g.r(enemy).origin + Vec3::new(0.0, 0.0, 10.0);

    if g.r(enemy).health < 1.0 {
        g.remove(s);
        return;
    }

    let dir = normalize(vtemp - g.r(s).origin);
    if g.skill == 3.0 {
        g.v(s).velocity = dir * 350.0;
    } else {
        g.v(s).velocity = dir * 250.0;
    }

    g.v(s).nextthink = g.time + 0.2;
    g.v(s).think = Some(ShalHome);
}

pub fn ShalMissileTouch(g: &mut Game) {
    let s = g.self_;
    let other = g.other;
    if other == g.r(s).owner {
        return; // don't explode on owner
    }

    if g.r(other).classname == "monster_zombie" {
        T_Damage(g, other, s, s, 110.0);
    }
    T_RadiusDamage(g, s, g.r(s).owner, 40.0, world);
    g.sound(s, CHAN_WEAPON, "weapons/r_exp3.wav", 1.0, ATTN_NORM);

    g.temp_entity(TempEntity::Explosion(g.r(s).origin));

    g.v(s).velocity = Vec3::ZERO;
    g.v(s).touch = Some(SUB_Null);
    g.setmodel(s, "progs/s_explod.spr");
    g.v(s).solid = SOLID_NOT;
    s_explode1(g);
}

pub fn monster_shalrath(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model2("progs/shalrath.mdl");
    g.precache_model2("progs/h_shal.mdl");
    g.precache_model2("progs/v_spike.mdl");

    g.precache_sound2("shalrath/attack.wav");
    g.precache_sound2("shalrath/attack2.wav");
    g.precache_sound2("shalrath/death.wav");
    g.precache_sound2("shalrath/idle.wav");
    g.precache_sound2("shalrath/pain.wav");
    g.precache_sound2("shalrath/sight.wav");

    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/shalrath.mdl");
    g.setsize(s, VEC_HULL2_MIN, VEC_HULL2_MAX);
    g.v(s).health = 400.0;

    g.v(s).th_stand = Some(shal_stand);
    g.v(s).th_walk = Some(shal_walk1);
    g.v(s).th_run = Some(shal_run1);
    g.v(s).th_die = Some(shalrath_die);
    g.v(s).th_pain = Some(shalrath_pain);
    g.v(s).th_missile = Some(shal_attack1);

    g.v(s).think = Some(walkmonster_start);
    g.v(s).nextthink = g.time + 0.1 + g.random() * 0.1;
}
