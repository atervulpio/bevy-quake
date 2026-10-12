//! `wizard.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;


// The model's frames, numbered as `$frame` numbers them: from nought, in order, in this file.
const F_hover1: f32 = 0.0;
const F_hover2: f32 = 1.0;
const F_hover3: f32 = 2.0;
const F_hover4: f32 = 3.0;
const F_hover5: f32 = 4.0;
const F_hover6: f32 = 5.0;
const F_hover7: f32 = 6.0;
const F_hover8: f32 = 7.0;
const F_fly1: f32 = 15.0;
const F_fly2: f32 = 16.0;
const F_fly3: f32 = 17.0;
const F_fly4: f32 = 18.0;
const F_fly5: f32 = 19.0;
const F_fly6: f32 = 20.0;
const F_fly7: f32 = 21.0;
const F_fly8: f32 = 22.0;
const F_fly9: f32 = 23.0;
const F_fly10: f32 = 24.0;
const F_fly11: f32 = 25.0;
const F_fly12: f32 = 26.0;
const F_fly13: f32 = 27.0;
const F_fly14: f32 = 28.0;
const F_magatt1: f32 = 29.0;
const F_magatt2: f32 = 30.0;
const F_magatt3: f32 = 31.0;
const F_magatt4: f32 = 32.0;
const F_magatt5: f32 = 33.0;
const F_magatt6: f32 = 34.0;
const F_pain1: f32 = 42.0;
const F_pain2: f32 = 43.0;
const F_pain3: f32 = 44.0;
const F_pain4: f32 = 45.0;
const F_death1: f32 = 46.0;
const F_death2: f32 = 47.0;
const F_death3: f32 = 48.0;
const F_death4: f32 = 49.0;
const F_death5: f32 = 50.0;
const F_death6: f32 = 51.0;
const F_death7: f32 = 52.0;
const F_death8: f32 = 53.0;


pub fn LaunchMissile(g: &mut Game, missile: Ent, mspeed: f32, accuracy: f32) {
    let s = g.self_;
    g.makevectors(g.r(s).angles);

    // set missile speed
    let e = g.r(s).enemy;
    let mut vec = g.r(e).origin + g.r(e).mins + g.r(e).size * 0.7 - g.r(missile).origin;

    // calc approximate time for missile to reach vec
    let fly = vlen(vec) / mspeed;

    // get the entities xy velocity
    let mut mv = g.r(e).velocity;
    mv.z = 0.0;

    // project the target forward in time
    vec = vec + mv * fly;

    vec = normalize(vec);
    let r1 = g.random();
    let up = accuracy * g.v_up * (r1 - 0.5);
    let r2 = g.random();
    let right = accuracy * g.v_right * (r2 - 0.5);
    vec = vec + up + right;

    g.v(missile).velocity = vec * mspeed;

    g.v(missile).angles = Vec3::ZERO;
    g.v(missile).angles.y = vectoyaw(g.r(missile).velocity);

    // set missile duration
    g.v(missile).nextthink = g.time + 5.0;
    g.v(missile).think = Some(SUB_Remove);
}

