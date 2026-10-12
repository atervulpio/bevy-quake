//! `shambler.qc`, id's QuakeC 1.06, in Rust.

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
const F_stand16: f32 = 15.0;
const F_stand17: f32 = 16.0;
const F_walk1: f32 = 17.0;
const F_walk2: f32 = 18.0;
const F_walk3: f32 = 19.0;
const F_walk4: f32 = 20.0;
const F_walk5: f32 = 21.0;
const F_walk6: f32 = 22.0;
const F_walk7: f32 = 23.0;
const F_walk8: f32 = 24.0;
const F_walk9: f32 = 25.0;
const F_walk10: f32 = 26.0;
const F_walk11: f32 = 27.0;
const F_walk12: f32 = 28.0;
const F_run1: f32 = 29.0;
const F_run2: f32 = 30.0;
const F_run3: f32 = 31.0;
const F_run4: f32 = 32.0;
const F_run5: f32 = 33.0;
const F_run6: f32 = 34.0;
const F_smash1: f32 = 35.0;
const F_smash2: f32 = 36.0;
const F_smash3: f32 = 37.0;
const F_smash4: f32 = 38.0;
const F_smash5: f32 = 39.0;
const F_smash6: f32 = 40.0;
const F_smash7: f32 = 41.0;
const F_smash8: f32 = 42.0;
const F_smash9: f32 = 43.0;
const F_smash10: f32 = 44.0;
const F_smash11: f32 = 45.0;
const F_smash12: f32 = 46.0;
const F_swingr1: f32 = 47.0;
const F_swingr2: f32 = 48.0;
const F_swingr3: f32 = 49.0;
const F_swingr4: f32 = 50.0;
const F_swingr5: f32 = 51.0;
const F_swingr6: f32 = 52.0;
const F_swingr7: f32 = 53.0;
const F_swingr8: f32 = 54.0;
const F_swingr9: f32 = 55.0;
const F_swingl1: f32 = 56.0;
const F_swingl2: f32 = 57.0;
const F_swingl3: f32 = 58.0;
const F_swingl4: f32 = 59.0;
const F_swingl5: f32 = 60.0;
const F_swingl6: f32 = 61.0;
const F_swingl7: f32 = 62.0;
const F_swingl8: f32 = 63.0;
const F_swingl9: f32 = 64.0;
const F_magic1: f32 = 65.0;
const F_magic2: f32 = 66.0;
const F_magic3: f32 = 67.0;
const F_magic4: f32 = 68.0;
const F_magic5: f32 = 69.0;
const F_magic6: f32 = 70.0;
const F_magic9: f32 = 73.0;
const F_magic10: f32 = 74.0;
const F_magic11: f32 = 75.0;
const F_magic12: f32 = 76.0;
const F_pain1: f32 = 77.0;
const F_pain2: f32 = 78.0;
const F_pain3: f32 = 79.0;
const F_pain4: f32 = 80.0;
const F_pain5: f32 = 81.0;
const F_pain6: f32 = 82.0;
const F_death1: f32 = 83.0;
const F_death2: f32 = 84.0;
const F_death3: f32 = 85.0;
const F_death4: f32 = 86.0;
const F_death5: f32 = 87.0;
const F_death6: f32 = 88.0;
const F_death7: f32 = 89.0;
const F_death8: f32 = 90.0;
const F_death9: f32 = 91.0;
const F_death10: f32 = 92.0;
const F_death11: f32 = 93.0;


pub fn sham_stand1(g: &mut Game) {
    g.frame_state(F_stand1, sham_stand2);
    ai_stand(g);
}

pub fn sham_stand2(g: &mut Game) {
    g.frame_state(F_stand2, sham_stand3);
    ai_stand(g);
}

pub fn sham_stand3(g: &mut Game) {
    g.frame_state(F_stand3, sham_stand4);
    ai_stand(g);
}

pub fn sham_stand4(g: &mut Game) {
    g.frame_state(F_stand4, sham_stand5);
    ai_stand(g);
}

