//! `knight.qc`, id's QuakeC 1.06, in Rust.

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
const F_stand9: f32 = 8.0;
const F_runb1: f32 = 9.0;
const F_runb2: f32 = 10.0;
const F_runb3: f32 = 11.0;
const F_runb4: f32 = 12.0;
const F_runb5: f32 = 13.0;
const F_runb6: f32 = 14.0;
const F_runb7: f32 = 15.0;
const F_runb8: f32 = 16.0;
const F_runattack1: f32 = 17.0;
const F_runattack2: f32 = 18.0;
const F_runattack3: f32 = 19.0;
const F_runattack4: f32 = 20.0;
const F_runattack5: f32 = 21.0;
const F_runattack6: f32 = 22.0;
const F_runattack7: f32 = 23.0;
const F_runattack8: f32 = 24.0;
const F_runattack9: f32 = 25.0;
const F_runattack10: f32 = 26.0;
const F_runattack11: f32 = 27.0;
const F_pain1: f32 = 28.0;
const F_pain2: f32 = 29.0;
const F_pain3: f32 = 30.0;
const F_painb1: f32 = 31.0;
const F_painb2: f32 = 32.0;
const F_painb3: f32 = 33.0;
const F_painb4: f32 = 34.0;
const F_painb5: f32 = 35.0;
const F_painb6: f32 = 36.0;
const F_painb7: f32 = 37.0;
const F_painb8: f32 = 38.0;
const F_painb9: f32 = 39.0;
const F_painb10: f32 = 40.0;
const F_painb11: f32 = 41.0;
const F_attackb1: f32 = 42.0;
const F_attackb2: f32 = 44.0;
const F_attackb3: f32 = 45.0;
const F_attackb4: f32 = 46.0;
const F_attackb5: f32 = 47.0;
const F_attackb6: f32 = 48.0;
const F_attackb7: f32 = 49.0;
const F_attackb8: f32 = 50.0;
const F_attackb9: f32 = 51.0;
const F_attackb10: f32 = 52.0;
const F_walk1: f32 = 53.0;
const F_walk2: f32 = 54.0;
const F_walk3: f32 = 55.0;
const F_walk4: f32 = 56.0;
const F_walk5: f32 = 57.0;
const F_walk6: f32 = 58.0;
const F_walk7: f32 = 59.0;
const F_walk8: f32 = 60.0;
const F_walk9: f32 = 61.0;
const F_walk10: f32 = 62.0;
const F_walk11: f32 = 63.0;
const F_walk12: f32 = 64.0;
const F_walk13: f32 = 65.0;
const F_walk14: f32 = 66.0;
const F_kneel1: f32 = 67.0;
const F_kneel2: f32 = 68.0;
const F_kneel3: f32 = 69.0;
const F_kneel4: f32 = 70.0;
const F_kneel5: f32 = 71.0;
const F_death1: f32 = 76.0;
const F_death2: f32 = 77.0;
const F_death3: f32 = 78.0;
const F_death4: f32 = 79.0;
const F_death5: f32 = 80.0;
const F_death6: f32 = 81.0;
const F_death7: f32 = 82.0;
const F_death8: f32 = 83.0;
const F_death9: f32 = 84.0;
const F_death10: f32 = 85.0;
const F_deathb1: f32 = 86.0;
const F_deathb2: f32 = 87.0;
const F_deathb3: f32 = 88.0;
const F_deathb4: f32 = 89.0;
const F_deathb5: f32 = 90.0;
const F_deathb6: f32 = 91.0;
const F_deathb7: f32 = 92.0;
const F_deathb8: f32 = 93.0;
const F_deathb9: f32 = 94.0;
const F_deathb10: f32 = 95.0;
const F_deathb11: f32 = 96.0;


pub fn knight_stand1(g: &mut Game) {
    g.frame_state(F_stand1, knight_stand2);
    ai_stand(g);
}

pub fn knight_stand2(g: &mut Game) {
    g.frame_state(F_stand2, knight_stand3);
    ai_stand(g);
}

