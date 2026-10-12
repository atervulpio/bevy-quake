//! `enforcer.qc`, id's QuakeC 1.06, in Rust.

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
const F_walk1: f32 = 7.0;
const F_walk2: f32 = 8.0;
const F_walk3: f32 = 9.0;
const F_walk4: f32 = 10.0;
const F_walk5: f32 = 11.0;
const F_walk6: f32 = 12.0;
const F_walk7: f32 = 13.0;
const F_walk8: f32 = 14.0;
const F_walk9: f32 = 15.0;
const F_walk10: f32 = 16.0;
const F_walk11: f32 = 17.0;
const F_walk12: f32 = 18.0;
const F_walk13: f32 = 19.0;
const F_walk14: f32 = 20.0;
const F_walk15: f32 = 21.0;
const F_walk16: f32 = 22.0;
const F_run1: f32 = 23.0;
const F_run2: f32 = 24.0;
const F_run3: f32 = 25.0;
const F_run4: f32 = 26.0;
const F_run5: f32 = 27.0;
const F_run6: f32 = 28.0;
const F_run7: f32 = 29.0;
const F_run8: f32 = 30.0;
const F_attack1: f32 = 31.0;
const F_attack2: f32 = 32.0;
const F_attack3: f32 = 33.0;
const F_attack4: f32 = 34.0;
const F_attack5: f32 = 35.0;
const F_attack6: f32 = 36.0;
const F_attack7: f32 = 37.0;
const F_attack8: f32 = 38.0;
const F_attack9: f32 = 39.0;
const F_attack10: f32 = 40.0;
const F_death1: f32 = 41.0;
const F_death2: f32 = 42.0;
const F_death3: f32 = 43.0;
const F_death4: f32 = 44.0;
const F_death5: f32 = 45.0;
const F_death6: f32 = 46.0;
const F_death7: f32 = 47.0;
const F_death8: f32 = 48.0;
const F_death9: f32 = 49.0;
const F_death10: f32 = 50.0;
const F_death11: f32 = 51.0;
const F_death12: f32 = 52.0;
const F_death13: f32 = 53.0;
const F_death14: f32 = 54.0;
const F_fdeath1: f32 = 55.0;
const F_fdeath2: f32 = 56.0;
const F_fdeath3: f32 = 57.0;
const F_fdeath4: f32 = 58.0;
const F_fdeath5: f32 = 59.0;
const F_fdeath6: f32 = 60.0;
const F_fdeath7: f32 = 61.0;
const F_fdeath8: f32 = 62.0;
const F_fdeath9: f32 = 63.0;
const F_fdeath10: f32 = 64.0;
const F_fdeath11: f32 = 65.0;
const F_paina1: f32 = 66.0;
const F_paina2: f32 = 67.0;
const F_paina3: f32 = 68.0;
const F_paina4: f32 = 69.0;
const F_painb1: f32 = 70.0;
const F_painb2: f32 = 71.0;
const F_painb3: f32 = 72.0;
const F_painb4: f32 = 73.0;
const F_painb5: f32 = 74.0;
const F_painc1: f32 = 75.0;
const F_painc2: f32 = 76.0;
const F_painc3: f32 = 77.0;
const F_painc4: f32 = 78.0;
const F_painc5: f32 = 79.0;
const F_painc6: f32 = 80.0;
const F_painc7: f32 = 81.0;
const F_painc8: f32 = 82.0;
const F_paind1: f32 = 83.0;
const F_paind2: f32 = 84.0;
const F_paind3: f32 = 85.0;
const F_paind4: f32 = 86.0;
const F_paind5: f32 = 87.0;
const F_paind6: f32 = 88.0;
const F_paind7: f32 = 89.0;
const F_paind8: f32 = 90.0;
const F_paind9: f32 = 91.0;
const F_paind10: f32 = 92.0;
const F_paind11: f32 = 93.0;
const F_paind12: f32 = 94.0;
const F_paind13: f32 = 95.0;
const F_paind14: f32 = 96.0;
const F_paind15: f32 = 97.0;
const F_paind16: f32 = 98.0;
const F_paind17: f32 = 99.0;
const F_paind18: f32 = 100.0;
const F_paind19: f32 = 101.0;