pub fn sham_stand5(g: &mut Game) {
    g.frame_state(F_stand5, sham_stand6);
    ai_stand(g);
}

pub fn sham_stand6(g: &mut Game) {
    g.frame_state(F_stand6, sham_stand7);
    ai_stand(g);
}

pub fn sham_stand7(g: &mut Game) {
    g.frame_state(F_stand7, sham_stand8);
    ai_stand(g);
}

pub fn sham_stand8(g: &mut Game) {
    g.frame_state(F_stand8, sham_stand9);
    ai_stand(g);
}

pub fn sham_stand9(g: &mut Game) {
    g.frame_state(F_stand9, sham_stand10);
    ai_stand(g);
}

pub fn sham_stand10(g: &mut Game) {
    g.frame_state(F_stand10, sham_stand11);
    ai_stand(g);
}

pub fn sham_stand11(g: &mut Game) {
    g.frame_state(F_stand11, sham_stand12);
    ai_stand(g);
}

pub fn sham_stand12(g: &mut Game) {
    g.frame_state(F_stand12, sham_stand13);
    ai_stand(g);
}

pub fn sham_stand13(g: &mut Game) {
    g.frame_state(F_stand13, sham_stand14);
    ai_stand(g);
}

pub fn sham_stand14(g: &mut Game) {
    g.frame_state(F_stand14, sham_stand15);
    ai_stand(g);
}

pub fn sham_stand15(g: &mut Game) {
    g.frame_state(F_stand15, sham_stand16);
    ai_stand(g);
}

pub fn sham_stand16(g: &mut Game) {
    g.frame_state(F_stand16, sham_stand17);
    ai_stand(g);
}

pub fn sham_stand17(g: &mut Game) {
    g.frame_state(F_stand17, sham_stand1);
    ai_stand(g);
}

pub fn sham_walk1(g: &mut Game) {
    g.frame_state(F_walk1, sham_walk2);
    ai_walk(g, 10.0);
}

pub fn sham_walk2(g: &mut Game) {
    g.frame_state(F_walk2, sham_walk3);
    ai_walk(g, 9.0);
}

pub fn sham_walk3(g: &mut Game) {
    g.frame_state(F_walk3, sham_walk4);
    ai_walk(g, 9.0);
}

pub fn sham_walk4(g: &mut Game) {
    g.frame_state(F_walk4, sham_walk5);
    ai_walk(g, 5.0);
}

pub fn sham_walk5(g: &mut Game) {
    g.frame_state(F_walk5, sham_walk6);
    ai_walk(g, 6.0);
}

pub fn sham_walk6(g: &mut Game) {
    g.frame_state(F_walk6, sham_walk7);
    ai_walk(g, 12.0);
}

pub fn sham_walk7(g: &mut Game) {
    g.frame_state(F_walk7, sham_walk8);
    ai_walk(g, 8.0);
}

pub fn sham_walk8(g: &mut Game) {
    g.frame_state(F_walk8, sham_walk9);
    ai_walk(g, 3.0);
}

pub fn sham_walk9(g: &mut Game) {
    g.frame_state(F_walk9, sham_walk10);
    ai_walk(g, 13.0);
}

pub fn sham_walk10(g: &mut Game) {
    g.frame_state(F_walk10, sham_walk11);
    ai_walk(g, 9.0);
}

pub fn sham_walk11(g: &mut Game) {
    g.frame_state(F_walk11, sham_walk12);
    ai_walk(g, 7.0);
}

pub fn sham_walk12(g: &mut Game) {
    g.frame_state(F_walk12, sham_walk1);
    ai_walk(g, 7.0);
    if g.random() > 0.8 {
        g.sound(g.self_, CHAN_VOICE, "shambler/sidle.wav", 1.0, ATTN_IDLE);
    }
}

pub fn sham_run1(g: &mut Game) {
    g.frame_state(F_run1, sham_run2);
    ai_run(g, 20.0);
}

