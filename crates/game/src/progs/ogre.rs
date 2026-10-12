//! `ogre.qc`, id's QuakeC 1.06, in Rust.

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
const F_run1: f32 = 25.0;
const F_run2: f32 = 26.0;
const F_run3: f32 = 27.0;
const F_run4: f32 = 28.0;
const F_run5: f32 = 29.0;
const F_run6: f32 = 30.0;
const F_run7: f32 = 31.0;
const F_run8: f32 = 32.0;
const F_swing1: f32 = 33.0;
const F_swing2: f32 = 34.0;
const F_swing3: f32 = 35.0;
const F_swing4: f32 = 36.0;
const F_swing5: f32 = 37.0;
const F_swing6: f32 = 38.0;
const F_swing7: f32 = 39.0;
const F_swing8: f32 = 40.0;
const F_swing9: f32 = 41.0;
const F_swing10: f32 = 42.0;
const F_swing11: f32 = 43.0;
const F_swing12: f32 = 44.0;
const F_swing13: f32 = 45.0;
const F_swing14: f32 = 46.0;
const F_smash1: f32 = 47.0;
const F_smash2: f32 = 48.0;
const F_smash3: f32 = 49.0;
const F_smash4: f32 = 50.0;
const F_smash5: f32 = 51.0;
const F_smash6: f32 = 52.0;
const F_smash7: f32 = 53.0;
const F_smash8: f32 = 54.0;
const F_smash9: f32 = 55.0;
const F_smash10: f32 = 56.0;
const F_smash11: f32 = 57.0;
const F_smash12: f32 = 58.0;
const F_smash13: f32 = 59.0;
const F_smash14: f32 = 60.0;
const F_shoot1: f32 = 61.0;
const F_shoot2: f32 = 62.0;
const F_shoot3: f32 = 63.0;
const F_shoot4: f32 = 64.0;
const F_shoot5: f32 = 65.0;
const F_shoot6: f32 = 66.0;
const F_pain1: f32 = 67.0;
const F_pain2: f32 = 68.0;
const F_pain3: f32 = 69.0;
const F_pain4: f32 = 70.0;
const F_pain5: f32 = 71.0;
const F_painb1: f32 = 72.0;
const F_painb2: f32 = 73.0;
const F_painb3: f32 = 74.0;
const F_painc1: f32 = 75.0;
const F_painc2: f32 = 76.0;
const F_painc3: f32 = 77.0;
const F_painc4: f32 = 78.0;
const F_painc5: f32 = 79.0;
const F_painc6: f32 = 80.0;
const F_paind1: f32 = 81.0;
const F_paind2: f32 = 82.0;
const F_paind3: f32 = 83.0;
const F_paind4: f32 = 84.0;
const F_paind5: f32 = 85.0;
const F_paind6: f32 = 86.0;
const F_paind7: f32 = 87.0;
const F_paind8: f32 = 88.0;
const F_paind9: f32 = 89.0;
const F_paind10: f32 = 90.0;
const F_paind11: f32 = 91.0;
const F_paind12: f32 = 92.0;
const F_paind13: f32 = 93.0;
const F_paind14: f32 = 94.0;
const F_paind15: f32 = 95.0;
const F_paind16: f32 = 96.0;
const F_paine1: f32 = 97.0;
const F_paine2: f32 = 98.0;
const F_paine3: f32 = 99.0;
const F_paine4: f32 = 100.0;
const F_paine5: f32 = 101.0;
const F_paine6: f32 = 102.0;
const F_paine7: f32 = 103.0;
const F_paine8: f32 = 104.0;
const F_paine9: f32 = 105.0;
const F_paine10: f32 = 106.0;
const F_paine11: f32 = 107.0;
const F_paine12: f32 = 108.0;
const F_paine13: f32 = 109.0;
const F_paine14: f32 = 110.0;
const F_paine15: f32 = 111.0;
const F_death1: f32 = 112.0;
const F_death2: f32 = 113.0;
const F_death3: f32 = 114.0;
const F_death4: f32 = 115.0;
const F_death5: f32 = 116.0;
const F_death6: f32 = 117.0;
const F_death7: f32 = 118.0;
const F_death8: f32 = 119.0;
const F_death9: f32 = 120.0;
const F_death10: f32 = 121.0;
const F_death11: f32 = 122.0;
const F_death12: f32 = 123.0;
const F_death13: f32 = 124.0;
const F_death14: f32 = 125.0;
const F_bdeath1: f32 = 126.0;
const F_bdeath2: f32 = 127.0;
const F_bdeath3: f32 = 128.0;
const F_bdeath4: f32 = 129.0;
const F_bdeath5: f32 = 130.0;
const F_bdeath6: f32 = 131.0;
const F_bdeath7: f32 = 132.0;
const F_bdeath8: f32 = 133.0;
const F_bdeath9: f32 = 134.0;
const F_bdeath10: f32 = 135.0;


