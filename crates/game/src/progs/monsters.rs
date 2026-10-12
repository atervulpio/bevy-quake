//! `monsters.qc`, id's QuakeC 1.06, in Rust. All monsters should be 1 0 0 in colour.

#![allow(clippy::all)]

use super::*;

/// Using a monster makes it angry at the current activator.
pub fn monster_use(g: &mut Game) {
    let s = g.self_;
    if g.r(s).enemy != world {
        return;
    }
    if g.r(s).health <= 0.0 {
        return;
    }

    let a = g.activator;
    if (g.r(a).items & IT_INVISIBILITY) != 0 {
        return;
    }
    if (g.r(a).flags & FL_NOTARGET) != 0 {
        return;
    }
    if g.r(a).classname != "player" {
        return;
    }

    // delay reaction so if the monster is teleported, its sound is still heard
    g.v(s).enemy = a;
    g.v(s).nextthink = g.time + 0.1;
    g.v(s).think = Some(FoundTarget);
}

/// When a monster dies, it fires all of its targets with the current enemy as activator.
pub fn monster_death_use(g: &mut Game) {
    let s = g.self_;

    // fall to ground
    if (g.r(s).flags & FL_FLY) != 0 {
        g.v(s).flags = g.r(s).flags - FL_FLY;
    }
    if (g.r(s).flags & FL_SWIM) != 0 {
        g.v(s).flags = g.r(s).flags - FL_SWIM;
    }

    if g.r(s).target == "" {
        return;
    }

    g.activator = g.r(s).enemy;
    SUB_UseTargets(g);
}

//============================================================================

pub fn walkmonster_start_go(g: &mut Game) {
    let s = g.self_;
    g.v(s).origin.z = g.r(s).origin.z + 1.0; // raise off floor a bit
    g.droptofloor();

    if !g.walkmove(0.0, 0.0) {
        g.dprint("walkmonster in wall at: ");
        g.dprint(&vtos(g.r(s).origin));
        g.dprint("\n");
    }

    g.v(s).takedamage = DAMAGE_AIM;

    g.v(s).ideal_yaw = g.r(s).angles.dot(Vec3::new(0.0, 1.0, 0.0));
    if g.r(s).yaw_speed == 0.0 {
        g.v(s).yaw_speed = 20.0;
    }
    g.v(s).view_ofs = Vec3::new(0.0, 0.0, 25.0);
    g.v(s).use_ = Some(monster_use);

    g.v(s).flags = g.r(s).flags | FL_MONSTER;

    if g.r(s).target != "" {
        let t = g.find(world, |v: &EntVars| v.targetname, g.r(s).target);
        g.v(s).movetarget = t;
        g.v(s).goalentity = t;
        g.v(s).ideal_yaw = vectoyaw(g.r(g.r(s).goalentity).origin - g.r(s).origin);
        if g.r(s).movetarget == world {
            // this used to be an objerror
            g.dprint("Monster can't find target at ");
            g.dprint(&vtos(g.r(s).origin));
            g.dprint("\n");
        }
        // id's missing braces kept: the `else` covers only the pausetime, and th_stand runs
        // after th_walk too.
        if g.r(g.r(s).movetarget).classname == "path_corner" {
            let f = g.r(s).th_walk;
            g.call(f);
        } else {
            g.v(g.self_).pausetime = 99999999.0;
        }
        let f = g.r(g.self_).th_stand;
        g.call(f);
    } else {
        g.v(s).pausetime = 99999999.0;
        let f = g.r(s).th_stand;
        g.call(f);
    }

    // spread think times so they don't all happen at same time
    let s = g.self_;
    g.v(s).nextthink = g.r(s).nextthink + g.random() * 0.5;
}

pub fn walkmonster_start(g: &mut Game) {
    let s = g.self_;
    // delay drop to floor to make sure all doors have been spawned
    // spread think times so they don't all happen at same time
    g.v(s).nextthink = g.r(s).nextthink + g.random() * 0.5;
    g.v(s).think = Some(walkmonster_start_go);
    g.total_monsters = g.total_monsters + 1.0;
}