pub fn Laser_Touch(g: &mut Game) {
    let s = g.self_;
    let other = g.other;
    if other == g.r(s).owner {
        return; // don't explode on owner
    }

    if g.pointcontents(g.r(s).origin) == CONTENT_SKY {
        g.remove(s);
        return;
    }

    g.sound(s, CHAN_WEAPON, "enforcer/enfstop.wav", 1.0, ATTN_STATIC);
    let org = g.r(s).origin - 8.0 * normalize(g.r(s).velocity);

    if g.r(other).health != 0.0 {
        SpawnBlood(g, org, g.r(s).velocity * 0.2, 15.0);
        T_Damage(g, other, s, g.r(s).owner, 15.0);
    } else {
        g.temp_entity(TempEntity::Gunshot(org));
    }

    g.remove(s);
}

// id's code declares `local vector vec;` over the parameter of the same name; id's compiler
// gives back the parameter's own definition for it, so `vec` here is the parameter.
pub fn LaunchLaser(g: &mut Game, org: Vec3, vec: Vec3) {
    let s = g.self_;
    if g.r(s).classname == "monster_enforcer" {
        g.sound(s, CHAN_WEAPON, "enforcer/enfire.wav", 1.0, ATTN_NORM);
    }

    let vec = normalize(vec);

    g.newmis = g.spawn();
    let newmis = g.newmis;
    g.v(newmis).owner = s;
    g.v(newmis).movetype = MOVETYPE_FLY;
    g.v(newmis).solid = SOLID_BBOX;
    g.v(newmis).effects = EF_DIMLIGHT;

    g.setmodel(newmis, "progs/laser.mdl");
    g.setsize(newmis, Vec3::ZERO, Vec3::ZERO);

    g.setorigin(newmis, org);

    g.v(newmis).velocity = vec * 600.0;
    g.v(newmis).angles = vectoangles(g.r(newmis).velocity);

    g.v(newmis).nextthink = g.time + 5.0;
    g.v(newmis).think = Some(SUB_Remove);
    g.v(newmis).touch = Some(Laser_Touch);
}

pub fn enforcer_fire(g: &mut Game) {
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;
    g.makevectors(g.r(s).angles);

    let org = g.r(s).origin + g.v_forward * 30.0 + g.v_right * 8.5 + Vec3::new(0.0, 0.0, 16.0);

    let enemy = g.r(s).enemy;
    LaunchLaser(g, org, g.r(enemy).origin - g.r(s).origin);
}

pub fn enf_stand1(g: &mut Game) {
    g.frame_state(F_stand1, enf_stand2);
    ai_stand(g);
}

pub fn enf_stand2(g: &mut Game) {
    g.frame_state(F_stand2, enf_stand3);
    ai_stand(g);
}

pub fn enf_stand3(g: &mut Game) {
    g.frame_state(F_stand3, enf_stand4);
    ai_stand(g);
}

pub fn enf_stand4(g: &mut Game) {
    g.frame_state(F_stand4, enf_stand5);
    ai_stand(g);
}

pub fn enf_stand5(g: &mut Game) {
    g.frame_state(F_stand5, enf_stand6);
    ai_stand(g);
}

pub fn enf_stand6(g: &mut Game) {
    g.frame_state(F_stand6, enf_stand7);
    ai_stand(g);
}

pub fn enf_stand7(g: &mut Game) {
    g.frame_state(F_stand7, enf_stand1);
    ai_stand(g);
}

pub fn enf_walk1(g: &mut Game) {
    g.frame_state(F_walk1, enf_walk2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "enforcer/idle1.wav", 1.0, ATTN_IDLE);
    }
    ai_walk(g, 2.0);
}

