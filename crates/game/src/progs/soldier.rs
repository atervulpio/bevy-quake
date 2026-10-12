//! `soldier.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;


// The model's frames, numbered as `$frame` numbers them: from nought, in order, in this file.
const F_stand1: f32 = 0.0;
const F_stand2: f32 = 1.0;
const F_stand3: f32 = 2.0;
const F_stand4: f32 = 3.0;
const F_stand5: f32 = 4.0;
const F_stand6: f32 = 5.0;
const F_stand7: f32 = 6.0;
const F_stand8: f32 = 7.0;
const F_death1: f32 = 8.0;
const F_death2: f32 = 9.0;
const F_death3: f32 = 10.0;
const F_death4: f32 = 11.0;
const F_death5: f32 = 12.0;
const F_death6: f32 = 13.0;
const F_death7: f32 = 14.0;
const F_death8: f32 = 15.0;
const F_death9: f32 = 16.0;
const F_death10: f32 = 17.0;
const F_deathc1: f32 = 18.0;
const F_deathc2: f32 = 19.0;
const F_deathc3: f32 = 20.0;
const F_deathc4: f32 = 21.0;
const F_deathc5: f32 = 22.0;
const F_deathc6: f32 = 23.0;
const F_deathc7: f32 = 24.0;
const F_deathc8: f32 = 25.0;
const F_deathc9: f32 = 26.0;
const F_deathc10: f32 = 27.0;
const F_deathc11: f32 = 28.0;
const F_pain1: f32 = 40.0;
const F_pain2: f32 = 41.0;
const F_pain3: f32 = 42.0;
const F_pain4: f32 = 43.0;
const F_pain5: f32 = 44.0;
const F_pain6: f32 = 45.0;
const F_painb1: f32 = 46.0;
const F_painb2: f32 = 47.0;
const F_painb3: f32 = 48.0;
const F_painb4: f32 = 49.0;
const F_painb5: f32 = 50.0;
const F_painb6: f32 = 51.0;
const F_painb7: f32 = 52.0;
const F_painb8: f32 = 53.0;
const F_painb9: f32 = 54.0;
const F_painb10: f32 = 55.0;
const F_painb11: f32 = 56.0;
const F_painb12: f32 = 57.0;
const F_painb13: f32 = 58.0;
const F_painb14: f32 = 59.0;
const F_painc1: f32 = 60.0;
const F_painc2: f32 = 61.0;
const F_painc3: f32 = 62.0;
const F_painc4: f32 = 63.0;
const F_painc5: f32 = 64.0;
const F_painc6: f32 = 65.0;
const F_painc7: f32 = 66.0;
const F_painc8: f32 = 67.0;
const F_painc9: f32 = 68.0;
const F_painc10: f32 = 69.0;
const F_painc11: f32 = 70.0;
const F_painc12: f32 = 71.0;
const F_painc13: f32 = 72.0;
const F_run1: f32 = 73.0;
const F_run2: f32 = 74.0;
const F_run3: f32 = 75.0;
const F_run4: f32 = 76.0;
const F_run5: f32 = 77.0;
const F_run6: f32 = 78.0;
const F_run7: f32 = 79.0;
const F_run8: f32 = 80.0;
const F_shoot1: f32 = 81.0;
const F_shoot2: f32 = 82.0;
const F_shoot3: f32 = 83.0;
const F_shoot4: f32 = 84.0;
const F_shoot5: f32 = 85.0;
const F_shoot6: f32 = 86.0;
const F_shoot7: f32 = 87.0;
const F_shoot8: f32 = 88.0;
const F_shoot9: f32 = 89.0;
const F_prowl_1: f32 = 90.0;
const F_prowl_2: f32 = 91.0;
const F_prowl_3: f32 = 92.0;
const F_prowl_4: f32 = 93.0;
const F_prowl_5: f32 = 94.0;
const F_prowl_6: f32 = 95.0;
const F_prowl_7: f32 = 96.0;
const F_prowl_8: f32 = 97.0;
const F_prowl_9: f32 = 98.0;
const F_prowl_10: f32 = 99.0;
const F_prowl_11: f32 = 100.0;
const F_prowl_12: f32 = 101.0;
const F_prowl_13: f32 = 102.0;
const F_prowl_14: f32 = 103.0;
const F_prowl_15: f32 = 104.0;
const F_prowl_16: f32 = 105.0;
const F_prowl_17: f32 = 106.0;
const F_prowl_18: f32 = 107.0;
const F_prowl_19: f32 = 108.0;
const F_prowl_20: f32 = 109.0;
const F_prowl_21: f32 = 110.0;
const F_prowl_22: f32 = 111.0;
const F_prowl_23: f32 = 112.0;
const F_prowl_24: f32 = 113.0;


