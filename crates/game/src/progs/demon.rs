//! `demon.qc`, id's QuakeC 1.06, in Rust.

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
const F_walk1: f32 = 13.0;
const F_walk2: f32 = 14.0;
const F_walk3: f32 = 15.0;
const F_walk4: f32 = 16.0;
const F_walk5: f32 = 17.0;
const F_walk6: f32 = 18.0;
const F_walk7: f32 = 19.0;
const F_walk8: f32 = 20.0;
const F_run1: f32 = 21.0;
const F_run2: f32 = 22.0;
const F_run3: f32 = 23.0;
const F_run4: f32 = 24.0;
const F_run5: f32 = 25.0;
const F_run6: f32 = 26.0;
const F_leap1: f32 = 27.0;
const F_leap2: f32 = 28.0;
const F_leap3: f32 = 29.0;
const F_leap4: f32 = 30.0;
const F_leap5: f32 = 31.0;
const F_leap6: f32 = 32.0;
const F_leap7: f32 = 33.0;
const F_leap8: f32 = 34.0;
const F_leap9: f32 = 35.0;
const F_leap10: f32 = 36.0;
const F_leap11: f32 = 37.0;
const F_leap12: f32 = 38.0;
const F_pain1: f32 = 39.0;
const F_pain2: f32 = 40.0;
const F_pain3: f32 = 41.0;
const F_pain4: f32 = 42.0;
const F_pain5: f32 = 43.0;
const F_pain6: f32 = 44.0;
const F_death1: f32 = 45.0;
const F_death2: f32 = 46.0;
const F_death3: f32 = 47.0;
const F_death4: f32 = 48.0;
const F_death5: f32 = 49.0;
const F_death6: f32 = 50.0;
const F_death7: f32 = 51.0;
const F_death8: f32 = 52.0;
const F_death9: f32 = 53.0;
const F_attacka1: f32 = 54.0;
const F_attacka2: f32 = 55.0;
const F_attacka3: f32 = 56.0;
const F_attacka4: f32 = 57.0;
const F_attacka5: f32 = 58.0;
const F_attacka6: f32 = 59.0;
const F_attacka7: f32 = 60.0;
const F_attacka8: f32 = 61.0;
const F_attacka9: f32 = 62.0;
const F_attacka10: f32 = 63.0;
const F_attacka11: f32 = 64.0;
const F_attacka12: f32 = 65.0;
const F_attacka13: f32 = 66.0;
const F_attacka14: f32 = 67.0;
const F_attacka15: f32 = 68.0;


pub fn demon1_stand1(g: &mut Game) {
    g.frame_state(F_stand1, demon1_stand2);
    ai_stand(g);
}

pub fn demon1_stand2(g: &mut Game) {
    g.frame_state(F_stand2, demon1_stand3);
    ai_stand(g);
}

pub fn demon1_stand3(g: &mut Game) {
    g.frame_state(F_stand3, demon1_stand4);
    ai_stand(g);
}

pub fn demon1_stand4(g: &mut Game) {
    g.frame_state(F_stand4, demon1_stand5);
    ai_stand(g);
}

pub fn demon1_stand5(g: &mut Game) {
    g.frame_state(F_stand5, demon1_stand6);
    ai_stand(g);
}

pub fn demon1_stand6(g: &mut Game) {
    g.frame_state(F_stand6, demon1_stand7);
    ai_stand(g);
}

pub fn demon1_stand7(g: &mut Game) {
    g.frame_state(F_stand7, demon1_stand8);
    ai_stand(g);
}

pub fn demon1_stand8(g: &mut Game) {
    g.frame_state(F_stand8, demon1_stand9);
    ai_stand(g);
}

pub fn demon1_stand9(g: &mut Game) {
    g.frame_state(F_stand9, demon1_stand10);
    ai_stand(g);
}

pub fn demon1_stand10(g: &mut Game) {
    g.frame_state(F_stand10, demon1_stand11);
    ai_stand(g);
}

pub fn demon1_stand11(g: &mut Game) {
    g.frame_state(F_stand11, demon1_stand12);
    ai_stand(g);
}

pub fn demon1_stand12(g: &mut Game) {
    g.frame_state(F_stand12, demon1_stand13);
    ai_stand(g);
}

pub fn demon1_stand13(g: &mut Game) {
    g.frame_state(F_stand13, demon1_stand1);
    ai_stand(g);
}

pub fn demon1_walk1(g: &mut Game) {
    g.frame_state(F_walk1, demon1_walk2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "demon/idle1.wav", 1.0, ATTN_IDLE);
    }
    ai_walk(g, 8.0);
}

pub fn demon1_walk2(g: &mut Game) {
    g.frame_state(F_walk2, demon1_walk3);
    ai_walk(g, 6.0);
}

