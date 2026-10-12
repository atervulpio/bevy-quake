//! `fish.qc`, id's QuakeC 1.06, in Rust.

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
const F_attack12: f32 = 11.0;
const F_attack13: f32 = 12.0;
const F_attack14: f32 = 13.0;
const F_attack15: f32 = 14.0;
const F_attack16: f32 = 15.0;
const F_attack17: f32 = 16.0;
const F_attack18: f32 = 17.0;
const F_death1: f32 = 18.0;
const F_death2: f32 = 19.0;
const F_death3: f32 = 20.0;
const F_death4: f32 = 21.0;
const F_death5: f32 = 22.0;
const F_death6: f32 = 23.0;
const F_death7: f32 = 24.0;
const F_death8: f32 = 25.0;
const F_death9: f32 = 26.0;
const F_death10: f32 = 27.0;
const F_death11: f32 = 28.0;
const F_death12: f32 = 29.0;
const F_death13: f32 = 30.0;
const F_death14: f32 = 31.0;
const F_death15: f32 = 32.0;
const F_death16: f32 = 33.0;
const F_death17: f32 = 34.0;
const F_death18: f32 = 35.0;
const F_death19: f32 = 36.0;
const F_death20: f32 = 37.0;
const F_death21: f32 = 38.0;
const F_swim1: f32 = 39.0;
const F_swim2: f32 = 40.0;
const F_swim3: f32 = 41.0;
const F_swim4: f32 = 42.0;
const F_swim5: f32 = 43.0;
const F_swim6: f32 = 44.0;
const F_swim7: f32 = 45.0;
const F_swim8: f32 = 46.0;
const F_swim9: f32 = 47.0;
const F_swim10: f32 = 48.0;
const F_swim11: f32 = 49.0;
const F_swim12: f32 = 50.0;
const F_swim13: f32 = 51.0;
const F_swim14: f32 = 52.0;
const F_swim15: f32 = 53.0;
const F_swim16: f32 = 54.0;
const F_swim17: f32 = 55.0;
const F_swim18: f32 = 56.0;
const F_pain1: f32 = 57.0;
const F_pain2: f32 = 58.0;
const F_pain3: f32 = 59.0;
const F_pain4: f32 = 60.0;
const F_pain5: f32 = 61.0;
const F_pain6: f32 = 62.0;
const F_pain7: f32 = 63.0;
const F_pain8: f32 = 64.0;
const F_pain9: f32 = 65.0;


pub fn f_stand1(g: &mut Game) {
    g.frame_state(F_swim1, f_stand2);
    ai_stand(g);
}

pub fn f_stand2(g: &mut Game) {
    g.frame_state(F_swim2, f_stand3);
    ai_stand(g);
}

pub fn f_stand3(g: &mut Game) {
    g.frame_state(F_swim3, f_stand4);
    ai_stand(g);
}

pub fn f_stand4(g: &mut Game) {
    g.frame_state(F_swim4, f_stand5);
    ai_stand(g);
}

pub fn f_stand5(g: &mut Game) {
    g.frame_state(F_swim5, f_stand6);
    ai_stand(g);
}

pub fn f_stand6(g: &mut Game) {
    g.frame_state(F_swim6, f_stand7);
    ai_stand(g);
}

pub fn f_stand7(g: &mut Game) {
    g.frame_state(F_swim7, f_stand8);
    ai_stand(g);
}

pub fn f_stand8(g: &mut Game) {
    g.frame_state(F_swim8, f_stand9);
    ai_stand(g);
}

pub fn f_stand9(g: &mut Game) {
    g.frame_state(F_swim9, f_stand10);
    ai_stand(g);
}

pub fn f_stand10(g: &mut Game) {
    g.frame_state(F_swim10, f_stand11);
    ai_stand(g);
}

pub fn f_stand11(g: &mut Game) {
    g.frame_state(F_swim11, f_stand12);
    ai_stand(g);
}

pub fn f_stand12(g: &mut Game) {
    g.frame_state(F_swim12, f_stand13);
    ai_stand(g);
}

