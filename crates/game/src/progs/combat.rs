//! `combat.qc`, id's QuakeC 1.06, in Rust.

#![allow(clippy::all)]

use super::*;

/// Returns true if the inflictor can directly damage the target. Used for explosions and melee
/// attacks.
pub fn CanDamage(g: &mut Game, targ: Ent, inflictor: Ent) -> f32 {
    let s = g.self_;
    // bmodels need special checking because their origin is 0, 0, 0
    if g.r(targ).movetype == MOVETYPE_PUSH {
        g.traceline(g.r(inflictor).origin, 0.5 * (g.r(targ).absmin + g.r(targ).absmax), TRUE, s);
        if g.trace_fraction == 1.0 {
            return TRUE;
        }
        if g.trace_ent == targ {
            return TRUE;
        }

        return FALSE;
    }

    g.traceline(g.r(inflictor).origin, g.r(targ).origin, TRUE, s);
    if g.trace_fraction == 1.0 {
        return TRUE;
    }
    g.traceline(g.r(inflictor).origin, g.r(targ).origin + Vec3::new(15.0, 15.0, 0.0), TRUE, s);
    if g.trace_fraction == 1.0 {
        return TRUE;
    }
    g.traceline(g.r(inflictor).origin, g.r(targ).origin + Vec3::new(-15.0, -15.0, 0.0), TRUE, s);
    if g.trace_fraction == 1.0 {
        return TRUE;
    }
    g.traceline(g.r(inflictor).origin, g.r(targ).origin + Vec3::new(-15.0, 15.0, 0.0), TRUE, s);
    if g.trace_fraction == 1.0 {
        return TRUE;
    }
    g.traceline(g.r(inflictor).origin, g.r(targ).origin + Vec3::new(15.0, -15.0, 0.0), TRUE, s);
    if g.trace_fraction == 1.0 {
        return TRUE;
    }

    FALSE
}

pub fn Killed(g: &mut Game, targ: Ent, attacker: Ent) {
    let oself = g.self_;
    g.self_ = targ;
    let s = targ;

    if g.r(s).health < -99.0 {
        g.v(s).health = -99.0; // don't let sbar look bad if a player
    }

    if g.r(s).movetype == MOVETYPE_PUSH || g.r(s).movetype == MOVETYPE_NONE {
        // doors, triggers, etc
        let f = g.r(g.self_).th_die;
        g.call(f);
        g.self_ = oself;
        return;
    }

    g.v(s).enemy = attacker;

    // bump the monster counter
    if g.r(s).flags & FL_MONSTER != 0 {
        g.killed_monsters = g.killed_monsters + 1.0;
        g.msg_killed_monster();
    }

    ClientObituary(g, g.self_, attacker);

    let s = g.self_;
    g.v(s).takedamage = DAMAGE_NO;
    g.v(s).touch = Some(SUB_Null);

    monster_death_use(g);
    let f = g.r(g.self_).th_die;
    g.call(f);

    g.self_ = oself;
}