pub fn flymonster_start_go(g: &mut Game) {
    let s = g.self_;
    g.v(s).takedamage = DAMAGE_AIM;

    g.v(s).ideal_yaw = g.r(s).angles.dot(Vec3::new(0.0, 1.0, 0.0));
    if g.r(s).yaw_speed == 0.0 {
        g.v(s).yaw_speed = 10.0;
    }
    g.v(s).view_ofs = Vec3::new(0.0, 0.0, 25.0);
    g.v(s).use_ = Some(monster_use);

    g.v(s).flags = g.r(s).flags | FL_FLY;
    g.v(s).flags = g.r(s).flags | FL_MONSTER;

    if !g.walkmove(0.0, 0.0) {
        g.dprint("flymonster in wall at: ");
        g.dprint(&vtos(g.r(s).origin));
        g.dprint("\n");
    }

    if g.r(s).target != "" {
        let t = g.find(world, |v: &EntVars| v.targetname, g.r(s).target);
        g.v(s).movetarget = t;
        g.v(s).goalentity = t;
        if g.r(s).movetarget == world {
            // this used to be an objerror
            g.dprint("Monster can't find target at ");
            g.dprint(&vtos(g.r(s).origin));
            g.dprint("\n");
        }
        // id's missing braces kept: th_stand runs after th_walk too.
        if g.r(g.r(s).movetarget).classname == "path_corner" {
            let f = g.r(s).th_walk;
            g.call(f);
        } else {
            g.v(g.self_).pausetime = 99999999.0;
        }
        let f = g.r(g.self_).th_stand;
        g.call(f);
    } else {
        g.v(s).pausetime = 99999999.0;
        let f = g.r(s).th_stand;
        g.call(f);
    }
}

pub fn flymonster_start(g: &mut Game) {
    let s = g.self_;
    // spread think times so they don't all happen at same time
    g.v(s).nextthink = g.r(s).nextthink + g.random() * 0.5;
    g.v(s).think = Some(flymonster_start_go);
    g.total_monsters = g.total_monsters + 1.0;
}

pub fn swimmonster_start_go(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.v(s).takedamage = DAMAGE_AIM;
    // id counts the swimming monster a second time here (swimmonster_start counted it already)
    g.total_monsters = g.total_monsters + 1.0;

    g.v(s).ideal_yaw = g.r(s).angles.dot(Vec3::new(0.0, 1.0, 0.0));
    if g.r(s).yaw_speed == 0.0 {
        g.v(s).yaw_speed = 10.0;
    }
    g.v(s).view_ofs = Vec3::new(0.0, 0.0, 10.0);
    g.v(s).use_ = Some(monster_use);

    g.v(s).flags = g.r(s).flags | FL_SWIM;
    g.v(s).flags = g.r(s).flags | FL_MONSTER;

    if g.r(s).target != "" {
        let t = g.find(world, |v: &EntVars| v.targetname, g.r(s).target);
        g.v(s).movetarget = t;
        g.v(s).goalentity = t;
        if g.r(s).movetarget == world {
            // this used to be an objerror
            g.dprint("Monster can't find target at ");
            g.dprint(&vtos(g.r(s).origin));
            g.dprint("\n");
        }
        g.v(s).ideal_yaw = vectoyaw(g.r(g.r(s).goalentity).origin - g.r(s).origin);
        let f = g.r(s).th_walk;
        g.call(f);
    } else {
        g.v(s).pausetime = 99999999.0;
        let f = g.r(s).th_stand;
        g.call(f);
    }

    // spread think times so they don't all happen at same time
    let s = g.self_;
    g.v(s).nextthink = g.r(s).nextthink + g.random() * 0.5;
}

pub fn swimmonster_start(g: &mut Game) {
    let s = g.self_;
    // spread think times so they don't all happen at same time
    g.v(s).nextthink = g.r(s).nextthink + g.random() * 0.5;
    g.v(s).think = Some(swimmonster_start_go);
    g.total_monsters = g.total_monsters + 1.0;
}
