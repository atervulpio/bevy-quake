//! `hknight.qc`, id's QuakeC 1.06, in Rust.

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
const F_walk1: f32 = 9.0;
const F_walk2: f32 = 10.0;
const F_walk3: f32 = 11.0;
const F_walk4: f32 = 12.0;
const F_walk5: f32 = 13.0;
const F_walk6: f32 = 14.0;
const F_walk7: f32 = 15.0;
const F_walk8: f32 = 16.0;
const F_walk9: f32 = 17.0;
const F_walk10: f32 = 18.0;
const F_walk11: f32 = 19.0;
const F_walk12: f32 = 20.0;
const F_walk13: f32 = 21.0;
const F_walk14: f32 = 22.0;
const F_walk15: f32 = 23.0;
const F_walk16: f32 = 24.0;
const F_walk17: f32 = 25.0;
const F_walk18: f32 = 26.0;
const F_walk19: f32 = 27.0;
const F_walk20: f32 = 28.0;
const F_run1: f32 = 29.0;
const F_run2: f32 = 30.0;
const F_run3: f32 = 31.0;
const F_run4: f32 = 32.0;
const F_run5: f32 = 33.0;
const F_run6: f32 = 34.0;
const F_run7: f32 = 35.0;
const F_run8: f32 = 36.0;
const F_pain1: f32 = 37.0;
const F_pain2: f32 = 38.0;
const F_pain3: f32 = 39.0;
const F_pain4: f32 = 40.0;
const F_pain5: f32 = 41.0;
const F_death1: f32 = 42.0;
const F_death2: f32 = 43.0;
const F_death3: f32 = 44.0;
const F_death4: f32 = 45.0;
const F_death5: f32 = 46.0;
const F_death6: f32 = 47.0;
const F_death7: f32 = 48.0;
const F_death8: f32 = 49.0;
const F_death9: f32 = 50.0;
const F_death10: f32 = 51.0;
const F_death11: f32 = 52.0;
const F_death12: f32 = 53.0;
const F_deathb1: f32 = 54.0;
const F_deathb2: f32 = 55.0;
const F_deathb3: f32 = 56.0;
const F_deathb4: f32 = 57.0;
const F_deathb5: f32 = 58.0;
const F_deathb6: f32 = 59.0;
const F_deathb7: f32 = 60.0;
const F_deathb8: f32 = 61.0;
const F_deathb9: f32 = 62.0;
const F_char_a1: f32 = 63.0;
const F_char_a2: f32 = 64.0;
const F_char_a3: f32 = 65.0;
const F_char_a4: f32 = 66.0;
const F_char_a5: f32 = 67.0;
const F_char_a6: f32 = 68.0;
const F_char_a7: f32 = 69.0;
const F_char_a8: f32 = 70.0;
const F_char_a9: f32 = 71.0;
const F_char_a10: f32 = 72.0;
const F_char_a11: f32 = 73.0;
const F_char_a12: f32 = 74.0;
const F_char_a13: f32 = 75.0;
const F_char_a14: f32 = 76.0;
const F_char_a15: f32 = 77.0;
const F_char_a16: f32 = 78.0;
const F_magica1: f32 = 79.0;
const F_magica2: f32 = 80.0;
const F_magica3: f32 = 81.0;
const F_magica4: f32 = 82.0;
const F_magica5: f32 = 83.0;
const F_magica6: f32 = 84.0;
const F_magica7: f32 = 85.0;
const F_magica8: f32 = 86.0;
const F_magica9: f32 = 87.0;
const F_magica10: f32 = 88.0;
const F_magica11: f32 = 89.0;
const F_magica12: f32 = 90.0;
const F_magica13: f32 = 91.0;
const F_magica14: f32 = 92.0;
const F_magicb1: f32 = 93.0;
const F_magicb2: f32 = 94.0;
const F_magicb3: f32 = 95.0;
const F_magicb4: f32 = 96.0;
const F_magicb5: f32 = 97.0;
const F_magicb6: f32 = 98.0;
const F_magicb7: f32 = 99.0;
const F_magicb8: f32 = 100.0;
const F_magicb9: f32 = 101.0;
const F_magicb10: f32 = 102.0;
const F_magicb11: f32 = 103.0;
const F_magicb12: f32 = 104.0;
const F_magicb13: f32 = 105.0;
const F_char_b1: f32 = 106.0;
const F_char_b2: f32 = 107.0;
const F_char_b3: f32 = 108.0;
const F_char_b4: f32 = 109.0;
const F_char_b5: f32 = 110.0;
const F_char_b6: f32 = 111.0;
const F_slice1: f32 = 112.0;
const F_slice2: f32 = 113.0;
const F_slice3: f32 = 114.0;
const F_slice4: f32 = 115.0;
const F_slice5: f32 = 116.0;
const F_slice6: f32 = 117.0;
const F_slice7: f32 = 118.0;
const F_slice8: f32 = 119.0;
const F_slice9: f32 = 120.0;
const F_slice10: f32 = 121.0;
const F_smash1: f32 = 122.0;
const F_smash2: f32 = 123.0;
const F_smash3: f32 = 124.0;
const F_smash4: f32 = 125.0;
const F_smash5: f32 = 126.0;
const F_smash6: f32 = 127.0;
const F_smash7: f32 = 128.0;
const F_smash8: f32 = 129.0;
const F_smash9: f32 = 130.0;
const F_smash10: f32 = 131.0;
const F_smash11: f32 = 132.0;
const F_w_attack1: f32 = 133.0;
const F_w_attack2: f32 = 134.0;
const F_w_attack3: f32 = 135.0;
const F_w_attack4: f32 = 136.0;
const F_w_attack5: f32 = 137.0;
const F_w_attack6: f32 = 138.0;
const F_w_attack7: f32 = 139.0;
const F_w_attack8: f32 = 140.0;
const F_w_attack9: f32 = 141.0;
const F_w_attack10: f32 = 142.0;
const F_w_attack11: f32 = 143.0;
const F_w_attack12: f32 = 144.0;
const F_w_attack13: f32 = 145.0;
const F_w_attack14: f32 = 146.0;
const F_w_attack15: f32 = 147.0;
const F_w_attack16: f32 = 148.0;
const F_w_attack17: f32 = 149.0;
const F_w_attack18: f32 = 150.0;
const F_w_attack19: f32 = 151.0;
const F_w_attack20: f32 = 152.0;
const F_w_attack21: f32 = 153.0;
const F_w_attack22: f32 = 154.0;
const F_magicc1: f32 = 155.0;
const F_magicc2: f32 = 156.0;
const F_magicc3: f32 = 157.0;
const F_magicc4: f32 = 158.0;
const F_magicc5: f32 = 159.0;
const F_magicc6: f32 = 160.0;
const F_magicc7: f32 = 161.0;
const F_magicc8: f32 = 162.0;
const F_magicc9: f32 = 163.0;
const F_magicc10: f32 = 164.0;
const F_magicc11: f32 = 165.0;