pub fn sham_run2(g: &mut Game) {
    g.frame_state(F_run2, sham_run3);
    ai_run(g, 24.0);
}

pub fn sham_run3(g: &mut Game) {
    g.frame_state(F_run3, sham_run4);
    ai_run(g, 20.0);
}

pub fn sham_run4(g: &mut Game) {
    g.frame_state(F_run4, sham_run5);
    ai_run(g, 20.0);
}

pub fn sham_run5(g: &mut Game) {
    g.frame_state(F_run5, sham_run6);
    ai_run(g, 24.0);
}

pub fn sham_run6(g: &mut Game) {
    g.frame_state(F_run6, sham_run1);
    ai_run(g, 20.0);
    if g.random() > 0.8 {
        g.sound(g.self_, CHAN_VOICE, "shambler/sidle.wav", 1.0, ATTN_IDLE);
    }
}

pub fn sham_smash1(g: &mut Game) {
    g.frame_state(F_smash1, sham_smash2);
    g.sound(g.self_, CHAN_VOICE, "shambler/melee1.wav", 1.0, ATTN_NORM);
    ai_charge(g, 2.0);
}

pub fn sham_smash2(g: &mut Game) {
    g.frame_state(F_smash2, sham_smash3);
    ai_charge(g, 6.0);
}

pub fn sham_smash3(g: &mut Game) {
    g.frame_state(F_smash3, sham_smash4);
    ai_charge(g, 6.0);
}

pub fn sham_smash4(g: &mut Game) {
    g.frame_state(F_smash4, sham_smash5);
    ai_charge(g, 5.0);
}

pub fn sham_smash5(g: &mut Game) {
    g.frame_state(F_smash5, sham_smash6);
    ai_charge(g, 4.0);
}

pub fn sham_smash6(g: &mut Game) {
    g.frame_state(F_smash6, sham_smash7);
    ai_charge(g, 1.0);
}

pub fn sham_smash7(g: &mut Game) {
    g.frame_state(F_smash7, sham_smash8);
    ai_charge(g, 0.0);
}

pub fn sham_smash8(g: &mut Game) {
    g.frame_state(F_smash8, sham_smash9);
    ai_charge(g, 0.0);
}

pub fn sham_smash9(g: &mut Game) {
    g.frame_state(F_smash9, sham_smash10);
    ai_charge(g, 0.0);
}

pub fn sham_smash10(g: &mut Game) {
    g.frame_state(F_smash10, sham_smash11);
    let s = g.self_;
    if g.r(s).enemy == world {
        return;
    }

    ai_charge(g, 0.0);

    let s = g.self_;
    let delta = g.r(g.r(s).enemy).origin - g.r(s).origin;
    if vlen(delta) > 100.0 {
        return;
    }

    let enemy = g.r(s).enemy;
    if CanDamage(g, enemy, s) == 0.0 {
        return;
    }

    let s = g.self_;
    let ldmg = g.random_sum3_as_compiled() * 40.0;
    let enemy = g.r(s).enemy;
    T_Damage(g, enemy, s, s, ldmg);
    let s = g.self_;
    g.sound(s, CHAN_VOICE, "shambler/smack.wav", 1.0, ATTN_NORM);

    // No makevectors here in id's code: v_forward and v_right are what the last one left.
    let org = g.r(s).origin + g.v_forward * 16.0;
    let vel = crandom(g) * 100.0 * g.v_right;
    SpawnMeatSpray(g, org, vel);
    let s = g.self_;
    let org = g.r(s).origin + g.v_forward * 16.0;
    let vel = crandom(g) * 100.0 * g.v_right;
    SpawnMeatSpray(g, org, vel);
}

pub fn sham_smash11(g: &mut Game) {
    g.frame_state(F_smash11, sham_smash12);
    ai_charge(g, 5.0);
}

pub fn sham_smash12(g: &mut Game) {
    g.frame_state(F_smash12, sham_run1);
    ai_charge(g, 4.0);
}