pub fn OgreGrenadeExplode(g: &mut Game) {
    let s = g.self_;
    let owner = g.r(s).owner;
    T_RadiusDamage(g, s, owner, 40.0, world);
    let s = g.self_;
    g.sound(s, CHAN_VOICE, "weapons/r_exp3.wav", 1.0, ATTN_NORM);

    g.temp_entity(TempEntity::Explosion(g.r(s).origin));

    g.v(s).velocity = Vec3::ZERO;
    g.v(s).touch = Some(SUB_Null);
    g.setmodel(s, "progs/s_explod.spr");
    g.v(s).solid = SOLID_NOT;
    s_explode1(g);
}

pub fn OgreGrenadeTouch(g: &mut Game) {
    let s = g.self_;
    if g.other == g.r(s).owner {
        return; // don't explode on owner
    }

    if g.r(g.other).takedamage == DAMAGE_AIM {
        OgreGrenadeExplode(g);
        return;
    }

    g.sound(s, CHAN_VOICE, "weapons/bounce.wav", 1.0, ATTN_NORM); // bounce sound

    if g.r(s).velocity == Vec3::ZERO {
        g.v(s).avelocity = Vec3::ZERO;
    }
}

pub fn OgreFireGrenade(g: &mut Game) {
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;

    g.sound(s, CHAN_WEAPON, "weapons/grenade.wav", 1.0, ATTN_NORM);

    let missile = g.spawn();
    g.v(missile).owner = s;
    g.v(missile).movetype = MOVETYPE_BOUNCE;
    g.v(missile).solid = SOLID_BBOX;

    // set missile speed
    g.makevectors(g.r(s).angles);

    let enemy = g.r(s).enemy;
    g.v(missile).velocity = normalize(g.r(enemy).origin - g.r(s).origin);
    g.v(missile).velocity = g.r(missile).velocity * 600.0;
    g.v(missile).velocity.z = 200.0;

    g.v(missile).avelocity = Vec3::new(300.0, 300.0, 300.0);

    g.v(missile).angles = vectoangles(g.r(missile).velocity);

    g.v(missile).touch = Some(OgreGrenadeTouch);

    // set missile duration
    g.v(missile).nextthink = g.time + 2.5;
    g.v(missile).think = Some(OgreGrenadeExplode);

    g.setmodel(missile, "progs/grenade.mdl");
    g.setsize(missile, Vec3::ZERO, Vec3::ZERO);
    g.setorigin(missile, g.r(s).origin);
}

pub fn chainsaw(g: &mut Game, side: f32) {
    let s = g.self_;
    if g.r(s).enemy == world {
        return;
    }
    let enemy = g.r(s).enemy;
    if CanDamage(g, enemy, s) == 0.0 {
        return;
    }

    ai_charge(g, 10.0);

    let s = g.self_;
    let delta = g.r(g.r(s).enemy).origin - g.r(s).origin;
    if vlen(delta) > 100.0 {
        return;
    }

    let ldmg = g.random_sum3_as_compiled() * 4.0;
    let enemy = g.r(s).enemy;
    T_Damage(g, enemy, s, s, ldmg);

    if side != 0.0 {
        let s = g.self_;
        g.makevectors(g.r(s).angles);
        if side == 1.0 {
            let org = g.r(s).origin + g.v_forward * 16.0;
            let vel = crandom(g) * 100.0 * g.v_right;
            SpawnMeatSpray(g, org, vel);
        } else {
            let org = g.r(s).origin + g.v_forward * 16.0;
            let vel = side * g.v_right;
            SpawnMeatSpray(g, org, vel);
        }
    }
}