pub fn hknight_shot(g: &mut Game, offset: f32) {
    let s = g.self_;
    let enemy = g.r(s).enemy;
    let mut offang = vectoangles(g.r(enemy).origin - g.r(s).origin);
    offang.y = offang.y + offset * 6.0;

    g.makevectors(offang);

    let org = g.r(s).origin + g.r(s).mins + g.r(s).size * 0.5 + g.v_forward * 20.0;

    // set missile speed
    let mut vec = normalize(g.v_forward);
    vec.z = 0.0 - vec.z + (g.random() - 0.5) * 0.1;

    launch_spike(g, org, vec);
    let newmis = g.newmis;
    g.v(newmis).classname = "knightspike";
    g.setmodel(newmis, "progs/k_spike.mdl");
    g.setsize(newmis, VEC_ORIGIN, VEC_ORIGIN);
    g.v(newmis).velocity = vec * 300.0;
    g.sound(g.self_, CHAN_WEAPON, "hknight/attack1.wav", 1.0, ATTN_NORM);
}

pub fn CheckForCharge(g: &mut Game) {
    let s = g.self_;
    // check for mad charge
    if g.enemy_vis == 0.0 {
        return;
    }

    if g.time < g.r(s).attack_finished {
        return;
    }

    let enemy = g.r(s).enemy;
    if fabs(g.r(s).origin.z - g.r(enemy).origin.z) > 20.0 {
        return; // too much height change
    }
    if vlen(g.r(s).origin - g.r(enemy).origin) < 80.0 {
        return; // use regular attack
    }

    // charge
    SUB_AttackFinished(g, 2.0);
    hknight_char_a1(g);
}

pub fn CheckContinueCharge(g: &mut Game) {
    if g.time > g.r(g.self_).attack_finished {
        SUB_AttackFinished(g, 3.0);
        hknight_run1(g);
        return; // done charging
    }

    if g.random() > 0.5 {
        g.sound(g.self_, CHAN_WEAPON, "knight/sword2.wav", 1.0, ATTN_NORM);
    } else {
        g.sound(g.self_, CHAN_WEAPON, "knight/sword1.wav", 1.0, ATTN_NORM);
    }
}

pub fn hknight_stand1(g: &mut Game) {
    g.frame_state(F_stand1, hknight_stand2);
    ai_stand(g);
}

pub fn hknight_stand2(g: &mut Game) {
    g.frame_state(F_stand2, hknight_stand3);
    ai_stand(g);
}

pub fn hknight_stand3(g: &mut Game) {
    g.frame_state(F_stand3, hknight_stand4);
    ai_stand(g);
}