pub fn WizardCheckAttack(g: &mut Game) -> f32 {
    let s = g.self_;
    if g.time < g.r(s).attack_finished {
        return FALSE;
    }

    if g.enemy_vis == 0.0 {
        return FALSE;
    }

    if g.enemy_range == RANGE_FAR {
        if g.r(s).attack_state != AS_STRAIGHT {
            g.v(s).attack_state = AS_STRAIGHT;
            wiz_run1(g);
        }
        return FALSE;
    }

    let targ = g.r(s).enemy;

    // see if any entities are in the way of the shot
    let spot1 = g.r(s).origin + g.r(s).view_ofs;
    let spot2 = g.r(targ).origin + g.r(targ).view_ofs;

    g.traceline(spot1, spot2, FALSE, s);

    if g.trace_ent != targ {
        // don't have a clear shot, so move to a side
        if g.r(s).attack_state != AS_STRAIGHT {
            g.v(s).attack_state = AS_STRAIGHT;
            wiz_run1(g);
        }
        return FALSE;
    }

    let chance = if g.enemy_range == RANGE_MELEE {
        0.9
    } else if g.enemy_range == RANGE_NEAR {
        0.6
    } else if g.enemy_range == RANGE_MID {
        0.2
    } else {
        0.0
    };

    if g.random() < chance {
        g.v(g.self_).attack_state = AS_MISSILE;
        return TRUE;
    }

    let s = g.self_;
    if g.enemy_range == RANGE_MID {
        if g.r(s).attack_state != AS_STRAIGHT {
            g.v(s).attack_state = AS_STRAIGHT;
            wiz_run1(g);
        }
    } else {
        if g.r(s).attack_state != AS_SLIDING {
            g.v(s).attack_state = AS_SLIDING;
            wiz_side1(g);
        }
    }

    FALSE
}

pub fn WizardAttackFinished(g: &mut Game) {
    let s = g.self_;
    if g.enemy_range >= RANGE_MID || g.enemy_vis == 0.0 {
        g.v(s).attack_state = AS_STRAIGHT;
        g.v(s).think = Some(wiz_run1);
    } else {
        g.v(s).attack_state = AS_SLIDING;
        g.v(s).think = Some(wiz_side1);
    }
}

pub fn Wiz_FastFire(g: &mut Game) {
    let s = g.self_;
    if g.r(g.r(s).owner).health > 0.0 {
        let owner = g.r(s).owner;
        g.v(owner).effects = g.r(owner).effects | EF_MUZZLEFLASH;

        g.makevectors(g.r(g.r(s).enemy).angles);
        let dst = g.r(g.r(s).enemy).origin - 13.0 * g.r(s).movedir;

        let vec = normalize(dst - g.r(s).origin);
        g.sound(s, CHAN_WEAPON, "wizard/wattack.wav", 1.0, ATTN_NORM);
        let org = g.r(s).origin;
        launch_spike(g, org, vec);
        let s = g.self_;
        let n = g.newmis;
        g.v(n).velocity = vec * 600.0;
        g.v(n).owner = g.r(s).owner;
        g.v(n).classname = "wizspike";
        g.setmodel(n, "progs/w_spike.mdl");
        g.setsize(n, VEC_ORIGIN, VEC_ORIGIN);
    }

    g.remove(g.self_);
}

pub fn Wiz_StartFast(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_WEAPON, "wizard/wattack.wav", 1.0, ATTN_NORM);
    g.v(s).v_angle = g.r(s).angles;
    g.makevectors(g.r(s).angles);

    let missile = g.spawn();
    g.v(missile).owner = s;
    g.v(missile).nextthink = g.time + 0.6;
    g.setsize(missile, Vec3::ZERO, Vec3::ZERO);
    g.setorigin(
        missile,
        g.r(s).origin + Vec3::new(0.0, 0.0, 30.0) + g.v_forward * 14.0 + g.v_right * 14.0,
    );
    g.v(missile).enemy = g.r(s).enemy;
    g.v(missile).nextthink = g.time + 0.8;
    g.v(missile).think = Some(Wiz_FastFire);
    g.v(missile).movedir = g.v_right;

    let missile = g.spawn();
    g.v(missile).owner = s;
    g.v(missile).nextthink = g.time + 1.0;
    g.setsize(missile, Vec3::ZERO, Vec3::ZERO);
    g.setorigin(
        missile,
        g.r(s).origin + Vec3::new(0.0, 0.0, 30.0) + g.v_forward * 14.0 + g.v_right * -14.0,
    );
    g.v(missile).enemy = g.r(s).enemy;
    g.v(missile).nextthink = g.time + 0.3;
    g.v(missile).think = Some(Wiz_FastFire);
    g.v(missile).movedir = VEC_ORIGIN - g.v_right;
}