pub fn demon1_walk3(g: &mut Game) {
    g.frame_state(F_walk3, demon1_walk4);
    ai_walk(g, 6.0);
}

pub fn demon1_walk4(g: &mut Game) {
    g.frame_state(F_walk4, demon1_walk5);
    ai_walk(g, 7.0);
}

pub fn demon1_walk5(g: &mut Game) {
    g.frame_state(F_walk5, demon1_walk6);
    ai_walk(g, 4.0);
}

pub fn demon1_walk6(g: &mut Game) {
    g.frame_state(F_walk6, demon1_walk7);
    ai_walk(g, 6.0);
}

pub fn demon1_walk7(g: &mut Game) {
    g.frame_state(F_walk7, demon1_walk8);
    ai_walk(g, 10.0);
}

pub fn demon1_walk8(g: &mut Game) {
    g.frame_state(F_walk8, demon1_walk1);
    ai_walk(g, 10.0);
}

pub fn demon1_run1(g: &mut Game) {
    g.frame_state(F_run1, demon1_run2);
    if g.random() < 0.2 {
        g.sound(g.self_, CHAN_VOICE, "demon/idle1.wav", 1.0, ATTN_IDLE);
    }
    ai_run(g, 20.0);
}

pub fn demon1_run2(g: &mut Game) {
    g.frame_state(F_run2, demon1_run3);
    ai_run(g, 15.0);
}

pub fn demon1_run3(g: &mut Game) {
    g.frame_state(F_run3, demon1_run4);
    ai_run(g, 36.0);
}

pub fn demon1_run4(g: &mut Game) {
    g.frame_state(F_run4, demon1_run5);
    ai_run(g, 20.0);
}

pub fn demon1_run5(g: &mut Game) {
    g.frame_state(F_run5, demon1_run6);
    ai_run(g, 15.0);
}

pub fn demon1_run6(g: &mut Game) {
    g.frame_state(F_run6, demon1_run1);
    ai_run(g, 36.0);
}

pub fn demon1_jump1(g: &mut Game) {
    g.frame_state(F_leap1, demon1_jump2);
    ai_face(g);
}

pub fn demon1_jump2(g: &mut Game) {
    g.frame_state(F_leap2, demon1_jump3);
    ai_face(g);
}

pub fn demon1_jump3(g: &mut Game) {
    g.frame_state(F_leap3, demon1_jump4);
    ai_face(g);
}

pub fn demon1_jump4(g: &mut Game) {
    g.frame_state(F_leap4, demon1_jump5);
    ai_face(g);

    let s = g.self_;
    g.v(s).touch = Some(Demon_JumpTouch);
    g.makevectors(g.r(s).angles);
    g.v(s).origin.z = g.r(s).origin.z + 1.0;
    g.v(s).velocity = g.v_forward * 600.0 + Vec3::new(0.0, 0.0, 250.0);
    if (g.r(s).flags & FL_ONGROUND) != 0 {
        g.v(s).flags = g.r(s).flags - FL_ONGROUND;
    }
}

pub fn demon1_jump5(g: &mut Game) {
    g.frame_state(F_leap5, demon1_jump6);
}

pub fn demon1_jump6(g: &mut Game) {
    g.frame_state(F_leap6, demon1_jump7);
}

pub fn demon1_jump7(g: &mut Game) {
    g.frame_state(F_leap7, demon1_jump8);
}

pub fn demon1_jump8(g: &mut Game) {
    g.frame_state(F_leap8, demon1_jump9);
}

pub fn demon1_jump9(g: &mut Game) {
    g.frame_state(F_leap9, demon1_jump10);
}

pub fn demon1_jump10(g: &mut Game) {
    g.frame_state(F_leap10, demon1_jump1);
    // if three seconds pass, assume demon is stuck and jump again
    g.v(g.self_).nextthink = g.time + 3.0;
}

pub fn demon1_jump11(g: &mut Game) {
    g.frame_state(F_leap11, demon1_jump12);
}

pub fn demon1_jump12(g: &mut Game) {
    g.frame_state(F_leap12, demon1_run1);
}

pub fn demon1_atta1(g: &mut Game) {
    g.frame_state(F_attacka1, demon1_atta2);
    ai_charge(g, 4.0);
}

pub fn demon1_atta2(g: &mut Game) {
    g.frame_state(F_attacka2, demon1_atta3);
    ai_charge(g, 0.0);
}

pub fn demon1_atta3(g: &mut Game) {
    g.frame_state(F_attacka3, demon1_atta4);
    ai_charge(g, 0.0);
}

pub fn demon1_atta4(g: &mut Game) {
    g.frame_state(F_attacka4, demon1_atta5);
    ai_charge(g, 1.0);
}

