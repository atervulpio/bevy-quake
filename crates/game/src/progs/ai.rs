//! `ai.qc`, id's QuakeC 1.06, in Rust.
//!
//! `.enemy` is world when the monster is angry at no one. `.movetarget` is the next path spot
//! to walk to; with an enemy it is ignored. `.ideal_yaw` is the yaw the monster turns towards.
//! `.pausetime`: a monster leaves its stand state for its `.movetarget` when time passes it.
//! `walkmove(angle, speed)` is all or nothing.
//!
//! The globals `sight_entity` and `sight_entity_time` live in `Game`: when a monster becomes
//! angry at a player, it is the sight target the next frame, so that monsters near it wake up
//! even if they would not have noticed the player.

#![allow(clippy::all)]

use super::*;

pub fn anglemod(g: &mut Game, v: f32) -> f32 {
    let _ = g;
    let mut v = v;
    while v >= 360.0 {
        v = v - 360.0;
    }
    while v < 0.0 {
        v = v + 360.0;
    }

    v
}

// ---- movetarget code ----------------------------------------------------------------------
// The angle of the movetarget affects standing and bowing direction, but has no effect on
// movement, which always heads to the next target. `targetname` must be present; `target` is
// the next spot to move to (none: stop here for good).

pub fn movetarget_f(g: &mut Game) {
    let s = g.self_;
    if g.r(s).targetname == "" {
        g.objerror("monster_movetarget: no targetname");
    }

    g.v(s).solid = SOLID_TRIGGER;
    g.v(s).touch = Some(t_movetarget);
    g.setsize(s, Vec3::new(-8.0, -8.0, -8.0), Vec3::new(8.0, 8.0, 8.0));
}

/// QUAKED path_corner: monsters will continue walking towards the next target corner.
pub fn path_corner(g: &mut Game) {
    movetarget_f(g);
}

/// Something has bumped into a movetarget. If it is a monster moving towards it, change the
/// next destination and continue.
pub fn t_movetarget(g: &mut Game) {
    if g.r(g.other).movetarget != g.self_ {
        return;
    }
    if g.r(g.other).enemy != world {
        return; // fighting, not following a path
    }

    // self and other swap, and are not put back: the engine restores them after a touch.
    let temp = g.self_;
    g.self_ = g.other;
    g.other = temp;

    let s = g.self_;
    if g.r(s).classname == "monster_ogre" {
        g.sound(s, CHAN_VOICE, "ogre/ogdrag.wav", 1.0, ATTN_IDLE); // play chainsaw drag sound
    }

    let t = g.find(world, |v: &EntVars| v.targetname, g.r(g.other).target);
    g.v(s).movetarget = t;
    g.v(s).goalentity = t;
    g.v(s).ideal_yaw = vectoyaw(g.r(g.r(s).goalentity).origin - g.r(s).origin);
    if g.r(s).movetarget == world {
        g.v(s).pausetime = g.time + 999999.0;
        let f = g.r(s).th_stand;
        g.call(f);
        return;
    }
}

//============================================================================

/// The range category of an entity relative to self:
/// RANGE_MELEE  melee range, will become hostile even if back is turned
/// RANGE_NEAR   visibility and infront, or visibility and show hostile
/// RANGE_MID    infront and show hostile
/// RANGE_FAR    only triggered by damage
pub fn range(g: &mut Game, targ: Ent) -> f32 {
    let s = g.self_;
    let spot1 = g.r(s).origin + g.r(s).view_ofs;
    let spot2 = g.r(targ).origin + g.r(targ).view_ofs;

    let r = vlen(spot1 - spot2);
    if r < 120.0 {
        return RANGE_MELEE as f32;
    }
    if r < 500.0 {
        return RANGE_NEAR as f32;
    }
    if r < 1000.0 {
        return RANGE_MID as f32;
    }

    RANGE_FAR as f32
}

/// 1 if the entity is visible to self, even if not infront().
pub fn visible(g: &mut Game, targ: Ent) -> f32 {
    let s = g.self_;
    let spot1 = g.r(s).origin + g.r(s).view_ofs;
    let spot2 = g.r(targ).origin + g.r(targ).view_ofs;
    g.traceline(spot1, spot2, TRUE, s); // see through other monsters

    if g.trace_inopen && g.trace_inwater {
        return FALSE; // sight line crossed contents
    }

    if g.trace_fraction == 1.0 {
        return TRUE;
    }

    FALSE
}