pub fn Wiz_idlesound(g: &mut Game) {
    let wr = g.random() * 5.0;

    let s = g.self_;
    if g.r(s).waitmin < g.time {
        g.v(s).waitmin = g.time + 2.0;
        if wr > 4.5 {
            g.sound(s, CHAN_VOICE, "wizard/widle1.wav", 1.0, ATTN_IDLE);
        } else if wr < 1.5 {
            g.sound(s, CHAN_VOICE, "wizard/widle2.wav", 1.0, ATTN_IDLE);
        }
    }
}

pub fn wiz_stand1(g: &mut Game) {
    g.frame_state(F_hover1, wiz_stand2);
    ai_stand(g);
}

pub fn wiz_stand2(g: &mut Game) {
    g.frame_state(F_hover2, wiz_stand3);
    ai_stand(g);
}

pub fn wiz_stand3(g: &mut Game) {
    g.frame_state(F_hover3, wiz_stand4);
    ai_stand(g);
}

pub fn wiz_stand4(g: &mut Game) {
    g.frame_state(F_hover4, wiz_stand5);
    ai_stand(g);
}

pub fn wiz_stand5(g: &mut Game) {
    g.frame_state(F_hover5, wiz_stand6);
    ai_stand(g);
}

pub fn wiz_stand6(g: &mut Game) {
    g.frame_state(F_hover6, wiz_stand7);
    ai_stand(g);
}

pub fn wiz_stand7(g: &mut Game) {
    g.frame_state(F_hover7, wiz_stand8);
    ai_stand(g);
}

pub fn wiz_stand8(g: &mut Game) {
    g.frame_state(F_hover8, wiz_stand1);
    ai_stand(g);
}

pub fn wiz_walk1(g: &mut Game) {
    g.frame_state(F_hover1, wiz_walk2);
    ai_walk(g, 8.0);
    Wiz_idlesound(g);
}

pub fn wiz_walk2(g: &mut Game) {
    g.frame_state(F_hover2, wiz_walk3);
    ai_walk(g, 8.0);
}

pub fn wiz_walk3(g: &mut Game) {
    g.frame_state(F_hover3, wiz_walk4);
    ai_walk(g, 8.0);
}

pub fn wiz_walk4(g: &mut Game) {
    g.frame_state(F_hover4, wiz_walk5);
    ai_walk(g, 8.0);
}

pub fn wiz_walk5(g: &mut Game) {
    g.frame_state(F_hover5, wiz_walk6);
    ai_walk(g, 8.0);
}

pub fn wiz_walk6(g: &mut Game) {
    g.frame_state(F_hover6, wiz_walk7);
    ai_walk(g, 8.0);
}

pub fn wiz_walk7(g: &mut Game) {
    g.frame_state(F_hover7, wiz_walk8);
    ai_walk(g, 8.0);
}

pub fn wiz_walk8(g: &mut Game) {
    g.frame_state(F_hover8, wiz_walk1);
    ai_walk(g, 8.0);
}

pub fn wiz_side1(g: &mut Game) {
    g.frame_state(F_hover1, wiz_side2);
    ai_run(g, 8.0);
    Wiz_idlesound(g);
}

pub fn wiz_side2(g: &mut Game) {
    g.frame_state(F_hover2, wiz_side3);
    ai_run(g, 8.0);
}

pub fn wiz_side3(g: &mut Game) {
    g.frame_state(F_hover3, wiz_side4);
    ai_run(g, 8.0);
}

pub fn wiz_side4(g: &mut Game) {
    g.frame_state(F_hover4, wiz_side5);
    ai_run(g, 8.0);
}

pub fn wiz_side5(g: &mut Game) {
    g.frame_state(F_hover5, wiz_side6);
    ai_run(g, 8.0);
}

pub fn wiz_side6(g: &mut Game) {
    g.frame_state(F_hover6, wiz_side7);
    ai_run(g, 8.0);
}