pub fn demon1_atta5(g: &mut Game) {
    g.frame_state(F_attacka5, demon1_atta6);
    ai_charge(g, 2.0);
    Demon_Melee(g, 200.0);
}

pub fn demon1_atta6(g: &mut Game) {
    g.frame_state(F_attacka6, demon1_atta7);
    ai_charge(g, 1.0);
}

pub fn demon1_atta7(g: &mut Game) {
    g.frame_state(F_attacka7, demon1_atta8);
    ai_charge(g, 6.0);
}

pub fn demon1_atta8(g: &mut Game) {
    g.frame_state(F_attacka8, demon1_atta9);
    ai_charge(g, 8.0);
}

pub fn demon1_atta9(g: &mut Game) {
    g.frame_state(F_attacka9, demon1_atta10);
    ai_charge(g, 4.0);
}

pub fn demon1_atta10(g: &mut Game) {
    g.frame_state(F_attacka10, demon1_atta11);
    ai_charge(g, 2.0);
}

pub fn demon1_atta11(g: &mut Game) {
    g.frame_state(F_attacka11, demon1_atta12);
    Demon_Melee(g, -200.0);
}

pub fn demon1_atta12(g: &mut Game) {
    g.frame_state(F_attacka12, demon1_atta13);
    ai_charge(g, 5.0);
}

pub fn demon1_atta13(g: &mut Game) {
    g.frame_state(F_attacka13, demon1_atta14);
    ai_charge(g, 8.0);
}

pub fn demon1_atta14(g: &mut Game) {
    g.frame_state(F_attacka14, demon1_atta15);
    ai_charge(g, 4.0);
}

pub fn demon1_atta15(g: &mut Game) {
    g.frame_state(F_attacka15, demon1_run1);
    ai_charge(g, 4.0);
}

pub fn demon1_pain1(g: &mut Game) {
    g.frame_state(F_pain1, demon1_pain2);
}

pub fn demon1_pain2(g: &mut Game) {
    g.frame_state(F_pain2, demon1_pain3);
}

pub fn demon1_pain3(g: &mut Game) {
    g.frame_state(F_pain3, demon1_pain4);
}

pub fn demon1_pain4(g: &mut Game) {
    g.frame_state(F_pain4, demon1_pain5);
}

pub fn demon1_pain5(g: &mut Game) {
    g.frame_state(F_pain5, demon1_pain6);
}

pub fn demon1_pain6(g: &mut Game) {
    g.frame_state(F_pain6, demon1_run1);
}

// QuakeC compares function fields by their number; here by address, which is one per function.
#[allow(unpredictable_function_pointer_comparisons)]
pub fn demon1_pain(g: &mut Game, attacker: Ent, damage: f32) {
    let s = g.self_;
    if g.r(s).touch == Some(Demon_JumpTouch as fn(&mut Game)) {
        return;
    }

    if g.r(s).pain_finished > g.time {
        return;
    }

    g.v(s).pain_finished = g.time + 1.0;
    g.sound(s, CHAN_VOICE, "demon/dpain1.wav", 1.0, ATTN_NORM);

    if (g.random() * 200.0) > damage {
        return; // didn't flinch
    }

    demon1_pain1(g);
}

pub fn demon1_die1(g: &mut Game) {
    g.frame_state(F_death1, demon1_die2);
    g.sound(g.self_, CHAN_VOICE, "demon/ddeath.wav", 1.0, ATTN_NORM);
}

pub fn demon1_die2(g: &mut Game) {
    g.frame_state(F_death2, demon1_die3);
}

pub fn demon1_die3(g: &mut Game) {
    g.frame_state(F_death3, demon1_die4);
}

pub fn demon1_die4(g: &mut Game) {
    g.frame_state(F_death4, demon1_die5);
}

pub fn demon1_die5(g: &mut Game) {
    g.frame_state(F_death5, demon1_die6);
}

pub fn demon1_die6(g: &mut Game) {
    g.frame_state(F_death6, demon1_die7);
    g.v(g.self_).solid = SOLID_NOT;
}

pub fn demon1_die7(g: &mut Game) {
    g.frame_state(F_death7, demon1_die8);
}

pub fn demon1_die8(g: &mut Game) {
    g.frame_state(F_death8, demon1_die9);
}

pub fn demon1_die9(g: &mut Game) {
    g.frame_state(F_death9, demon1_die9);
}

