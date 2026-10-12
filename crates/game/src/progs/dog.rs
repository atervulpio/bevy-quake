//! `dog.qc`, id's QuakeC 1.06, in Rust.

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
const F_death1: f32 = 8.0;
const F_death2: f32 = 9.0;
const F_death3: f32 = 10.0;
const F_death4: f32 = 11.0;
const F_death5: f32 = 12.0;
const F_death6: f32 = 13.0;
const F_death7: f32 = 14.0;
const F_death8: f32 = 15.0;
const F_death9: f32 = 16.0;
const F_deathb1: f32 = 17.0;
const F_deathb2: f32 = 18.0;
const F_deathb3: f32 = 19.0;
const F_deathb4: f32 = 20.0;
const F_deathb5: f32 = 21.0;
const F_deathb6: f32 = 22.0;
const F_deathb7: f32 = 23.0;
const F_deathb8: f32 = 24.0;
const F_deathb9: f32 = 25.0;
const F_pain1: f32 = 26.0;
const F_pain2: f32 = 27.0;
const F_pain3: f32 = 28.0;
const F_pain4: f32 = 29.0;
const F_pain5: f32 = 30.0;
const F_pain6: f32 = 31.0;
const F_painb1: f32 = 32.0;
const F_painb2: f32 = 33.0;
const F_painb3: f32 = 34.0;
const F_painb4: f32 = 35.0;
const F_painb5: f32 = 36.0;
const F_painb6: f32 = 37.0;
const F_painb7: f32 = 38.0;
const F_painb8: f32 = 39.0;
const F_painb9: f32 = 40.0;
const F_painb10: f32 = 41.0;
const F_painb11: f32 = 42.0;
const F_painb12: f32 = 43.0;
const F_painb13: f32 = 44.0;
const F_painb14: f32 = 45.0;
const F_painb15: f32 = 46.0;
const F_painb16: f32 = 47.0;
const F_run1: f32 = 48.0;
const F_run2: f32 = 49.0;
const F_run3: f32 = 50.0;
const F_run4: f32 = 51.0;
const F_run5: f32 = 52.0;
const F_run6: f32 = 53.0;
const F_run7: f32 = 54.0;
const F_run8: f32 = 55.0;
const F_run9: f32 = 56.0;
const F_run10: f32 = 57.0;
const F_run11: f32 = 58.0;
const F_run12: f32 = 59.0;
const F_leap1: f32 = 60.0;
const F_leap2: f32 = 61.0;
const F_leap3: f32 = 62.0;
const F_leap4: f32 = 63.0;
const F_leap5: f32 = 64.0;
const F_leap6: f32 = 65.0;
const F_leap7: f32 = 66.0;
const F_leap8: f32 = 67.0;
const F_leap9: f32 = 68.0;
const F_stand1: f32 = 69.0;
const F_stand2: f32 = 70.0;
const F_stand3: f32 = 71.0;
const F_stand4: f32 = 72.0;
const F_stand5: f32 = 73.0;
const F_stand6: f32 = 74.0;
const F_stand7: f32 = 75.0;
const F_stand8: f32 = 76.0;
const F_stand9: f32 = 77.0;
const F_walk1: f32 = 78.0;
const F_walk2: f32 = 79.0;
const F_walk3: f32 = 80.0;
const F_walk4: f32 = 81.0;
const F_walk5: f32 = 82.0;
const F_walk6: f32 = 83.0;
const F_walk7: f32 = 84.0;
const F_walk8: f32 = 85.0;


pub fn dog_bite(g: &mut Game) {
    if g.r(g.self_).enemy == world {
        return;
    }

    ai_charge(g, 10.0);

    let s = g.self_;
    let enemy = g.r(s).enemy;
    if CanDamage(g, enemy, s) == 0.0 {
        return;
    }

    let s = g.self_;
    let enemy = g.r(s).enemy;
    let delta = g.r(enemy).origin - g.r(s).origin;
    if vlen(delta) > 100.0 {
        return;
    }

    let ldmg = g.random_sum3_as_compiled() * 8.0;
    T_Damage(g, enemy, s, s, ldmg);
}