pub fn army_stand1(g: &mut Game) {
    g.frame_state(F_stand1, army_stand2);
    ai_stand(g);
}

pub fn army_stand2(g: &mut Game) {
    g.frame_state(F_stand2, army_stand3);
    ai_stand(g);
}

pub fn army_stand3(g: &mut Game) {
    g.frame_state(F_stand3, army_stand4);
    ai_stand(g);
}

pub fn army_stand4(g: &mut Game) {
    g.frame_state(F_stand4, army_stand5);
    ai_stand(g);
}

pub fn army_stand5(g: &mut Game) {
    g.frame_state(F_stand5, army_stand6);
    ai_stand(g);
}

pub fn army_stand6(g: &mut Game) {
    g.frame_state(F_stand6, army_stand7);
    ai_stand(g);
}

pub fn army_stand7(g: &mut Game) {
    g.frame_state(F_stand7, army_stand8);
    ai_stand(g);
}

pub fn army_stand8(g: &mut Game) {
    g.frame_state(F_stand8, army_stand1);
    ai_stand(g);
}

pub fn army_walk1(g: &mut Game) {
    g.frame_state(F_prowl_1, army_walk2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "soldier/idle.wav", 1.0, ATTN_IDLE);
    }
    ai_walk(g, 1.0);
}

pub fn army_walk2(g: &mut Game) {
    g.frame_state(F_prowl_2, army_walk3);
    ai_walk(g, 1.0);
}

pub fn army_walk3(g: &mut Game) {
    g.frame_state(F_prowl_3, army_walk4);
    ai_walk(g, 1.0);
}

pub fn army_walk4(g: &mut Game) {
    g.frame_state(F_prowl_4, army_walk5);
    ai_walk(g, 1.0);
}

pub fn army_walk5(g: &mut Game) {
    g.frame_state(F_prowl_5, army_walk6);
    ai_walk(g, 2.0);
}

pub fn army_walk6(g: &mut Game) {
    g.frame_state(F_prowl_6, army_walk7);
    ai_walk(g, 3.0);
}

pub fn army_walk7(g: &mut Game) {
    g.frame_state(F_prowl_7, army_walk8);
    ai_walk(g, 4.0);
}

pub fn army_walk8(g: &mut Game) {
    g.frame_state(F_prowl_8, army_walk9);
    ai_walk(g, 4.0);
}

pub fn army_walk9(g: &mut Game) {
    g.frame_state(F_prowl_9, army_walk10);
    ai_walk(g, 2.0);
}

pub fn army_walk10(g: &mut Game) {
    g.frame_state(F_prowl_10, army_walk11);
    ai_walk(g, 2.0);
}

pub fn army_walk11(g: &mut Game) {
    g.frame_state(F_prowl_11, army_walk12);
    ai_walk(g, 2.0);
}

pub fn army_walk12(g: &mut Game) {
    g.frame_state(F_prowl_12, army_walk13);
    ai_walk(g, 1.0);
}

pub fn army_walk13(g: &mut Game) {
    g.frame_state(F_prowl_13, army_walk14);
    ai_walk(g, 0.0);
}

pub fn army_walk14(g: &mut Game) {
    g.frame_state(F_prowl_14, army_walk15);
    ai_walk(g, 1.0);
}

pub fn army_walk15(g: &mut Game) {
    g.frame_state(F_prowl_15, army_walk16);
    ai_walk(g, 1.0);
}

pub fn army_walk16(g: &mut Game) {
    g.frame_state(F_prowl_16, army_walk17);
    ai_walk(g, 1.0);
}

pub fn army_walk17(g: &mut Game) {
    g.frame_state(F_prowl_17, army_walk18);
    ai_walk(g, 3.0);
}

pub fn army_walk18(g: &mut Game) {
    g.frame_state(F_prowl_18, army_walk19);
    ai_walk(g, 3.0);
}

pub fn army_walk19(g: &mut Game) {
    g.frame_state(F_prowl_19, army_walk20);
    ai_walk(g, 3.0);
}