pub fn hknight_stand4(g: &mut Game) {
    g.frame_state(F_stand4, hknight_stand5);
    ai_stand(g);
}

pub fn hknight_stand5(g: &mut Game) {
    g.frame_state(F_stand5, hknight_stand6);
    ai_stand(g);
}

pub fn hknight_stand6(g: &mut Game) {
    g.frame_state(F_stand6, hknight_stand7);
    ai_stand(g);
}

pub fn hknight_stand7(g: &mut Game) {
    g.frame_state(F_stand7, hknight_stand8);
    ai_stand(g);
}

pub fn hknight_stand8(g: &mut Game) {
    g.frame_state(F_stand8, hknight_stand9);
    ai_stand(g);
}

pub fn hknight_stand9(g: &mut Game) {
    g.frame_state(F_stand9, hknight_stand1);
    ai_stand(g);
}

pub fn hknight_walk1(g: &mut Game) {
    g.frame_state(F_walk1, hknight_walk2);
    hk_idle_sound(g);
    ai_walk(g, 2.0);
}

pub fn hknight_walk2(g: &mut Game) {
    g.frame_state(F_walk2, hknight_walk3);
    ai_walk(g, 5.0);
}

pub fn hknight_walk3(g: &mut Game) {
    g.frame_state(F_walk3, hknight_walk4);
    ai_walk(g, 5.0);
}

pub fn hknight_walk4(g: &mut Game) {
    g.frame_state(F_walk4, hknight_walk5);
    ai_walk(g, 4.0);
}

pub fn hknight_walk5(g: &mut Game) {
    g.frame_state(F_walk5, hknight_walk6);
    ai_walk(g, 4.0);
}

pub fn hknight_walk6(g: &mut Game) {
    g.frame_state(F_walk6, hknight_walk7);
    ai_walk(g, 2.0);
}

pub fn hknight_walk7(g: &mut Game) {
    g.frame_state(F_walk7, hknight_walk8);
    ai_walk(g, 2.0);
}

pub fn hknight_walk8(g: &mut Game) {
    g.frame_state(F_walk8, hknight_walk9);
    ai_walk(g, 3.0);
}

pub fn hknight_walk9(g: &mut Game) {
    g.frame_state(F_walk9, hknight_walk10);
    ai_walk(g, 3.0);
}

pub fn hknight_walk10(g: &mut Game) {
    g.frame_state(F_walk10, hknight_walk11);
    ai_walk(g, 4.0);
}

pub fn hknight_walk11(g: &mut Game) {
    g.frame_state(F_walk11, hknight_walk12);
    ai_walk(g, 3.0);
}

pub fn hknight_walk12(g: &mut Game) {
    g.frame_state(F_walk12, hknight_walk13);
    ai_walk(g, 4.0);
}

pub fn hknight_walk13(g: &mut Game) {
    g.frame_state(F_walk13, hknight_walk14);
    ai_walk(g, 6.0);
}

pub fn hknight_walk14(g: &mut Game) {
    g.frame_state(F_walk14, hknight_walk15);
    ai_walk(g, 2.0);
}

pub fn hknight_walk15(g: &mut Game) {
    g.frame_state(F_walk15, hknight_walk16);
    ai_walk(g, 2.0);
}

pub fn hknight_walk16(g: &mut Game) {
    g.frame_state(F_walk16, hknight_walk17);
    ai_walk(g, 4.0);
}

pub fn hknight_walk17(g: &mut Game) {
    g.frame_state(F_walk17, hknight_walk18);
    ai_walk(g, 3.0);
}

pub fn hknight_walk18(g: &mut Game) {
    g.frame_state(F_walk18, hknight_walk19);
    ai_walk(g, 3.0);
}

pub fn hknight_walk19(g: &mut Game) {
    g.frame_state(F_walk19, hknight_walk20);
    ai_walk(g, 3.0);
}

pub fn hknight_walk20(g: &mut Game) {
    g.frame_state(F_walk20, hknight_walk1);
    ai_walk(g, 2.0);
}

pub fn hknight_run1(g: &mut Game) {
    g.frame_state(F_run1, hknight_run2);
    hk_idle_sound(g);
    ai_run(g, 20.0);
    CheckForCharge(g);
}

pub fn hknight_run2(g: &mut Game) {
    g.frame_state(F_run2, hknight_run3);
    ai_run(g, 25.0);
}

pub fn hknight_run3(g: &mut Game) {
    g.frame_state(F_run3, hknight_run4);
    ai_run(g, 18.0);
}

pub fn hknight_run4(g: &mut Game) {
    g.frame_state(F_run4, hknight_run5);
    ai_run(g, 16.0);
}

pub fn hknight_run5(g: &mut Game) {
    g.frame_state(F_run5, hknight_run6);
    ai_run(g, 14.0);
}