pub fn Dog_JumpTouch(g: &mut Game) {
    let s = g.self_;
    if g.r(s).health <= 0.0 {
        return;
    }

    let other = g.other;
    if g.r(other).takedamage != 0 {
        if vlen(g.r(s).velocity) > 300.0 {
            let ldmg = 10.0 + 10.0 * g.random();
            T_Damage(g, other, s, s, ldmg);
        }
    }

    let s = g.self_;
    if !g.checkbottom(s) {
        if (g.r(s).flags & FL_ONGROUND) != 0 {
            // jump randomly to not get hung up
            g.v(s).touch = Some(SUB_Null);
            g.v(s).think = Some(dog_leap1);
            g.v(s).nextthink = g.time + 0.1;
        }
        return; // not on ground yet
    }

    g.v(s).touch = Some(SUB_Null);
    g.v(s).think = Some(dog_run1);
    g.v(s).nextthink = g.time + 0.1;
}

pub fn dog_stand1(g: &mut Game) {
    g.frame_state(F_stand1, dog_stand2);
    ai_stand(g);
}

pub fn dog_stand2(g: &mut Game) {
    g.frame_state(F_stand2, dog_stand3);
    ai_stand(g);
}

pub fn dog_stand3(g: &mut Game) {
    g.frame_state(F_stand3, dog_stand4);
    ai_stand(g);
}

pub fn dog_stand4(g: &mut Game) {
    g.frame_state(F_stand4, dog_stand5);
    ai_stand(g);
}

pub fn dog_stand5(g: &mut Game) {
    g.frame_state(F_stand5, dog_stand6);
    ai_stand(g);
}

pub fn dog_stand6(g: &mut Game) {
    g.frame_state(F_stand6, dog_stand7);
    ai_stand(g);
}

pub fn dog_stand7(g: &mut Game) {
    g.frame_state(F_stand7, dog_stand8);
    ai_stand(g);
}

pub fn dog_stand8(g: &mut Game) {
    g.frame_state(F_stand8, dog_stand9);
    ai_stand(g);
}

pub fn dog_stand9(g: &mut Game) {
    g.frame_state(F_stand9, dog_stand1);
    ai_stand(g);
}

pub fn dog_walk1(g: &mut Game) {
    g.frame_state(F_walk1, dog_walk2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "dog/idle.wav", 1.0, ATTN_IDLE);
    }
    ai_walk(g, 8.0);
}

pub fn dog_walk2(g: &mut Game) {
    g.frame_state(F_walk2, dog_walk3);
    ai_walk(g, 8.0);
}

pub fn dog_walk3(g: &mut Game) {
    g.frame_state(F_walk3, dog_walk4);
    ai_walk(g, 8.0);
}

pub fn dog_walk4(g: &mut Game) {
    g.frame_state(F_walk4, dog_walk5);
    ai_walk(g, 8.0);
}

pub fn dog_walk5(g: &mut Game) {
    g.frame_state(F_walk5, dog_walk6);
    ai_walk(g, 8.0);
}

pub fn dog_walk6(g: &mut Game) {
    g.frame_state(F_walk6, dog_walk7);
    ai_walk(g, 8.0);
}

pub fn dog_walk7(g: &mut Game) {
    g.frame_state(F_walk7, dog_walk8);
    ai_walk(g, 8.0);
}

pub fn dog_walk8(g: &mut Game) {
    g.frame_state(F_walk8, dog_walk1);
    ai_walk(g, 8.0);
}

pub fn dog_run1(g: &mut Game) {
    g.frame_state(F_run1, dog_run2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "dog/idle.wav", 1.0, ATTN_IDLE);
    }
    ai_run(g, 16.0);
}

pub fn dog_run2(g: &mut Game) {
    g.frame_state(F_run2, dog_run3);
    ai_run(g, 32.0);
}

pub fn dog_run3(g: &mut Game) {
    g.frame_state(F_run3, dog_run4);
    ai_run(g, 32.0);
}

pub fn dog_run4(g: &mut Game) {
    g.frame_state(F_run4, dog_run5);
    ai_run(g, 20.0);
}

