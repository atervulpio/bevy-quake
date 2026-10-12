//! `tarbaby.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;


// The model's frames, numbered as `$frame` numbers them: from nought, in order, in this file.
const F_walk1: f32 = 0.0;
const F_walk2: f32 = 1.0;
const F_walk3: f32 = 2.0;
const F_walk4: f32 = 3.0;
const F_walk5: f32 = 4.0;
const F_walk6: f32 = 5.0;
const F_walk7: f32 = 6.0;
const F_walk8: f32 = 7.0;
const F_walk9: f32 = 8.0;
const F_walk10: f32 = 9.0;
const F_walk11: f32 = 10.0;
const F_walk12: f32 = 11.0;
const F_walk13: f32 = 12.0;
const F_walk14: f32 = 13.0;
const F_walk15: f32 = 14.0;
const F_walk16: f32 = 15.0;
const F_walk17: f32 = 16.0;
const F_walk18: f32 = 17.0;
const F_walk19: f32 = 18.0;
const F_walk20: f32 = 19.0;
const F_walk21: f32 = 20.0;
const F_walk22: f32 = 21.0;
const F_walk23: f32 = 22.0;
const F_walk24: f32 = 23.0;
const F_walk25: f32 = 24.0;
const F_run1: f32 = 25.0;
const F_run2: f32 = 26.0;
const F_run3: f32 = 27.0;
const F_run4: f32 = 28.0;
const F_run5: f32 = 29.0;
const F_run6: f32 = 30.0;
const F_run7: f32 = 31.0;
const F_run8: f32 = 32.0;
const F_run9: f32 = 33.0;
const F_run10: f32 = 34.0;
const F_run11: f32 = 35.0;
const F_run12: f32 = 36.0;
const F_run13: f32 = 37.0;
const F_run14: f32 = 38.0;
const F_run15: f32 = 39.0;
const F_run16: f32 = 40.0;
const F_run17: f32 = 41.0;
const F_run18: f32 = 42.0;
const F_run19: f32 = 43.0;
const F_run20: f32 = 44.0;
const F_run21: f32 = 45.0;
const F_run22: f32 = 46.0;
const F_run23: f32 = 47.0;
const F_run24: f32 = 48.0;
const F_run25: f32 = 49.0;
const F_jump1: f32 = 50.0;
const F_jump2: f32 = 51.0;
const F_jump3: f32 = 52.0;
const F_jump4: f32 = 53.0;
const F_jump5: f32 = 54.0;
const F_jump6: f32 = 55.0;
const F_fly1: f32 = 56.0;
const F_fly2: f32 = 57.0;
const F_fly3: f32 = 58.0;
const F_fly4: f32 = 59.0;
const F_exp: f32 = 60.0;


pub fn tbaby_stand1(g: &mut Game) {
    g.frame_state(F_walk1, tbaby_stand1);
    ai_stand(g);
}

pub fn tbaby_hang1(g: &mut Game) {
    g.frame_state(F_walk1, tbaby_hang1);
    ai_stand(g);
}

pub fn tbaby_walk1(g: &mut Game) {
    g.frame_state(F_walk1, tbaby_walk2);
    ai_turn(g);
}

pub fn tbaby_walk2(g: &mut Game) {
    g.frame_state(F_walk2, tbaby_walk3);
    ai_turn(g);
}

pub fn tbaby_walk3(g: &mut Game) {
    g.frame_state(F_walk3, tbaby_walk4);
    ai_turn(g);
}

pub fn tbaby_walk4(g: &mut Game) {
    g.frame_state(F_walk4, tbaby_walk5);
    ai_turn(g);
}

pub fn tbaby_walk5(g: &mut Game) {
    g.frame_state(F_walk5, tbaby_walk6);
    ai_turn(g);
}

pub fn tbaby_walk6(g: &mut Game) {
    g.frame_state(F_walk6, tbaby_walk7);
    ai_turn(g);
}

pub fn tbaby_walk7(g: &mut Game) {
    g.frame_state(F_walk7, tbaby_walk8);
    ai_turn(g);
}

pub fn tbaby_walk8(g: &mut Game) {
    g.frame_state(F_walk8, tbaby_walk9);
    ai_turn(g);
}

pub fn tbaby_walk9(g: &mut Game) {
    g.frame_state(F_walk9, tbaby_walk10);
    ai_turn(g);
}

pub fn tbaby_walk10(g: &mut Game) {
    g.frame_state(F_walk10, tbaby_walk11);
    ai_turn(g);
}