/// 1 if the entity is in front (in sight) of self.
pub fn infront(g: &mut Game, targ: Ent) -> f32 {
    let s = g.self_;
    g.makevectors(g.r(s).angles);
    let vec = normalize(g.r(targ).origin - g.r(s).origin);
    let dot = vec.dot(g.v_forward);

    if dot > 0.3 {
        return TRUE;
    }

    FALSE
}

// ChangeYaw was moved to C code by id: `g.ChangeYaw()`.

//============================================================================

pub fn HuntTarget(g: &mut Game) {
    let s = g.self_;
    g.v(s).goalentity = g.r(s).enemy;
    g.v(s).think = g.r(s).th_run;
    g.v(s).ideal_yaw = vectoyaw(g.r(g.r(s).enemy).origin - g.r(s).origin);
    g.v(s).nextthink = g.time + 0.1;
    SUB_AttackFinished(g, 1.0); // wait a while before first attack
}

pub fn SightSound(g: &mut Game) {
    let s = g.self_;
    let c = g.r(s).classname;

    if c == "monster_ogre" {
        g.sound(s, CHAN_VOICE, "ogre/ogwake.wav", 1.0, ATTN_NORM);
    } else if c == "monster_knight" {
        g.sound(s, CHAN_VOICE, "knight/ksight.wav", 1.0, ATTN_NORM);
    } else if c == "monster_shambler" {
        g.sound(s, CHAN_VOICE, "shambler/ssight.wav", 1.0, ATTN_NORM);
    } else if c == "monster_demon1" {
        g.sound(s, CHAN_VOICE, "demon/sight2.wav", 1.0, ATTN_NORM);
    } else if c == "monster_wizard" {
        g.sound(s, CHAN_VOICE, "wizard/wsight.wav", 1.0, ATTN_NORM);
    } else if c == "monster_zombie" {
        g.sound(s, CHAN_VOICE, "zombie/z_idle.wav", 1.0, ATTN_NORM);
    } else if c == "monster_dog" {
        g.sound(s, CHAN_VOICE, "dog/dsight.wav", 1.0, ATTN_NORM);
    } else if c == "monster_hell_knight" {
        g.sound(s, CHAN_VOICE, "hknight/sight1.wav", 1.0, ATTN_NORM);
    } else if c == "monster_tarbaby" {
        g.sound(s, CHAN_VOICE, "blob/sight1.wav", 1.0, ATTN_NORM);
    } else if c == "monster_vomit" {
        g.sound(s, CHAN_VOICE, "vomitus/v_sight1.wav", 1.0, ATTN_NORM);
    } else if c == "monster_enforcer" {
        let rsnd = rint(g.random() * 3.0);
        if rsnd == 1.0 {
            g.sound(s, CHAN_VOICE, "enforcer/sight1.wav", 1.0, ATTN_NORM);
        } else if rsnd == 2.0 {
            g.sound(s, CHAN_VOICE, "enforcer/sight2.wav", 1.0, ATTN_NORM);
        } else if rsnd == 0.0 {
            g.sound(s, CHAN_VOICE, "enforcer/sight3.wav", 1.0, ATTN_NORM);
        } else {
            g.sound(s, CHAN_VOICE, "enforcer/sight4.wav", 1.0, ATTN_NORM);
        }
    } else if c == "monster_army" {
        g.sound(s, CHAN_VOICE, "soldier/sight1.wav", 1.0, ATTN_NORM);
    } else if c == "monster_shalrath" {
        g.sound(s, CHAN_VOICE, "shalrath/sight.wav", 1.0, ATTN_NORM);
    }
}

pub fn FoundTarget(g: &mut Game) {
    let s = g.self_;
    if g.r(g.r(s).enemy).classname == "player" {
        // let other monsters see this monster for a while
        g.sight_entity = s;
        g.sight_entity_time = g.time;
    }

    g.v(s).show_hostile = g.time + 1.0; // wake up other monsters

    SightSound(g);
    HuntTarget(g);
}