pub fn ogre_stand1(g: &mut Game) {
    g.frame_state(F_stand1, ogre_stand2);
    ai_stand(g);
}

pub fn ogre_stand2(g: &mut Game) {
    g.frame_state(F_stand2, ogre_stand3);
    ai_stand(g);
}

pub fn ogre_stand3(g: &mut Game) {
    g.frame_state(F_stand3, ogre_stand4);
    ai_stand(g);
}

pub fn ogre_stand4(g: &mut Game) {
    g.frame_state(F_stand4, ogre_stand5);
    ai_stand(g);
}

pub fn ogre_stand5(g: &mut Game) {
    g.frame_state(F_stand5, ogre_stand6);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "ogre/ogidle.wav", 1.0, ATTN_IDLE);
    }
    ai_stand(g);
}

pub fn ogre_stand6(g: &mut Game) {
    g.frame_state(F_stand6, ogre_stand7);
    ai_stand(g);
}

pub fn ogre_stand7(g: &mut Game) {
    g.frame_state(F_stand7, ogre_stand8);
    ai_stand(g);
}

pub fn ogre_stand8(g: &mut Game) {
    g.frame_state(F_stand8, ogre_stand9);
    ai_stand(g);
}

pub fn ogre_stand9(g: &mut Game) {
    g.frame_state(F_stand9, ogre_stand1);
    ai_stand(g);
}

pub fn ogre_walk1(g: &mut Game) {
    g.frame_state(F_walk1, ogre_walk2);
    ai_walk(g, 3.0);
}

pub fn ogre_walk2(g: &mut Game) {
    g.frame_state(F_walk2, ogre_walk3);
    ai_walk(g, 2.0);
}

pub fn ogre_walk3(g: &mut Game) {
    g.frame_state(F_walk3, ogre_walk4);
    ai_walk(g, 2.0);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "ogre/ogidle.wav", 1.0, ATTN_IDLE);
    }
}

pub fn ogre_walk4(g: &mut Game) {
    g.frame_state(F_walk4, ogre_walk5);
    ai_walk(g, 2.0);
}

pub fn ogre_walk5(g: &mut Game) {
    g.frame_state(F_walk5, ogre_walk6);
    ai_walk(g, 2.0);
}

pub fn ogre_walk6(g: &mut Game) {
    g.frame_state(F_walk6, ogre_walk7);
    ai_walk(g, 5.0);
    if g.random() < 0.1 {
        g.sound(g.self_, CHAN_VOICE, "ogre/ogdrag.wav", 1.0, ATTN_IDLE);
    }
}

pub fn ogre_walk7(g: &mut Game) {
    g.frame_state(F_walk7, ogre_walk8);
    ai_walk(g, 3.0);
}

pub fn ogre_walk8(g: &mut Game) {
    g.frame_state(F_walk8, ogre_walk9);
    ai_walk(g, 2.0);
}

pub fn ogre_walk9(g: &mut Game) {
    g.frame_state(F_walk9, ogre_walk10);
    ai_walk(g, 3.0);
}

pub fn ogre_walk10(g: &mut Game) {
    g.frame_state(F_walk10, ogre_walk11);
    ai_walk(g, 1.0);
}

pub fn ogre_walk11(g: &mut Game) {
    g.frame_state(F_walk11, ogre_walk12);
    ai_walk(g, 2.0);
}

pub fn ogre_walk12(g: &mut Game) {
    g.frame_state(F_walk12, ogre_walk13);
    ai_walk(g, 3.0);
}

pub fn ogre_walk13(g: &mut Game) {
    g.frame_state(F_walk13, ogre_walk14);
    ai_walk(g, 3.0);
}

pub fn ogre_walk14(g: &mut Game) {
    g.frame_state(F_walk14, ogre_walk15);
    ai_walk(g, 3.0);
}