pub fn knight_stand3(g: &mut Game) {
    g.frame_state(F_stand3, knight_stand4);
    ai_stand(g);
}

pub fn knight_stand4(g: &mut Game) {
    g.frame_state(F_stand4, knight_stand5);
    ai_stand(g);
}

pub fn knight_stand5(g: &mut Game) {
    g.frame_state(F_stand5, knight_stand6);
    ai_stand(g);
}

pub fn knight_stand6(g: &mut Game) {
    g.frame_state(F_stand6, knight_stand7);
    ai_stand(g);
}

pub fn knight_stand7(g: &mut Game) {
    g.frame_state(F_stand7, knight_stand8);
    ai_stand(g);
}

pub fn knight_stand8(g: &mut Game) {
    g.frame_state(F_stand8, knight_stand9);
    ai_stand(g);
}

pub fn knight_stand9(g: &mut Game) {
    g.frame_state(F_stand9, knight_stand1);
    ai_stand(g);
}

pub fn knight_walk1(g: &mut Game) {
    g.frame_state(F_walk1, knight_walk2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "knight/idle.wav", 1.0, ATTN_IDLE);
    }
    ai_walk(g, 3.0);
}

pub fn knight_walk2(g: &mut Game) {
    g.frame_state(F_walk2, knight_walk3);
    ai_walk(g, 2.0);
}

pub fn knight_walk3(g: &mut Game) {
    g.frame_state(F_walk3, knight_walk4);
    ai_walk(g, 3.0);
}

pub fn knight_walk4(g: &mut Game) {
    g.frame_state(F_walk4, knight_walk5);
    ai_walk(g, 4.0);
}

pub fn knight_walk5(g: &mut Game) {
    g.frame_state(F_walk5, knight_walk6);
    ai_walk(g, 3.0);
}

pub fn knight_walk6(g: &mut Game) {
    g.frame_state(F_walk6, knight_walk7);
    ai_walk(g, 3.0);
}

pub fn knight_walk7(g: &mut Game) {
    g.frame_state(F_walk7, knight_walk8);
    ai_walk(g, 3.0);
}

pub fn knight_walk8(g: &mut Game) {
    g.frame_state(F_walk8, knight_walk9);
    ai_walk(g, 4.0);
}

pub fn knight_walk9(g: &mut Game) {
    g.frame_state(F_walk9, knight_walk10);
    ai_walk(g, 3.0);
}

pub fn knight_walk10(g: &mut Game) {
    g.frame_state(F_walk10, knight_walk11);
    ai_walk(g, 3.0);
}

pub fn knight_walk11(g: &mut Game) {
    g.frame_state(F_walk11, knight_walk12);
    ai_walk(g, 2.0);
}

pub fn knight_walk12(g: &mut Game) {
    g.frame_state(F_walk12, knight_walk13);
    ai_walk(g, 3.0);
}

pub fn knight_walk13(g: &mut Game) {
    g.frame_state(F_walk13, knight_walk14);
    ai_walk(g, 4.0);
}

pub fn knight_walk14(g: &mut Game) {
    g.frame_state(F_walk14, knight_walk1);
    ai_walk(g, 3.0);
}

pub fn knight_run1(g: &mut Game) {
    g.frame_state(F_runb1, knight_run2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "knight/idle.wav", 1.0, ATTN_IDLE);
    }
    ai_run(g, 16.0);
}

pub fn knight_run2(g: &mut Game) {
    g.frame_state(F_runb2, knight_run3);
    ai_run(g, 20.0);
}

pub fn knight_run3(g: &mut Game) {
    g.frame_state(F_runb3, knight_run4);
    ai_run(g, 13.0);
}

pub fn knight_run4(g: &mut Game) {
    g.frame_state(F_runb4, knight_run5);
    ai_run(g, 7.0);
}

pub fn knight_run5(g: &mut Game) {
    g.frame_state(F_runb5, knight_run6);
    ai_run(g, 16.0);
}

pub fn knight_run6(g: &mut Game) {
    g.frame_state(F_runb6, knight_run7);
    ai_run(g, 20.0);
}