/// Self is not attacking anything, so try to find a target. TRUE if an enemy was sighted.
///
/// When a player fires a missile, the point of impact becomes a fakeplayer so that monsters
/// that see the impact will respond as if they had seen the player. Only a single client is
/// checked each frame.
pub fn FindTarget(g: &mut Game) -> f32 {
    let s = g.self_;
    let client;

    // If the first spawnflag bit is set, the monster will only wake up on really seeing the
    // player, not another monster getting angry. spawnflags & 3 is id's hack: the crucified
    // zombie used the first flag before the ambush flag, so the second works as well.
    if g.sight_entity_time >= (g.time - 0.1) && !((g.r(s).spawnflags & 3) != 0) {
        client = g.sight_entity;
        if g.r(client).enemy == g.r(s).enemy {
            return 0.0; // a bare `return;`: QuakeC returns nought
        }
    } else {
        client = g.checkclient();
        if client == world {
            return FALSE; // current check entity isn't in PVS
        }
    }

    if client == g.r(s).enemy {
        return FALSE;
    }
    if (g.r(client).flags & FL_NOTARGET) != 0 {
        return FALSE;
    }
    if (g.r(client).items & IT_INVISIBILITY) != 0 {
        return FALSE;
    }

    let r = range(g, client);
    if r == RANGE_FAR as f32 {
        return FALSE;
    }

    if visible(g, client) == 0.0 {
        return FALSE;
    }

    if r == RANGE_NEAR as f32 {
        // both sides evaluated, as QuakeC does: infront() always runs
        let hostile_past = g.r(client).show_hostile < g.time;
        let not_infront = infront(g, client) == 0.0;
        if hostile_past && not_infront {
            return FALSE;
        }
    } else if r == RANGE_MID as f32 {
        if /* client.show_hostile < time || */ infront(g, client) == 0.0 {
            return FALSE;
        }
    }

    //
    // got one
    //
    g.v(s).enemy = client;
    if g.r(g.r(s).enemy).classname != "player" {
        g.v(s).enemy = g.r(g.r(s).enemy).enemy;
        if g.r(g.r(s).enemy).classname != "player" {
            g.v(s).enemy = world;
            return FALSE;
        }
    }

    FoundTarget(g);

    TRUE
}

//=============================================================================

pub fn ai_forward(g: &mut Game, dist: f32) {
    g.walkmove(g.r(g.self_).angles.y, dist);
}

pub fn ai_back(g: &mut Game, dist: f32) {
    g.walkmove(g.r(g.self_).angles.y + 180.0, dist);
}

/// Stagger back a bit.
pub fn ai_pain(g: &mut Game, dist: f32) {
    ai_back(g, dist);
}

/// Stagger back a bit.
pub fn ai_painforward(g: &mut Game, dist: f32) {
    g.walkmove(g.r(g.self_).ideal_yaw, dist);
}

/// The monster is walking its beat.
pub fn ai_walk(g: &mut Game, dist: f32) {
    g.movedist = dist;

    if g.r(g.self_).classname == "monster_dragon" {
        g.movetogoal(dist);
        return;
    }

    // check for noticing a player
    if FindTarget(g) != 0.0 {
        return;
    }

    g.movetogoal(dist);
}

/// The monster is staying in one place for a while, with slight angle turns.
pub fn ai_stand(g: &mut Game) {
    if FindTarget(g) != 0.0 {
        return;
    }

    let s = g.self_;
    if g.time > g.r(s).pausetime {
        let f = g.r(s).th_walk;
        g.call(f);
        return;
    }

    // change angle slightly
}

/// Don't move, but turn towards ideal_yaw.
pub fn ai_turn(g: &mut Game) {
    if FindTarget(g) != 0.0 {
        return;
    }

    g.ChangeYaw();
}

//=============================================================================

pub fn ChooseTurn(g: &mut Game, dest3: Vec3) {
    let s = g.self_;
    let mut dir = g.r(s).origin - dest3;

    let n = g.trace_plane_normal;
    let newdir = Vec3::new(n.y, 0.0 - n.x, 0.0);

    if dir.dot(newdir) > 0.0 {
        dir.x = 0.0 - n.y;
        dir.y = n.x;
    } else {
        dir.x = n.y;
        dir.y = 0.0 - n.x;
    }

    dir.z = 0.0;
    g.v(s).ideal_yaw = vectoyaw(dir);
}

pub fn FacingIdeal(g: &mut Game) -> f32 {
    let s = g.self_;
    let delta = anglemod(g, g.r(s).angles.y - g.r(s).ideal_yaw);
    if delta > 45.0 && delta < 315.0 {
        return FALSE;
    }

    TRUE
}