pub fn ogre_walk15(g: &mut Game) {
    g.frame_state(F_walk15, ogre_walk16);
    ai_walk(g, 3.0);
}

pub fn ogre_walk16(g: &mut Game) {
    g.frame_state(F_walk16, ogre_walk1);
    ai_walk(g, 4.0);
}

pub fn ogre_run1(g: &mut Game) {
    g.frame_state(F_run1, ogre_run2);
    ai_run(g, 9.0);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "ogre/ogidle2.wav", 1.0, ATTN_IDLE);
    }
}

pub fn ogre_run2(g: &mut Game) {
    g.frame_state(F_run2, ogre_run3);
    ai_run(g, 12.0);
}

pub fn ogre_run3(g: &mut Game) {
    g.frame_state(F_run3, ogre_run4);
    ai_run(g, 8.0);
}

pub fn ogre_run4(g: &mut Game) {
    g.frame_state(F_run4, ogre_run5);
    ai_run(g, 22.0);
}

pub fn ogre_run5(g: &mut Game) {
    g.frame_state(F_run5, ogre_run6);
    ai_run(g, 16.0);
}

pub fn ogre_run6(g: &mut Game) {
    g.frame_state(F_run6, ogre_run7);
    ai_run(g, 4.0);
}

pub fn ogre_run7(g: &mut Game) {
    g.frame_state(F_run7, ogre_run8);
    ai_run(g, 13.0);
}

pub fn ogre_run8(g: &mut Game) {
    g.frame_state(F_run8, ogre_run1);
    ai_run(g, 24.0);
}

pub fn ogre_swing1(g: &mut Game) {
    g.frame_state(F_swing1, ogre_swing2);
    ai_charge(g, 11.0);
    g.sound(g.self_, CHAN_WEAPON, "ogre/ogsawatk.wav", 1.0, ATTN_NORM);
}

pub fn ogre_swing2(g: &mut Game) {
    g.frame_state(F_swing2, ogre_swing3);
    ai_charge(g, 1.0);
}

pub fn ogre_swing3(g: &mut Game) {
    g.frame_state(F_swing3, ogre_swing4);
    ai_charge(g, 4.0);
}

pub fn ogre_swing4(g: &mut Game) {
    g.frame_state(F_swing4, ogre_swing5);
    ai_charge(g, 13.0);
}

pub fn ogre_swing5(g: &mut Game) {
    g.frame_state(F_swing5, ogre_swing6);
    ai_charge(g, 9.0);
    chainsaw(g, 0.0);
    let s = g.self_;
    let r = g.random();
    g.v(s).angles.y = g.r(s).angles.y + r * 25.0;
}

pub fn ogre_swing6(g: &mut Game) {
    g.frame_state(F_swing6, ogre_swing7);
    chainsaw(g, 200.0);
    let s = g.self_;
    let r = g.random();
    g.v(s).angles.y = g.r(s).angles.y + r * 25.0;
}

pub fn ogre_swing7(g: &mut Game) {
    g.frame_state(F_swing7, ogre_swing8);
    chainsaw(g, 0.0);
    let s = g.self_;
    let r = g.random();
    g.v(s).angles.y = g.r(s).angles.y + r * 25.0;
}

pub fn ogre_swing8(g: &mut Game) {
    g.frame_state(F_swing8, ogre_swing9);
    chainsaw(g, 0.0);
    let s = g.self_;
    let r = g.random();
    g.v(s).angles.y = g.r(s).angles.y + r * 25.0;
}

pub fn ogre_swing9(g: &mut Game) {
    g.frame_state(F_swing9, ogre_swing10);
    chainsaw(g, 0.0);
    let s = g.self_;
    let r = g.random();
    g.v(s).angles.y = g.r(s).angles.y + r * 25.0;
}

pub fn ogre_swing10(g: &mut Game) {
    g.frame_state(F_swing10, ogre_swing11);
    chainsaw(g, -200.0);
    let s = g.self_;
    let r = g.random();
    g.v(s).angles.y = g.r(s).angles.y + r * 25.0;
}