pub fn army_walk20(g: &mut Game) {
    g.frame_state(F_prowl_20, army_walk21);
    ai_walk(g, 3.0);
}

pub fn army_walk21(g: &mut Game) {
    g.frame_state(F_prowl_21, army_walk22);
    ai_walk(g, 2.0);
}

pub fn army_walk22(g: &mut Game) {
    g.frame_state(F_prowl_22, army_walk23);
    ai_walk(g, 1.0);
}

pub fn army_walk23(g: &mut Game) {
    g.frame_state(F_prowl_23, army_walk24);
    ai_walk(g, 1.0);
}

pub fn army_walk24(g: &mut Game) {
    g.frame_state(F_prowl_24, army_walk1);
    ai_walk(g, 1.0);
}

pub fn army_run1(g: &mut Game) {
    g.frame_state(F_run1, army_run2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "soldier/idle.wav", 1.0, ATTN_IDLE);
    }
    ai_run(g, 11.0);
}

pub fn army_run2(g: &mut Game) {
    g.frame_state(F_run2, army_run3);
    ai_run(g, 15.0);
}

pub fn army_run3(g: &mut Game) {
    g.frame_state(F_run3, army_run4);
    ai_run(g, 10.0);
}

pub fn army_run4(g: &mut Game) {
    g.frame_state(F_run4, army_run5);
    ai_run(g, 10.0);
}

pub fn army_run5(g: &mut Game) {
    g.frame_state(F_run5, army_run6);
    ai_run(g, 8.0);
}

pub fn army_run6(g: &mut Game) {
    g.frame_state(F_run6, army_run7);
    ai_run(g, 15.0);
}

pub fn army_run7(g: &mut Game) {
    g.frame_state(F_run7, army_run8);
    ai_run(g, 10.0);
}

pub fn army_run8(g: &mut Game) {
    g.frame_state(F_run8, army_run1);
    ai_run(g, 8.0);
}

pub fn army_atk1(g: &mut Game) {
    g.frame_state(F_shoot1, army_atk2);
    ai_face(g);
}

pub fn army_atk2(g: &mut Game) {
    g.frame_state(F_shoot2, army_atk3);
    ai_face(g);
}

pub fn army_atk3(g: &mut Game) {
    g.frame_state(F_shoot3, army_atk4);
    ai_face(g);
}

pub fn army_atk4(g: &mut Game) {
    g.frame_state(F_shoot4, army_atk5);
    ai_face(g);
}

pub fn army_atk5(g: &mut Game) {
    g.frame_state(F_shoot5, army_atk6);
    ai_face(g);
    army_fire(g);
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;
}

pub fn army_atk6(g: &mut Game) {
    g.frame_state(F_shoot6, army_atk7);
    ai_face(g);
}

pub fn army_atk7(g: &mut Game) {
    g.frame_state(F_shoot7, army_atk8);
    ai_face(g);
    SUB_CheckRefire(g, army_atk1);
}

pub fn army_atk8(g: &mut Game) {
    g.frame_state(F_shoot8, army_atk9);
    ai_face(g);
}

pub fn army_atk9(g: &mut Game) {
    g.frame_state(F_shoot9, army_run1);
    ai_face(g);
}

pub fn army_pain1(g: &mut Game) {
    g.frame_state(F_pain1, army_pain2);
}

pub fn army_pain2(g: &mut Game) {
    g.frame_state(F_pain2, army_pain3);
}

pub fn army_pain3(g: &mut Game) {
    g.frame_state(F_pain3, army_pain4);
}

pub fn army_pain4(g: &mut Game) {
    g.frame_state(F_pain4, army_pain5);
}

pub fn army_pain5(g: &mut Game) {
    g.frame_state(F_pain5, army_pain6);
}

pub fn army_pain6(g: &mut Game) {
    g.frame_state(F_pain6, army_run1);
    ai_pain(g, 1.0);
}

pub fn army_painb1(g: &mut Game) {
    g.frame_state(F_painb1, army_painb2);
}

pub fn army_painb2(g: &mut Game) {
    g.frame_state(F_painb2, army_painb3);
    ai_painforward(g, 13.0);
}

pub fn army_painb3(g: &mut Game) {
    g.frame_state(F_painb3, army_painb4);
    ai_painforward(g, 9.0);
}

pub fn army_painb4(g: &mut Game) {
    g.frame_state(F_painb4, army_painb5);
}

