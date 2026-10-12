//! `zombie.qc`, id's QuakeC 1.06, in Rust.

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
const F_stand10: f32 = 9.0;
const F_stand11: f32 = 10.0;
const F_stand12: f32 = 11.0;
const F_stand13: f32 = 12.0;
const F_stand14: f32 = 13.0;
const F_stand15: f32 = 14.0;
const F_walk1: f32 = 15.0;
const F_walk2: f32 = 16.0;
const F_walk3: f32 = 17.0;
const F_walk4: f32 = 18.0;
const F_walk5: f32 = 19.0;
const F_walk6: f32 = 20.0;
const F_walk7: f32 = 21.0;
const F_walk8: f32 = 22.0;
const F_walk9: f32 = 23.0;
const F_walk10: f32 = 24.0;
const F_walk11: f32 = 25.0;
const F_walk12: f32 = 26.0;
const F_walk13: f32 = 27.0;
const F_walk14: f32 = 28.0;
const F_walk15: f32 = 29.0;
const F_walk16: f32 = 30.0;
const F_walk17: f32 = 31.0;
const F_walk18: f32 = 32.0;
const F_walk19: f32 = 33.0;
const F_run1: f32 = 34.0;
const F_run2: f32 = 35.0;
const F_run3: f32 = 36.0;
const F_run4: f32 = 37.0;
const F_run5: f32 = 38.0;
const F_run6: f32 = 39.0;
const F_run7: f32 = 40.0;
const F_run8: f32 = 41.0;
const F_run9: f32 = 42.0;
const F_run10: f32 = 43.0;
const F_run11: f32 = 44.0;
const F_run12: f32 = 45.0;
const F_run13: f32 = 46.0;
const F_run14: f32 = 47.0;
const F_run15: f32 = 48.0;
const F_run16: f32 = 49.0;
const F_run17: f32 = 50.0;
const F_run18: f32 = 51.0;
const F_atta1: f32 = 52.0;
const F_atta2: f32 = 53.0;
const F_atta3: f32 = 54.0;
const F_atta4: f32 = 55.0;
const F_atta5: f32 = 56.0;
const F_atta6: f32 = 57.0;
const F_atta7: f32 = 58.0;
const F_atta8: f32 = 59.0;
const F_atta9: f32 = 60.0;
const F_atta10: f32 = 61.0;
const F_atta11: f32 = 62.0;
const F_atta12: f32 = 63.0;
const F_atta13: f32 = 64.0;
const F_attb1: f32 = 65.0;
const F_attb2: f32 = 66.0;
const F_attb3: f32 = 67.0;
const F_attb4: f32 = 68.0;
const F_attb5: f32 = 69.0;
const F_attb6: f32 = 70.0;
const F_attb7: f32 = 71.0;
const F_attb8: f32 = 72.0;
const F_attb9: f32 = 73.0;
const F_attb10: f32 = 74.0;
const F_attb11: f32 = 75.0;
const F_attb12: f32 = 76.0;
const F_attb13: f32 = 77.0;
const F_attc1: f32 = 79.0;
const F_attc2: f32 = 80.0;
const F_attc3: f32 = 81.0;
const F_attc4: f32 = 82.0;
const F_attc5: f32 = 83.0;
const F_attc6: f32 = 84.0;
const F_attc7: f32 = 85.0;
const F_attc8: f32 = 86.0;
const F_attc9: f32 = 87.0;
const F_attc10: f32 = 88.0;
const F_attc11: f32 = 89.0;
const F_attc12: f32 = 90.0;
const F_paina1: f32 = 91.0;
const F_paina2: f32 = 92.0;
const F_paina3: f32 = 93.0;
const F_paina4: f32 = 94.0;
const F_paina5: f32 = 95.0;
const F_paina6: f32 = 96.0;
const F_paina7: f32 = 97.0;
const F_paina8: f32 = 98.0;
const F_paina9: f32 = 99.0;
const F_paina10: f32 = 100.0;
const F_paina11: f32 = 101.0;
const F_paina12: f32 = 102.0;
const F_painb1: f32 = 103.0;
const F_painb2: f32 = 104.0;
const F_painb3: f32 = 105.0;
const F_painb4: f32 = 106.0;
const F_painb5: f32 = 107.0;
const F_painb6: f32 = 108.0;
const F_painb7: f32 = 109.0;
const F_painb8: f32 = 110.0;
const F_painb9: f32 = 111.0;
const F_painb10: f32 = 112.0;
const F_painb11: f32 = 113.0;
const F_painb12: f32 = 114.0;
const F_painb13: f32 = 115.0;
const F_painb14: f32 = 116.0;
const F_painb15: f32 = 117.0;
const F_painb16: f32 = 118.0;
const F_painb17: f32 = 119.0;
const F_painb18: f32 = 120.0;
const F_painb19: f32 = 121.0;
const F_painb20: f32 = 122.0;
const F_painb21: f32 = 123.0;
const F_painb22: f32 = 124.0;
const F_painb23: f32 = 125.0;
const F_painb24: f32 = 126.0;
const F_painb25: f32 = 127.0;
const F_painb26: f32 = 128.0;
const F_painb27: f32 = 129.0;
const F_painb28: f32 = 130.0;
const F_painc1: f32 = 131.0;
const F_painc2: f32 = 132.0;
const F_painc3: f32 = 133.0;
const F_painc4: f32 = 134.0;
const F_painc5: f32 = 135.0;
const F_painc6: f32 = 136.0;
const F_painc7: f32 = 137.0;
const F_painc8: f32 = 138.0;
const F_painc9: f32 = 139.0;
const F_painc10: f32 = 140.0;
const F_painc11: f32 = 141.0;
const F_painc12: f32 = 142.0;
const F_painc13: f32 = 143.0;
const F_painc14: f32 = 144.0;
const F_painc15: f32 = 145.0;
const F_painc16: f32 = 146.0;
const F_painc17: f32 = 147.0;
const F_painc18: f32 = 148.0;
const F_paind1: f32 = 149.0;
const F_paind2: f32 = 150.0;
const F_paind3: f32 = 151.0;
const F_paind4: f32 = 152.0;
const F_paind5: f32 = 153.0;
const F_paind6: f32 = 154.0;
const F_paind7: f32 = 155.0;
const F_paind8: f32 = 156.0;
const F_paind9: f32 = 157.0;
const F_paind10: f32 = 158.0;
const F_paind11: f32 = 159.0;
const F_paind12: f32 = 160.0;
const F_paind13: f32 = 161.0;
const F_paine1: f32 = 162.0;
const F_paine2: f32 = 163.0;
const F_paine3: f32 = 164.0;
const F_paine4: f32 = 165.0;
const F_paine5: f32 = 166.0;
const F_paine6: f32 = 167.0;
const F_paine7: f32 = 168.0;
const F_paine8: f32 = 169.0;
const F_paine9: f32 = 170.0;
const F_paine10: f32 = 171.0;
const F_paine11: f32 = 172.0;
const F_paine12: f32 = 173.0;
const F_paine13: f32 = 174.0;
const F_paine14: f32 = 175.0;
const F_paine15: f32 = 176.0;
const F_paine16: f32 = 177.0;
const F_paine17: f32 = 178.0;
const F_paine18: f32 = 179.0;
const F_paine19: f32 = 180.0;
const F_paine20: f32 = 181.0;
const F_paine21: f32 = 182.0;
const F_paine22: f32 = 183.0;
const F_paine23: f32 = 184.0;
const F_paine24: f32 = 185.0;
const F_paine25: f32 = 186.0;
const F_paine26: f32 = 187.0;
const F_paine27: f32 = 188.0;
const F_paine28: f32 = 189.0;
const F_paine29: f32 = 190.0;
const F_paine30: f32 = 191.0;
const F_cruc_1: f32 = 192.0;
const F_cruc_2: f32 = 193.0;
const F_cruc_3: f32 = 194.0;
const F_cruc_4: f32 = 195.0;
const F_cruc_5: f32 = 196.0;
const F_cruc_6: f32 = 197.0;