/// The damage is coming from inflictor, but get mad at attacker. This should be the only
/// function that ever reduces health.
pub fn T_Damage(g: &mut Game, targ: Ent, inflictor: Ent, attacker: Ent, mut damage: f32) {
    if g.r(targ).takedamage == 0 {
        return;
    }

    // used by buttons and triggers to set activator for target firing
    g.damage_attacker = attacker;

    // check for quad damage powerup on the attacker
    if g.r(attacker).super_damage_finished > g.time {
        damage = damage * 4.0;
    }

    // save damage based on the target's armor level
    let mut save = ceil(g.r(targ).armortype * damage);
    if save >= g.r(targ).armorvalue {
        save = g.r(targ).armorvalue;
        g.v(targ).armortype = 0.0; // lost all armor
        g.v(targ).items = g.r(targ).items - (g.r(targ).items & (IT_ARMOR1 | IT_ARMOR2 | IT_ARMOR3));
    }

    g.v(targ).armorvalue = g.r(targ).armorvalue - save;
    let take = ceil(damage - save);

    // add to the damage total for clients, which will be sent as a single message at the end of
    // the frame
    if g.r(targ).flags & FL_CLIENT != 0 {
        g.v(targ).dmg_take = g.r(targ).dmg_take + take;
        g.v(targ).dmg_save = g.r(targ).dmg_save + save;
        g.v(targ).dmg_inflictor = inflictor;
    }

    // figure momentum add
    if inflictor != world && g.r(targ).movetype == MOVETYPE_WALK {
        let mut dir = g.r(targ).origin - (g.r(inflictor).absmin + g.r(inflictor).absmax) * 0.5;
        dir = normalize(dir);
        g.v(targ).velocity = g.r(targ).velocity + dir * damage * 8.0;
    }

    // check for godmode or invincibility
    if g.r(targ).flags & FL_GODMODE != 0 {
        return;
    }
    if g.r(targ).invincible_finished >= g.time {
        // id's code reads and sets `self`'s invincible_sound here, not the target's
        let s = g.self_;
        if g.r(s).invincible_sound < g.time {
            g.sound(targ, CHAN_ITEM, "items/protect3.wav", 1.0, ATTN_NORM);
            g.v(s).invincible_sound = g.time + 2.0;
        }
        return;
    }

    // team play damage avoidance
    if g.teamplay == 1.0 && g.r(targ).team > 0.0 && g.r(targ).team == g.r(attacker).team {
        return;
    }

    // do the damage
    g.v(targ).health = g.r(targ).health - take;

    if g.r(targ).health <= 0.0 {
        Killed(g, targ, attacker);
        return;
    }

    // react to the damage
    let oldself = g.self_;
    g.self_ = targ;
    let s = targ;

    if (g.r(s).flags & FL_MONSTER != 0) && attacker != world {
        // get mad unless of the same class (except for soldiers)
        if s != attacker && attacker != g.r(s).enemy {
            if g.r(s).classname != g.r(attacker).classname || g.r(s).classname == "monster_army" {
                if g.r(g.r(s).enemy).classname == "player" {
                    g.v(s).oldenemy = g.r(s).enemy;
                }
                g.v(s).enemy = attacker;
                FoundTarget(g);
            }
        }
    }

    let f = g.r(g.self_).th_pain;
    if f.is_some() {
        g.call_pain(f, attacker, take);
        // nightmare mode monsters don't go into pain frames often
        if g.skill == 3.0 {
            let s = g.self_;
            g.v(s).pain_finished = g.time + 5.0;
        }
    }

    g.self_ = oldself;
}

pub fn T_RadiusDamage(g: &mut Game, inflictor: Ent, attacker: Ent, damage: f32, ignore: Ent) {
    let mut head = g.findradius(g.r(inflictor).origin, damage + 40.0);

    while head != world {
        if head != ignore {
            if g.r(head).takedamage != 0 {
                let org = g.r(head).origin + (g.r(head).mins + g.r(head).maxs) * 0.5;
                let mut points = 0.5 * vlen(g.r(inflictor).origin - org);
                if points < 0.0 {
                    points = 0.0;
                }
                points = damage - points;
                if head == attacker {
                    points = points * 0.5;
                }
                if points > 0.0 {
                    if CanDamage(g, head, inflictor) != 0.0 {
                        // shambler takes half damage from all explosions
                        if g.r(head).classname == "monster_shambler" {
                            T_Damage(g, head, inflictor, attacker, points * 0.5);
                        } else {
                            T_Damage(g, head, inflictor, attacker, points);
                        }
                    }
                }
            }
        }
        head = g.r(head).chain;
    }
}

pub fn T_BeamDamage(g: &mut Game, attacker: Ent, damage: f32) {
    let mut head = g.findradius(g.r(attacker).origin, damage + 40.0);

    while head != world {
        if g.r(head).takedamage != 0 {
            let mut points = 0.5 * vlen(g.r(attacker).origin - g.r(head).origin);
            if points < 0.0 {
                points = 0.0;
            }
            points = damage - points;
            if head == attacker {
                points = points * 0.5;
            }
            if points > 0.0 {
                if CanDamage(g, head, attacker) != 0.0 {
                    if g.r(head).classname == "monster_shambler" {
                        T_Damage(g, head, attacker, attacker, points * 0.5);
                    } else {
                        T_Damage(g, head, attacker, attacker, points);
                    }
                }
            }
        }
        head = g.r(head).chain;
    }
}
