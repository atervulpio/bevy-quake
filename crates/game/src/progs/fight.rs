//! `fight.qc`, id's QuakeC 1.06, in Rust.
//!
//! A monster is in fight mode if it thinks it can effectively attack its enemy. When it decides
//! it can't attack, it goes into hunt mode. The globals `enemy_vis`, `enemy_infront`,
//! `enemy_range` and `enemy_yaw` live in `Game`.

#![allow(clippy::all)]

use super::*;

pub fn knight_attack(g: &mut Game) {
    let s = g.self_;
    let e = g.r(s).enemy;

    // decide if now is a good swing time
    let len = vlen(g.r(e).origin + g.r(e).view_ofs - (g.r(s).origin + g.r(s).view_ofs));

    if len < 80.0 {
        knight_atk1(g);
    } else {
        knight_runatk1(g);
    }
}

//=============================================================================

/// The player is in view, so decide to move or launch an attack. FALSE if movement should
/// continue.
pub fn CheckAttack(g: &mut Game) -> f32 {
    let s = g.self_;
    let targ = g.r(s).enemy;

    // see if any entities are in the way of the shot
    let spot1 = g.r(s).origin + g.r(s).view_ofs;
    let spot2 = g.r(targ).origin + g.r(targ).view_ofs;

    g.traceline(spot1, spot2, FALSE, s);

    if g.trace_ent != targ {
        return FALSE; // don't have a clear shot
    }

    if g.trace_inopen && g.trace_inwater {
        return FALSE; // sight line crossed contents
    }

    if g.enemy_range == RANGE_MELEE {
        // melee attack
        if g.r(s).th_melee.is_some() {
            if g.r(s).classname == "monster_knight" {
                knight_attack(g);
            } else {
                let f = g.r(s).th_melee;
                g.call(f);
            }
            return TRUE;
        }
    }

    // missile attack
    if g.r(s).th_missile.is_none() {
        return FALSE;
    }

    if g.time < g.r(s).attack_finished {
        return FALSE;
    }

    if g.enemy_range == RANGE_FAR {
        return FALSE;
    }

    let chance;
    if g.enemy_range == RANGE_MELEE {
        chance = 0.9;
        g.v(s).attack_finished = 0.0;
    } else if g.enemy_range == RANGE_NEAR {
        if g.r(s).th_melee.is_some() {
            chance = 0.2;
        } else {
            chance = 0.4;
        }
    } else if g.enemy_range == RANGE_MID {
        if g.r(s).th_melee.is_some() {
            chance = 0.05;
        } else {
            chance = 0.1;
        }
    } else {
        chance = 0.0;
    }

    if g.random() < chance {
        let f = g.r(s).th_missile;
        g.call(f);
        let r = 2.0 * g.random();
        SUB_AttackFinished(g, r);
        return TRUE;
    }

    FALSE
}

/// Stay facing the enemy.
pub fn ai_face(g: &mut Game) {
    let s = g.self_;
    g.v(s).ideal_yaw = vectoyaw(g.r(g.r(s).enemy).origin - g.r(s).origin);
    g.ChangeYaw();
}

/// The monster is in a melee attack, so get as close as possible to .enemy.
pub fn ai_charge(g: &mut Game, d: f32) {
    ai_face(g);
    g.movetogoal(d); // done in C code...
}

pub fn ai_charge_side(g: &mut Game) {
    let s = g.self_;

    // aim to the left of the enemy for a flyby
    g.v(s).ideal_yaw = vectoyaw(g.r(g.r(s).enemy).origin - g.r(s).origin);
    g.ChangeYaw();

    g.makevectors(g.r(s).angles);
    let dtemp = g.r(g.r(s).enemy).origin - 30.0 * g.v_right;
    let heading = vectoyaw(dtemp - g.r(s).origin);

    g.walkmove(heading, 20.0);
}

pub fn ai_melee(g: &mut Game) {
    let s = g.self_;
    if g.r(s).enemy == world {
        return; // removed before stroke
    }

    let delta = g.r(g.r(s).enemy).origin - g.r(s).origin;
    if vlen(delta) > 60.0 {
        return;
    }

    let ldmg = g.random_sum3_as_compiled() * 3.0;
    let e = g.r(s).enemy;
    T_Damage(g, e, s, s, ldmg);
}

pub fn ai_melee_side(g: &mut Game) {
    let s = g.self_;
    if g.r(s).enemy == world {
        return; // removed before stroke
    }

    ai_charge_side(g);

    let s = g.self_;
    let delta = g.r(g.r(s).enemy).origin - g.r(s).origin;
    if vlen(delta) > 60.0 {
        return;
    }

    let e = g.r(s).enemy;
    if CanDamage(g, e, s) == 0.0 {
        return;
    }

    let ldmg = g.random_sum3_as_compiled() * 3.0;
    let s = g.self_; // read again after CanDamage
    let e = g.r(s).enemy;
    T_Damage(g, e, s, s, ldmg);
}