pub fn ogre_swing11(g: &mut Game) {
    g.frame_state(F_swing11, ogre_swing12);
    chainsaw(g, 0.0);
    let s = g.self_;
    let r = g.random();
    g.v(s).angles.y = g.r(s).angles.y + r * 25.0;
}

pub fn ogre_swing12(g: &mut Game) {
    g.frame_state(F_swing12, ogre_swing13);
    ai_charge(g, 3.0);
}

pub fn ogre_swing13(g: &mut Game) {
    g.frame_state(F_swing13, ogre_swing14);
    ai_charge(g, 8.0);
}

pub fn ogre_swing14(g: &mut Game) {
    g.frame_state(F_swing14, ogre_run1);
    ai_charge(g, 9.0);
}

pub fn ogre_smash1(g: &mut Game) {
    g.frame_state(F_smash1, ogre_smash2);
    ai_charge(g, 6.0);
    g.sound(g.self_, CHAN_WEAPON, "ogre/ogsawatk.wav", 1.0, ATTN_NORM);
}

pub fn ogre_smash2(g: &mut Game) {
    g.frame_state(F_smash2, ogre_smash3);
    ai_charge(g, 0.0);
}

pub fn ogre_smash3(g: &mut Game) {
    g.frame_state(F_smash3, ogre_smash4);
    ai_charge(g, 0.0);
}

pub fn ogre_smash4(g: &mut Game) {
    g.frame_state(F_smash4, ogre_smash5);
    ai_charge(g, 1.0);
}

pub fn ogre_smash5(g: &mut Game) {
    g.frame_state(F_smash5, ogre_smash6);
    ai_charge(g, 4.0);
}

pub fn ogre_smash6(g: &mut Game) {
    g.frame_state(F_smash6, ogre_smash7);
    ai_charge(g, 4.0);
    chainsaw(g, 0.0);
}

pub fn ogre_smash7(g: &mut Game) {
    g.frame_state(F_smash7, ogre_smash8);
    ai_charge(g, 4.0);
    chainsaw(g, 0.0);
}

pub fn ogre_smash8(g: &mut Game) {
    g.frame_state(F_smash8, ogre_smash9);
    ai_charge(g, 10.0);
    chainsaw(g, 0.0);
}

pub fn ogre_smash9(g: &mut Game) {
    g.frame_state(F_smash9, ogre_smash10);
    ai_charge(g, 13.0);
    chainsaw(g, 0.0);
}

pub fn ogre_smash10(g: &mut Game) {
    g.frame_state(F_smash10, ogre_smash11);
    chainsaw(g, 1.0);
}

pub fn ogre_smash11(g: &mut Game) {
    g.frame_state(F_smash11, ogre_smash12);
    ai_charge(g, 2.0);
    chainsaw(g, 0.0);
    let s = g.self_;
    let r = g.random();
    g.v(s).nextthink = g.r(s).nextthink + r * 0.2; // slight variation
}

pub fn ogre_smash12(g: &mut Game) {
    g.frame_state(F_smash12, ogre_smash13);
    // id calls `ai_charge()` with no argument here, so the original moves by whatever the
    // parameter slot last held; nought is the honest stand-in.
    ai_charge(g, 0.0);
}

pub fn ogre_smash13(g: &mut Game) {
    g.frame_state(F_smash13, ogre_smash14);
    ai_charge(g, 4.0);
}

pub fn ogre_smash14(g: &mut Game) {
    g.frame_state(F_smash14, ogre_run1);
    ai_charge(g, 12.0);
}

pub fn ogre_nail1(g: &mut Game) {
    g.frame_state(F_shoot1, ogre_nail2);
    ai_face(g);
}

pub fn ogre_nail2(g: &mut Game) {
    g.frame_state(F_shoot2, ogre_nail3);
    ai_face(g);
}

pub fn ogre_nail3(g: &mut Game) {
    g.frame_state(F_shoot2, ogre_nail4);
    ai_face(g);
}

pub fn ogre_nail4(g: &mut Game) {
    g.frame_state(F_shoot3, ogre_nail5);
    ai_face(g);
    OgreFireGrenade(g);
}