pub fn knight_run7(g: &mut Game) {
    g.frame_state(F_runb7, knight_run8);
    ai_run(g, 14.0);
}

pub fn knight_run8(g: &mut Game) {
    g.frame_state(F_runb8, knight_run1);
    ai_run(g, 6.0);
}

pub fn knight_runatk1(g: &mut Game) {
    g.frame_state(F_runattack1, knight_runatk2);
    if g.random() > 0.5 {
        g.sound(g.self_, CHAN_WEAPON, "knight/sword2.wav", 1.0, ATTN_NORM);
    } else {
        g.sound(g.self_, CHAN_WEAPON, "knight/sword1.wav", 1.0, ATTN_NORM);
    }
    ai_charge(g, 20.0);
}

pub fn knight_runatk2(g: &mut Game) {
    g.frame_state(F_runattack2, knight_runatk3);
    ai_charge_side(g);
}

pub fn knight_runatk3(g: &mut Game) {
    g.frame_state(F_runattack3, knight_runatk4);
    ai_charge_side(g);
}

pub fn knight_runatk4(g: &mut Game) {
    g.frame_state(F_runattack4, knight_runatk5);
    ai_charge_side(g);
}

pub fn knight_runatk5(g: &mut Game) {
    g.frame_state(F_runattack5, knight_runatk6);
    ai_melee_side(g);
}

pub fn knight_runatk6(g: &mut Game) {
    g.frame_state(F_runattack6, knight_runatk7);
    ai_melee_side(g);
}

pub fn knight_runatk7(g: &mut Game) {
    g.frame_state(F_runattack7, knight_runatk8);
    ai_melee_side(g);
}

pub fn knight_runatk8(g: &mut Game) {
    g.frame_state(F_runattack8, knight_runatk9);
    ai_melee_side(g);
}

pub fn knight_runatk9(g: &mut Game) {
    g.frame_state(F_runattack9, knight_runatk10);
    ai_melee_side(g);
}

pub fn knight_runatk10(g: &mut Game) {
    g.frame_state(F_runattack10, knight_runatk11);
    ai_charge_side(g);
}

pub fn knight_runatk11(g: &mut Game) {
    g.frame_state(F_runattack11, knight_run1);
    ai_charge(g, 10.0);
}

pub fn knight_atk1(g: &mut Game) {
    g.frame_state(F_attackb1, knight_atk2);
    g.sound(g.self_, CHAN_WEAPON, "knight/sword1.wav", 1.0, ATTN_NORM);
    ai_charge(g, 0.0);
}

pub fn knight_atk2(g: &mut Game) {
    g.frame_state(F_attackb2, knight_atk3);
    ai_charge(g, 7.0);
}

pub fn knight_atk3(g: &mut Game) {
    g.frame_state(F_attackb3, knight_atk4);
    ai_charge(g, 4.0);
}

pub fn knight_atk4(g: &mut Game) {
    g.frame_state(F_attackb4, knight_atk5);
    ai_charge(g, 0.0);
}

pub fn knight_atk5(g: &mut Game) {
    g.frame_state(F_attackb5, knight_atk6);
    ai_charge(g, 3.0);
}

pub fn knight_atk6(g: &mut Game) {
    g.frame_state(F_attackb6, knight_atk7);
    ai_charge(g, 4.0);
    ai_melee(g);
}

pub fn knight_atk7(g: &mut Game) {
    g.frame_state(F_attackb7, knight_atk8);
    ai_charge(g, 1.0);
    ai_melee(g);
}

pub fn knight_atk8(g: &mut Game) {
    g.frame_state(F_attackb8, knight_atk9);
    ai_charge(g, 3.0);
    ai_melee(g);
}

pub fn knight_atk9(g: &mut Game) {
    g.frame_state(F_attackb9, knight_atk10);
    ai_charge(g, 1.0);
}

pub fn knight_atk10(g: &mut Game) {
    g.frame_state(F_attackb10, knight_run1);
    ai_charge(g, 5.0);
    //void() knight_atk9  = [ $attack9,  knight_atk10 ] {};
}