pub fn hknight_run6(g: &mut Game) {
    g.frame_state(F_run6, hknight_run7);
    ai_run(g, 25.0);
}

pub fn hknight_run7(g: &mut Game) {
    g.frame_state(F_run7, hknight_run8);
    ai_run(g, 21.0);
}

pub fn hknight_run8(g: &mut Game) {
    g.frame_state(F_run8, hknight_run1);
    ai_run(g, 13.0);
}

pub fn hknight_pain1(g: &mut Game) {
    g.frame_state(F_pain1, hknight_pain2);
    g.sound(g.self_, CHAN_VOICE, "hknight/pain1.wav", 1.0, ATTN_NORM);
}

pub fn hknight_pain2(g: &mut Game) {
    g.frame_state(F_pain2, hknight_pain3);
}

pub fn hknight_pain3(g: &mut Game) {
    g.frame_state(F_pain3, hknight_pain4);
}

pub fn hknight_pain4(g: &mut Game) {
    g.frame_state(F_pain4, hknight_pain5);
}

pub fn hknight_pain5(g: &mut Game) {
    g.frame_state(F_pain5, hknight_run1);
}

pub fn hknight_die1(g: &mut Game) {
    g.frame_state(F_death1, hknight_die2);
    ai_forward(g, 10.0);
}

pub fn hknight_die2(g: &mut Game) {
    g.frame_state(F_death2, hknight_die3);
    ai_forward(g, 8.0);
}

pub fn hknight_die3(g: &mut Game) {
    g.frame_state(F_death3, hknight_die4);
    g.v(g.self_).solid = SOLID_NOT;
    ai_forward(g, 7.0);
}

pub fn hknight_die4(g: &mut Game) {
    g.frame_state(F_death4, hknight_die5);
}

pub fn hknight_die5(g: &mut Game) {
    g.frame_state(F_death5, hknight_die6);
}

pub fn hknight_die6(g: &mut Game) {
    g.frame_state(F_death6, hknight_die7);
}

pub fn hknight_die7(g: &mut Game) {
    g.frame_state(F_death7, hknight_die8);
}

pub fn hknight_die8(g: &mut Game) {
    g.frame_state(F_death8, hknight_die9);
    ai_forward(g, 10.0);
}

pub fn hknight_die9(g: &mut Game) {
    g.frame_state(F_death9, hknight_die10);
    ai_forward(g, 11.0);
}

pub fn hknight_die10(g: &mut Game) {
    g.frame_state(F_death10, hknight_die11);
}

pub fn hknight_die11(g: &mut Game) {
    g.frame_state(F_death11, hknight_die12);
}

pub fn hknight_die12(g: &mut Game) {
    g.frame_state(F_death12, hknight_die12);
}

pub fn hknight_dieb1(g: &mut Game) {
    g.frame_state(F_deathb1, hknight_dieb2);
}

pub fn hknight_dieb2(g: &mut Game) {
    g.frame_state(F_deathb2, hknight_dieb3);
}

pub fn hknight_dieb3(g: &mut Game) {
    g.frame_state(F_deathb3, hknight_dieb4);
    g.v(g.self_).solid = SOLID_NOT;
}

pub fn hknight_dieb4(g: &mut Game) {
    g.frame_state(F_deathb4, hknight_dieb5);
}

pub fn hknight_dieb5(g: &mut Game) {
    g.frame_state(F_deathb5, hknight_dieb6);
}

pub fn hknight_dieb6(g: &mut Game) {
    g.frame_state(F_deathb6, hknight_dieb7);
}

pub fn hknight_dieb7(g: &mut Game) {
    g.frame_state(F_deathb7, hknight_dieb8);
}

pub fn hknight_dieb8(g: &mut Game) {
    g.frame_state(F_deathb8, hknight_dieb9);
}

pub fn hknight_dieb9(g: &mut Game) {
    g.frame_state(F_deathb9, hknight_dieb9);
}