pub fn f_stand13(g: &mut Game) {
    g.frame_state(F_swim13, f_stand14);
    ai_stand(g);
}

pub fn f_stand14(g: &mut Game) {
    g.frame_state(F_swim14, f_stand15);
    ai_stand(g);
}

pub fn f_stand15(g: &mut Game) {
    g.frame_state(F_swim15, f_stand16);
    ai_stand(g);
}

pub fn f_stand16(g: &mut Game) {
    g.frame_state(F_swim16, f_stand17);
    ai_stand(g);
}

pub fn f_stand17(g: &mut Game) {
    g.frame_state(F_swim17, f_stand18);
    ai_stand(g);
}

pub fn f_stand18(g: &mut Game) {
    g.frame_state(F_swim18, f_stand1);
    ai_stand(g);
}

pub fn f_walk1(g: &mut Game) {
    g.frame_state(F_swim1, f_walk2);
    ai_walk(g, 8.0);
}

pub fn f_walk2(g: &mut Game) {
    g.frame_state(F_swim2, f_walk3);
    ai_walk(g, 8.0);
}

pub fn f_walk3(g: &mut Game) {
    g.frame_state(F_swim3, f_walk4);
    ai_walk(g, 8.0);
}

pub fn f_walk4(g: &mut Game) {
    g.frame_state(F_swim4, f_walk5);
    ai_walk(g, 8.0);
}

pub fn f_walk5(g: &mut Game) {
    g.frame_state(F_swim5, f_walk6);
    ai_walk(g, 8.0);
}

pub fn f_walk6(g: &mut Game) {
    g.frame_state(F_swim6, f_walk7);
    ai_walk(g, 8.0);
}

pub fn f_walk7(g: &mut Game) {
    g.frame_state(F_swim7, f_walk8);
    ai_walk(g, 8.0);
}

pub fn f_walk8(g: &mut Game) {
    g.frame_state(F_swim8, f_walk9);
    ai_walk(g, 8.0);
}

pub fn f_walk9(g: &mut Game) {
    g.frame_state(F_swim9, f_walk10);
    ai_walk(g, 8.0);
}

pub fn f_walk10(g: &mut Game) {
    g.frame_state(F_swim10, f_walk11);
    ai_walk(g, 8.0);
}

pub fn f_walk11(g: &mut Game) {
    g.frame_state(F_swim11, f_walk12);
    ai_walk(g, 8.0);
}

pub fn f_walk12(g: &mut Game) {
    g.frame_state(F_swim12, f_walk13);
    ai_walk(g, 8.0);
}

pub fn f_walk13(g: &mut Game) {
    g.frame_state(F_swim13, f_walk14);
    ai_walk(g, 8.0);
}

pub fn f_walk14(g: &mut Game) {
    g.frame_state(F_swim14, f_walk15);
    ai_walk(g, 8.0);
}

pub fn f_walk15(g: &mut Game) {
    g.frame_state(F_swim15, f_walk16);
    ai_walk(g, 8.0);
}

pub fn f_walk16(g: &mut Game) {
    g.frame_state(F_swim16, f_walk17);
    ai_walk(g, 8.0);
}

pub fn f_walk17(g: &mut Game) {
    g.frame_state(F_swim17, f_walk18);
    ai_walk(g, 8.0);
}

pub fn f_walk18(g: &mut Game) {
    g.frame_state(F_swim18, f_walk1);
    ai_walk(g, 8.0);
}

pub fn f_run1(g: &mut Game) {
    g.frame_state(F_swim1, f_run2);
    ai_run(g, 12.0);
    if g.random() < 0.5 {
        g.sound(g.self_, CHAN_VOICE, "fish/idle.wav", 1.0, ATTN_NORM);
    }
}

pub fn f_run2(g: &mut Game) {
    g.frame_state(F_swim3, f_run3);
    ai_run(g, 12.0);
}

pub fn f_run3(g: &mut Game) {
    g.frame_state(F_swim5, f_run4);
    ai_run(g, 12.0);
}

pub fn f_run4(g: &mut Game) {
    g.frame_state(F_swim7, f_run5);
    ai_run(g, 12.0);
}