/// `float SPAWN_CRUCIFIED = 1;`: a spawn flag, so an integer here as `spawnflags` is.
const SPAWN_CRUCIFIED: i32 = 1;


pub fn zombie_stand1(g: &mut Game) {
    g.frame_state(F_stand1, zombie_stand2);
    ai_stand(g);
}

pub fn zombie_stand2(g: &mut Game) {
    g.frame_state(F_stand2, zombie_stand3);
    ai_stand(g);
}

pub fn zombie_stand3(g: &mut Game) {
    g.frame_state(F_stand3, zombie_stand4);
    ai_stand(g);
}

pub fn zombie_stand4(g: &mut Game) {
    g.frame_state(F_stand4, zombie_stand5);
    ai_stand(g);
}

pub fn zombie_stand5(g: &mut Game) {
    g.frame_state(F_stand5, zombie_stand6);
    ai_stand(g);
}

pub fn zombie_stand6(g: &mut Game) {
    g.frame_state(F_stand6, zombie_stand7);
    ai_stand(g);
}

pub fn zombie_stand7(g: &mut Game) {
    g.frame_state(F_stand7, zombie_stand8);
    ai_stand(g);
}

pub fn zombie_stand8(g: &mut Game) {
    g.frame_state(F_stand8, zombie_stand9);
    ai_stand(g);
}

pub fn zombie_stand9(g: &mut Game) {
    g.frame_state(F_stand9, zombie_stand10);
    ai_stand(g);
}

pub fn zombie_stand10(g: &mut Game) {
    g.frame_state(F_stand10, zombie_stand11);
    ai_stand(g);
}

pub fn zombie_stand11(g: &mut Game) {
    g.frame_state(F_stand11, zombie_stand12);
    ai_stand(g);
}

pub fn zombie_stand12(g: &mut Game) {
    g.frame_state(F_stand12, zombie_stand13);
    ai_stand(g);
}

pub fn zombie_stand13(g: &mut Game) {
    g.frame_state(F_stand13, zombie_stand14);
    ai_stand(g);
}

pub fn zombie_stand14(g: &mut Game) {
    g.frame_state(F_stand14, zombie_stand15);
    ai_stand(g);
}

pub fn zombie_stand15(g: &mut Game) {
    g.frame_state(F_stand15, zombie_stand1);
    ai_stand(g);
}

pub fn zombie_cruc1(g: &mut Game) {
    g.frame_state(F_cruc_1, zombie_cruc2);
    if g.random() < 0.1 {
        g.sound(g.self_, CHAN_VOICE, "zombie/idle_w2.wav", 1.0, ATTN_STATIC);
    }
}

pub fn zombie_cruc2(g: &mut Game) {
    g.frame_state(F_cruc_2, zombie_cruc3);
    g.v(g.self_).nextthink = g.time + 0.1 + g.random() * 0.1;
}