pub fn wiz_side7(g: &mut Game) {
    g.frame_state(F_hover7, wiz_side8);
    ai_run(g, 8.0);
}

pub fn wiz_side8(g: &mut Game) {
    g.frame_state(F_hover8, wiz_side1);
    ai_run(g, 8.0);
}

pub fn wiz_run1(g: &mut Game) {
    g.frame_state(F_fly1, wiz_run2);
    ai_run(g, 16.0);
    Wiz_idlesound(g);
}

pub fn wiz_run2(g: &mut Game) {
    g.frame_state(F_fly2, wiz_run3);
    ai_run(g, 16.0);
}

pub fn wiz_run3(g: &mut Game) {
    g.frame_state(F_fly3, wiz_run4);
    ai_run(g, 16.0);
}

pub fn wiz_run4(g: &mut Game) {
    g.frame_state(F_fly4, wiz_run5);
    ai_run(g, 16.0);
}

pub fn wiz_run5(g: &mut Game) {
    g.frame_state(F_fly5, wiz_run6);
    ai_run(g, 16.0);
}

pub fn wiz_run6(g: &mut Game) {
    g.frame_state(F_fly6, wiz_run7);
    ai_run(g, 16.0);
}

pub fn wiz_run7(g: &mut Game) {
    g.frame_state(F_fly7, wiz_run8);
    ai_run(g, 16.0);
}

pub fn wiz_run8(g: &mut Game) {
    g.frame_state(F_fly8, wiz_run9);
    ai_run(g, 16.0);
}

pub fn wiz_run9(g: &mut Game) {
    g.frame_state(F_fly9, wiz_run10);
    ai_run(g, 16.0);
}

pub fn wiz_run10(g: &mut Game) {
    g.frame_state(F_fly10, wiz_run11);
    ai_run(g, 16.0);
}

pub fn wiz_run11(g: &mut Game) {
    g.frame_state(F_fly11, wiz_run12);
    ai_run(g, 16.0);
}

pub fn wiz_run12(g: &mut Game) {
    g.frame_state(F_fly12, wiz_run13);
    ai_run(g, 16.0);
}

pub fn wiz_run13(g: &mut Game) {
    g.frame_state(F_fly13, wiz_run14);
    ai_run(g, 16.0);
}

pub fn wiz_run14(g: &mut Game) {
    g.frame_state(F_fly14, wiz_run1);
    ai_run(g, 16.0);
}

pub fn wiz_fast1(g: &mut Game) {
    g.frame_state(F_magatt1, wiz_fast2);
    ai_face(g);
    Wiz_StartFast(g);
}

pub fn wiz_fast2(g: &mut Game) {
    g.frame_state(F_magatt2, wiz_fast3);
    ai_face(g);
}

pub fn wiz_fast3(g: &mut Game) {
    g.frame_state(F_magatt3, wiz_fast4);
    ai_face(g);
}

pub fn wiz_fast4(g: &mut Game) {
    g.frame_state(F_magatt4, wiz_fast5);
    ai_face(g);
}

pub fn wiz_fast5(g: &mut Game) {
    g.frame_state(F_magatt5, wiz_fast6);
    ai_face(g);
}

pub fn wiz_fast6(g: &mut Game) {
    g.frame_state(F_magatt6, wiz_fast7);
    ai_face(g);
}

pub fn wiz_fast7(g: &mut Game) {
    g.frame_state(F_magatt5, wiz_fast8);
    ai_face(g);
}

pub fn wiz_fast8(g: &mut Game) {
    g.frame_state(F_magatt4, wiz_fast9);
    ai_face(g);
}

pub fn wiz_fast9(g: &mut Game) {
    g.frame_state(F_magatt3, wiz_fast10);
    ai_face(g);
}

pub fn wiz_fast10(g: &mut Game) {
    g.frame_state(F_magatt2, wiz_run1);
    ai_face(g);
    SUB_AttackFinished(g, 2.0);
    WizardAttackFinished(g);
}