pub fn ogre_nail5(g: &mut Game) {
    g.frame_state(F_shoot4, ogre_nail6);
    ai_face(g);
}

pub fn ogre_nail6(g: &mut Game) {
    g.frame_state(F_shoot5, ogre_nail7);
    ai_face(g);
}

pub fn ogre_nail7(g: &mut Game) {
    g.frame_state(F_shoot6, ogre_run1);
    ai_face(g);
}

pub fn ogre_pain1(g: &mut Game) {
    g.frame_state(F_pain1, ogre_pain2);
}

pub fn ogre_pain2(g: &mut Game) {
    g.frame_state(F_pain2, ogre_pain3);
}

pub fn ogre_pain3(g: &mut Game) {
    g.frame_state(F_pain3, ogre_pain4);
}

pub fn ogre_pain4(g: &mut Game) {
    g.frame_state(F_pain4, ogre_pain5);
}

pub fn ogre_pain5(g: &mut Game) {
    g.frame_state(F_pain5, ogre_run1);
}

pub fn ogre_painb1(g: &mut Game) {
    g.frame_state(F_painb1, ogre_painb2);
}

pub fn ogre_painb2(g: &mut Game) {
    g.frame_state(F_painb2, ogre_painb3);
}

pub fn ogre_painb3(g: &mut Game) {
    g.frame_state(F_painb3, ogre_run1);
}

pub fn ogre_painc1(g: &mut Game) {
    g.frame_state(F_painc1, ogre_painc2);
}

pub fn ogre_painc2(g: &mut Game) {
    g.frame_state(F_painc2, ogre_painc3);
}

pub fn ogre_painc3(g: &mut Game) {
    g.frame_state(F_painc3, ogre_painc4);
}

pub fn ogre_painc4(g: &mut Game) {
    g.frame_state(F_painc4, ogre_painc5);
}

pub fn ogre_painc5(g: &mut Game) {
    g.frame_state(F_painc5, ogre_painc6);
}

pub fn ogre_painc6(g: &mut Game) {
    g.frame_state(F_painc6, ogre_run1);
}

pub fn ogre_paind1(g: &mut Game) {
    g.frame_state(F_paind1, ogre_paind2);
}

pub fn ogre_paind2(g: &mut Game) {
    g.frame_state(F_paind2, ogre_paind3);
    ai_pain(g, 10.0);
}

pub fn ogre_paind3(g: &mut Game) {
    g.frame_state(F_paind3, ogre_paind4);
    ai_pain(g, 9.0);
}

pub fn ogre_paind4(g: &mut Game) {
    g.frame_state(F_paind4, ogre_paind5);
    ai_pain(g, 4.0);
}

pub fn ogre_paind5(g: &mut Game) {
    g.frame_state(F_paind5, ogre_paind6);
}

pub fn ogre_paind6(g: &mut Game) {
    g.frame_state(F_paind6, ogre_paind7);
}

pub fn ogre_paind7(g: &mut Game) {
    g.frame_state(F_paind7, ogre_paind8);
}

pub fn ogre_paind8(g: &mut Game) {
    g.frame_state(F_paind8, ogre_paind9);
}

pub fn ogre_paind9(g: &mut Game) {
    g.frame_state(F_paind9, ogre_paind10);
}

pub fn ogre_paind10(g: &mut Game) {
    g.frame_state(F_paind10, ogre_paind11);
}

pub fn ogre_paind11(g: &mut Game) {
    g.frame_state(F_paind11, ogre_paind12);
}

pub fn ogre_paind12(g: &mut Game) {
    g.frame_state(F_paind12, ogre_paind13);
}

pub fn ogre_paind13(g: &mut Game) {
    g.frame_state(F_paind13, ogre_paind14);
}

pub fn ogre_paind14(g: &mut Game) {
    g.frame_state(F_paind14, ogre_paind15);
}

pub fn ogre_paind15(g: &mut Game) {
    g.frame_state(F_paind15, ogre_paind16);
}

pub fn ogre_paind16(g: &mut Game) {
    g.frame_state(F_paind16, ogre_run1);
}

pub fn ogre_paine1(g: &mut Game) {
    g.frame_state(F_paine1, ogre_paine2);
}