pub fn zombie_cruc3(g: &mut Game) {
    g.frame_state(F_cruc_3, zombie_cruc4);
    g.v(g.self_).nextthink = g.time + 0.1 + g.random() * 0.1;
}

pub fn zombie_cruc4(g: &mut Game) {
    g.frame_state(F_cruc_4, zombie_cruc5);
    g.v(g.self_).nextthink = g.time + 0.1 + g.random() * 0.1;
}

pub fn zombie_cruc5(g: &mut Game) {
    g.frame_state(F_cruc_5, zombie_cruc6);
    g.v(g.self_).nextthink = g.time + 0.1 + g.random() * 0.1;
}

pub fn zombie_cruc6(g: &mut Game) {
    g.frame_state(F_cruc_6, zombie_cruc1);
    g.v(g.self_).nextthink = g.time + 0.1 + g.random() * 0.1;
}

pub fn zombie_walk1(g: &mut Game) {
    g.frame_state(F_walk1, zombie_walk2);
    ai_walk(g, 0.0);
}

pub fn zombie_walk2(g: &mut Game) {
    g.frame_state(F_walk2, zombie_walk3);
    ai_walk(g, 2.0);
}

pub fn zombie_walk3(g: &mut Game) {
    g.frame_state(F_walk3, zombie_walk4);
    ai_walk(g, 3.0);
}

pub fn zombie_walk4(g: &mut Game) {
    g.frame_state(F_walk4, zombie_walk5);
    ai_walk(g, 2.0);
}

pub fn zombie_walk5(g: &mut Game) {
    g.frame_state(F_walk5, zombie_walk6);
    ai_walk(g, 1.0);
}

pub fn zombie_walk6(g: &mut Game) {
    g.frame_state(F_walk6, zombie_walk7);
    ai_walk(g, 0.0);
}

pub fn zombie_walk7(g: &mut Game) {
    g.frame_state(F_walk7, zombie_walk8);
    ai_walk(g, 0.0);
}

pub fn zombie_walk8(g: &mut Game) {
    g.frame_state(F_walk8, zombie_walk9);
    ai_walk(g, 0.0);
}

pub fn zombie_walk9(g: &mut Game) {
    g.frame_state(F_walk9, zombie_walk10);
    ai_walk(g, 0.0);
}

pub fn zombie_walk10(g: &mut Game) {
    g.frame_state(F_walk10, zombie_walk11);
    ai_walk(g, 0.0);
}

pub fn zombie_walk11(g: &mut Game) {
    g.frame_state(F_walk11, zombie_walk12);
    ai_walk(g, 2.0);
}

pub fn zombie_walk12(g: &mut Game) {
    g.frame_state(F_walk12, zombie_walk13);
    ai_walk(g, 2.0);
}

pub fn zombie_walk13(g: &mut Game) {
    g.frame_state(F_walk13, zombie_walk14);
    ai_walk(g, 1.0);
}

pub fn zombie_walk14(g: &mut Game) {
    g.frame_state(F_walk14, zombie_walk15);
    ai_walk(g, 0.0);
}

pub fn zombie_walk15(g: &mut Game) {
    g.frame_state(F_walk15, zombie_walk16);
    ai_walk(g, 0.0);
}

pub fn zombie_walk16(g: &mut Game) {
    g.frame_state(F_walk16, zombie_walk17);
    ai_walk(g, 0.0);
}

pub fn zombie_walk17(g: &mut Game) {
    g.frame_state(F_walk17, zombie_walk18);
    ai_walk(g, 0.0);
}

pub fn zombie_walk18(g: &mut Game) {
    g.frame_state(F_walk18, zombie_walk19);
    ai_walk(g, 0.0);
}

pub fn zombie_walk19(g: &mut Game) {
    g.frame_state(F_walk19, zombie_walk1);
    ai_walk(g, 0.0);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "zombie/z_idle.wav", 1.0, ATTN_IDLE);
    }
}

pub fn zombie_run1(g: &mut Game) {
    g.frame_state(F_run1, zombie_run2);
    ai_run(g, 1.0);
    g.v(g.self_).inpain = 0.0;
}

pub fn zombie_run2(g: &mut Game) {
    g.frame_state(F_run2, zombie_run3);
    ai_run(g, 1.0);
}

pub fn zombie_run3(g: &mut Game) {
    g.frame_state(F_run3, zombie_run4);
    ai_run(g, 0.0);
}

pub fn zombie_run4(g: &mut Game) {
    g.frame_state(F_run4, zombie_run5);
    ai_run(g, 1.0);
}

pub fn zombie_run5(g: &mut Game) {
    g.frame_state(F_run5, zombie_run6);
    ai_run(g, 2.0);
}

pub fn zombie_run6(g: &mut Game) {
    g.frame_state(F_run6, zombie_run7);
    ai_run(g, 3.0);
}

pub fn zombie_run7(g: &mut Game) {
    g.frame_state(F_run7, zombie_run8);
    ai_run(g, 4.0);
}

pub fn zombie_run8(g: &mut Game) {
    g.frame_state(F_run8, zombie_run9);
    ai_run(g, 4.0);
}

pub fn zombie_run9(g: &mut Game) {
    g.frame_state(F_run9, zombie_run10);
    ai_run(g, 2.0);
}

pub fn zombie_run10(g: &mut Game) {
    g.frame_state(F_run10, zombie_run11);
    ai_run(g, 0.0);
}

pub fn zombie_run11(g: &mut Game) {
    g.frame_state(F_run11, zombie_run12);
    ai_run(g, 0.0);
}