//=============================================================================

/// The player is in view, so decide to move or launch an attack. FALSE if movement should
/// continue.
pub fn SoldierCheckAttack(g: &mut Game) -> f32 {
    let s = g.self_;
    let targ = g.r(s).enemy;

    // see if any entities are in the way of the shot
    let spot1 = g.r(s).origin + g.r(s).view_ofs;
    let spot2 = g.r(targ).origin + g.r(targ).view_ofs;

    g.traceline(spot1, spot2, FALSE, s);

    if g.trace_inopen && g.trace_inwater {
        return FALSE; // sight line crossed contents
    }

    if g.trace_ent != targ {
        return FALSE; // don't have a clear shot
    }

    // missile attack
    if g.time < g.r(s).attack_finished {
        return FALSE;
    }

    if g.enemy_range == RANGE_FAR {
        return FALSE;
    }

    let chance = if g.enemy_range == RANGE_MELEE {
        0.9
    } else if g.enemy_range == RANGE_NEAR {
        0.4
    } else if g.enemy_range == RANGE_MID {
        0.05
    } else {
        0.0
    };

    if g.random() < chance {
        let f = g.r(s).th_missile;
        g.call(f);
        let r = 1.0 + g.random();
        SUB_AttackFinished(g, r);
        if g.random() < 0.3 {
            let s = g.self_;
            g.v(s).lefty = if g.r(s).lefty == 0.0 { 1.0 } else { 0.0 };
        }

        return TRUE;
    }

    FALSE
}

//=============================================================================

/// The player is in view, so decide to move or launch an attack. FALSE if movement should
/// continue.
pub fn ShamCheckAttack(g: &mut Game) -> f32 {
    let s = g.self_;

    if g.enemy_range == RANGE_MELEE {
        let e = g.r(s).enemy;
        if CanDamage(g, e, s) != 0.0 {
            g.v(g.self_).attack_state = AS_MELEE;
            return TRUE;
        }
    }

    let s = g.self_;
    if g.time < g.r(s).attack_finished {
        return FALSE;
    }

    if g.enemy_vis == 0.0 {
        return FALSE;
    }

    let targ = g.r(s).enemy;

    // see if any entities are in the way of the shot
    let spot1 = g.r(s).origin + g.r(s).view_ofs;
    let spot2 = g.r(targ).origin + g.r(targ).view_ofs;

    if vlen(spot1 - spot2) > 600.0 {
        return FALSE;
    }

    g.traceline(spot1, spot2, FALSE, s);

    if g.trace_inopen && g.trace_inwater {
        return FALSE; // sight line crossed contents
    }

    if g.trace_ent != targ {
        return FALSE; // don't have a clear shot
    }

    // missile attack
    if g.enemy_range == RANGE_FAR {
        return FALSE;
    }

    g.v(s).attack_state = AS_MISSILE;
    let r = 2.0 + 2.0 * g.random();
    SUB_AttackFinished(g, r);
    TRUE
}

//============================================================================

/// The player is in view, so decide to move or launch an attack. FALSE if movement should
/// continue.
pub fn OgreCheckAttack(g: &mut Game) -> f32 {
    let s = g.self_;

    if g.enemy_range == RANGE_MELEE {
        let e = g.r(s).enemy;
        if CanDamage(g, e, s) != 0.0 {
            g.v(g.self_).attack_state = AS_MELEE;
            return TRUE;
        }
    }

    let s = g.self_;
    if g.time < g.r(s).attack_finished {
        return FALSE;
    }

    if g.enemy_vis == 0.0 {
        return FALSE;
    }

    let targ = g.r(s).enemy;

    // see if any entities are in the way of the shot
    let spot1 = g.r(s).origin + g.r(s).view_ofs;
    let spot2 = g.r(targ).origin + g.r(targ).view_ofs;

    g.traceline(spot1, spot2, FALSE, s);

    if g.trace_inopen && g.trace_inwater {
        return FALSE; // sight line crossed contents
    }

    if g.trace_ent != targ {
        return FALSE; // don't have a clear shot
    }

    // missile attack
    if g.time < g.r(s).attack_finished {
        return FALSE;
    }

    // id computes a chance here and never uses it; the ogre always fires.
    let _chance: f32 = if g.enemy_range == RANGE_FAR {
        return FALSE;
    } else if g.enemy_range == RANGE_NEAR {
        0.10
    } else if g.enemy_range == RANGE_MID {
        0.05
    } else {
        0.0
    };

    g.v(s).attack_state = AS_MISSILE;
    let r = 1.0 + 2.0 * g.random();
    SUB_AttackFinished(g, r);
    TRUE
}