pub fn hknight_die(g: &mut Game) {
    let s = g.self_;
    // check for gib
    if g.r(s).health < -40.0 {
        g.sound(s, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        ThrowHead(g, "progs/h_hellkn.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib1.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib2.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib3.mdl", g.r(s).health);
        return;
    }

    // regular death
    g.sound(s, CHAN_VOICE, "hknight/death1.wav", 1.0, ATTN_NORM);

    if g.random() > 0.5 {
        hknight_die1(g);
    } else {
        hknight_dieb1(g);
    }
}

pub fn hknight_magica1(g: &mut Game) {
    g.frame_state(F_magica1, hknight_magica2);
    ai_face(g);
}

pub fn hknight_magica2(g: &mut Game) {
    g.frame_state(F_magica2, hknight_magica3);
    ai_face(g);
}

pub fn hknight_magica3(g: &mut Game) {
    g.frame_state(F_magica3, hknight_magica4);
    ai_face(g);
}

pub fn hknight_magica4(g: &mut Game) {
    g.frame_state(F_magica4, hknight_magica5);
    ai_face(g);
}

pub fn hknight_magica5(g: &mut Game) {
    g.frame_state(F_magica5, hknight_magica6);
    ai_face(g);
}

pub fn hknight_magica6(g: &mut Game) {
    g.frame_state(F_magica6, hknight_magica7);
    ai_face(g);
}

pub fn hknight_magica7(g: &mut Game) {
    g.frame_state(F_magica7, hknight_magica8);
    hknight_shot(g, -2.0);
}

pub fn hknight_magica8(g: &mut Game) {
    g.frame_state(F_magica8, hknight_magica9);
    hknight_shot(g, -1.0);
}

pub fn hknight_magica9(g: &mut Game) {
    g.frame_state(F_magica9, hknight_magica10);
    hknight_shot(g, 0.0);
}

pub fn hknight_magica10(g: &mut Game) {
    g.frame_state(F_magica10, hknight_magica11);
    hknight_shot(g, 1.0);
}

pub fn hknight_magica11(g: &mut Game) {
    g.frame_state(F_magica11, hknight_magica12);
    hknight_shot(g, 2.0);
}

pub fn hknight_magica12(g: &mut Game) {
    g.frame_state(F_magica12, hknight_magica13);
    hknight_shot(g, 3.0);
}

pub fn hknight_magica13(g: &mut Game) {
    g.frame_state(F_magica13, hknight_magica14);
    ai_face(g);
}

pub fn hknight_magica14(g: &mut Game) {
    g.frame_state(F_magica14, hknight_run1);
    ai_face(g);
}

pub fn hknight_magicb1(g: &mut Game) {
    g.frame_state(F_magicb1, hknight_magicb2);
    ai_face(g);
}

pub fn hknight_magicb2(g: &mut Game) {
    g.frame_state(F_magicb2, hknight_magicb3);
    ai_face(g);
}

pub fn hknight_magicb3(g: &mut Game) {
    g.frame_state(F_magicb3, hknight_magicb4);
    ai_face(g);
}

pub fn hknight_magicb4(g: &mut Game) {
    g.frame_state(F_magicb4, hknight_magicb5);
    ai_face(g);
}

pub fn hknight_magicb5(g: &mut Game) {
    g.frame_state(F_magicb5, hknight_magicb6);
    ai_face(g);
}

pub fn hknight_magicb6(g: &mut Game) {
    g.frame_state(F_magicb6, hknight_magicb7);
    ai_face(g);
}

pub fn hknight_magicb7(g: &mut Game) {
    g.frame_state(F_magicb7, hknight_magicb8);
    hknight_shot(g, -2.0);
}

pub fn hknight_magicb8(g: &mut Game) {
    g.frame_state(F_magicb8, hknight_magicb9);
    hknight_shot(g, -1.0);
}

pub fn hknight_magicb9(g: &mut Game) {
    g.frame_state(F_magicb9, hknight_magicb10);
    hknight_shot(g, 0.0);
}

pub fn hknight_magicb10(g: &mut Game) {
    g.frame_state(F_magicb10, hknight_magicb11);
    hknight_shot(g, 1.0);
}

pub fn hknight_magicb11(g: &mut Game) {
    g.frame_state(F_magicb11, hknight_magicb12);
    hknight_shot(g, 2.0);
}

pub fn hknight_magicb12(g: &mut Game) {
    g.frame_state(F_magicb12, hknight_magicb13);
    hknight_shot(g, 3.0);
}

pub fn hknight_magicb13(g: &mut Game) {
    g.frame_state(F_magicb13, hknight_run1);
    ai_face(g);
}

pub fn hknight_magicc1(g: &mut Game) {
    g.frame_state(F_magicc1, hknight_magicc2);
    ai_face(g);
}

pub fn hknight_magicc2(g: &mut Game) {
    g.frame_state(F_magicc2, hknight_magicc3);
    ai_face(g);
}

pub fn hknight_magicc3(g: &mut Game) {
    g.frame_state(F_magicc3, hknight_magicc4);
    ai_face(g);
}

pub fn hknight_magicc4(g: &mut Game) {
    g.frame_state(F_magicc4, hknight_magicc5);
    ai_face(g);
}

pub fn hknight_magicc5(g: &mut Game) {
    g.frame_state(F_magicc5, hknight_magicc6);
    ai_face(g);
}

pub fn hknight_magicc6(g: &mut Game) {
    g.frame_state(F_magicc6, hknight_magicc7);
    hknight_shot(g, -2.0);
}

pub fn hknight_magicc7(g: &mut Game) {
    g.frame_state(F_magicc7, hknight_magicc8);
    hknight_shot(g, -1.0);
}

pub fn hknight_magicc8(g: &mut Game) {
    g.frame_state(F_magicc8, hknight_magicc9);
    hknight_shot(g, 0.0);
}

pub fn hknight_magicc9(g: &mut Game) {
    g.frame_state(F_magicc9, hknight_magicc10);
    hknight_shot(g, 1.0);
}

pub fn hknight_magicc10(g: &mut Game) {
    g.frame_state(F_magicc10, hknight_magicc11);
    hknight_shot(g, 2.0);
}

pub fn hknight_magicc11(g: &mut Game) {
    g.frame_state(F_magicc11, hknight_run1);
    hknight_shot(g, 3.0);
}

pub fn hknight_char_a1(g: &mut Game) {
    g.frame_state(F_char_a1, hknight_char_a2);
    ai_charge(g, 20.0);
}

pub fn hknight_char_a2(g: &mut Game) {
    g.frame_state(F_char_a2, hknight_char_a3);
    ai_charge(g, 25.0);
}

pub fn hknight_char_a3(g: &mut Game) {
    g.frame_state(F_char_a3, hknight_char_a4);
    ai_charge(g, 18.0);
}

pub fn hknight_char_a4(g: &mut Game) {
    g.frame_state(F_char_a4, hknight_char_a5);
    ai_charge(g, 16.0);
}

pub fn hknight_char_a5(g: &mut Game) {
    g.frame_state(F_char_a5, hknight_char_a6);
    ai_charge(g, 14.0);
}

pub fn hknight_char_a6(g: &mut Game) {
    g.frame_state(F_char_a6, hknight_char_a7);
    ai_charge(g, 20.0);
    ai_melee(g);
}

pub fn hknight_char_a7(g: &mut Game) {
    g.frame_state(F_char_a7, hknight_char_a8);
    ai_charge(g, 21.0);
    ai_melee(g);
}

pub fn hknight_char_a8(g: &mut Game) {
    g.frame_state(F_char_a8, hknight_char_a9);
    ai_charge(g, 13.0);
    ai_melee(g);
}

pub fn hknight_char_a9(g: &mut Game) {
    g.frame_state(F_char_a9, hknight_char_a10);
    ai_charge(g, 20.0);
    ai_melee(g);
}

pub fn hknight_char_a10(g: &mut Game) {
    g.frame_state(F_char_a10, hknight_char_a11);
    ai_charge(g, 20.0);
    ai_melee(g);
}

pub fn hknight_char_a11(g: &mut Game) {
    g.frame_state(F_char_a11, hknight_char_a12);
    ai_charge(g, 18.0);
    ai_melee(g);
}

pub fn hknight_char_a12(g: &mut Game) {
    g.frame_state(F_char_a12, hknight_char_a13);
    ai_charge(g, 16.0);
}

pub fn hknight_char_a13(g: &mut Game) {
    g.frame_state(F_char_a13, hknight_char_a14);
    ai_charge(g, 14.0);
}

pub fn hknight_char_a14(g: &mut Game) {
    g.frame_state(F_char_a14, hknight_char_a15);
    ai_charge(g, 25.0);
}

pub fn hknight_char_a15(g: &mut Game) {
    g.frame_state(F_char_a15, hknight_char_a16);
    ai_charge(g, 21.0);
}

pub fn hknight_char_a16(g: &mut Game) {
    g.frame_state(F_char_a16, hknight_run1);
    ai_charge(g, 13.0);
}

pub fn hknight_char_b1(g: &mut Game) {
    g.frame_state(F_char_b1, hknight_char_b2);
    CheckContinueCharge(g);
    ai_charge(g, 23.0);
    ai_melee(g);
}

pub fn hknight_char_b2(g: &mut Game) {
    g.frame_state(F_char_b2, hknight_char_b3);
    ai_charge(g, 17.0);
    ai_melee(g);
}

pub fn hknight_char_b3(g: &mut Game) {
    g.frame_state(F_char_b3, hknight_char_b4);
    ai_charge(g, 12.0);
    ai_melee(g);
}

pub fn hknight_char_b4(g: &mut Game) {
    g.frame_state(F_char_b4, hknight_char_b5);
    ai_charge(g, 22.0);
    ai_melee(g);
}

pub fn hknight_char_b5(g: &mut Game) {
    g.frame_state(F_char_b5, hknight_char_b6);
    ai_charge(g, 18.0);
    ai_melee(g);
}

pub fn hknight_char_b6(g: &mut Game) {
    g.frame_state(F_char_b6, hknight_char_b1);
    ai_charge(g, 8.0);
    ai_melee(g);
}

pub fn hknight_slice1(g: &mut Game) {
    g.frame_state(F_slice1, hknight_slice2);
    ai_charge(g, 9.0);
}

pub fn hknight_slice2(g: &mut Game) {
    g.frame_state(F_slice2, hknight_slice3);
    ai_charge(g, 6.0);
}

pub fn hknight_slice3(g: &mut Game) {
    g.frame_state(F_slice3, hknight_slice4);
    ai_charge(g, 13.0);
}

pub fn hknight_slice4(g: &mut Game) {
    g.frame_state(F_slice4, hknight_slice5);
    ai_charge(g, 4.0);
}

pub fn hknight_slice5(g: &mut Game) {
    g.frame_state(F_slice5, hknight_slice6);
    ai_charge(g, 7.0);
    ai_melee(g);
}

pub fn hknight_slice6(g: &mut Game) {
    g.frame_state(F_slice6, hknight_slice7);
    ai_charge(g, 15.0);
    ai_melee(g);
}

pub fn hknight_slice7(g: &mut Game) {
    g.frame_state(F_slice7, hknight_slice8);
    ai_charge(g, 8.0);
    ai_melee(g);
}

pub fn hknight_slice8(g: &mut Game) {
    g.frame_state(F_slice8, hknight_slice9);
    ai_charge(g, 2.0);
    ai_melee(g);
}

pub fn hknight_slice9(g: &mut Game) {
    g.frame_state(F_slice9, hknight_slice10);
    ai_melee(g);
}

pub fn hknight_slice10(g: &mut Game) {
    g.frame_state(F_slice10, hknight_run1);
    ai_charge(g, 3.0);
}

pub fn hknight_smash1(g: &mut Game) {
    g.frame_state(F_smash1, hknight_smash2);
    ai_charge(g, 1.0);
}

pub fn hknight_smash2(g: &mut Game) {
    g.frame_state(F_smash2, hknight_smash3);
    ai_charge(g, 13.0);
}

pub fn hknight_smash3(g: &mut Game) {
    g.frame_state(F_smash3, hknight_smash4);
    ai_charge(g, 9.0);
}

pub fn hknight_smash4(g: &mut Game) {
    g.frame_state(F_smash4, hknight_smash5);
    ai_charge(g, 11.0);
}

pub fn hknight_smash5(g: &mut Game) {
    g.frame_state(F_smash5, hknight_smash6);
    ai_charge(g, 10.0);
    ai_melee(g);
}

pub fn hknight_smash6(g: &mut Game) {
    g.frame_state(F_smash6, hknight_smash7);
    ai_charge(g, 7.0);
    ai_melee(g);
}

pub fn hknight_smash7(g: &mut Game) {
    g.frame_state(F_smash7, hknight_smash8);
    ai_charge(g, 12.0);
    ai_melee(g);
}

pub fn hknight_smash8(g: &mut Game) {
    g.frame_state(F_smash8, hknight_smash9);
    ai_charge(g, 2.0);
    ai_melee(g);
}

pub fn hknight_smash9(g: &mut Game) {
    g.frame_state(F_smash9, hknight_smash10);
    ai_charge(g, 3.0);
    ai_melee(g);
}

pub fn hknight_smash10(g: &mut Game) {
    g.frame_state(F_smash10, hknight_smash11);
    ai_charge(g, 0.0);
}

pub fn hknight_smash11(g: &mut Game) {
    g.frame_state(F_smash11, hknight_run1);
    ai_charge(g, 0.0);
}

pub fn hknight_watk1(g: &mut Game) {
    g.frame_state(F_w_attack1, hknight_watk2);
    ai_charge(g, 2.0);
}

pub fn hknight_watk2(g: &mut Game) {
    g.frame_state(F_w_attack2, hknight_watk3);
    ai_charge(g, 0.0);
}

pub fn hknight_watk3(g: &mut Game) {
    g.frame_state(F_w_attack3, hknight_watk4);
    ai_charge(g, 0.0);
}

pub fn hknight_watk4(g: &mut Game) {
    g.frame_state(F_w_attack4, hknight_watk5);
    ai_melee(g);
}

pub fn hknight_watk5(g: &mut Game) {
    g.frame_state(F_w_attack5, hknight_watk6);
    ai_melee(g);
}

pub fn hknight_watk6(g: &mut Game) {
    g.frame_state(F_w_attack6, hknight_watk7);
    ai_melee(g);
}

pub fn hknight_watk7(g: &mut Game) {
    g.frame_state(F_w_attack7, hknight_watk8);
    ai_charge(g, 1.0);
}

pub fn hknight_watk8(g: &mut Game) {
    g.frame_state(F_w_attack8, hknight_watk9);
    ai_charge(g, 4.0);
}

pub fn hknight_watk9(g: &mut Game) {
    g.frame_state(F_w_attack9, hknight_watk10);
    ai_charge(g, 5.0);
}

pub fn hknight_watk10(g: &mut Game) {
    g.frame_state(F_w_attack10, hknight_watk11);
    ai_charge(g, 3.0);
    ai_melee(g);
}

pub fn hknight_watk11(g: &mut Game) {
    g.frame_state(F_w_attack11, hknight_watk12);
    ai_charge(g, 2.0);
    ai_melee(g);
}

pub fn hknight_watk12(g: &mut Game) {
    g.frame_state(F_w_attack12, hknight_watk13);
    ai_charge(g, 2.0);
    ai_melee(g);
}

pub fn hknight_watk13(g: &mut Game) {
    g.frame_state(F_w_attack13, hknight_watk14);
    ai_charge(g, 0.0);
}

pub fn hknight_watk14(g: &mut Game) {
    g.frame_state(F_w_attack14, hknight_watk15);
    ai_charge(g, 0.0);
}

pub fn hknight_watk15(g: &mut Game) {
    g.frame_state(F_w_attack15, hknight_watk16);
    ai_charge(g, 0.0);
}

pub fn hknight_watk16(g: &mut Game) {
    g.frame_state(F_w_attack16, hknight_watk17);
    ai_charge(g, 1.0);
}

pub fn hknight_watk17(g: &mut Game) {
    g.frame_state(F_w_attack17, hknight_watk18);
    ai_charge(g, 1.0);
    ai_melee(g);
}

pub fn hknight_watk18(g: &mut Game) {
    g.frame_state(F_w_attack18, hknight_watk19);
    ai_charge(g, 3.0);
    ai_melee(g);
}

pub fn hknight_watk19(g: &mut Game) {
    g.frame_state(F_w_attack19, hknight_watk20);
    ai_charge(g, 4.0);
    ai_melee(g);
}

pub fn hknight_watk20(g: &mut Game) {
    g.frame_state(F_w_attack20, hknight_watk21);
    ai_charge(g, 6.0);
}

pub fn hknight_watk21(g: &mut Game) {
    g.frame_state(F_w_attack21, hknight_watk22);
    ai_charge(g, 7.0);
}

pub fn hknight_watk22(g: &mut Game) {
    g.frame_state(F_w_attack22, hknight_run1);
    ai_charge(g, 3.0);
}

pub fn hk_idle_sound(g: &mut Game) {
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "hknight/idle.wav", 1.0, ATTN_NORM);
    }
}