pub fn dog_run5(g: &mut Game) {
    g.frame_state(F_run5, dog_run6);
    ai_run(g, 64.0);
}

pub fn dog_run6(g: &mut Game) {
    g.frame_state(F_run6, dog_run7);
    ai_run(g, 32.0);
}

pub fn dog_run7(g: &mut Game) {
    g.frame_state(F_run7, dog_run8);
    ai_run(g, 16.0);
}

pub fn dog_run8(g: &mut Game) {
    g.frame_state(F_run8, dog_run9);
    ai_run(g, 32.0);
}

pub fn dog_run9(g: &mut Game) {
    g.frame_state(F_run9, dog_run10);
    ai_run(g, 32.0);
}

pub fn dog_run10(g: &mut Game) {
    g.frame_state(F_run10, dog_run11);
    ai_run(g, 20.0);
}

pub fn dog_run11(g: &mut Game) {
    g.frame_state(F_run11, dog_run12);
    ai_run(g, 64.0);
}

pub fn dog_run12(g: &mut Game) {
    g.frame_state(F_run12, dog_run1);
    ai_run(g, 32.0);
}

pub fn dog_atta1(g: &mut Game) {
    g.frame_state(F_attack1, dog_atta2);
    ai_charge(g, 10.0);
}

pub fn dog_atta2(g: &mut Game) {
    g.frame_state(F_attack2, dog_atta3);
    ai_charge(g, 10.0);
}

pub fn dog_atta3(g: &mut Game) {
    g.frame_state(F_attack3, dog_atta4);
    ai_charge(g, 10.0);
}

pub fn dog_atta4(g: &mut Game) {
    g.frame_state(F_attack4, dog_atta5);
    g.sound(g.self_, CHAN_VOICE, "dog/dattack1.wav", 1.0, ATTN_NORM);
    dog_bite(g);
}

pub fn dog_atta5(g: &mut Game) {
    g.frame_state(F_attack5, dog_atta6);
    ai_charge(g, 10.0);
}

pub fn dog_atta6(g: &mut Game) {
    g.frame_state(F_attack6, dog_atta7);
    ai_charge(g, 10.0);
}

pub fn dog_atta7(g: &mut Game) {
    g.frame_state(F_attack7, dog_atta8);
    ai_charge(g, 10.0);
}

pub fn dog_atta8(g: &mut Game) {
    g.frame_state(F_attack8, dog_run1);
    ai_charge(g, 10.0);
}

pub fn dog_leap1(g: &mut Game) {
    g.frame_state(F_leap1, dog_leap2);
    ai_face(g);
}

pub fn dog_leap2(g: &mut Game) {
    g.frame_state(F_leap2, dog_leap3);
    ai_face(g);

    let s = g.self_;
    g.v(s).touch = Some(Dog_JumpTouch);
    g.makevectors(g.r(s).angles);
    g.v(s).origin.z = g.r(s).origin.z + 1.0;
    g.v(s).velocity = g.v_forward * 300.0 + Vec3::new(0.0, 0.0, 200.0);
    if (g.r(s).flags & FL_ONGROUND) != 0 {
        g.v(s).flags = g.r(s).flags - FL_ONGROUND;
    }
}

pub fn dog_leap3(g: &mut Game) {
    g.frame_state(F_leap3, dog_leap4);
}

pub fn dog_leap4(g: &mut Game) {
    g.frame_state(F_leap4, dog_leap5);
}

pub fn dog_leap5(g: &mut Game) {
    g.frame_state(F_leap5, dog_leap6);
}

pub fn dog_leap6(g: &mut Game) {
    g.frame_state(F_leap6, dog_leap7);
}

pub fn dog_leap7(g: &mut Game) {
    g.frame_state(F_leap7, dog_leap8);
}

pub fn dog_leap8(g: &mut Game) {
    g.frame_state(F_leap8, dog_leap9);
}

pub fn dog_leap9(g: &mut Game) {
    g.frame_state(F_leap9, dog_leap9);
}

pub fn dog_pain1(g: &mut Game) {
    g.frame_state(F_pain1, dog_pain2);
}