pub fn ogre_paine2(g: &mut Game) {
    g.frame_state(F_paine2, ogre_paine3);
    ai_pain(g, 10.0);
}

pub fn ogre_paine3(g: &mut Game) {
    g.frame_state(F_paine3, ogre_paine4);
    ai_pain(g, 9.0);
}

pub fn ogre_paine4(g: &mut Game) {
    g.frame_state(F_paine4, ogre_paine5);
    ai_pain(g, 4.0);
}

pub fn ogre_paine5(g: &mut Game) {
    g.frame_state(F_paine5, ogre_paine6);
}

pub fn ogre_paine6(g: &mut Game) {
    g.frame_state(F_paine6, ogre_paine7);
}

pub fn ogre_paine7(g: &mut Game) {
    g.frame_state(F_paine7, ogre_paine8);
}

pub fn ogre_paine8(g: &mut Game) {
    g.frame_state(F_paine8, ogre_paine9);
}

pub fn ogre_paine9(g: &mut Game) {
    g.frame_state(F_paine9, ogre_paine10);
}

pub fn ogre_paine10(g: &mut Game) {
    g.frame_state(F_paine10, ogre_paine11);
}

pub fn ogre_paine11(g: &mut Game) {
    g.frame_state(F_paine11, ogre_paine12);
}

pub fn ogre_paine12(g: &mut Game) {
    g.frame_state(F_paine12, ogre_paine13);
}

pub fn ogre_paine13(g: &mut Game) {
    g.frame_state(F_paine13, ogre_paine14);
}

pub fn ogre_paine14(g: &mut Game) {
    g.frame_state(F_paine14, ogre_paine15);
}

pub fn ogre_paine15(g: &mut Game) {
    g.frame_state(F_paine15, ogre_run1);
}

pub fn ogre_pain(g: &mut Game, attacker: Ent, damage: f32) {
    // don't make multiple pain sounds right after each other
    if g.r(g.self_).pain_finished > g.time {
        return;
    }

    g.sound(g.self_, CHAN_VOICE, "ogre/ogpain1.wav", 1.0, ATTN_NORM);

    let r = g.random();

    if r < 0.25 {
        ogre_pain1(g);
        g.v(g.self_).pain_finished = g.time + 1.0;
    } else if r < 0.5 {
        ogre_painb1(g);
        g.v(g.self_).pain_finished = g.time + 1.0;
    } else if r < 0.75 {
        ogre_painc1(g);
        g.v(g.self_).pain_finished = g.time + 1.0;
    } else if r < 0.88 {
        ogre_paind1(g);
        g.v(g.self_).pain_finished = g.time + 2.0;
    } else {
        ogre_paine1(g);
        g.v(g.self_).pain_finished = g.time + 2.0;
    }
}

pub fn ogre_die1(g: &mut Game) {
    g.frame_state(F_death1, ogre_die2);
}

pub fn ogre_die2(g: &mut Game) {
    g.frame_state(F_death2, ogre_die3);
}

pub fn ogre_die3(g: &mut Game) {
    g.frame_state(F_death3, ogre_die4);
    g.v(g.self_).solid = SOLID_NOT;
    g.v(g.self_).ammo_rockets = 2.0;
    DropBackpack(g);
}

pub fn ogre_die4(g: &mut Game) {
    g.frame_state(F_death4, ogre_die5);
}

pub fn ogre_die5(g: &mut Game) {
    g.frame_state(F_death5, ogre_die6);
}

pub fn ogre_die6(g: &mut Game) {
    g.frame_state(F_death6, ogre_die7);
}

pub fn ogre_die7(g: &mut Game) {
    g.frame_state(F_death7, ogre_die8);
}

pub fn ogre_die8(g: &mut Game) {
    g.frame_state(F_death8, ogre_die9);
}

pub fn ogre_die9(g: &mut Game) {
    g.frame_state(F_death9, ogre_die10);
}

pub fn ogre_die10(g: &mut Game) {
    g.frame_state(F_death10, ogre_die11);
}

pub fn ogre_die11(g: &mut Game) {
    g.frame_state(F_death11, ogre_die12);
}