pub fn army_painb5(g: &mut Game) {
    g.frame_state(F_painb5, army_painb6);
}

pub fn army_painb6(g: &mut Game) {
    g.frame_state(F_painb6, army_painb7);
}

pub fn army_painb7(g: &mut Game) {
    g.frame_state(F_painb7, army_painb8);
}

pub fn army_painb8(g: &mut Game) {
    g.frame_state(F_painb8, army_painb9);
}

pub fn army_painb9(g: &mut Game) {
    g.frame_state(F_painb9, army_painb10);
}

pub fn army_painb10(g: &mut Game) {
    g.frame_state(F_painb10, army_painb11);
}

pub fn army_painb11(g: &mut Game) {
    g.frame_state(F_painb11, army_painb12);
}

pub fn army_painb12(g: &mut Game) {
    g.frame_state(F_painb12, army_painb13);
    ai_pain(g, 2.0);
}

pub fn army_painb13(g: &mut Game) {
    g.frame_state(F_painb13, army_painb14);
}

pub fn army_painb14(g: &mut Game) {
    g.frame_state(F_painb14, army_run1);
}

pub fn army_painc1(g: &mut Game) {
    g.frame_state(F_painc1, army_painc2);
}

pub fn army_painc2(g: &mut Game) {
    g.frame_state(F_painc2, army_painc3);
    ai_pain(g, 1.0);
}

pub fn army_painc3(g: &mut Game) {
    g.frame_state(F_painc3, army_painc4);
}

pub fn army_painc4(g: &mut Game) {
    g.frame_state(F_painc4, army_painc5);
}

pub fn army_painc5(g: &mut Game) {
    g.frame_state(F_painc5, army_painc6);
    ai_painforward(g, 1.0);
}

pub fn army_painc6(g: &mut Game) {
    g.frame_state(F_painc6, army_painc7);
    ai_painforward(g, 1.0);
}

pub fn army_painc7(g: &mut Game) {
    g.frame_state(F_painc7, army_painc8);
}

pub fn army_painc8(g: &mut Game) {
    g.frame_state(F_painc8, army_painc9);
    ai_pain(g, 1.0);
}

pub fn army_painc9(g: &mut Game) {
    g.frame_state(F_painc9, army_painc10);
    ai_painforward(g, 4.0);
}

pub fn army_painc10(g: &mut Game) {
    g.frame_state(F_painc10, army_painc11);
    ai_painforward(g, 3.0);
}

pub fn army_painc11(g: &mut Game) {
    g.frame_state(F_painc11, army_painc12);
    ai_painforward(g, 6.0);
}

pub fn army_painc12(g: &mut Game) {
    g.frame_state(F_painc12, army_painc13);
    ai_painforward(g, 8.0);
}

pub fn army_painc13(g: &mut Game) {
    g.frame_state(F_painc13, army_run1);
}

pub fn army_pain(g: &mut Game, attacker: Ent, damage: f32) {
    if g.r(g.self_).pain_finished > g.time {
        return;
    }

    let r = g.random();

    if r < 0.2 {
        g.v(g.self_).pain_finished = g.time + 0.6;
        army_pain1(g);
        g.sound(g.self_, CHAN_VOICE, "soldier/pain1.wav", 1.0, ATTN_NORM);
    } else if r < 0.6 {
        g.v(g.self_).pain_finished = g.time + 1.1;
        army_painb1(g);
        g.sound(g.self_, CHAN_VOICE, "soldier/pain2.wav", 1.0, ATTN_NORM);
    } else {
        g.v(g.self_).pain_finished = g.time + 1.1;
        army_painc1(g);
        g.sound(g.self_, CHAN_VOICE, "soldier/pain2.wav", 1.0, ATTN_NORM);
    }
}

pub fn army_fire(g: &mut Game) {
    ai_face(g);

    let s = g.self_;
    g.sound(s, CHAN_WEAPON, "soldier/sattck1.wav", 1.0, ATTN_NORM);

    // fire somewhat behind the player, so a dodging player is harder to hit
    let en = g.r(s).enemy;

    let mut dir = g.r(en).origin - g.r(en).velocity * 0.2;
    dir = normalize(dir - g.r(s).origin);

    FireBullets(g, 4.0, dir, Vec3::new(0.1, 0.1, 0.0));
}

pub fn army_die1(g: &mut Game) {
    g.frame_state(F_death1, army_die2);
}