pub fn dog_pain2(g: &mut Game) {
    g.frame_state(F_pain2, dog_pain3);
}

pub fn dog_pain3(g: &mut Game) {
    g.frame_state(F_pain3, dog_pain4);
}

pub fn dog_pain4(g: &mut Game) {
    g.frame_state(F_pain4, dog_pain5);
}

pub fn dog_pain5(g: &mut Game) {
    g.frame_state(F_pain5, dog_pain6);
}

pub fn dog_pain6(g: &mut Game) {
    g.frame_state(F_pain6, dog_run1);
}

pub fn dog_painb1(g: &mut Game) {
    g.frame_state(F_painb1, dog_painb2);
}

pub fn dog_painb2(g: &mut Game) {
    g.frame_state(F_painb2, dog_painb3);
}

pub fn dog_painb3(g: &mut Game) {
    g.frame_state(F_painb3, dog_painb4);
    ai_pain(g, 4.0);
}

pub fn dog_painb4(g: &mut Game) {
    g.frame_state(F_painb4, dog_painb5);
    ai_pain(g, 12.0);
}

pub fn dog_painb5(g: &mut Game) {
    g.frame_state(F_painb5, dog_painb6);
    ai_pain(g, 12.0);
}

pub fn dog_painb6(g: &mut Game) {
    g.frame_state(F_painb6, dog_painb7);
    ai_pain(g, 2.0);
}

pub fn dog_painb7(g: &mut Game) {
    g.frame_state(F_painb7, dog_painb8);
}

pub fn dog_painb8(g: &mut Game) {
    g.frame_state(F_painb8, dog_painb9);
    ai_pain(g, 4.0);
}

pub fn dog_painb9(g: &mut Game) {
    g.frame_state(F_painb9, dog_painb10);
}

pub fn dog_painb10(g: &mut Game) {
    g.frame_state(F_painb10, dog_painb11);
    ai_pain(g, 10.0);
}

pub fn dog_painb11(g: &mut Game) {
    g.frame_state(F_painb11, dog_painb12);
}

pub fn dog_painb12(g: &mut Game) {
    g.frame_state(F_painb12, dog_painb13);
}

pub fn dog_painb13(g: &mut Game) {
    g.frame_state(F_painb13, dog_painb14);
}

pub fn dog_painb14(g: &mut Game) {
    g.frame_state(F_painb14, dog_painb15);
}

pub fn dog_painb15(g: &mut Game) {
    g.frame_state(F_painb15, dog_painb16);
}

pub fn dog_painb16(g: &mut Game) {
    g.frame_state(F_painb16, dog_run1);
}

pub fn dog_pain(g: &mut Game, attacker: Ent, damage: f32) {
    g.sound(g.self_, CHAN_VOICE, "dog/dpain1.wav", 1.0, ATTN_NORM);

    if g.random() > 0.5 {
        dog_pain1(g);
    } else {
        dog_painb1(g);
    }
}

pub fn dog_die1(g: &mut Game) {
    g.frame_state(F_death1, dog_die2);
}

pub fn dog_die2(g: &mut Game) {
    g.frame_state(F_death2, dog_die3);
}

pub fn dog_die3(g: &mut Game) {
    g.frame_state(F_death3, dog_die4);
}

pub fn dog_die4(g: &mut Game) {
    g.frame_state(F_death4, dog_die5);
}

pub fn dog_die5(g: &mut Game) {
    g.frame_state(F_death5, dog_die6);
}

pub fn dog_die6(g: &mut Game) {
    g.frame_state(F_death6, dog_die7);
}

pub fn dog_die7(g: &mut Game) {
    g.frame_state(F_death7, dog_die8);
}

pub fn dog_die8(g: &mut Game) {
    g.frame_state(F_death8, dog_die9);
}

pub fn dog_die9(g: &mut Game) {
    g.frame_state(F_death9, dog_die9);
}

pub fn dog_dieb1(g: &mut Game) {
    g.frame_state(F_deathb1, dog_dieb2);
}