pub fn zombie_run12(g: &mut Game) {
    g.frame_state(F_run12, zombie_run13);
    ai_run(g, 0.0);
}

pub fn zombie_run13(g: &mut Game) {
    g.frame_state(F_run13, zombie_run14);
    ai_run(g, 2.0);
}

pub fn zombie_run14(g: &mut Game) {
    g.frame_state(F_run14, zombie_run15);
    ai_run(g, 4.0);
}

pub fn zombie_run15(g: &mut Game) {
    g.frame_state(F_run15, zombie_run16);
    ai_run(g, 6.0);
}

pub fn zombie_run16(g: &mut Game) {
    g.frame_state(F_run16, zombie_run17);
    ai_run(g, 7.0);
}

pub fn zombie_run17(g: &mut Game) {
    g.frame_state(F_run17, zombie_run18);
    ai_run(g, 3.0);
}

pub fn zombie_run18(g: &mut Game) {
    g.frame_state(F_run18, zombie_run1);
    ai_run(g, 8.0);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "zombie/z_idle.wav", 1.0, ATTN_IDLE);
    } else if g.random() > 0.8 {
        g.sound(g.self_, CHAN_VOICE, "zombie/z_idle1.wav", 1.0, ATTN_IDLE);
    }
}

pub fn ZombieGrenadeTouch(g: &mut Game) {
    let s = g.self_;
    let other = g.other;
    if other == g.r(s).owner {
        return; // don't explode on owner
    }

    if g.r(other).takedamage != 0 {
        T_Damage(g, other, s, g.r(s).owner, 10.0);
        g.sound(s, CHAN_WEAPON, "zombie/z_hit.wav", 1.0, ATTN_NORM);
        g.remove(s);
        return;
    }

    g.sound(s, CHAN_WEAPON, "zombie/z_miss.wav", 1.0, ATTN_NORM); // bounce sound

    g.v(s).velocity = Vec3::ZERO;
    g.v(s).avelocity = Vec3::ZERO;
    g.v(s).touch = Some(SUB_Remove);
}

pub fn ZombieFireGrenade(g: &mut Game, st: Vec3) {
    let s = g.self_;
    g.sound(s, CHAN_WEAPON, "zombie/z_shot1.wav", 1.0, ATTN_NORM);

    let missile = g.spawn();
    g.v(missile).owner = s;
    g.v(missile).movetype = MOVETYPE_BOUNCE;
    g.v(missile).solid = SOLID_BBOX;

    // calc org (with whatever v_forward was before, as id's code does)
    let org = g.r(s).origin + st.x * g.v_forward + st.y * g.v_right + (st.z - 24.0) * g.v_up;

    // set missile speed
    g.makevectors(g.r(s).angles);

    let enemy = g.r(s).enemy;
    g.v(missile).velocity = normalize(g.r(enemy).origin - org);
    g.v(missile).velocity = g.r(missile).velocity * 600.0;
    g.v(missile).velocity.z = 200.0;

    g.v(missile).avelocity = Vec3::new(3000.0, 1000.0, 2000.0);

    g.v(missile).touch = Some(ZombieGrenadeTouch);

    // set missile duration
    g.v(missile).nextthink = g.time + 2.5;
    g.v(missile).think = Some(SUB_Remove);

    g.setmodel(missile, "progs/zom_gib.mdl");
    g.setsize(missile, Vec3::ZERO, Vec3::ZERO);
    g.setorigin(missile, org);
}

pub fn zombie_atta1(g: &mut Game) {
    g.frame_state(F_atta1, zombie_atta2);
    ai_face(g);
}

pub fn zombie_atta2(g: &mut Game) {
    g.frame_state(F_atta2, zombie_atta3);
    ai_face(g);
}

pub fn zombie_atta3(g: &mut Game) {
    g.frame_state(F_atta3, zombie_atta4);
    ai_face(g);
}

pub fn zombie_atta4(g: &mut Game) {
    g.frame_state(F_atta4, zombie_atta5);
    ai_face(g);
}

pub fn zombie_atta5(g: &mut Game) {
    g.frame_state(F_atta5, zombie_atta6);
    ai_face(g);
}

pub fn zombie_atta6(g: &mut Game) {
    g.frame_state(F_atta6, zombie_atta7);
    ai_face(g);
}

pub fn zombie_atta7(g: &mut Game) {
    g.frame_state(F_atta7, zombie_atta8);
    ai_face(g);
}

pub fn zombie_atta8(g: &mut Game) {
    g.frame_state(F_atta8, zombie_atta9);
    ai_face(g);
}

pub fn zombie_atta9(g: &mut Game) {
    g.frame_state(F_atta9, zombie_atta10);
    ai_face(g);
}

pub fn zombie_atta10(g: &mut Game) {
    g.frame_state(F_atta10, zombie_atta11);
    ai_face(g);
}

pub fn zombie_atta11(g: &mut Game) {
    g.frame_state(F_atta11, zombie_atta12);
    ai_face(g);
}

pub fn zombie_atta12(g: &mut Game) {
    g.frame_state(F_atta12, zombie_atta13);
    ai_face(g);
}

pub fn zombie_atta13(g: &mut Game) {
    g.frame_state(F_atta13, zombie_run1);
    ai_face(g);
    ZombieFireGrenade(g, Vec3::new(-10.0, -22.0, 30.0));
}

pub fn zombie_attb1(g: &mut Game) {
    g.frame_state(F_attb1, zombie_attb2);
    ai_face(g);
}