pub fn tbaby_walk11(g: &mut Game) {
    g.frame_state(F_walk11, tbaby_walk12);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk12(g: &mut Game) {
    g.frame_state(F_walk12, tbaby_walk13);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk13(g: &mut Game) {
    g.frame_state(F_walk13, tbaby_walk14);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk14(g: &mut Game) {
    g.frame_state(F_walk14, tbaby_walk15);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk15(g: &mut Game) {
    g.frame_state(F_walk15, tbaby_walk16);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk16(g: &mut Game) {
    g.frame_state(F_walk16, tbaby_walk17);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk17(g: &mut Game) {
    g.frame_state(F_walk17, tbaby_walk18);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk18(g: &mut Game) {
    g.frame_state(F_walk18, tbaby_walk19);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk19(g: &mut Game) {
    g.frame_state(F_walk19, tbaby_walk20);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk20(g: &mut Game) {
    g.frame_state(F_walk20, tbaby_walk21);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk21(g: &mut Game) {
    g.frame_state(F_walk21, tbaby_walk22);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk22(g: &mut Game) {
    g.frame_state(F_walk22, tbaby_walk23);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk23(g: &mut Game) {
    g.frame_state(F_walk23, tbaby_walk24);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk24(g: &mut Game) {
    g.frame_state(F_walk24, tbaby_walk25);
    ai_walk(g, 2.0);
}

pub fn tbaby_walk25(g: &mut Game) {
    g.frame_state(F_walk25, tbaby_walk1);
    ai_walk(g, 2.0);
}

pub fn tbaby_run1(g: &mut Game) {
    g.frame_state(F_run1, tbaby_run2);
    ai_face(g);
}

pub fn tbaby_run2(g: &mut Game) {
    g.frame_state(F_run2, tbaby_run3);
    ai_face(g);
}

pub fn tbaby_run3(g: &mut Game) {
    g.frame_state(F_run3, tbaby_run4);
    ai_face(g);
}

pub fn tbaby_run4(g: &mut Game) {
    g.frame_state(F_run4, tbaby_run5);
    ai_face(g);
}

pub fn tbaby_run5(g: &mut Game) {
    g.frame_state(F_run5, tbaby_run6);
    ai_face(g);
}

pub fn tbaby_run6(g: &mut Game) {
    g.frame_state(F_run6, tbaby_run7);
    ai_face(g);
}

pub fn tbaby_run7(g: &mut Game) {
    g.frame_state(F_run7, tbaby_run8);
    ai_face(g);
}

pub fn tbaby_run8(g: &mut Game) {
    g.frame_state(F_run8, tbaby_run9);
    ai_face(g);
}

pub fn tbaby_run9(g: &mut Game) {
    g.frame_state(F_run9, tbaby_run10);
    ai_face(g);
}

pub fn tbaby_run10(g: &mut Game) {
    g.frame_state(F_run10, tbaby_run11);
    ai_face(g);
}

pub fn tbaby_run11(g: &mut Game) {
    g.frame_state(F_run11, tbaby_run12);
    ai_run(g, 2.0);
}

pub fn tbaby_run12(g: &mut Game) {
    g.frame_state(F_run12, tbaby_run13);
    ai_run(g, 2.0);
}

pub fn tbaby_run13(g: &mut Game) {
    g.frame_state(F_run13, tbaby_run14);
    ai_run(g, 2.0);
}

pub fn tbaby_run14(g: &mut Game) {
    g.frame_state(F_run14, tbaby_run15);
    ai_run(g, 2.0);
}

pub fn tbaby_run15(g: &mut Game) {
    g.frame_state(F_run15, tbaby_run16);
    ai_run(g, 2.0);
}

pub fn tbaby_run16(g: &mut Game) {
    g.frame_state(F_run16, tbaby_run17);
    ai_run(g, 2.0);
}

pub fn tbaby_run17(g: &mut Game) {
    g.frame_state(F_run17, tbaby_run18);
    ai_run(g, 2.0);
}

pub fn tbaby_run18(g: &mut Game) {
    g.frame_state(F_run18, tbaby_run19);
    ai_run(g, 2.0);
}

pub fn tbaby_run19(g: &mut Game) {
    g.frame_state(F_run19, tbaby_run20);
    ai_run(g, 2.0);
}

pub fn tbaby_run20(g: &mut Game) {
    g.frame_state(F_run20, tbaby_run21);
    ai_run(g, 2.0);
}

pub fn tbaby_run21(g: &mut Game) {
    g.frame_state(F_run21, tbaby_run22);
    ai_run(g, 2.0);
}

pub fn tbaby_run22(g: &mut Game) {
    g.frame_state(F_run22, tbaby_run23);
    ai_run(g, 2.0);
}

pub fn tbaby_run23(g: &mut Game) {
    g.frame_state(F_run23, tbaby_run24);
    ai_run(g, 2.0);
}

pub fn tbaby_run24(g: &mut Game) {
    g.frame_state(F_run24, tbaby_run25);
    ai_run(g, 2.0);
}

pub fn tbaby_run25(g: &mut Game) {
    g.frame_state(F_run25, tbaby_run1);
    ai_run(g, 2.0);
}

pub fn Tar_JumpTouch(g: &mut Game) {
    let s = g.self_;
    let other = g.other;
    if g.r(other).takedamage != 0 && g.r(other).classname != g.r(s).classname {
        if vlen(g.r(s).velocity) > 400.0 {
            let ldmg = 10.0 + 10.0 * g.random();
            T_Damage(g, other, s, s, ldmg);
            g.sound(s, CHAN_WEAPON, "blob/hit1.wav", 1.0, ATTN_NORM);
        }
    } else {
        g.sound(s, CHAN_WEAPON, "blob/land1.wav", 1.0, ATTN_NORM);
    }

    let s = g.self_;
    if !g.checkbottom(s) {
        if (g.r(s).flags & FL_ONGROUND) != 0 {
            // jump randomly to not get hung up
            g.v(s).touch = Some(SUB_Null);
            g.v(s).think = Some(tbaby_run1);
            g.v(s).movetype = MOVETYPE_STEP;
            g.v(s).nextthink = g.time + 0.1;
        }
        return; // not on ground yet
    }

    g.v(s).touch = Some(SUB_Null);
    g.v(s).think = Some(tbaby_jump1);
    g.v(s).nextthink = g.time + 0.1;
}

pub fn tbaby_fly1(g: &mut Game) {
    g.frame_state(F_fly1, tbaby_fly2);
}

pub fn tbaby_fly2(g: &mut Game) {
    g.frame_state(F_fly2, tbaby_fly3);
}

pub fn tbaby_fly3(g: &mut Game) {
    g.frame_state(F_fly3, tbaby_fly4);
}

pub fn tbaby_fly4(g: &mut Game) {
    g.frame_state(F_fly4, tbaby_fly1);
    let s = g.self_;
    g.v(s).cnt = g.r(s).cnt + 1.0;
    if g.r(s).cnt == 4.0 {
        tbaby_jump5(g);
    }
}

pub fn tbaby_jump1(g: &mut Game) {
    g.frame_state(F_jump1, tbaby_jump2);
    ai_face(g);
}

pub fn tbaby_jump2(g: &mut Game) {
    g.frame_state(F_jump2, tbaby_jump3);
    ai_face(g);
}

pub fn tbaby_jump3(g: &mut Game) {
    g.frame_state(F_jump3, tbaby_jump4);
    ai_face(g);
}

pub fn tbaby_jump4(g: &mut Game) {
    g.frame_state(F_jump4, tbaby_jump5);
    ai_face(g);
}

pub fn tbaby_jump5(g: &mut Game) {
    g.frame_state(F_jump5, tbaby_jump6);
    let s = g.self_;
    g.v(s).movetype = MOVETYPE_BOUNCE;
    g.v(s).touch = Some(Tar_JumpTouch);
    g.makevectors(g.r(s).angles);
    g.v(s).origin.z = g.r(s).origin.z + 1.0;
    g.v(s).velocity = g.v_forward * 600.0 + Vec3::new(0.0, 0.0, 200.0);
    g.v(s).velocity.z = g.r(s).velocity.z + g.random() * 150.0;
    if (g.r(s).flags & FL_ONGROUND) != 0 {
        g.v(s).flags = g.r(s).flags - FL_ONGROUND;
    }
    g.v(s).cnt = 0.0;
}

pub fn tbaby_jump6(g: &mut Game) {
    g.frame_state(F_jump6, tbaby_fly1);
}

pub fn tbaby_die1(g: &mut Game) {
    g.frame_state(F_exp, tbaby_die2);
    g.v(g.self_).takedamage = DAMAGE_NO;
}

pub fn tbaby_die2(g: &mut Game) {
    g.frame_state(F_exp, tbaby_run1);
    let s = g.self_;
    T_RadiusDamage(g, s, s, 120.0, world);

    g.sound(s, CHAN_VOICE, "blob/death1.wav", 1.0, ATTN_NORM);
    g.v(s).origin = g.r(s).origin - 8.0 * normalize(g.r(s).velocity);

    g.temp_entity(TempEntity::TarExplosion(g.r(s).origin));

    BecomeExplosion(g);
}

pub fn monster_tarbaby(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model2("progs/tarbaby.mdl");

    g.precache_sound2("blob/death1.wav");
    g.precache_sound2("blob/hit1.wav");
    g.precache_sound2("blob/land1.wav");
    g.precache_sound2("blob/sight1.wav");

    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/tarbaby.mdl");

    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 40.0));
    g.v(s).health = 80.0;

    g.v(s).th_stand = Some(tbaby_stand1);
    g.v(s).th_walk = Some(tbaby_walk1);
    g.v(s).th_run = Some(tbaby_run1);
    g.v(s).th_missile = Some(tbaby_jump1);
    g.v(s).th_melee = Some(tbaby_jump1);
    g.v(s).th_die = Some(tbaby_die1);

    walkmonster_start(g);
}