pub fn army_die2(g: &mut Game) {
    g.frame_state(F_death2, army_die3);
}

pub fn army_die3(g: &mut Game) {
    g.frame_state(F_death3, army_die4);
    g.v(g.self_).solid = SOLID_NOT;
    g.v(g.self_).ammo_shells = 5.0;
    DropBackpack(g);
}

pub fn army_die4(g: &mut Game) {
    g.frame_state(F_death4, army_die5);
}

pub fn army_die5(g: &mut Game) {
    g.frame_state(F_death5, army_die6);
}

pub fn army_die6(g: &mut Game) {
    g.frame_state(F_death6, army_die7);
}

pub fn army_die7(g: &mut Game) {
    g.frame_state(F_death7, army_die8);
}

pub fn army_die8(g: &mut Game) {
    g.frame_state(F_death8, army_die9);
}

pub fn army_die9(g: &mut Game) {
    g.frame_state(F_death9, army_die10);
}

pub fn army_die10(g: &mut Game) {
    g.frame_state(F_death10, army_die10);
}

pub fn army_cdie1(g: &mut Game) {
    g.frame_state(F_deathc1, army_cdie2);
}

pub fn army_cdie2(g: &mut Game) {
    g.frame_state(F_deathc2, army_cdie3);
    ai_back(g, 5.0);
}

pub fn army_cdie3(g: &mut Game) {
    g.frame_state(F_deathc3, army_cdie4);
    g.v(g.self_).solid = SOLID_NOT;
    g.v(g.self_).ammo_shells = 5.0;
    DropBackpack(g);
    ai_back(g, 4.0);
}

pub fn army_cdie4(g: &mut Game) {
    g.frame_state(F_deathc4, army_cdie5);
    ai_back(g, 13.0);
}

pub fn army_cdie5(g: &mut Game) {
    g.frame_state(F_deathc5, army_cdie6);
    ai_back(g, 3.0);
}

pub fn army_cdie6(g: &mut Game) {
    g.frame_state(F_deathc6, army_cdie7);
    ai_back(g, 4.0);
}

pub fn army_cdie7(g: &mut Game) {
    g.frame_state(F_deathc7, army_cdie8);
}

pub fn army_cdie8(g: &mut Game) {
    g.frame_state(F_deathc8, army_cdie9);
}

pub fn army_cdie9(g: &mut Game) {
    g.frame_state(F_deathc9, army_cdie10);
}

pub fn army_cdie10(g: &mut Game) {
    g.frame_state(F_deathc10, army_cdie11);
}

pub fn army_cdie11(g: &mut Game) {
    g.frame_state(F_deathc11, army_cdie11);
}

pub fn army_die(g: &mut Game) {
    // check for gib
    if g.r(g.self_).health < -35.0 {
        g.sound(g.self_, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        let h = g.r(g.self_).health;
        ThrowHead(g, "progs/h_guard.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib1.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib2.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib3.mdl", h);
        return;
    }

    // regular death
    g.sound(g.self_, CHAN_VOICE, "soldier/death1.wav", 1.0, ATTN_NORM);
    if g.random() < 0.5 {
        army_die1(g);
    } else {
        army_cdie1(g);
    }
}

pub fn monster_army(g: &mut Game) {
    if g.deathmatch != 0.0 {
        g.remove(g.self_);
        return;
    }

    g.precache_model("progs/soldier.mdl");
    g.precache_model("progs/h_guard.mdl");
    g.precache_model("progs/gib1.mdl");
    g.precache_model("progs/gib2.mdl");
    g.precache_model("progs/gib3.mdl");

    g.precache_sound("soldier/death1.wav");
    g.precache_sound("soldier/idle.wav");
    g.precache_sound("soldier/pain1.wav");
    g.precache_sound("soldier/pain2.wav");
    g.precache_sound("soldier/sattck1.wav");
    g.precache_sound("soldier/sight1.wav");

    g.precache_sound("player/udeath.wav"); // gib death

    let s = g.self_;
    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/soldier.mdl");

    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 40.0));
    g.v(s).health = 30.0;

    g.v(s).th_stand = Some(army_stand1);
    g.v(s).th_walk = Some(army_walk1);
    g.v(s).th_run = Some(army_run1);
    g.v(s).th_missile = Some(army_atk1);
    g.v(s).th_pain = Some(army_pain);
    g.v(s).th_die = Some(army_die);

    walkmonster_start(g);
}