//=============================================================================

pub fn CheckAnyAttack(g: &mut Game) -> f32 {
    if g.enemy_vis == 0.0 {
        return 0.0; // a bare `return;`: QuakeC returns nought
    }

    let c = g.r(g.self_).classname;
    if c == "monster_army" {
        return SoldierCheckAttack(g);
    }
    if c == "monster_ogre" {
        return OgreCheckAttack(g);
    }
    if c == "monster_shambler" {
        return ShamCheckAttack(g);
    }
    if c == "monster_demon1" {
        return DemonCheckAttack(g);
    }
    if c == "monster_dog" {
        return DogCheckAttack(g);
    }
    if c == "monster_wizard" {
        return WizardCheckAttack(g);
    }

    CheckAttack(g)
}

/// Turn and close until within an angle to launch a melee attack.
pub fn ai_run_melee(g: &mut Game) {
    let s = g.self_;
    g.v(s).ideal_yaw = g.enemy_yaw;
    g.ChangeYaw();

    if FacingIdeal(g) != 0.0 {
        let f = g.r(s).th_melee;
        g.call(f);
        g.v(g.self_).attack_state = AS_STRAIGHT;
    }
}

/// Turn in place until within an angle to launch a missile attack.
pub fn ai_run_missile(g: &mut Game) {
    let s = g.self_;
    g.v(s).ideal_yaw = g.enemy_yaw;
    g.ChangeYaw();
    if FacingIdeal(g) != 0.0 {
        let f = g.r(s).th_missile;
        g.call(f);
        g.v(g.self_).attack_state = AS_STRAIGHT;
    }
}

/// Strafe sideways, but stay at approximately the same range.
pub fn ai_run_slide(g: &mut Game) {
    let s = g.self_;
    g.v(s).ideal_yaw = g.enemy_yaw;
    g.ChangeYaw();
    let ofs = if g.r(s).lefty != 0.0 { 90.0 } else { -90.0 };

    if g.walkmove(g.r(s).ideal_yaw + ofs, g.movedist) {
        return;
    }

    g.v(s).lefty = 1.0 - g.r(s).lefty;

    g.walkmove(g.r(s).ideal_yaw - ofs, g.movedist);
}

/// The monster has an enemy it is trying to kill.
pub fn ai_run(g: &mut Game, dist: f32) {
    let s = g.self_;
    g.movedist = dist;
    // see if the enemy is dead
    if g.r(g.r(s).enemy).health <= 0.0 {
        g.v(s).enemy = world;
        // FIXME: look all around for other targets
        if g.r(g.r(s).oldenemy).health > 0.0 {
            g.v(s).enemy = g.r(s).oldenemy;
            HuntTarget(g);
        } else {
            if g.r(s).movetarget != world {
                let f = g.r(s).th_walk;
                g.call(f);
            } else {
                let f = g.r(s).th_stand;
                g.call(f);
            }
            return;
        }
    }

    let s = g.self_; // read again after HuntTarget
    g.v(s).show_hostile = g.time + 1.0; // wake up other monsters

    // check knowledge of enemy
    g.enemy_vis = visible(g, g.r(s).enemy);
    if g.enemy_vis != 0.0 {
        g.v(s).search_time = g.time + 5.0;
    }

    // look for other coop players
    if g.coop != 0.0 && g.r(s).search_time < g.time {
        if FindTarget(g) != 0.0 {
            return;
        }
    }

    let s = g.self_; // read again after FindTarget
    g.enemy_infront = infront(g, g.r(s).enemy);
    g.enemy_range = range(g, g.r(s).enemy) as i32;
    g.enemy_yaw = vectoyaw(g.r(g.r(s).enemy).origin - g.r(s).origin);

    if g.r(s).attack_state == AS_MISSILE {
        ai_run_missile(g);
        return;
    }

    if g.r(s).attack_state == AS_MELEE {
        ai_run_melee(g);
        return;
    }

    if CheckAnyAttack(g) != 0.0 {
        return; // beginning an attack
    }

    let s = g.self_; // read again after CheckAnyAttack
    if g.r(s).attack_state == AS_SLIDING {
        ai_run_slide(g);
        return;
    }

    // head straight in
    g.movetogoal(dist); // done in C code...
}