pub fn f_run5(g: &mut Game) {
    g.frame_state(F_swim9, f_run6);
    ai_run(g, 12.0);
}

pub fn f_run6(g: &mut Game) {
    g.frame_state(F_swim11, f_run7);
    ai_run(g, 12.0);
}

pub fn f_run7(g: &mut Game) {
    g.frame_state(F_swim13, f_run8);
    ai_run(g, 12.0);
}

pub fn f_run8(g: &mut Game) {
    g.frame_state(F_swim15, f_run9);
    ai_run(g, 12.0);
}

pub fn f_run9(g: &mut Game) {
    g.frame_state(F_swim17, f_run1);
    ai_run(g, 12.0);
}

pub fn fish_melee(g: &mut Game) {
    let s = g.self_;
    let enemy = g.r(s).enemy;
    if enemy == world {
        return; // removed before stroke
    }

    let delta = g.r(enemy).origin - g.r(s).origin;
    if vlen(delta) > 60.0 {
        return;
    }

    g.sound(s, CHAN_VOICE, "fish/bite.wav", 1.0, ATTN_NORM);
    let ldmg = g.random_sum2_as_compiled() * 3.0;
    T_Damage(g, enemy, s, s, ldmg);
}

pub fn f_attack1(g: &mut Game) {
    g.frame_state(F_attack1, f_attack2);
    ai_charge(g, 10.0);
}

pub fn f_attack2(g: &mut Game) {
    g.frame_state(F_attack2, f_attack3);
    ai_charge(g, 10.0);
}

pub fn f_attack3(g: &mut Game) {
    g.frame_state(F_attack3, f_attack4);
    fish_melee(g);
}

pub fn f_attack4(g: &mut Game) {
    g.frame_state(F_attack4, f_attack5);
    ai_charge(g, 10.0);
}

pub fn f_attack5(g: &mut Game) {
    g.frame_state(F_attack5, f_attack6);
    ai_charge(g, 10.0);
}

pub fn f_attack6(g: &mut Game) {
    g.frame_state(F_attack6, f_attack7);
    ai_charge(g, 10.0);
}

pub fn f_attack7(g: &mut Game) {
    g.frame_state(F_attack7, f_attack8);
    ai_charge(g, 10.0);
}

pub fn f_attack8(g: &mut Game) {
    g.frame_state(F_attack8, f_attack9);
    ai_charge(g, 10.0);
}

pub fn f_attack9(g: &mut Game) {
    g.frame_state(F_attack9, f_attack10);
    fish_melee(g);
}

pub fn f_attack10(g: &mut Game) {
    g.frame_state(F_attack10, f_attack11);
    ai_charge(g, 10.0);
}

pub fn f_attack11(g: &mut Game) {
    g.frame_state(F_attack11, f_attack12);
    ai_charge(g, 10.0);
}

pub fn f_attack12(g: &mut Game) {
    g.frame_state(F_attack12, f_attack13);
    ai_charge(g, 10.0);
}

pub fn f_attack13(g: &mut Game) {
    g.frame_state(F_attack13, f_attack14);
    ai_charge(g, 10.0);
}

pub fn f_attack14(g: &mut Game) {
    g.frame_state(F_attack14, f_attack15);
    ai_charge(g, 10.0);
}

pub fn f_attack15(g: &mut Game) {
    g.frame_state(F_attack15, f_attack16);
    fish_melee(g);
}

pub fn f_attack16(g: &mut Game) {
    g.frame_state(F_attack16, f_attack17);
    ai_charge(g, 10.0);
}

pub fn f_attack17(g: &mut Game) {
    g.frame_state(F_attack17, f_attack18);
    ai_charge(g, 10.0);
}

pub fn f_attack18(g: &mut Game) {
    g.frame_state(F_attack18, f_run1);
    ai_charge(g, 10.0);
}

pub fn f_death1(g: &mut Game) {
    g.frame_state(F_death1, f_death2);
    g.sound(g.self_, CHAN_VOICE, "fish/death.wav", 1.0, ATTN_NORM);
}

pub fn f_death2(g: &mut Game) {
    g.frame_state(F_death2, f_death3);
}