pub fn zombie_attb2(g: &mut Game) {
    g.frame_state(F_attb2, zombie_attb3);
    ai_face(g);
}

pub fn zombie_attb3(g: &mut Game) {
    g.frame_state(F_attb3, zombie_attb4);
    ai_face(g);
}

pub fn zombie_attb4(g: &mut Game) {
    g.frame_state(F_attb4, zombie_attb5);
    ai_face(g);
}

pub fn zombie_attb5(g: &mut Game) {
    g.frame_state(F_attb5, zombie_attb6);
    ai_face(g);
}

pub fn zombie_attb6(g: &mut Game) {
    g.frame_state(F_attb6, zombie_attb7);
    ai_face(g);
}

pub fn zombie_attb7(g: &mut Game) {
    g.frame_state(F_attb7, zombie_attb8);
    ai_face(g);
}

pub fn zombie_attb8(g: &mut Game) {
    g.frame_state(F_attb8, zombie_attb9);
    ai_face(g);
}

pub fn zombie_attb9(g: &mut Game) {
    g.frame_state(F_attb9, zombie_attb10);
    ai_face(g);
}

pub fn zombie_attb10(g: &mut Game) {
    g.frame_state(F_attb10, zombie_attb11);
    ai_face(g);
}

pub fn zombie_attb11(g: &mut Game) {
    g.frame_state(F_attb11, zombie_attb12);
    ai_face(g);
}

pub fn zombie_attb12(g: &mut Game) {
    g.frame_state(F_attb12, zombie_attb13);
    ai_face(g);
}

pub fn zombie_attb13(g: &mut Game) {
    g.frame_state(F_attb13, zombie_attb14);
    ai_face(g);
}

pub fn zombie_attb14(g: &mut Game) {
    g.frame_state(F_attb13, zombie_run1);
    ai_face(g);
    ZombieFireGrenade(g, Vec3::new(-10.0, -24.0, 29.0));
}

pub fn zombie_attc1(g: &mut Game) {
    g.frame_state(F_attc1, zombie_attc2);
    ai_face(g);
}

pub fn zombie_attc2(g: &mut Game) {
    g.frame_state(F_attc2, zombie_attc3);
    ai_face(g);
}

pub fn zombie_attc3(g: &mut Game) {
    g.frame_state(F_attc3, zombie_attc4);
    ai_face(g);
}

pub fn zombie_attc4(g: &mut Game) {
    g.frame_state(F_attc4, zombie_attc5);
    ai_face(g);
}

pub fn zombie_attc5(g: &mut Game) {
    g.frame_state(F_attc5, zombie_attc6);
    ai_face(g);
}

pub fn zombie_attc6(g: &mut Game) {
    g.frame_state(F_attc6, zombie_attc7);
    ai_face(g);
}

pub fn zombie_attc7(g: &mut Game) {
    g.frame_state(F_attc7, zombie_attc8);
    ai_face(g);
}

pub fn zombie_attc8(g: &mut Game) {
    g.frame_state(F_attc8, zombie_attc9);
    ai_face(g);
}

pub fn zombie_attc9(g: &mut Game) {
    g.frame_state(F_attc9, zombie_attc10);
    ai_face(g);
}

pub fn zombie_attc10(g: &mut Game) {
    g.frame_state(F_attc10, zombie_attc11);
    ai_face(g);
}

pub fn zombie_attc11(g: &mut Game) {
    g.frame_state(F_attc11, zombie_attc12);
    ai_face(g);
}

pub fn zombie_attc12(g: &mut Game) {
    g.frame_state(F_attc12, zombie_run1);
    ai_face(g);
    ZombieFireGrenade(g, Vec3::new(-12.0, -19.0, 29.0));
}

pub fn zombie_missile(g: &mut Game) {
    let r = g.random();

    if r < 0.3 {
        zombie_atta1(g);
    } else if r < 0.6 {
        zombie_attb1(g);
    } else {
        zombie_attc1(g);
    }
}

pub fn zombie_paina1(g: &mut Game) {
    g.frame_state(F_paina1, zombie_paina2);
    g.sound(g.self_, CHAN_VOICE, "zombie/z_pain.wav", 1.0, ATTN_NORM);
}

pub fn zombie_paina2(g: &mut Game) {
    g.frame_state(F_paina2, zombie_paina3);
    ai_painforward(g, 3.0);
}

pub fn zombie_paina3(g: &mut Game) {
    g.frame_state(F_paina3, zombie_paina4);
    ai_painforward(g, 1.0);
}

pub fn zombie_paina4(g: &mut Game) {
    g.frame_state(F_paina4, zombie_paina5);
    ai_pain(g, 1.0);
}

pub fn zombie_paina5(g: &mut Game) {
    g.frame_state(F_paina5, zombie_paina6);
    ai_pain(g, 3.0);
}

pub fn zombie_paina6(g: &mut Game) {
    g.frame_state(F_paina6, zombie_paina7);
    ai_pain(g, 1.0);
}

pub fn zombie_paina7(g: &mut Game) {
    g.frame_state(F_paina7, zombie_paina8);
}

pub fn zombie_paina8(g: &mut Game) {
    g.frame_state(F_paina8, zombie_paina9);
}

pub fn zombie_paina9(g: &mut Game) {
    g.frame_state(F_paina9, zombie_paina10);
}

pub fn zombie_paina10(g: &mut Game) {
    g.frame_state(F_paina10, zombie_paina11);
}

pub fn zombie_paina11(g: &mut Game) {
    g.frame_state(F_paina11, zombie_paina12);
}