pub fn enf_walk2(g: &mut Game) {
    g.frame_state(F_walk2, enf_walk3);
    ai_walk(g, 4.0);
}

pub fn enf_walk3(g: &mut Game) {
    g.frame_state(F_walk3, enf_walk4);
    ai_walk(g, 4.0);
}

pub fn enf_walk4(g: &mut Game) {
    g.frame_state(F_walk4, enf_walk5);
    ai_walk(g, 3.0);
}

pub fn enf_walk5(g: &mut Game) {
    g.frame_state(F_walk5, enf_walk6);
    ai_walk(g, 1.0);
}

pub fn enf_walk6(g: &mut Game) {
    g.frame_state(F_walk6, enf_walk7);
    ai_walk(g, 2.0);
}

pub fn enf_walk7(g: &mut Game) {
    g.frame_state(F_walk7, enf_walk8);
    ai_walk(g, 2.0);
}

pub fn enf_walk8(g: &mut Game) {
    g.frame_state(F_walk8, enf_walk9);
    ai_walk(g, 1.0);
}

pub fn enf_walk9(g: &mut Game) {
    g.frame_state(F_walk9, enf_walk10);
    ai_walk(g, 2.0);
}

pub fn enf_walk10(g: &mut Game) {
    g.frame_state(F_walk10, enf_walk11);
    ai_walk(g, 4.0);
}

pub fn enf_walk11(g: &mut Game) {
    g.frame_state(F_walk11, enf_walk12);
    ai_walk(g, 4.0);
}

pub fn enf_walk12(g: &mut Game) {
    g.frame_state(F_walk12, enf_walk13);
    ai_walk(g, 1.0);
}

pub fn enf_walk13(g: &mut Game) {
    g.frame_state(F_walk13, enf_walk14);
    ai_walk(g, 2.0);
}

pub fn enf_walk14(g: &mut Game) {
    g.frame_state(F_walk14, enf_walk15);
    ai_walk(g, 3.0);
}

pub fn enf_walk15(g: &mut Game) {
    g.frame_state(F_walk15, enf_walk16);
    ai_walk(g, 4.0);
}

pub fn enf_walk16(g: &mut Game) {
    g.frame_state(F_walk16, enf_walk1);
    ai_walk(g, 2.0);
}

pub fn enf_run1(g: &mut Game) {
    g.frame_state(F_run1, enf_run2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "enforcer/idle1.wav", 1.0, ATTN_IDLE);
    }
    ai_run(g, 18.0);
}

pub fn enf_run2(g: &mut Game) {
    g.frame_state(F_run2, enf_run3);
    ai_run(g, 14.0);
}

pub fn enf_run3(g: &mut Game) {
    g.frame_state(F_run3, enf_run4);
    ai_run(g, 7.0);
}

pub fn enf_run4(g: &mut Game) {
    g.frame_state(F_run4, enf_run5);
    ai_run(g, 12.0);
}

pub fn enf_run5(g: &mut Game) {
    g.frame_state(F_run5, enf_run6);
    ai_run(g, 14.0);
}

pub fn enf_run6(g: &mut Game) {
    g.frame_state(F_run6, enf_run7);
    ai_run(g, 14.0);
}

pub fn enf_run7(g: &mut Game) {
    g.frame_state(F_run7, enf_run8);
    ai_run(g, 7.0);
}

pub fn enf_run8(g: &mut Game) {
    g.frame_state(F_run8, enf_run1);
    ai_run(g, 11.0);
}

pub fn enf_atk1(g: &mut Game) {
    g.frame_state(F_attack1, enf_atk2);
    ai_face(g);
}

pub fn enf_atk2(g: &mut Game) {
    g.frame_state(F_attack2, enf_atk3);
    ai_face(g);
}

pub fn enf_atk3(g: &mut Game) {
    g.frame_state(F_attack3, enf_atk4);
    ai_face(g);
}

pub fn enf_atk4(g: &mut Game) {
    g.frame_state(F_attack4, enf_atk5);
    ai_face(g);
}