pub fn ShamClaw(g: &mut Game, side: f32) {
    let s = g.self_;
    if g.r(s).enemy == world {
        return;
    }

    ai_charge(g, 10.0);

    let s = g.self_;
    let delta = g.r(g.r(s).enemy).origin - g.r(s).origin;
    if vlen(delta) > 100.0 {
        return;
    }

    let ldmg = g.random_sum3_as_compiled() * 20.0;
    let enemy = g.r(s).enemy;
    T_Damage(g, enemy, s, s, ldmg);
    let s = g.self_;
    g.sound(s, CHAN_VOICE, "shambler/smack.wav", 1.0, ATTN_NORM);

    if side != 0.0 {
        g.makevectors(g.r(s).angles);
        let org = g.r(s).origin + g.v_forward * 16.0;
        let vel = side * g.v_right;
        SpawnMeatSpray(g, org, vel);
    }
}

pub fn sham_swingl1(g: &mut Game) {
    g.frame_state(F_swingl1, sham_swingl2);
    g.sound(g.self_, CHAN_VOICE, "shambler/melee2.wav", 1.0, ATTN_NORM);
    ai_charge(g, 5.0);
}

pub fn sham_swingl2(g: &mut Game) {
    g.frame_state(F_swingl2, sham_swingl3);
    ai_charge(g, 3.0);
}

pub fn sham_swingl3(g: &mut Game) {
    g.frame_state(F_swingl3, sham_swingl4);
    ai_charge(g, 7.0);
}

pub fn sham_swingl4(g: &mut Game) {
    g.frame_state(F_swingl4, sham_swingl5);
    ai_charge(g, 3.0);
}

pub fn sham_swingl5(g: &mut Game) {
    g.frame_state(F_swingl5, sham_swingl6);
    ai_charge(g, 7.0);
}

pub fn sham_swingl6(g: &mut Game) {
    g.frame_state(F_swingl6, sham_swingl7);
    ai_charge(g, 9.0);
}

pub fn sham_swingl7(g: &mut Game) {
    g.frame_state(F_swingl7, sham_swingl8);
    ai_charge(g, 5.0);
    ShamClaw(g, 250.0);
}

pub fn sham_swingl8(g: &mut Game) {
    g.frame_state(F_swingl8, sham_swingl9);
    ai_charge(g, 4.0);
}

pub fn sham_swingl9(g: &mut Game) {
    g.frame_state(F_swingl9, sham_run1);
    ai_charge(g, 8.0);
    if g.random() < 0.5 {
        g.v(g.self_).think = Some(sham_swingr1);
    }
}

pub fn sham_swingr1(g: &mut Game) {
    g.frame_state(F_swingr1, sham_swingr2);
    g.sound(g.self_, CHAN_VOICE, "shambler/melee1.wav", 1.0, ATTN_NORM);
    ai_charge(g, 1.0);
}

pub fn sham_swingr2(g: &mut Game) {
    g.frame_state(F_swingr2, sham_swingr3);
    ai_charge(g, 8.0);
}

pub fn sham_swingr3(g: &mut Game) {
    g.frame_state(F_swingr3, sham_swingr4);
    ai_charge(g, 14.0);
}

pub fn sham_swingr4(g: &mut Game) {
    g.frame_state(F_swingr4, sham_swingr5);
    ai_charge(g, 7.0);
}

pub fn sham_swingr5(g: &mut Game) {
    g.frame_state(F_swingr5, sham_swingr6);
    ai_charge(g, 3.0);
}

pub fn sham_swingr6(g: &mut Game) {
    g.frame_state(F_swingr6, sham_swingr7);
    ai_charge(g, 6.0);
}

pub fn sham_swingr7(g: &mut Game) {
    g.frame_state(F_swingr7, sham_swingr8);
    ai_charge(g, 6.0);
    ShamClaw(g, -250.0);
}

pub fn sham_swingr8(g: &mut Game) {
    g.frame_state(F_swingr8, sham_swingr9);
    ai_charge(g, 3.0);
}