pub fn f_death3(g: &mut Game) {
    g.frame_state(F_death3, f_death4);
}

pub fn f_death4(g: &mut Game) {
    g.frame_state(F_death4, f_death5);
}

pub fn f_death5(g: &mut Game) {
    g.frame_state(F_death5, f_death6);
}

pub fn f_death6(g: &mut Game) {
    g.frame_state(F_death6, f_death7);
}

pub fn f_death7(g: &mut Game) {
    g.frame_state(F_death7, f_death8);
}

pub fn f_death8(g: &mut Game) {
    g.frame_state(F_death8, f_death9);
}

pub fn f_death9(g: &mut Game) {
    g.frame_state(F_death9, f_death10);
}

pub fn f_death10(g: &mut Game) {
    g.frame_state(F_death10, f_death11);
}

pub fn f_death11(g: &mut Game) {
    g.frame_state(F_death11, f_death12);
}

pub fn f_death12(g: &mut Game) {
    g.frame_state(F_death12, f_death13);
}

pub fn f_death13(g: &mut Game) {
    g.frame_state(F_death13, f_death14);
}

pub fn f_death14(g: &mut Game) {
    g.frame_state(F_death14, f_death15);
}

pub fn f_death15(g: &mut Game) {
    g.frame_state(F_death15, f_death16);
}

pub fn f_death16(g: &mut Game) {
    g.frame_state(F_death16, f_death17);
}

pub fn f_death17(g: &mut Game) {
    g.frame_state(F_death17, f_death18);
}

pub fn f_death18(g: &mut Game) {
    g.frame_state(F_death18, f_death19);
}

pub fn f_death19(g: &mut Game) {
    g.frame_state(F_death19, f_death20);
}

pub fn f_death20(g: &mut Game) {
    g.frame_state(F_death20, f_death21);
}

pub fn f_death21(g: &mut Game) {
    g.frame_state(F_death21, f_death21);
    g.v(g.self_).solid = SOLID_NOT;
}

pub fn f_pain1(g: &mut Game) {
    g.frame_state(F_pain1, f_pain2);
}

pub fn f_pain2(g: &mut Game) {
    g.frame_state(F_pain2, f_pain3);
    ai_pain(g, 6.0);
}

pub fn f_pain3(g: &mut Game) {
    g.frame_state(F_pain3, f_pain4);
    ai_pain(g, 6.0);
}

pub fn f_pain4(g: &mut Game) {
    g.frame_state(F_pain4, f_pain5);
    ai_pain(g, 6.0);
}

pub fn f_pain5(g: &mut Game) {
    g.frame_state(F_pain5, f_pain6);
    ai_pain(g, 6.0);
}

pub fn f_pain6(g: &mut Game) {
    g.frame_state(F_pain6, f_pain7);
    ai_pain(g, 6.0);
}

pub fn f_pain7(g: &mut Game) {
    g.frame_state(F_pain7, f_pain8);
    ai_pain(g, 6.0);
}

pub fn f_pain8(g: &mut Game) {
    g.frame_state(F_pain8, f_pain9);
    ai_pain(g, 6.0);
}

pub fn f_pain9(g: &mut Game) {
    g.frame_state(F_pain9, f_run1);
    ai_pain(g, 6.0);
}

pub fn fish_pain(g: &mut Game, attacker: Ent, damage: f32) {
    // fish always do pain frames
    f_pain1(g);
}

pub fn monster_fish(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model2("progs/fish.mdl");

    g.precache_sound2("fish/death.wav");
    g.precache_sound2("fish/bite.wav");
    g.precache_sound2("fish/idle.wav");

    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/fish.mdl");

    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 24.0));
    g.v(s).health = 25.0;

    g.v(s).th_stand = Some(f_stand1);
    g.v(s).th_walk = Some(f_walk1);
    g.v(s).th_run = Some(f_run1);
    g.v(s).th_die = Some(f_death1);
    g.v(s).th_pain = Some(fish_pain);
    g.v(s).th_melee = Some(f_attack1);

    swimmonster_start(g);
}