pub fn enf_atk5(g: &mut Game) {
    g.frame_state(F_attack5, enf_atk6);
    ai_face(g);
}

pub fn enf_atk6(g: &mut Game) {
    g.frame_state(F_attack6, enf_atk7);
    enforcer_fire(g);
}

pub fn enf_atk7(g: &mut Game) {
    g.frame_state(F_attack7, enf_atk8);
    ai_face(g);
}

pub fn enf_atk8(g: &mut Game) {
    g.frame_state(F_attack8, enf_atk9);
    ai_face(g);
}

pub fn enf_atk9(g: &mut Game) {
    g.frame_state(F_attack5, enf_atk10);
    ai_face(g);
}

pub fn enf_atk10(g: &mut Game) {
    g.frame_state(F_attack6, enf_atk11);
    enforcer_fire(g);
}

pub fn enf_atk11(g: &mut Game) {
    g.frame_state(F_attack7, enf_atk12);
    ai_face(g);
}

pub fn enf_atk12(g: &mut Game) {
    g.frame_state(F_attack8, enf_atk13);
    ai_face(g);
}

pub fn enf_atk13(g: &mut Game) {
    g.frame_state(F_attack9, enf_atk14);
    ai_face(g);
}

pub fn enf_atk14(g: &mut Game) {
    g.frame_state(F_attack10, enf_run1);
    ai_face(g);
    SUB_CheckRefire(g, enf_atk1);
}

pub fn enf_paina1(g: &mut Game) {
    g.frame_state(F_paina1, enf_paina2);
}

pub fn enf_paina2(g: &mut Game) {
    g.frame_state(F_paina2, enf_paina3);
}

pub fn enf_paina3(g: &mut Game) {
    g.frame_state(F_paina3, enf_paina4);
}

pub fn enf_paina4(g: &mut Game) {
    g.frame_state(F_paina4, enf_run1);
}

pub fn enf_painb1(g: &mut Game) {
    g.frame_state(F_painb1, enf_painb2);
}

pub fn enf_painb2(g: &mut Game) {
    g.frame_state(F_painb2, enf_painb3);
}

pub fn enf_painb3(g: &mut Game) {
    g.frame_state(F_painb3, enf_painb4);
}

pub fn enf_painb4(g: &mut Game) {
    g.frame_state(F_painb4, enf_painb5);
}

pub fn enf_painb5(g: &mut Game) {
    g.frame_state(F_painb5, enf_run1);
}

pub fn enf_painc1(g: &mut Game) {
    g.frame_state(F_painc1, enf_painc2);
}

pub fn enf_painc2(g: &mut Game) {
    g.frame_state(F_painc2, enf_painc3);
}

pub fn enf_painc3(g: &mut Game) {
    g.frame_state(F_painc3, enf_painc4);
}

pub fn enf_painc4(g: &mut Game) {
    g.frame_state(F_painc4, enf_painc5);
}

pub fn enf_painc5(g: &mut Game) {
    g.frame_state(F_painc5, enf_painc6);
}

pub fn enf_painc6(g: &mut Game) {
    g.frame_state(F_painc6, enf_painc7);
}

pub fn enf_painc7(g: &mut Game) {
    g.frame_state(F_painc7, enf_painc8);
}

pub fn enf_painc8(g: &mut Game) {
    g.frame_state(F_painc8, enf_run1);
}

pub fn enf_paind1(g: &mut Game) {
    g.frame_state(F_paind1, enf_paind2);
}

pub fn enf_paind2(g: &mut Game) {
    g.frame_state(F_paind2, enf_paind3);
}

pub fn enf_paind3(g: &mut Game) {
    g.frame_state(F_paind3, enf_paind4);
}

pub fn enf_paind4(g: &mut Game) {
    g.frame_state(F_paind4, enf_paind5);
    ai_painforward(g, 2.0);
}

pub fn enf_paind5(g: &mut Game) {
    g.frame_state(F_paind5, enf_paind6);
    ai_painforward(g, 1.0);
}