pub fn sham_swingr9(g: &mut Game) {
    g.frame_state(F_swingr9, sham_run1);
    ai_charge(g, 1.0);
    ai_charge(g, 10.0);
    if g.random() < 0.5 {
        g.v(g.self_).think = Some(sham_swingl1);
    }
}

pub fn sham_melee(g: &mut Game) {
    let chance = g.random();
    // both sides evaluated, as QuakeC does
    if (chance > 0.6) | (g.r(g.self_).health == 600.0) {
        sham_smash1(g);
    } else if chance > 0.3 {
        sham_swingr1(g);
    } else {
        sham_swingl1(g);
    }
}

pub fn CastLightning(g: &mut Game) {
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;

    ai_face(g);

    let s = g.self_;
    let org = g.r(s).origin + Vec3::new(0.0, 0.0, 40.0);

    let mut dir = g.r(g.r(s).enemy).origin + Vec3::new(0.0, 0.0, 16.0) - org;
    dir = normalize(dir);

    g.traceline(org, g.r(s).origin + dir * 600.0, TRUE, s);

    g.temp_entity(TempEntity::Lightning1 { ent: s, start: org, end: g.trace_endpos });

    let end = g.trace_endpos;
    LightningDamage(g, org, end, s, 10.0);
}

pub fn sham_magic1(g: &mut Game) {
    g.frame_state(F_magic1, sham_magic2);
    ai_face(g);
    g.sound(g.self_, CHAN_WEAPON, "shambler/sattck1.wav", 1.0, ATTN_NORM);
}

pub fn sham_magic2(g: &mut Game) {
    g.frame_state(F_magic2, sham_magic3);
    ai_face(g);
}

pub fn sham_magic3(g: &mut Game) {
    g.frame_state(F_magic3, sham_magic4);
    ai_face(g);
    let s = g.self_;
    g.v(s).nextthink = g.r(s).nextthink + 0.2;

    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;
    ai_face(g);
    let s = g.self_;
    let o = g.spawn();
    g.v(s).owner = o;
    let o = g.r(s).owner;
    g.setmodel(o, "progs/s_light.mdl");
    g.setorigin(o, g.r(s).origin);
    g.v(o).angles = g.r(s).angles;
    g.v(o).nextthink = g.time + 0.7;
    g.v(o).think = Some(SUB_Remove);
}

pub fn sham_magic4(g: &mut Game) {
    g.frame_state(F_magic4, sham_magic5);
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;
    let o = g.r(s).owner;
    g.v(o).frame = 1.0;
}

pub fn sham_magic5(g: &mut Game) {
    g.frame_state(F_magic5, sham_magic6);
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;
    let o = g.r(s).owner;
    g.v(o).frame = 2.0;
}

pub fn sham_magic6(g: &mut Game) {
    g.frame_state(F_magic6, sham_magic9);
    let o = g.r(g.self_).owner;
    g.remove(o);
    CastLightning(g);
    g.sound(g.self_, CHAN_WEAPON, "shambler/sboom.wav", 1.0, ATTN_NORM);
}

pub fn sham_magic9(g: &mut Game) {
    g.frame_state(F_magic9, sham_magic10);
    CastLightning(g);
}

pub fn sham_magic10(g: &mut Game) {
    g.frame_state(F_magic10, sham_magic11);
    CastLightning(g);
}

pub fn sham_magic11(g: &mut Game) {
    g.frame_state(F_magic11, sham_magic12);
    if g.skill == 3.0 {
        CastLightning(g);
    }
}

pub fn sham_magic12(g: &mut Game) {
    g.frame_state(F_magic12, sham_run1);
}

pub fn sham_pain1(g: &mut Game) {
    g.frame_state(F_pain1, sham_pain2);
}

pub fn sham_pain2(g: &mut Game) {
    g.frame_state(F_pain2, sham_pain3);
}

pub fn sham_pain3(g: &mut Game) {
    g.frame_state(F_pain3, sham_pain4);
}

pub fn sham_pain4(g: &mut Game) {
    g.frame_state(F_pain4, sham_pain5);
}