pub fn zombie_paina12(g: &mut Game) {
    g.frame_state(F_paina12, zombie_run1);
}

pub fn zombie_painb1(g: &mut Game) {
    g.frame_state(F_painb1, zombie_painb2);
    g.sound(g.self_, CHAN_VOICE, "zombie/z_pain1.wav", 1.0, ATTN_NORM);
}

pub fn zombie_painb2(g: &mut Game) {
    g.frame_state(F_painb2, zombie_painb3);
    ai_pain(g, 2.0);
}

pub fn zombie_painb3(g: &mut Game) {
    g.frame_state(F_painb3, zombie_painb4);
    ai_pain(g, 8.0);
}

pub fn zombie_painb4(g: &mut Game) {
    g.frame_state(F_painb4, zombie_painb5);
    ai_pain(g, 6.0);
}

pub fn zombie_painb5(g: &mut Game) {
    g.frame_state(F_painb5, zombie_painb6);
    ai_pain(g, 2.0);
}

pub fn zombie_painb6(g: &mut Game) {
    g.frame_state(F_painb6, zombie_painb7);
}

pub fn zombie_painb7(g: &mut Game) {
    g.frame_state(F_painb7, zombie_painb8);
}

pub fn zombie_painb8(g: &mut Game) {
    g.frame_state(F_painb8, zombie_painb9);
}

pub fn zombie_painb9(g: &mut Game) {
    g.frame_state(F_painb9, zombie_painb10);
    g.sound(g.self_, CHAN_BODY, "zombie/z_fall.wav", 1.0, ATTN_NORM);
}

pub fn zombie_painb10(g: &mut Game) {
    g.frame_state(F_painb10, zombie_painb11);
}

pub fn zombie_painb11(g: &mut Game) {
    g.frame_state(F_painb11, zombie_painb12);
}

pub fn zombie_painb12(g: &mut Game) {
    g.frame_state(F_painb12, zombie_painb13);
}

pub fn zombie_painb13(g: &mut Game) {
    g.frame_state(F_painb13, zombie_painb14);
}

pub fn zombie_painb14(g: &mut Game) {
    g.frame_state(F_painb14, zombie_painb15);
}

pub fn zombie_painb15(g: &mut Game) {
    g.frame_state(F_painb15, zombie_painb16);
}

pub fn zombie_painb16(g: &mut Game) {
    g.frame_state(F_painb16, zombie_painb17);
}

pub fn zombie_painb17(g: &mut Game) {
    g.frame_state(F_painb17, zombie_painb18);
}

pub fn zombie_painb18(g: &mut Game) {
    g.frame_state(F_painb18, zombie_painb19);
}

pub fn zombie_painb19(g: &mut Game) {
    g.frame_state(F_painb19, zombie_painb20);
}

pub fn zombie_painb20(g: &mut Game) {
    g.frame_state(F_painb20, zombie_painb21);
}

pub fn zombie_painb21(g: &mut Game) {
    g.frame_state(F_painb21, zombie_painb22);
}

pub fn zombie_painb22(g: &mut Game) {
    g.frame_state(F_painb22, zombie_painb23);
}

pub fn zombie_painb23(g: &mut Game) {
    g.frame_state(F_painb23, zombie_painb24);
}

pub fn zombie_painb24(g: &mut Game) {
    g.frame_state(F_painb24, zombie_painb25);
}

pub fn zombie_painb25(g: &mut Game) {
    g.frame_state(F_painb25, zombie_painb26);
    ai_painforward(g, 1.0);
}

pub fn zombie_painb26(g: &mut Game) {
    g.frame_state(F_painb26, zombie_painb27);
}

pub fn zombie_painb27(g: &mut Game) {
    g.frame_state(F_painb27, zombie_painb28);
}

pub fn zombie_painb28(g: &mut Game) {
    g.frame_state(F_painb28, zombie_run1);
}

pub fn zombie_painc1(g: &mut Game) {
    g.frame_state(F_painc1, zombie_painc2);
    g.sound(g.self_, CHAN_VOICE, "zombie/z_pain1.wav", 1.0, ATTN_NORM);
}

pub fn zombie_painc2(g: &mut Game) {
    g.frame_state(F_painc2, zombie_painc3);
}

pub fn zombie_painc3(g: &mut Game) {
    g.frame_state(F_painc3, zombie_painc4);
    ai_pain(g, 3.0);
}

pub fn zombie_painc4(g: &mut Game) {
    g.frame_state(F_painc4, zombie_painc5);
    ai_pain(g, 1.0);
}

pub fn zombie_painc5(g: &mut Game) {
    g.frame_state(F_painc5, zombie_painc6);
}

pub fn zombie_painc6(g: &mut Game) {
    g.frame_state(F_painc6, zombie_painc7);
}

pub fn zombie_painc7(g: &mut Game) {
    g.frame_state(F_painc7, zombie_painc8);
}

pub fn zombie_painc8(g: &mut Game) {
    g.frame_state(F_painc8, zombie_painc9);
}

pub fn zombie_painc9(g: &mut Game) {
    g.frame_state(F_painc9, zombie_painc10);
}

pub fn zombie_painc10(g: &mut Game) {
    g.frame_state(F_painc10, zombie_painc11);
}

pub fn zombie_painc11(g: &mut Game) {
    g.frame_state(F_painc11, zombie_painc12);
    ai_painforward(g, 1.0);
}

pub fn zombie_painc12(g: &mut Game) {
    g.frame_state(F_painc12, zombie_painc13);
    ai_painforward(g, 1.0);
}