pub fn wiz_pain1(g: &mut Game) {
    g.frame_state(F_pain1, wiz_pain2);
}

pub fn wiz_pain2(g: &mut Game) {
    g.frame_state(F_pain2, wiz_pain3);
}

pub fn wiz_pain3(g: &mut Game) {
    g.frame_state(F_pain3, wiz_pain4);
}

pub fn wiz_pain4(g: &mut Game) {
    g.frame_state(F_pain4, wiz_run1);
}

pub fn wiz_death1(g: &mut Game) {
    g.frame_state(F_death1, wiz_death2);
    let s = g.self_;
    let r = g.random();
    g.v(s).velocity.x = -200.0 + 400.0 * r;
    let r = g.random();
    g.v(s).velocity.y = -200.0 + 400.0 * r;
    let r = g.random();
    g.v(s).velocity.z = 100.0 + 100.0 * r;
    g.v(s).flags = g.r(s).flags - (g.r(s).flags & FL_ONGROUND);
    g.sound(s, CHAN_VOICE, "wizard/wdeath.wav", 1.0, ATTN_NORM);
}

pub fn wiz_death2(g: &mut Game) {
    g.frame_state(F_death2, wiz_death3);
}

pub fn wiz_death3(g: &mut Game) {
    g.frame_state(F_death3, wiz_death4);
    g.v(g.self_).solid = SOLID_NOT;
}

pub fn wiz_death4(g: &mut Game) {
    g.frame_state(F_death4, wiz_death5);
}

pub fn wiz_death5(g: &mut Game) {
    g.frame_state(F_death5, wiz_death6);
}

pub fn wiz_death6(g: &mut Game) {
    g.frame_state(F_death6, wiz_death7);
}

pub fn wiz_death7(g: &mut Game) {
    g.frame_state(F_death7, wiz_death8);
}

pub fn wiz_death8(g: &mut Game) {
    g.frame_state(F_death8, wiz_death8);
}

pub fn wiz_die(g: &mut Game) {
    // check for gib
    if g.r(g.self_).health < -40.0 {
        g.sound(g.self_, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NORM);
        let h = g.r(g.self_).health;
        ThrowHead(g, "progs/h_wizard.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib2.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib2.mdl", h);
        let h = g.r(g.self_).health;
        ThrowGib(g, "progs/gib2.mdl", h);
        return;
    }

    wiz_death1(g);
}

pub fn Wiz_Pain(g: &mut Game, attacker: Ent, damage: f32) {
    g.sound(g.self_, CHAN_VOICE, "wizard/wpain.wav", 1.0, ATTN_NORM);
    if (g.random() * 70.0) > damage {
        return; // didn't flinch
    }

    wiz_pain1(g);
}

pub fn Wiz_Missile(g: &mut Game) {
    wiz_fast1(g);
}

pub fn monster_wizard(g: &mut Game) {
    if g.deathmatch != 0.0 {
        g.remove(g.self_);
        return;
    }

    g.precache_model("progs/wizard.mdl");
    g.precache_model("progs/h_wizard.mdl");
    g.precache_model("progs/w_spike.mdl");

    g.precache_sound("wizard/hit.wav"); // used by C code
    g.precache_sound("wizard/wattack.wav");
    g.precache_sound("wizard/wdeath.wav");
    g.precache_sound("wizard/widle1.wav");
    g.precache_sound("wizard/widle2.wav");
    g.precache_sound("wizard/wpain.wav");
    g.precache_sound("wizard/wsight.wav");

    let s = g.self_;
    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/wizard.mdl");

    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 40.0));
    g.v(s).health = 80.0;

    g.v(s).th_stand = Some(wiz_stand1);
    g.v(s).th_walk = Some(wiz_walk1);
    g.v(s).th_run = Some(wiz_run1);
    g.v(s).th_missile = Some(Wiz_Missile);
    g.v(s).th_pain = Some(Wiz_Pain);
    g.v(s).th_die = Some(wiz_die);

    flymonster_start(g);
}