pub fn sham_pain5(g: &mut Game) {
    g.frame_state(F_pain5, sham_pain6);
}

pub fn sham_pain6(g: &mut Game) {
    g.frame_state(F_pain6, sham_run1);
}

pub fn sham_pain(g: &mut Game, attacker: Ent, damage: f32) {
    g.sound(g.self_, CHAN_VOICE, "shambler/shurt2.wav", 1.0, ATTN_NORM);

    if g.r(g.self_).health <= 0.0 {
        return; // already dying, don't go into pain frame
    }

    if (g.random() * 400.0) > damage {
        return; // didn't flinch
    }

    if g.r(g.self_).pain_finished > g.time {
        return;
    }

    g.v(g.self_).pain_finished = g.time + 2.0;
    sham_pain1(g);
}

pub fn sham_death1(g: &mut Game) {
    g.frame_state(F_death1, sham_death2);
}

pub fn sham_death2(g: &mut Game) {
    g.frame_state(F_death2, sham_death3);
}

pub fn sham_death3(g: &mut Game) {
    g.frame_state(F_death3, sham_death4);
    g.v(g.self_).solid = SOLID_NOT;
}

pub fn sham_death4(g: &mut Game) {
    g.frame_state(F_death4, sham_death5);
}

pub fn sham_death5(g: &mut Game) {
    g.frame_state(F_death5, sham_death6);
}

pub fn sham_death6(g: &mut Game) {
    g.frame_state(F_death6, sham_death7);
}

pub fn sham_death7(g: &mut Game) {
    g.frame_state(F_death7, sham_death8);
}

pub fn sham_death8(g: &mut Game) {
    g.frame_state(F_death8, sham_death9);
}

pub fn sham_death9(g: &mut Game) {
    g.frame_state(F_death9, sham_death10);
}

pub fn sham_death10(g: &mut Game) {
    g.frame_state(F_death10, sham_death11);
}

pub fn sham_death11(g: &mut Game) {
    g.frame_state(F_death11, sham_death11);
}

pub fn sham_die(g: &mut Game) {
    // check for gib
    if g.r(g.self_).health < -60.0 {
        g.sound(g.self_, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        let h = g.r(g.self_).health;
        ThrowHead(g, "progs/h_shams.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib1.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib2.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib3.mdl", h);
        return;
    }

    // regular death
    g.sound(g.self_, CHAN_VOICE, "shambler/sdeath.wav", 1.0, ATTN_NORM);
    sham_death1(g);
}

pub fn monster_shambler(g: &mut Game) {
    if g.deathmatch != 0.0 {
        g.remove(g.self_);
        return;
    }

    g.precache_model("progs/shambler.mdl");
    g.precache_model("progs/s_light.mdl");
    g.precache_model("progs/h_shams.mdl");
    g.precache_model("progs/bolt.mdl");

    g.precache_sound("shambler/sattck1.wav");
    g.precache_sound("shambler/sboom.wav");
    g.precache_sound("shambler/sdeath.wav");
    g.precache_sound("shambler/shurt2.wav");
    g.precache_sound("shambler/sidle.wav");
    g.precache_sound("shambler/ssight.wav");
    g.precache_sound("shambler/melee1.wav");
    g.precache_sound("shambler/melee2.wav");
    g.precache_sound("shambler/smack.wav");

    let s = g.self_;
    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;
    g.setmodel(s, "progs/shambler.mdl");

    g.setsize(s, VEC_HULL2_MIN, VEC_HULL2_MAX);
    g.v(s).health = 600.0;

    g.v(s).th_stand = Some(sham_stand1);
    g.v(s).th_walk = Some(sham_walk1);
    g.v(s).th_run = Some(sham_run1);
    g.v(s).th_die = Some(sham_die);
    g.v(s).th_melee = Some(sham_melee);
    g.v(s).th_missile = Some(sham_magic1);
    g.v(s).th_pain = Some(sham_pain);

    walkmonster_start(g);
}