pub fn zombie_painc13(g: &mut Game) {
    g.frame_state(F_painc13, zombie_painc14);
}

pub fn zombie_painc14(g: &mut Game) {
    g.frame_state(F_painc14, zombie_painc15);
}

pub fn zombie_painc15(g: &mut Game) {
    g.frame_state(F_painc15, zombie_painc16);
}

pub fn zombie_painc16(g: &mut Game) {
    g.frame_state(F_painc16, zombie_painc17);
}

pub fn zombie_painc17(g: &mut Game) {
    g.frame_state(F_painc17, zombie_painc18);
}

pub fn zombie_painc18(g: &mut Game) {
    g.frame_state(F_painc18, zombie_run1);
}

pub fn zombie_paind1(g: &mut Game) {
    g.frame_state(F_paind1, zombie_paind2);
    g.sound(g.self_, CHAN_VOICE, "zombie/z_pain.wav", 1.0, ATTN_NORM);
}

pub fn zombie_paind2(g: &mut Game) {
    g.frame_state(F_paind2, zombie_paind3);
}

pub fn zombie_paind3(g: &mut Game) {
    g.frame_state(F_paind3, zombie_paind4);
}

pub fn zombie_paind4(g: &mut Game) {
    g.frame_state(F_paind4, zombie_paind5);
}

pub fn zombie_paind5(g: &mut Game) {
    g.frame_state(F_paind5, zombie_paind6);
}

pub fn zombie_paind6(g: &mut Game) {
    g.frame_state(F_paind6, zombie_paind7);
}

pub fn zombie_paind7(g: &mut Game) {
    g.frame_state(F_paind7, zombie_paind8);
}

pub fn zombie_paind8(g: &mut Game) {
    g.frame_state(F_paind8, zombie_paind9);
}

pub fn zombie_paind9(g: &mut Game) {
    g.frame_state(F_paind9, zombie_paind10);
    ai_pain(g, 1.0);
}

pub fn zombie_paind10(g: &mut Game) {
    g.frame_state(F_paind10, zombie_paind11);
}

pub fn zombie_paind11(g: &mut Game) {
    g.frame_state(F_paind11, zombie_paind12);
}

pub fn zombie_paind12(g: &mut Game) {
    g.frame_state(F_paind12, zombie_paind13);
}

pub fn zombie_paind13(g: &mut Game) {
    g.frame_state(F_paind13, zombie_run1);
}

pub fn zombie_paine1(g: &mut Game) {
    g.frame_state(F_paine1, zombie_paine2);
    g.sound(g.self_, CHAN_VOICE, "zombie/z_pain.wav", 1.0, ATTN_NORM);
    g.v(g.self_).health = 60.0;
}

pub fn zombie_paine2(g: &mut Game) {
    g.frame_state(F_paine2, zombie_paine3);
    ai_pain(g, 8.0);
}

pub fn zombie_paine3(g: &mut Game) {
    g.frame_state(F_paine3, zombie_paine4);
    ai_pain(g, 5.0);
}

pub fn zombie_paine4(g: &mut Game) {
    g.frame_state(F_paine4, zombie_paine5);
    ai_pain(g, 3.0);
}

pub fn zombie_paine5(g: &mut Game) {
    g.frame_state(F_paine5, zombie_paine6);
    ai_pain(g, 1.0);
}

pub fn zombie_paine6(g: &mut Game) {
    g.frame_state(F_paine6, zombie_paine7);
    ai_pain(g, 2.0);
}

pub fn zombie_paine7(g: &mut Game) {
    g.frame_state(F_paine7, zombie_paine8);
    ai_pain(g, 1.0);
}

pub fn zombie_paine8(g: &mut Game) {
    g.frame_state(F_paine8, zombie_paine9);
    ai_pain(g, 1.0);
}

pub fn zombie_paine9(g: &mut Game) {
    g.frame_state(F_paine9, zombie_paine10);
    ai_pain(g, 2.0);
}

pub fn zombie_paine10(g: &mut Game) {
    g.frame_state(F_paine10, zombie_paine11);
    g.sound(g.self_, CHAN_BODY, "zombie/z_fall.wav", 1.0, ATTN_NORM);
    g.v(g.self_).solid = SOLID_NOT;
}

pub fn zombie_paine11(g: &mut Game) {
    g.frame_state(F_paine11, zombie_paine12);
    let s = g.self_;
    g.v(s).nextthink = g.r(s).nextthink + 5.0;
    g.v(s).health = 60.0;
}

pub fn zombie_paine12(g: &mut Game) {
    g.frame_state(F_paine12, zombie_paine13);
    let s = g.self_;
    // see if ok to stand up
    g.v(s).health = 60.0;
    g.sound(s, CHAN_VOICE, "zombie/z_idle.wav", 1.0, ATTN_IDLE);
    g.v(s).solid = SOLID_SLIDEBOX;
    if !g.walkmove(0.0, 0.0) {
        let s = g.self_;
        g.v(s).think = Some(zombie_paine11);
        g.v(s).solid = SOLID_NOT;
        return;
    }
}

pub fn zombie_paine13(g: &mut Game) {
    g.frame_state(F_paine13, zombie_paine14);
}

pub fn zombie_paine14(g: &mut Game) {
    g.frame_state(F_paine14, zombie_paine15);
}

pub fn zombie_paine15(g: &mut Game) {
    g.frame_state(F_paine15, zombie_paine16);
}

pub fn zombie_paine16(g: &mut Game) {
    g.frame_state(F_paine16, zombie_paine17);
}