pub fn enf_paind6(g: &mut Game) {
    g.frame_state(F_paind6, enf_paind7);
}

pub fn enf_paind7(g: &mut Game) {
    g.frame_state(F_paind7, enf_paind8);
}

pub fn enf_paind8(g: &mut Game) {
    g.frame_state(F_paind8, enf_paind9);
}

pub fn enf_paind9(g: &mut Game) {
    g.frame_state(F_paind9, enf_paind10);
}

pub fn enf_paind10(g: &mut Game) {
    g.frame_state(F_paind10, enf_paind11);
}

pub fn enf_paind11(g: &mut Game) {
    g.frame_state(F_paind11, enf_paind12);
    ai_painforward(g, 1.0);
}

pub fn enf_paind12(g: &mut Game) {
    g.frame_state(F_paind12, enf_paind13);
    ai_painforward(g, 1.0);
}

pub fn enf_paind13(g: &mut Game) {
    g.frame_state(F_paind13, enf_paind14);
    ai_painforward(g, 1.0);
}

pub fn enf_paind14(g: &mut Game) {
    g.frame_state(F_paind14, enf_paind15);
}

pub fn enf_paind15(g: &mut Game) {
    g.frame_state(F_paind15, enf_paind16);
}

pub fn enf_paind16(g: &mut Game) {
    g.frame_state(F_paind16, enf_paind17);
    ai_pain(g, 1.0);
}

pub fn enf_paind17(g: &mut Game) {
    g.frame_state(F_paind17, enf_paind18);
    ai_pain(g, 1.0);
}

pub fn enf_paind18(g: &mut Game) {
    g.frame_state(F_paind18, enf_paind19);
}

pub fn enf_paind19(g: &mut Game) {
    g.frame_state(F_paind19, enf_run1);
}

pub fn enf_pain(g: &mut Game, attacker: Ent, damage: f32) {
    let s = g.self_;
    let r = g.random();
    if g.r(s).pain_finished > g.time {
        return;
    }

    if r < 0.5 {
        g.sound(s, CHAN_VOICE, "enforcer/pain1.wav", 1.0, ATTN_NORM);
    } else {
        g.sound(s, CHAN_VOICE, "enforcer/pain2.wav", 1.0, ATTN_NORM);
    }

    if r < 0.2 {
        g.v(s).pain_finished = g.time + 1.0;
        enf_paina1(g);
    } else if r < 0.4 {
        g.v(s).pain_finished = g.time + 1.0;
        enf_painb1(g);
    } else if r < 0.7 {
        g.v(s).pain_finished = g.time + 1.0;
        enf_painc1(g);
    } else {
        g.v(s).pain_finished = g.time + 2.0;
        enf_paind1(g);
    }
}

pub fn enf_die1(g: &mut Game) {
    g.frame_state(F_death1, enf_die2);
}

pub fn enf_die2(g: &mut Game) {
    g.frame_state(F_death2, enf_die3);
}

pub fn enf_die3(g: &mut Game) {
    g.frame_state(F_death3, enf_die4);
    let s = g.self_;
    g.v(s).solid = SOLID_NOT;
    g.v(s).ammo_cells = 5.0;
    DropBackpack(g);
}

pub fn enf_die4(g: &mut Game) {
    g.frame_state(F_death4, enf_die5);
    ai_forward(g, 14.0);
}

pub fn enf_die5(g: &mut Game) {
    g.frame_state(F_death5, enf_die6);
    ai_forward(g, 2.0);
}

pub fn enf_die6(g: &mut Game) {
    g.frame_state(F_death6, enf_die7);
}

pub fn enf_die7(g: &mut Game) {
    g.frame_state(F_death7, enf_die8);
}

pub fn enf_die8(g: &mut Game) {
    g.frame_state(F_death8, enf_die9);
}

pub fn enf_die9(g: &mut Game) {
    g.frame_state(F_death9, enf_die10);
    ai_forward(g, 3.0);
}