pub fn ogre_die12(g: &mut Game) {
    g.frame_state(F_death12, ogre_die13);
}

pub fn ogre_die13(g: &mut Game) {
    g.frame_state(F_death13, ogre_die14);
}

pub fn ogre_die14(g: &mut Game) {
    g.frame_state(F_death14, ogre_die14);
}

pub fn ogre_bdie1(g: &mut Game) {
    g.frame_state(F_bdeath1, ogre_bdie2);
}

pub fn ogre_bdie2(g: &mut Game) {
    g.frame_state(F_bdeath2, ogre_bdie3);
    ai_forward(g, 5.0);
}

pub fn ogre_bdie3(g: &mut Game) {
    g.frame_state(F_bdeath3, ogre_bdie4);
    g.v(g.self_).solid = SOLID_NOT;
    g.v(g.self_).ammo_rockets = 2.0;
    DropBackpack(g);
}

pub fn ogre_bdie4(g: &mut Game) {
    g.frame_state(F_bdeath4, ogre_bdie5);
    ai_forward(g, 1.0);
}

pub fn ogre_bdie5(g: &mut Game) {
    g.frame_state(F_bdeath5, ogre_bdie6);
    ai_forward(g, 3.0);
}

pub fn ogre_bdie6(g: &mut Game) {
    g.frame_state(F_bdeath6, ogre_bdie7);
    ai_forward(g, 7.0);
}

pub fn ogre_bdie7(g: &mut Game) {
    g.frame_state(F_bdeath7, ogre_bdie8);
    ai_forward(g, 25.0);
}

pub fn ogre_bdie8(g: &mut Game) {
    g.frame_state(F_bdeath8, ogre_bdie9);
}

pub fn ogre_bdie9(g: &mut Game) {
    g.frame_state(F_bdeath9, ogre_bdie10);
}

pub fn ogre_bdie10(g: &mut Game) {
    g.frame_state(F_bdeath10, ogre_bdie10);
}

pub fn ogre_die(g: &mut Game) {
    // check for gib
    if g.r(g.self_).health < -80.0 {
        g.sound(g.self_, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        let h = g.r(g.self_).health;
        ThrowHead(g, "progs/h_ogre.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib3.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib3.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib3.mdl", h);
        return;
    }

    g.sound(g.self_, CHAN_VOICE, "ogre/ogdth.wav", 1.0, ATTN_NORM);

    if g.random() < 0.5 {
        ogre_die1(g);
    } else {
        ogre_bdie1(g);
    }
}

pub fn ogre_melee(g: &mut Game) {
    if g.random() > 0.5 {
        ogre_smash1(g);
    } else {
        ogre_swing1(g);
    }
}

pub fn monster_ogre(g: &mut Game) {
    if g.deathmatch != 0.0 {
        g.remove(g.self_);
        return;
    }

    g.precache_model("progs/ogre.mdl");
    g.precache_model("progs/h_ogre.mdl");
    g.precache_model("progs/grenade.mdl");

    g.precache_sound("ogre/ogdrag.wav");
    g.precache_sound("ogre/ogdth.wav");
    g.precache_sound("ogre/ogidle.wav");
    g.precache_sound("ogre/ogidle2.wav");
    g.precache_sound("ogre/ogpain1.wav");
    g.precache_sound("ogre/ogsawatk.wav");
    g.precache_sound("ogre/ogwake.wav");

    let s = g.self_;
    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/ogre.mdl");

    g.setsize(s, VEC_HULL2_MIN, VEC_HULL2_MAX);
    g.v(s).health = 200.0;

    g.v(s).th_stand = Some(ogre_stand1);
    g.v(s).th_walk = Some(ogre_walk1);
    g.v(s).th_run = Some(ogre_run1);
    g.v(s).th_die = Some(ogre_die);
    g.v(s).th_melee = Some(ogre_melee);
    g.v(s).th_missile = Some(ogre_nail1);
    g.v(s).th_pain = Some(ogre_pain);

    walkmonster_start(g);
}

pub fn monster_ogre_marksman(g: &mut Game) {
    monster_ogre(g);
}