pub fn hknight_pain(g: &mut Game, attacker: Ent, damage: f32) {
    let s = g.self_;
    if g.r(s).pain_finished > g.time {
        return;
    }

    g.sound(s, CHAN_VOICE, "hknight/pain1.wav", 1.0, ATTN_NORM);

    if (g.time - g.r(s).pain_finished) > 5.0 {
        // always go into pain frame if it has been a while
        hknight_pain1(g);
        g.v(s).pain_finished = g.time + 1.0;
        return;
    }

    if (g.random() * 30.0) > damage {
        return; // didn't flinch
    }

    g.v(s).pain_finished = g.time + 1.0;
    hknight_pain1(g);
}

pub fn hknight_melee(g: &mut Game) {
    g.hknight_type = g.hknight_type + 1.0;

    g.sound(g.self_, CHAN_WEAPON, "hknight/slash1.wav", 1.0, ATTN_NORM);
    if g.hknight_type == 1.0 {
        hknight_slice1(g);
    } else if g.hknight_type == 2.0 {
        hknight_smash1(g);
    } else if g.hknight_type == 3.0 {
        hknight_watk1(g);
        g.hknight_type = 0.0;
    }
}

pub fn monster_hell_knight(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model2("progs/hknight.mdl");
    g.precache_model2("progs/k_spike.mdl");
    g.precache_model2("progs/h_hellkn.mdl");

    g.precache_sound2("hknight/attack1.wav");
    g.precache_sound2("hknight/death1.wav");
    g.precache_sound2("hknight/pain1.wav");
    g.precache_sound2("hknight/sight1.wav");
    g.precache_sound("hknight/hit.wav"); // used by C code, so don't sound2
    g.precache_sound2("hknight/slash1.wav");
    g.precache_sound2("hknight/idle.wav");
    g.precache_sound2("hknight/grunt.wav");

    g.precache_sound("knight/sword1.wav");
    g.precache_sound("knight/sword2.wav");

    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/hknight.mdl");

    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 40.0));
    g.v(s).health = 250.0;

    g.v(s).th_stand = Some(hknight_stand1);
    g.v(s).th_walk = Some(hknight_walk1);
    g.v(s).th_run = Some(hknight_run1);
    g.v(s).th_melee = Some(hknight_melee);
    g.v(s).th_missile = Some(hknight_magicc1);
    g.v(s).th_pain = Some(hknight_pain);
    g.v(s).th_die = Some(hknight_die);

    walkmonster_start(g);
}