pub fn knight_pain1(g: &mut Game) {
    g.frame_state(F_pain1, knight_pain2);
}

pub fn knight_pain2(g: &mut Game) {
    g.frame_state(F_pain2, knight_pain3);
}

pub fn knight_pain3(g: &mut Game) {
    g.frame_state(F_pain3, knight_run1);
}

pub fn knight_painb1(g: &mut Game) {
    g.frame_state(F_painb1, knight_painb2);
    ai_painforward(g, 0.0);
}

pub fn knight_painb2(g: &mut Game) {
    g.frame_state(F_painb2, knight_painb3);
    ai_painforward(g, 3.0);
}

pub fn knight_painb3(g: &mut Game) {
    g.frame_state(F_painb3, knight_painb4);
}

pub fn knight_painb4(g: &mut Game) {
    g.frame_state(F_painb4, knight_painb5);
}

pub fn knight_painb5(g: &mut Game) {
    g.frame_state(F_painb5, knight_painb6);
    ai_painforward(g, 2.0);
}

pub fn knight_painb6(g: &mut Game) {
    g.frame_state(F_painb6, knight_painb7);
    ai_painforward(g, 4.0);
}

pub fn knight_painb7(g: &mut Game) {
    g.frame_state(F_painb7, knight_painb8);
    ai_painforward(g, 2.0);
}

pub fn knight_painb8(g: &mut Game) {
    g.frame_state(F_painb8, knight_painb9);
    ai_painforward(g, 5.0);
}

pub fn knight_painb9(g: &mut Game) {
    g.frame_state(F_painb9, knight_painb10);
    ai_painforward(g, 5.0);
}

pub fn knight_painb10(g: &mut Game) {
    g.frame_state(F_painb10, knight_painb11);
    ai_painforward(g, 0.0);
}

pub fn knight_painb11(g: &mut Game) {
    g.frame_state(F_painb11, knight_run1);
}

pub fn knight_pain(g: &mut Game, attacker: Ent, damage: f32) {
    if g.r(g.self_).pain_finished > g.time {
        return;
    }

    let r = g.random();

    g.sound(g.self_, CHAN_VOICE, "knight/khurt.wav", 1.0, ATTN_NORM);
    if r < 0.85 {
        knight_pain1(g);
        g.v(g.self_).pain_finished = g.time + 1.0;
    } else {
        knight_painb1(g);
        g.v(g.self_).pain_finished = g.time + 1.0;
    }
}

pub fn knight_bow1(g: &mut Game) {
    g.frame_state(F_kneel1, knight_bow2);
    ai_turn(g);
}

pub fn knight_bow2(g: &mut Game) {
    g.frame_state(F_kneel2, knight_bow3);
    ai_turn(g);
}

pub fn knight_bow3(g: &mut Game) {
    g.frame_state(F_kneel3, knight_bow4);
    ai_turn(g);
}

pub fn knight_bow4(g: &mut Game) {
    g.frame_state(F_kneel4, knight_bow5);
    ai_turn(g);
}

pub fn knight_bow5(g: &mut Game) {
    g.frame_state(F_kneel5, knight_bow5);
    ai_turn(g);
}

pub fn knight_bow6(g: &mut Game) {
    g.frame_state(F_kneel4, knight_bow7);
    ai_turn(g);
}

pub fn knight_bow7(g: &mut Game) {
    g.frame_state(F_kneel3, knight_bow8);
    ai_turn(g);
}

pub fn knight_bow8(g: &mut Game) {
    g.frame_state(F_kneel2, knight_bow9);
    ai_turn(g);
}

pub fn knight_bow9(g: &mut Game) {
    g.frame_state(F_kneel1, knight_bow10);
    ai_turn(g);
}

pub fn knight_bow10(g: &mut Game) {
    g.frame_state(F_walk1, knight_walk1);
    ai_turn(g);
}

pub fn knight_die1(g: &mut Game) {
    g.frame_state(F_death1, knight_die2);
}