pub fn demon_die(g: &mut Game) {
    // check for gib
    if g.r(g.self_).health < -80.0 {
        g.sound(g.self_, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        let h = g.r(g.self_).health;
        ThrowHead(g, "progs/h_demon.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib1.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib1.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib1.mdl", h);
        return;
    }

    // regular death
    demon1_die1(g);
}

pub fn Demon_MeleeAttack(g: &mut Game) {
    demon1_atta1(g);
}

pub fn monster_demon1(g: &mut Game) {
    if g.deathmatch != 0.0 {
        g.remove(g.self_);
        return;
    }

    g.precache_model("progs/demon.mdl");
    g.precache_model("progs/h_demon.mdl");

    g.precache_sound("demon/ddeath.wav");
    g.precache_sound("demon/dhit2.wav");
    g.precache_sound("demon/djump.wav");
    g.precache_sound("demon/dpain1.wav");
    g.precache_sound("demon/idle1.wav");
    g.precache_sound("demon/sight2.wav");

    let s = g.self_;
    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/demon.mdl");

    g.setsize(s, VEC_HULL2_MIN, VEC_HULL2_MAX);
    g.v(s).health = 300.0;

    g.v(s).th_stand = Some(demon1_stand1);
    g.v(s).th_walk = Some(demon1_walk1);
    g.v(s).th_run = Some(demon1_run1);
    g.v(s).th_die = Some(demon_die);
    g.v(s).th_melee = Some(Demon_MeleeAttack); // one of two attacks
    g.v(s).th_missile = Some(demon1_jump1); // jump attack
    g.v(s).th_pain = Some(demon1_pain);

    walkmonster_start(g);
}

pub fn CheckDemonMelee(g: &mut Game) -> f32 {
    if g.enemy_range == RANGE_MELEE {
        // FIXME: check can reach
        g.v(g.self_).attack_state = AS_MELEE;
        return TRUE;
    }

    FALSE
}

pub fn CheckDemonJump(g: &mut Game) -> f32 {
    let s = g.self_;
    let e = g.r(s).enemy;

    if (g.r(s).origin.z + g.r(s).mins.z)
        > (g.r(e).origin.z + g.r(e).mins.z + (0.75 * g.r(e).size.z))
    {
        return FALSE;
    }
    if (g.r(s).origin.z + g.r(s).maxs.z)
        < (g.r(e).origin.z + g.r(e).mins.z + (0.25 * g.r(e).size.z))
    {
        return FALSE;
    }

    let mut dist = g.r(e).origin - g.r(s).origin;
    dist.z = 0.0;

    let d = vlen(dist);

    if d < 100.0 {
        return FALSE;
    }
    if d > 200.0 {
        if g.random() < 0.9 {
            return FALSE;
        }
    }

    TRUE
}

pub fn DemonCheckAttack(g: &mut Game) -> f32 {
    // if close enough for slashing, go for it
    if CheckDemonMelee(g) != 0.0 {
        g.v(g.self_).attack_state = AS_MELEE;
        return TRUE;
    }

    if CheckDemonJump(g) != 0.0 {
        g.v(g.self_).attack_state = AS_MISSILE;
        g.sound(g.self_, CHAN_VOICE, "demon/djump.wav", 1.0, ATTN_NORM);
        return TRUE;
    }

    FALSE
}

pub fn Demon_Melee(g: &mut Game, side: f32) {
    ai_face(g);
    let s = g.self_;
    g.walkmove(g.r(s).ideal_yaw, 12.0); // allow a little closing

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
    g.sound(s, CHAN_WEAPON, "demon/dhit2.wav", 1.0, ATTN_NORM);
    let ldmg = 10.0 + 5.0 * g.random();
    let enemy = g.r(s).enemy;
    T_Damage(g, enemy, s, s, ldmg);

    let s = g.self_;
    g.makevectors(g.r(s).angles);
    let org = g.r(s).origin + g.v_forward * 16.0;
    let vel = side * g.v_right;
    SpawnMeatSpray(g, org, vel);
}

pub fn Demon_JumpTouch(g: &mut Game) {
    let s = g.self_;
    if g.r(s).health <= 0.0 {
        return;
    }

    if g.r(g.other).takedamage != 0 {
        if vlen(g.r(s).velocity) > 400.0 {
            let ldmg = 40.0 + 10.0 * g.random();
            let o = g.other;
            T_Damage(g, o, s, s, ldmg);
        }
    }

    let s = g.self_;
    if !g.checkbottom(s) {
        if (g.r(s).flags & FL_ONGROUND) != 0 {
            // jump randomly to not get hung up
            g.v(s).touch = Some(SUB_Null);
            g.v(s).think = Some(demon1_jump1);
            g.v(s).nextthink = g.time + 0.1;
        }
        return; // not on ground yet
    }

    g.v(s).touch = Some(SUB_Null);
    g.v(s).think = Some(demon1_jump11);
    g.v(s).nextthink = g.time + 0.1;
}