pub fn zombie_paine17(g: &mut Game) {
    g.frame_state(F_paine17, zombie_paine18);
}

pub fn zombie_paine18(g: &mut Game) {
    g.frame_state(F_paine18, zombie_paine19);
}

pub fn zombie_paine19(g: &mut Game) {
    g.frame_state(F_paine19, zombie_paine20);
}

pub fn zombie_paine20(g: &mut Game) {
    g.frame_state(F_paine20, zombie_paine21);
}

pub fn zombie_paine21(g: &mut Game) {
    g.frame_state(F_paine21, zombie_paine22);
}

pub fn zombie_paine22(g: &mut Game) {
    g.frame_state(F_paine22, zombie_paine23);
}

pub fn zombie_paine23(g: &mut Game) {
    g.frame_state(F_paine23, zombie_paine24);
}

pub fn zombie_paine24(g: &mut Game) {
    g.frame_state(F_paine24, zombie_paine25);
}

pub fn zombie_paine25(g: &mut Game) {
    g.frame_state(F_paine25, zombie_paine26);
    ai_painforward(g, 5.0);
}

pub fn zombie_paine26(g: &mut Game) {
    g.frame_state(F_paine26, zombie_paine27);
    ai_painforward(g, 3.0);
}

pub fn zombie_paine27(g: &mut Game) {
    g.frame_state(F_paine27, zombie_paine28);
    ai_painforward(g, 1.0);
}

pub fn zombie_paine28(g: &mut Game) {
    g.frame_state(F_paine28, zombie_paine29);
    ai_pain(g, 1.0);
}

pub fn zombie_paine29(g: &mut Game) {
    g.frame_state(F_paine29, zombie_paine30);
}

pub fn zombie_paine30(g: &mut Game) {
    g.frame_state(F_paine30, zombie_run1);
}

pub fn zombie_die(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_VOICE, "zombie/z_gib.wav", 1.0, ATTN_NORM);
    ThrowHead(g, "progs/h_zombie.mdl", g.r(s).health);
    ThrowGib(g, "progs/gib1.mdl", g.r(s).health);
    ThrowGib(g, "progs/gib2.mdl", g.r(s).health);
    ThrowGib(g, "progs/gib3.mdl", g.r(s).health);
}

// Zombies can only be killed (gibbed) by doing 60 hit points of damage in a single frame
// (rockets, grenades, quad shotgun, quad nailgun).
//
// A hit of 25 points or more (super shotgun, quad nailgun) will always put it down to the ground.
//
// A hit of from 10 to 40 points in one frame will cause it to go down if it has been twice in two
// seconds, otherwise it goes into one of the four fast pain frames.
//
// A hit of less than 10 points of damage (winged by a shotgun) will be ignored.
//
// FIXME: don't use pain_finished because of nightmare hack
pub fn zombie_pain(g: &mut Game, attacker: Ent, damage: f32) {
    let s = g.self_;
    g.v(s).health = 60.0; // always reset health

    if damage < 9.0 {
        return; // totally ignore
    }

    if g.r(s).inpain == 2.0 {
        return; // down on ground, so don't reset any counters
    }

    // go down immediately if a big enough hit
    if damage >= 25.0 {
        g.v(s).inpain = 2.0;
        zombie_paine1(g);
        return;
    }

    if g.r(s).inpain != 0.0 {
        // if hit again in next three seconds while not in pain frames, definitely drop
        g.v(s).pain_finished = g.time + 3.0;
        return; // currently going through an animation, don't change
    }

    if g.r(s).pain_finished > g.time {
        // hit again, so drop down
        g.v(s).inpain = 2.0;
        zombie_paine1(g);
        return;
    }

    // go into one of the fast pain animations
    g.v(s).inpain = 1.0;

    let r = g.random();
    if r < 0.25 {
        zombie_paina1(g);
    } else if r < 0.5 {
        zombie_painb1(g);
    } else if r < 0.75 {
        zombie_painc1(g);
    } else {
        zombie_paind1(g);
    }
}

// If crucified, stick the bounding box 12 pixels back into a wall to look right.
pub fn monster_zombie(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model("progs/zombie.mdl");
    g.precache_model("progs/h_zombie.mdl");
    g.precache_model("progs/zom_gib.mdl");

    g.precache_sound("zombie/z_idle.wav");
    g.precache_sound("zombie/z_idle1.wav");
    g.precache_sound("zombie/z_shot1.wav");
    g.precache_sound("zombie/z_gib.wav");
    g.precache_sound("zombie/z_pain.wav");
    g.precache_sound("zombie/z_pain1.wav");
    g.precache_sound("zombie/z_fall.wav");
    g.precache_sound("zombie/z_miss.wav");
    g.precache_sound("zombie/z_hit.wav");
    g.precache_sound("zombie/idle_w2.wav");

    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/zombie.mdl");

    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 40.0));
    g.v(s).health = 60.0;

    g.v(s).th_stand = Some(zombie_stand1);
    g.v(s).th_walk = Some(zombie_walk1);
    g.v(s).th_run = Some(zombie_run1);
    g.v(s).th_pain = Some(zombie_pain);
    g.v(s).th_die = Some(zombie_die);
    g.v(s).th_missile = Some(zombie_missile);

    if (g.r(s).spawnflags & SPAWN_CRUCIFIED) != 0 {
        g.v(s).movetype = MOVETYPE_NONE;
        zombie_cruc1(g);
    } else {
        walkmonster_start(g);
    }
}