pub fn dog_dieb2(g: &mut Game) {
    g.frame_state(F_deathb2, dog_dieb3);
}

pub fn dog_dieb3(g: &mut Game) {
    g.frame_state(F_deathb3, dog_dieb4);
}

pub fn dog_dieb4(g: &mut Game) {
    g.frame_state(F_deathb4, dog_dieb5);
}

pub fn dog_dieb5(g: &mut Game) {
    g.frame_state(F_deathb5, dog_dieb6);
}

pub fn dog_dieb6(g: &mut Game) {
    g.frame_state(F_deathb6, dog_dieb7);
}

pub fn dog_dieb7(g: &mut Game) {
    g.frame_state(F_deathb7, dog_dieb8);
}

pub fn dog_dieb8(g: &mut Game) {
    g.frame_state(F_deathb8, dog_dieb9);
}

pub fn dog_dieb9(g: &mut Game) {
    g.frame_state(F_deathb9, dog_dieb9);
}

pub fn dog_die(g: &mut Game) {
    let s = g.self_;
    // check for gib
    if g.r(s).health < -35.0 {
        g.sound(s, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        ThrowGib(g, "progs/gib3.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib3.mdl", g.r(s).health);
        ThrowGib(g, "progs/gib3.mdl", g.r(s).health);
        ThrowHead(g, "progs/h_dog.mdl", g.r(s).health);
        return;
    }

    // regular death
    g.sound(s, CHAN_VOICE, "dog/ddeath.wav", 1.0, ATTN_NORM);
    g.v(s).solid = SOLID_NOT;

    if g.random() > 0.5 {
        dog_die1(g);
    } else {
        dog_dieb1(g);
    }
}

// Returns TRUE if a melee attack would hit right now.
pub fn CheckDogMelee(g: &mut Game) -> f32 {
    if g.enemy_range == RANGE_MELEE {
        // FIXME: check can reach
        g.v(g.self_).attack_state = AS_MELEE;
        return TRUE;
    }

    FALSE
}

pub fn CheckDogJump(g: &mut Game) -> f32 {
    let s = g.self_;
    let enemy = g.r(s).enemy;
    let (me, en) = (g.r(s), g.r(enemy));

    if (me.origin.z + me.mins.z) > (en.origin.z + en.mins.z + (0.75 * en.size.z)) {
        return FALSE;
    }
    if (me.origin.z + me.maxs.z) < (en.origin.z + en.mins.z + (0.25 * en.size.z)) {
        return FALSE;
    }

    let mut dist = en.origin - me.origin;
    dist.z = 0.0;

    let d = vlen(dist);

    if d < 80.0 {
        return FALSE;
    }
    if d > 150.0 {
        return FALSE;
    }

    TRUE
}

pub fn DogCheckAttack(g: &mut Game) -> f32 {
    // if close enough for slashing, go for it
    if CheckDogMelee(g) != 0.0 {
        g.v(g.self_).attack_state = AS_MELEE;
        return TRUE;
    }

    if CheckDogJump(g) != 0.0 {
        g.v(g.self_).attack_state = AS_MISSILE;
        return TRUE;
    }

    FALSE
}

pub fn monster_dog(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model("progs/h_dog.mdl");
    g.precache_model("progs/dog.mdl");

    g.precache_sound("dog/dattack1.wav");
    g.precache_sound("dog/ddeath.wav");
    g.precache_sound("dog/dpain1.wav");
    g.precache_sound("dog/dsight.wav");
    g.precache_sound("dog/idle.wav");

    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/dog.mdl");

    g.setsize(s, Vec3::new(-32.0, -32.0, -24.0), Vec3::new(32.0, 32.0, 40.0));
    g.v(s).health = 25.0;

    g.v(s).th_stand = Some(dog_stand1);
    g.v(s).th_walk = Some(dog_walk1);
    g.v(s).th_run = Some(dog_run1);
    g.v(s).th_pain = Some(dog_pain);
    g.v(s).th_die = Some(dog_die);
    g.v(s).th_melee = Some(dog_atta1);
    g.v(s).th_missile = Some(dog_leap1);

    walkmonster_start(g);
}