pub fn enf_die10(g: &mut Game) {
    g.frame_state(F_death10, enf_die11);
    ai_forward(g, 5.0);
}

pub fn enf_die11(g: &mut Game) {
    g.frame_state(F_death11, enf_die12);
    ai_forward(g, 5.0);
}

pub fn enf_die12(g: &mut Game) {
    g.frame_state(F_death12, enf_die13);
    ai_forward(g, 5.0);
}

pub fn enf_die13(g: &mut Game) {
    g.frame_state(F_death13, enf_die14);
}

pub fn enf_die14(g: &mut Game) {
    g.frame_state(F_death14, enf_die14);
}

pub fn enf_fdie1(g: &mut Game) {
    g.frame_state(F_fdeath1, enf_fdie2);
}

pub fn enf_fdie2(g: &mut Game) {
    g.frame_state(F_fdeath2, enf_fdie3);
}

pub fn enf_fdie3(g: &mut Game) {
    g.frame_state(F_fdeath3, enf_fdie4);
    let s = g.self_;
    g.v(s).solid = SOLID_NOT;
    g.v(s).ammo_cells = 5.0;
    DropBackpack(g);
}

pub fn enf_fdie4(g: &mut Game) {
    g.frame_state(F_fdeath4, enf_fdie5);
}

pub fn enf_fdie5(g: &mut Game) {
    g.frame_state(F_fdeath5, enf_fdie6);
}

pub fn enf_fdie6(g: &mut Game) {
    g.frame_state(F_fdeath6, enf_fdie7);
}

pub fn enf_fdie7(g: &mut Game) {
    g.frame_state(F_fdeath7, enf_fdie8);
}

pub fn enf_fdie8(g: &mut Game) {
    g.frame_state(F_fdeath8, enf_fdie9);
}

pub fn enf_fdie9(g: &mut Game) {
    g.frame_state(F_fdeath9, enf_fdie10);
}

pub fn enf_fdie10(g: &mut Game) {
    g.frame_state(F_fdeath10, enf_fdie11);
}

pub fn enf_fdie11(g: &mut Game) {
    g.frame_state(F_fdeath11, enf_fdie11);
}

pub fn enf_die(g: &mut Game) {
    let s = g.self_;
    // check for gib
    if g.r(s).health < -35.0 {
        g.sound(s, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        ThrowHead(g, "progs/h_mega.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib1.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib2.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib3.mdl", g.r(s).health);
        return;
    }

    // regular death
    g.sound(s, CHAN_VOICE, "enforcer/death1.wav", 1.0, ATTN_NORM);

    if g.random() > 0.5 {
        enf_die1(g);
    } else {
        enf_fdie1(g);
    }
}

pub fn monster_enforcer(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model2("progs/enforcer.mdl");
    g.precache_model2("progs/h_mega.mdl");
    g.precache_model2("progs/laser.mdl");

    g.precache_sound2("enforcer/death1.wav");
    g.precache_sound2("enforcer/enfire.wav");
    g.precache_sound2("enforcer/enfstop.wav");
    g.precache_sound2("enforcer/idle1.wav");
    g.precache_sound2("enforcer/pain1.wav");
    g.precache_sound2("enforcer/pain2.wav");
    g.precache_sound2("enforcer/sight1.wav");
    g.precache_sound2("enforcer/sight2.wav");
    g.precache_sound2("enforcer/sight3.wav");
    g.precache_sound2("enforcer/sight4.wav");

    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/enforcer.mdl");

    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 40.0));
    g.v(s).health = 80.0;

    g.v(s).th_stand = Some(enf_stand1);
    g.v(s).th_walk = Some(enf_walk1);
    g.v(s).th_run = Some(enf_run1);
    g.v(s).th_pain = Some(enf_pain);
    g.v(s).th_die = Some(enf_die);
    g.v(s).th_missile = Some(enf_atk1);

    walkmonster_start(g);
}