pub fn knight_die2(g: &mut Game) {
    g.frame_state(F_death2, knight_die3);
}

pub fn knight_die3(g: &mut Game) {
    g.frame_state(F_death3, knight_die4);
    g.v(g.self_).solid = SOLID_NOT;
}

pub fn knight_die4(g: &mut Game) {
    g.frame_state(F_death4, knight_die5);
}

pub fn knight_die5(g: &mut Game) {
    g.frame_state(F_death5, knight_die6);
}

pub fn knight_die6(g: &mut Game) {
    g.frame_state(F_death6, knight_die7);
}

pub fn knight_die7(g: &mut Game) {
    g.frame_state(F_death7, knight_die8);
}

pub fn knight_die8(g: &mut Game) {
    g.frame_state(F_death8, knight_die9);
}

pub fn knight_die9(g: &mut Game) {
    g.frame_state(F_death9, knight_die10);
}

pub fn knight_die10(g: &mut Game) {
    g.frame_state(F_death10, knight_die10);
}

pub fn knight_dieb1(g: &mut Game) {
    g.frame_state(F_deathb1, knight_dieb2);
}

pub fn knight_dieb2(g: &mut Game) {
    g.frame_state(F_deathb2, knight_dieb3);
}

pub fn knight_dieb3(g: &mut Game) {
    g.frame_state(F_deathb3, knight_dieb4);
    g.v(g.self_).solid = SOLID_NOT;
}

pub fn knight_dieb4(g: &mut Game) {
    g.frame_state(F_deathb4, knight_dieb5);
}

pub fn knight_dieb5(g: &mut Game) {
    g.frame_state(F_deathb5, knight_dieb6);
}

pub fn knight_dieb6(g: &mut Game) {
    g.frame_state(F_deathb6, knight_dieb7);
}

pub fn knight_dieb7(g: &mut Game) {
    g.frame_state(F_deathb7, knight_dieb8);
}

pub fn knight_dieb8(g: &mut Game) {
    g.frame_state(F_deathb8, knight_dieb9);
}

pub fn knight_dieb9(g: &mut Game) {
    g.frame_state(F_deathb9, knight_dieb10);
}

pub fn knight_dieb10(g: &mut Game) {
    g.frame_state(F_deathb10, knight_dieb11);
}

pub fn knight_dieb11(g: &mut Game) {
    g.frame_state(F_deathb11, knight_dieb11);
}

pub fn knight_die(g: &mut Game) {
    // check for gib
    if g.r(g.self_).health < -40.0 {
        g.sound(g.self_, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        let h = g.r(g.self_).health;
        ThrowHead(g, "progs/h_knight.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib1.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib2.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib3.mdl", h);
        return;
    }

    // regular death
    g.sound(g.self_, CHAN_VOICE, "knight/kdeath.wav", 1.0, ATTN_NORM);

    if g.random() < 0.5 {
        knight_die1(g);
    } else {
        knight_dieb1(g);
    }
}

pub fn monster_knight(g: &mut Game) {
    if g.deathmatch != 0.0 {
        g.remove(g.self_);
        return;
    }

    g.precache_model("progs/knight.mdl");
    g.precache_model("progs/h_knight.mdl");

    g.precache_sound("knight/kdeath.wav");
    g.precache_sound("knight/khurt.wav");
    g.precache_sound("knight/ksight.wav");
    g.precache_sound("knight/sword1.wav");
    g.precache_sound("knight/sword2.wav");
    g.precache_sound("knight/idle.wav");

    let s = g.self_;
    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/knight.mdl");

    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 40.0));
    g.v(s).health = 75.0;

    g.v(s).th_stand = Some(knight_stand1);
    g.v(s).th_walk = Some(knight_walk1);
    g.v(s).th_run = Some(knight_run1);
    g.v(s).th_melee = Some(knight_atk1);
    g.v(s).th_pain = Some(knight_pain);
    g.v(s).th_die = Some(knight_die);

    walkmonster_start(g);
}
