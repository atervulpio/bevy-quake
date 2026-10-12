//! `items.qc`, id's QuakeC 1.06, in Rust.

#![allow(clippy::all)]

use super::*;

// ALL LIGHTS SHOULD BE 0 1 0 IN COLOR ALL OTHER ITEMS SHOULD BE .8 .3 .4 IN COLOR

pub fn SUB_regen(g: &mut Game) {
    let s = g.self_;
    g.v(s).model = g.r(s).mdl; // restore original model
    g.v(s).solid = SOLID_TRIGGER; // allow it to be touched again
    g.sound(s, CHAN_VOICE, "items/itembk2.wav", 1.0, ATTN_NORM); // play respawn sound
    g.setorigin(s, g.r(s).origin);
}

/// Prints a warning message when spawned.
pub fn noclass(g: &mut Game) {
    let s = g.self_;
    g.dprint("noclass spawned at");
    g.dprint(&vtos(g.r(s).origin));
    g.dprint("\n");
    g.remove(s);
}

/// Plants the object on the floor.
pub fn PlaceItem(g: &mut Game) {
    let s = g.self_;
    g.v(s).mdl = g.r(s).model; // so it can be restored on respawn
    g.v(s).flags = FL_ITEM; // make extra wide
    g.v(s).solid = SOLID_TRIGGER;
    g.v(s).movetype = MOVETYPE_TOSS;
    g.v(s).velocity = Vec3::ZERO;
    g.v(s).origin.z = g.r(s).origin.z + 6.0;
    // id's `oldz = self.origin_z;` is never read, and is left out.
    if !g.droptofloor() {
        g.dprint("Bonus item fell out of level at ");
        g.dprint(&vtos(g.r(s).origin));
        g.dprint("\n");
        g.remove(s);
        return;
    }
}

/// Sets the clipping size and plants the object on the floor.
pub fn StartItem(g: &mut Game) {
    let s = g.self_;
    g.v(s).nextthink = g.time + 0.2; // items start after other solids
    g.v(s).think = Some(PlaceItem);
}

// ---- health box -------------------------------------------------------------------------------

/// Add health to an entity, limiting health to max_health; `ignore` ignores the max_health
/// limit. As in id's code, the limit is `other`'s, not `e`'s.
pub fn T_Heal(g: &mut Game, e: Ent, mut healamount: f32, ignore: f32) -> f32 {
    let o = g.other;
    if g.r(e).health <= 0.0 {
        return FALSE;
    }
    if ignore == 0.0 && g.r(e).health >= g.r(o).max_health {
        return FALSE;
    }

    healamount = ceil(healamount);

    g.v(e).health = g.r(e).health + healamount;
    if ignore == 0.0 && g.r(e).health >= g.r(o).max_health {
        g.v(e).health = g.r(o).max_health;
    }

    if g.r(e).health > 250.0 {
        g.v(e).health = 250.0;
    }

    TRUE
}

/// `item_health`'s spawnflags: a rotten box heals 5-10 points (15 in fact), a megahealth adds
/// 100, then rots down to the maximum one point a second.
pub const H_ROTTEN: i32 = 1;
pub const H_MEGA: i32 = 2;

pub fn item_health(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(health_touch);

    if g.r(s).spawnflags & H_ROTTEN != 0 {
        g.precache_model("maps/b_bh10.bsp");

        g.precache_sound("items/r_item1.wav");
        g.setmodel(s, "maps/b_bh10.bsp");
        g.v(s).noise = "items/r_item1.wav";
        g.v(s).healamount = 15.0;
        g.v(s).healtype = 0.0;
    } else if g.r(s).spawnflags & H_MEGA != 0 {
        g.precache_model("maps/b_bh100.bsp");
        g.precache_sound("items/r_item2.wav");
        g.setmodel(s, "maps/b_bh100.bsp");
        g.v(s).noise = "items/r_item2.wav";
        g.v(s).healamount = 100.0;
        g.v(s).healtype = 2.0;
    } else {
        g.precache_model("maps/b_bh25.bsp");
        g.precache_sound("items/health1.wav");
        g.setmodel(s, "maps/b_bh25.bsp");
        g.v(s).noise = "items/health1.wav";
        g.v(s).healamount = 25.0;
        g.v(s).healtype = 1.0;
    }
    g.setsize(s, Vec3::new(0.0, 0.0, 0.0), Vec3::new(32.0, 32.0, 56.0));
    StartItem(g);
}

pub fn health_touch(g: &mut Game) {
    let s = g.self_;
    let o = g.other;

    if g.r(o).classname != "player" {
        return;
    }

    if g.r(s).healtype == 2.0 {
        // Megahealth? Ignore max_health...
        if g.r(o).health >= 250.0 {
            return;
        }
        if T_Heal(g, o, g.r(s).healamount, TRUE) == 0.0 {
            return;
        }
    } else {
        if T_Heal(g, o, g.r(s).healamount, FALSE) == 0.0 {
            return;
        }
    }

    g.sprint(o, "You receive ");
    let st = ftos(g.r(s).healamount);
    g.sprint(o, &st);
    g.sprint(o, " health\n");

    // health touch sound
    g.sound(o, CHAN_ITEM, g.r(s).noise, 1.0, ATTN_NORM);

    g.stuffcmd(o, "bf\n");

    g.v(s).model = g.string_null;
    g.v(s).solid = SOLID_NOT;

    // Megahealth = rot down the player's super health
    if g.r(s).healtype == 2.0 {
        g.v(o).items = g.r(o).items | IT_SUPERHEALTH;
        g.v(s).nextthink = g.time + 5.0;
        g.v(s).think = Some(item_megahealth_rot);
        g.v(s).owner = o;
    } else {
        if g.deathmatch != 2.0 {
            // deathmatch 2 is the silly old rules
            if g.deathmatch != 0.0 {
                g.v(s).nextthink = g.time + 20.0;
            }
            g.v(s).think = Some(SUB_regen);
        }
    }

    g.activator = o;
    SUB_UseTargets(g); // fire all targets / killtargets
}

pub fn item_megahealth_rot(g: &mut Game) {
    let s = g.self_;
    g.other = g.r(s).owner;
    let o = g.other;

    if g.r(o).health > g.r(o).max_health {
        g.v(o).health = g.r(o).health - 1.0;
        g.v(s).nextthink = g.time + 1.0;
        return;
    }

    // it is possible for a player to die and respawn between rots, so don't just blindly
    // subtract the flag off
    g.v(o).items = g.r(o).items - (g.r(o).items & IT_SUPERHEALTH);

    if g.deathmatch == 1.0 {
        // deathmatch 2 is silly old rules
        g.v(s).nextthink = g.time + 20.0;
        g.v(s).think = Some(SUB_regen);
    }
}

// ---- armor ------------------------------------------------------------------------------------

pub fn armor_touch(g: &mut Game) {
    let s = g.self_;
    let o = g.other;
    // QuakeC leaves these as they were if no classname below matches; here they start at nought.
    let mut type_: f32 = 0.0;
    let mut value: f32 = 0.0;
    let mut bit: i32 = 0;

    if g.r(o).health <= 0.0 {
        return;
    }
    if g.r(o).classname != "player" {
        return;
    }

    if g.r(s).classname == "item_armor1" {
        type_ = 0.3;
        value = 100.0;
        bit = IT_ARMOR1;
    } else if g.r(s).classname == "item_armor2" {
        type_ = 0.6;
        value = 150.0;
        bit = IT_ARMOR2;
    } else if g.r(s).classname == "item_armorInv" {
        type_ = 0.8;
        value = 200.0;
        bit = IT_ARMOR3;
    }

    if (g.r(o).armortype * g.r(o).armorvalue) >= (type_ * value) {
        return;
    }

    g.v(o).armortype = type_;
    g.v(o).armorvalue = value;
    g.v(o).items = g.r(o).items - (g.r(o).items & (IT_ARMOR1 | IT_ARMOR2 | IT_ARMOR3)) + bit;

    g.v(s).solid = SOLID_NOT;
    g.v(s).model = g.string_null;
    if g.deathmatch == 1.0 {
        g.v(s).nextthink = g.time + 20.0;
    }
    g.v(s).think = Some(SUB_regen);

    g.sprint(o, "You got armor\n");
    // armor touch sound
    g.sound(o, CHAN_ITEM, "items/armor1.wav", 1.0, ATTN_NORM);
    g.stuffcmd(o, "bf\n");

    g.activator = o;
    SUB_UseTargets(g); // fire all targets / killtargets
}

pub fn item_armor1(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(armor_touch);
    g.precache_model("progs/armor.mdl");
    g.setmodel(s, "progs/armor.mdl");
    g.v(s).skin = 0.0;
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

pub fn item_armor2(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(armor_touch);
    g.precache_model("progs/armor.mdl");
    g.setmodel(s, "progs/armor.mdl");
    g.v(s).skin = 1.0;
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

pub fn item_armorInv(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(armor_touch);
    g.precache_model("progs/armor.mdl");
    g.setmodel(s, "progs/armor.mdl");
    g.v(s).skin = 2.0;
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

// ---- weapons ----------------------------------------------------------------------------------

pub fn bound_other_ammo(g: &mut Game) {
    let o = g.other;
    if g.r(o).ammo_shells > 100.0 {
        g.v(o).ammo_shells = 100.0;
    }
    if g.r(o).ammo_nails > 200.0 {
        g.v(o).ammo_nails = 200.0;
    }
    if g.r(o).ammo_rockets > 100.0 {
        g.v(o).ammo_rockets = 100.0;
    }
    if g.r(o).ammo_cells > 100.0 {
        g.v(o).ammo_cells = 100.0;
    }
}

pub fn RankForWeapon(_g: &mut Game, w: f32) -> f32 {
    if w == IT_LIGHTNING as f32 {
        return 1.0;
    }
    if w == IT_ROCKET_LAUNCHER as f32 {
        return 2.0;
    }
    if w == IT_SUPER_NAILGUN as f32 {
        return 3.0;
    }
    if w == IT_GRENADE_LAUNCHER as f32 {
        return 4.0;
    }
    if w == IT_SUPER_SHOTGUN as f32 {
        return 5.0;
    }
    if w == IT_NAILGUN as f32 {
        return 6.0;
    }

    7.0
}

/// Deathmatch weapon change rules for picking up a weapon.
pub fn Deathmatch_Weapon(g: &mut Game, _old: f32, new: f32) {
    let s = g.self_;

    // change self.weapon if desired
    let or = RankForWeapon(g, g.r(s).weapon as f32);
    let nr = RankForWeapon(g, new);
    if nr < or {
        g.v(s).weapon = new as i32;
    }
}

pub fn weapon_touch(g: &mut Game) {
    let o = g.other;

    if g.r(o).flags & FL_CLIENT == 0 {
        return;
    }

    // if the player was using his best weapon, change up to the new one if better
    // (id's code never reads `best` again here, but the call is kept)
    let stemp = g.self_;
    g.self_ = o;
    W_BestWeapon(g);
    g.self_ = stemp;
    let s = g.self_;

    let leave = if g.deathmatch == 2.0 || g.coop != 0.0 { TRUE } else { FALSE };

    // id's `hadammo`, set in each branch below, is never read, and is left out.
    let new: i32;
    if g.r(s).classname == "weapon_nailgun" {
        if leave != 0.0 && (g.r(o).items & IT_NAILGUN != 0) {
            return;
        }

        new = IT_NAILGUN;
        g.v(o).ammo_nails = g.r(o).ammo_nails + 30.0;
    } else if g.r(s).classname == "weapon_supernailgun" {
        if leave != 0.0 && (g.r(o).items & IT_SUPER_NAILGUN != 0) {
            return;
        }

        new = IT_SUPER_NAILGUN;
        g.v(o).ammo_nails = g.r(o).ammo_nails + 30.0;
    } else if g.r(s).classname == "weapon_supershotgun" {
        if leave != 0.0 && (g.r(o).items & IT_SUPER_SHOTGUN != 0) {
            return;
        }

        new = IT_SUPER_SHOTGUN;
        g.v(o).ammo_shells = g.r(o).ammo_shells + 5.0;
    } else if g.r(s).classname == "weapon_rocketlauncher" {
        if leave != 0.0 && (g.r(o).items & IT_ROCKET_LAUNCHER != 0) {
            return;
        }

        new = IT_ROCKET_LAUNCHER;
        g.v(o).ammo_rockets = g.r(o).ammo_rockets + 5.0;
    } else if g.r(s).classname == "weapon_grenadelauncher" {
        if leave != 0.0 && (g.r(o).items & IT_GRENADE_LAUNCHER != 0) {
            return;
        }

        new = IT_GRENADE_LAUNCHER;
        g.v(o).ammo_rockets = g.r(o).ammo_rockets + 5.0;
    } else if g.r(s).classname == "weapon_lightning" {
        if leave != 0.0 && (g.r(o).items & IT_LIGHTNING != 0) {
            return;
        }

        new = IT_LIGHTNING;
        g.v(o).ammo_cells = g.r(o).ammo_cells + 15.0;
    } else {
        g.objerror("weapon_touch: unknown classname");
    }

    g.sprint(o, "You got the ");
    g.sprint(o, g.r(s).netname);
    g.sprint(o, "\n");
    // weapon touch sound
    g.sound(o, CHAN_ITEM, "weapons/pkup.wav", 1.0, ATTN_NORM);
    g.stuffcmd(o, "bf\n");

    bound_other_ammo(g);

    // change to the weapon
    let old = g.r(o).items;
    g.v(o).items = g.r(o).items | new;

    let stemp = g.self_;
    g.self_ = o;

    if g.deathmatch == 0.0 {
        let me = g.self_;
        g.v(me).weapon = new;
    } else {
        Deathmatch_Weapon(g, old as f32, new as f32);
    }

    W_SetCurrentAmmo(g);

    g.self_ = stemp;
    let s = g.self_;

    if leave != 0.0 {
        return;
    }

    // remove it in single player, or setup for respawning in deathmatch
    g.v(s).model = g.string_null;
    g.v(s).solid = SOLID_NOT;
    if g.deathmatch == 1.0 {
        g.v(s).nextthink = g.time + 30.0;
    }
    g.v(s).think = Some(SUB_regen);

    g.activator = g.other;
    SUB_UseTargets(g); // fire all targets / killtargets
}

pub fn weapon_supershotgun(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/g_shot.mdl");
    g.setmodel(s, "progs/g_shot.mdl");
    g.v(s).weapon = IT_SUPER_SHOTGUN;
    g.v(s).netname = "Double-barrelled Shotgun";
    g.v(s).touch = Some(weapon_touch);
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

pub fn weapon_nailgun(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/g_nail.mdl");
    g.setmodel(s, "progs/g_nail.mdl");
    g.v(s).weapon = IT_NAILGUN;
    g.v(s).netname = "Nailgun";
    g.v(s).touch = Some(weapon_touch);
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

pub fn weapon_supernailgun(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/g_nail2.mdl");
    g.setmodel(s, "progs/g_nail2.mdl");
    g.v(s).weapon = IT_SUPER_NAILGUN;
    g.v(s).netname = "Super Nailgun";
    g.v(s).touch = Some(weapon_touch);
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

pub fn weapon_grenadelauncher(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/g_rock.mdl");
    g.setmodel(s, "progs/g_rock.mdl");
    g.v(s).weapon = 3;
    g.v(s).netname = "Grenade Launcher";
    g.v(s).touch = Some(weapon_touch);
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

pub fn weapon_rocketlauncher(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/g_rock2.mdl");
    g.setmodel(s, "progs/g_rock2.mdl");
    g.v(s).weapon = 3;
    g.v(s).netname = "Rocket Launcher";
    g.v(s).touch = Some(weapon_touch);
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

pub fn weapon_lightning(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/g_light.mdl");
    g.setmodel(s, "progs/g_light.mdl");
    g.v(s).weapon = 3;
    g.v(s).netname = "Thunderbolt";
    g.v(s).touch = Some(weapon_touch);
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    StartItem(g);
}

// ---- ammo -------------------------------------------------------------------------------------

pub fn ammo_touch(g: &mut Game) {
    let o = g.other;

    if g.r(o).classname != "player" {
        return;
    }
    if g.r(o).health <= 0.0 {
        return;
    }

    // if the player was using his best weapon, change up to the new one if better
    let stemp = g.self_;
    g.self_ = o;
    let best = W_BestWeapon(g);
    g.self_ = stemp;
    let s = g.self_;

    if g.r(s).weapon == 1 {
        // shotgun
        if g.r(o).ammo_shells >= 100.0 {
            return;
        }

        g.v(o).ammo_shells = g.r(o).ammo_shells + g.r(s).aflag;
    } else if g.r(s).weapon == 2 {
        // spikes
        if g.r(o).ammo_nails >= 200.0 {
            return;
        }

        g.v(o).ammo_nails = g.r(o).ammo_nails + g.r(s).aflag;
    } else if g.r(s).weapon == 3 {
        // rockets
        if g.r(o).ammo_rockets >= 100.0 {
            return;
        }

        g.v(o).ammo_rockets = g.r(o).ammo_rockets + g.r(s).aflag;
    } else if g.r(s).weapon == 4 {
        // cells
        if g.r(o).ammo_cells >= 100.0 {
            return;
        }

        g.v(o).ammo_cells = g.r(o).ammo_cells + g.r(s).aflag;
    }

    bound_other_ammo(g);

    g.sprint(o, "You got the ");
    g.sprint(o, g.r(s).netname);
    g.sprint(o, "\n");
    // ammo touch sound
    g.sound(o, CHAN_ITEM, "weapons/lock4.wav", 1.0, ATTN_NORM);
    g.stuffcmd(o, "bf\n");

    // change to a better weapon if appropriate
    if g.r(o).weapon as f32 == best {
        let stemp = g.self_;
        g.self_ = o;
        let w = W_BestWeapon(g);
        let me = g.self_;
        g.v(me).weapon = w as i32;
        W_SetCurrentAmmo(g);
        g.self_ = stemp;
    }

    // if changed current ammo, update it
    let stemp = g.self_;
    g.self_ = o;
    W_SetCurrentAmmo(g);
    g.self_ = stemp;
    let s = g.self_;

    // remove it in single player, or setup for respawning in deathmatch
    g.v(s).model = g.string_null;
    g.v(s).solid = SOLID_NOT;
    if g.deathmatch == 1.0 {
        g.v(s).nextthink = g.time + 30.0;
    }
    g.v(s).think = Some(SUB_regen);

    g.activator = g.other;
    SUB_UseTargets(g); // fire all targets / killtargets
}

pub const WEAPON_BIG2: i32 = 1;

pub fn item_shells(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(ammo_touch);

    if g.r(s).spawnflags & WEAPON_BIG2 != 0 {
        g.precache_model("maps/b_shell1.bsp");
        g.setmodel(s, "maps/b_shell1.bsp");
        g.v(s).aflag = 40.0;
    } else {
        g.precache_model("maps/b_shell0.bsp");
        g.setmodel(s, "maps/b_shell0.bsp");
        g.v(s).aflag = 20.0;
    }
    g.v(s).weapon = 1;
    g.v(s).netname = "shells";
    g.setsize(s, Vec3::new(0.0, 0.0, 0.0), Vec3::new(32.0, 32.0, 56.0));
    StartItem(g);
}

pub fn item_spikes(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(ammo_touch);

    if g.r(s).spawnflags & WEAPON_BIG2 != 0 {
        g.precache_model("maps/b_nail1.bsp");
        g.setmodel(s, "maps/b_nail1.bsp");
        g.v(s).aflag = 50.0;
    } else {
        g.precache_model("maps/b_nail0.bsp");
        g.setmodel(s, "maps/b_nail0.bsp");
        g.v(s).aflag = 25.0;
    }
    g.v(s).weapon = 2;
    g.v(s).netname = "nails";
    g.setsize(s, Vec3::new(0.0, 0.0, 0.0), Vec3::new(32.0, 32.0, 56.0));
    StartItem(g);
}

pub fn item_rockets(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(ammo_touch);

    if g.r(s).spawnflags & WEAPON_BIG2 != 0 {
        g.precache_model("maps/b_rock1.bsp");
        g.setmodel(s, "maps/b_rock1.bsp");
        g.v(s).aflag = 10.0;
    } else {
        g.precache_model("maps/b_rock0.bsp");
        g.setmodel(s, "maps/b_rock0.bsp");
        g.v(s).aflag = 5.0;
    }
    g.v(s).weapon = 3;
    g.v(s).netname = "rockets";
    g.setsize(s, Vec3::new(0.0, 0.0, 0.0), Vec3::new(32.0, 32.0, 56.0));
    StartItem(g);
}

pub fn item_cells(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(ammo_touch);

    if g.r(s).spawnflags & WEAPON_BIG2 != 0 {
        g.precache_model("maps/b_batt1.bsp");
        g.setmodel(s, "maps/b_batt1.bsp");
        g.v(s).aflag = 12.0;
    } else {
        g.precache_model("maps/b_batt0.bsp");
        g.setmodel(s, "maps/b_batt0.bsp");
        g.v(s).aflag = 6.0;
    }
    g.v(s).weapon = 4;
    g.v(s).netname = "cells";
    g.setsize(s, Vec3::new(0.0, 0.0, 0.0), Vec3::new(32.0, 32.0, 56.0));
    StartItem(g);
}

// item_weapon: DO NOT USE THIS!!!! IT WILL BE REMOVED!
pub const WEAPON_SHOTGUN: i32 = 1;
pub const WEAPON_ROCKET: i32 = 2;
pub const WEAPON_SPIKES: i32 = 4;
pub const WEAPON_BIG: i32 = 8;

pub fn item_weapon(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(ammo_touch);

    if g.r(s).spawnflags & WEAPON_SHOTGUN != 0 {
        if g.r(s).spawnflags & WEAPON_BIG != 0 {
            g.precache_model("maps/b_shell1.bsp");
            g.setmodel(s, "maps/b_shell1.bsp");
            g.v(s).aflag = 40.0;
        } else {
            g.precache_model("maps/b_shell0.bsp");
            g.setmodel(s, "maps/b_shell0.bsp");
            g.v(s).aflag = 20.0;
        }
        g.v(s).weapon = 1;
        g.v(s).netname = "shells";
    }

    if g.r(s).spawnflags & WEAPON_SPIKES != 0 {
        if g.r(s).spawnflags & WEAPON_BIG != 0 {
            g.precache_model("maps/b_nail1.bsp");
            g.setmodel(s, "maps/b_nail1.bsp");
            g.v(s).aflag = 40.0;
        } else {
            g.precache_model("maps/b_nail0.bsp");
            g.setmodel(s, "maps/b_nail0.bsp");
            g.v(s).aflag = 20.0;
        }
        g.v(s).weapon = 2;
        g.v(s).netname = "nails";
    }

    if g.r(s).spawnflags & WEAPON_ROCKET != 0 {
        if g.r(s).spawnflags & WEAPON_BIG != 0 {
            g.precache_model("maps/b_rock1.bsp");
            g.setmodel(s, "maps/b_rock1.bsp");
            g.v(s).aflag = 10.0;
        } else {
            g.precache_model("maps/b_rock0.bsp");
            g.setmodel(s, "maps/b_rock0.bsp");
            g.v(s).aflag = 5.0;
        }
        g.v(s).weapon = 3;
        g.v(s).netname = "rockets";
    }

    g.setsize(s, Vec3::new(0.0, 0.0, 0.0), Vec3::new(32.0, 32.0, 56.0));
    StartItem(g);
}

// ---- keys -------------------------------------------------------------------------------------

pub fn key_touch(g: &mut Game) {
    let s = g.self_;
    let o = g.other;

    if g.r(o).classname != "player" {
        return;
    }
    if g.r(o).health <= 0.0 {
        return;
    }
    if g.r(o).items & g.r(s).items != 0 {
        return;
    }

    g.sprint(o, "You got the ");
    g.sprint(o, g.r(s).netname);
    g.sprint(o, "\n");

    g.sound(o, CHAN_ITEM, g.r(s).noise, 1.0, ATTN_NORM);
    g.stuffcmd(o, "bf\n");
    g.v(o).items = g.r(o).items | g.r(s).items;

    if g.coop == 0.0 {
        g.v(s).solid = SOLID_NOT;
        g.v(s).model = g.string_null;
    }

    g.activator = o;
    SUB_UseTargets(g); // fire all targets / killtargets
}

pub fn key_setsounds(g: &mut Game) {
    let s = g.self_;
    if g.r(world).worldtype == 0.0 {
        g.precache_sound("misc/medkey.wav");
        g.v(s).noise = "misc/medkey.wav";
    } else if g.r(world).worldtype == 1.0 {
        g.precache_sound("misc/runekey.wav");
        g.v(s).noise = "misc/runekey.wav";
    } else if g.r(world).worldtype == 2.0 {
        g.precache_sound2("misc/basekey.wav");
        g.v(s).noise = "misc/basekey.wav";
    }
}

/// SILVER key. In order for keys to work you MUST set your map's worldtype to one of: 0
/// medieval, 1 metal, 2 base.
pub fn item_key1(g: &mut Game) {
    let s = g.self_;
    if g.r(world).worldtype == 0.0 {
        g.precache_model("progs/w_s_key.mdl");
        g.setmodel(s, "progs/w_s_key.mdl");
        g.v(s).netname = "silver key";
    } else if g.r(world).worldtype == 1.0 {
        g.precache_model("progs/m_s_key.mdl");
        g.setmodel(s, "progs/m_s_key.mdl");
        g.v(s).netname = "silver runekey";
    } else if g.r(world).worldtype == 2.0 {
        g.precache_model2("progs/b_s_key.mdl");
        g.setmodel(s, "progs/b_s_key.mdl");
        g.v(s).netname = "silver keycard";
    }
    key_setsounds(g);
    g.v(s).touch = Some(key_touch);
    g.v(s).items = IT_KEY1;
    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 32.0));
    StartItem(g);
}

/// GOLD key. In order for keys to work you MUST set your map's worldtype to one of: 0
/// medieval, 1 metal, 2 base.
pub fn item_key2(g: &mut Game) {
    let s = g.self_;
    if g.r(world).worldtype == 0.0 {
        g.precache_model("progs/w_g_key.mdl");
        g.setmodel(s, "progs/w_g_key.mdl");
        g.v(s).netname = "gold key";
    } else if g.r(world).worldtype == 1.0 {
        g.precache_model("progs/m_g_key.mdl");
        g.setmodel(s, "progs/m_g_key.mdl");
        g.v(s).netname = "gold runekey";
    } else if g.r(world).worldtype == 2.0 {
        g.precache_model2("progs/b_g_key.mdl");
        g.setmodel(s, "progs/b_g_key.mdl");
        g.v(s).netname = "gold keycard";
    }
    key_setsounds(g);
    g.v(s).touch = Some(key_touch);
    g.v(s).items = IT_KEY2;
    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 32.0));
    StartItem(g);
}

// ---- end of level runes -----------------------------------------------------------------------

pub fn sigil_touch(g: &mut Game) {
    let s = g.self_;
    let o = g.other;

    if g.r(o).classname != "player" {
        return;
    }
    if g.r(o).health <= 0.0 {
        return;
    }

    g.centerprint(o, "You got the rune!");

    g.sound(o, CHAN_ITEM, g.r(s).noise, 1.0, ATTN_NORM);
    g.stuffcmd(o, "bf\n");
    g.v(s).solid = SOLID_NOT;
    g.v(s).model = g.string_null;
    g.serverflags = g.serverflags | (g.r(s).spawnflags & 15);
    g.v(s).classname = ""; // so rune doors won't find it

    g.activator = o;
    SUB_UseTargets(g); // fire all targets / killtargets
}

/// End of level sigil, pick up to end episode and return to jrstart.
pub fn item_sigil(g: &mut Game) {
    let s = g.self_;
    if g.r(s).spawnflags == 0 {
        g.objerror("no spawnflags");
    }

    g.precache_sound("misc/runekey.wav");
    g.v(s).noise = "misc/runekey.wav";

    if g.r(s).spawnflags & 1 != 0 {
        g.precache_model("progs/end1.mdl");
        g.setmodel(s, "progs/end1.mdl");
    }
    if g.r(s).spawnflags & 2 != 0 {
        g.precache_model2("progs/end2.mdl");
        g.setmodel(s, "progs/end2.mdl");
    }
    if g.r(s).spawnflags & 4 != 0 {
        g.precache_model2("progs/end3.mdl");
        g.setmodel(s, "progs/end3.mdl");
    }
    if g.r(s).spawnflags & 8 != 0 {
        g.precache_model2("progs/end4.mdl");
        g.setmodel(s, "progs/end4.mdl");
    }

    g.v(s).touch = Some(sigil_touch);
    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 32.0));
    StartItem(g);
}

// ---- powerups ---------------------------------------------------------------------------------

pub fn powerup_touch(g: &mut Game) {
    let s = g.self_;
    let o = g.other;

    if g.r(o).classname != "player" {
        return;
    }
    if g.r(o).health <= 0.0 {
        return;
    }

    g.sprint(o, "You got the ");
    g.sprint(o, g.r(s).netname);
    g.sprint(o, "\n");

    if g.deathmatch != 0.0 {
        g.v(s).mdl = g.r(s).model;

        if g.r(s).classname == "item_artifact_invulnerability" || g.r(s).classname == "item_artifact_invisibility" {
            g.v(s).nextthink = g.time + 60.0 * 5.0;
        } else {
            g.v(s).nextthink = g.time + 60.0;
        }

        g.v(s).think = Some(SUB_regen);
    }

    g.sound(o, CHAN_VOICE, g.r(s).noise, 1.0, ATTN_NORM);
    g.stuffcmd(o, "bf\n");
    g.v(s).solid = SOLID_NOT;
    g.v(o).items = g.r(o).items | g.r(s).items;
    g.v(s).model = g.string_null;

    // do the appropriate action
    if g.r(s).classname == "item_artifact_envirosuit" {
        g.v(o).rad_time = 1.0;
        g.v(o).radsuit_finished = g.time + 30.0;
    } else if g.r(s).classname == "item_artifact_invulnerability" {
        g.v(o).invincible_time = 1.0;
        g.v(o).invincible_finished = g.time + 30.0;
    } else if g.r(s).classname == "item_artifact_invisibility" {
        g.v(o).invisible_time = 1.0;
        g.v(o).invisible_finished = g.time + 30.0;
    } else if g.r(s).classname == "item_artifact_super_damage" {
        g.v(o).super_time = 1.0;
        g.v(o).super_damage_finished = g.time + 30.0;
    }

    g.activator = o;
    SUB_UseTargets(g); // fire all targets / killtargets
}

/// Player is invulnerable for 30 seconds.
pub fn item_artifact_invulnerability(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(powerup_touch);

    g.precache_model("progs/invulner.mdl");
    g.precache_sound("items/protect.wav");
    g.precache_sound("items/protect2.wav");
    g.precache_sound("items/protect3.wav");
    g.v(s).noise = "items/protect.wav";
    g.setmodel(s, "progs/invulner.mdl");
    g.v(s).netname = "Pentagram of Protection";
    g.v(s).items = IT_INVULNERABILITY;
    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 32.0));
    StartItem(g);
}

/// Player takes no damage from water or slime for 30 seconds.
pub fn item_artifact_envirosuit(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(powerup_touch);

    g.precache_model("progs/suit.mdl");
    g.precache_sound("items/suit.wav");
    g.precache_sound("items/suit2.wav");
    g.v(s).noise = "items/suit.wav";
    g.setmodel(s, "progs/suit.mdl");
    g.v(s).netname = "Biosuit";
    g.v(s).items = IT_SUIT;
    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 32.0));
    StartItem(g);
}

/// Player is invisible for 30 seconds.
pub fn item_artifact_invisibility(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(powerup_touch);

    g.precache_model("progs/invisibl.mdl");
    g.precache_sound("items/inv1.wav");
    g.precache_sound("items/inv2.wav");
    g.precache_sound("items/inv3.wav");
    g.v(s).noise = "items/inv1.wav";
    g.setmodel(s, "progs/invisibl.mdl");
    g.v(s).netname = "Ring of Shadows";
    g.v(s).items = IT_INVISIBILITY;
    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 32.0));
    StartItem(g);
}

/// The next attack from the player will do 4x damage.
pub fn item_artifact_super_damage(g: &mut Game) {
    let s = g.self_;
    g.v(s).touch = Some(powerup_touch);

    g.precache_model("progs/quaddama.mdl");
    g.precache_sound("items/damage.wav");
    g.precache_sound("items/damage2.wav");
    g.precache_sound("items/damage3.wav");
    g.v(s).noise = "items/damage.wav";
    g.setmodel(s, "progs/quaddama.mdl");
    g.v(s).netname = "Quad Damage";
    g.v(s).items = IT_QUAD;
    g.setsize(s, Vec3::new(-16.0, -16.0, -24.0), Vec3::new(16.0, 16.0, 32.0));
    StartItem(g);
}

// ---- player backpacks -------------------------------------------------------------------------

pub fn BackpackTouch(g: &mut Game) {
    let s = g.self_;
    let o = g.other;

    if g.r(o).classname != "player" {
        return;
    }
    if g.r(o).health <= 0.0 {
        return;
    }

    let mut acount = FALSE;
    g.sprint(o, "You get ");

    if g.r(s).items != 0 {
        if (g.r(o).items & g.r(s).items) == 0 {
            acount = TRUE;
            g.sprint(o, "the ");
            g.sprint(o, g.r(s).netname);
        }
    }

    // if the player was using his best weapon, change up to the new one if better
    // (id's code never reads `best` again here, but the call is kept)
    let stemp = g.self_;
    g.self_ = o;
    W_BestWeapon(g);
    g.self_ = stemp;
    let s = g.self_;

    // change weapons
    g.v(o).ammo_shells = g.r(o).ammo_shells + g.r(s).ammo_shells;
    g.v(o).ammo_nails = g.r(o).ammo_nails + g.r(s).ammo_nails;
    g.v(o).ammo_rockets = g.r(o).ammo_rockets + g.r(s).ammo_rockets;
    g.v(o).ammo_cells = g.r(o).ammo_cells + g.r(s).ammo_cells;

    let mut new = g.r(s).items;
    if new == 0 {
        new = g.r(o).weapon;
    }
    let old = g.r(o).items;
    g.v(o).items = g.r(o).items | new;

    bound_other_ammo(g);
    if g.r(s).ammo_shells != 0.0 {
        if acount != 0.0 {
            g.sprint(o, ", ");
        }
        acount = TRUE;
        let st = ftos(g.r(s).ammo_shells);
        g.sprint(o, &st);
        g.sprint(o, " shells");
    }
    if g.r(s).ammo_nails != 0.0 {
        if acount != 0.0 {
            g.sprint(o, ", ");
        }
        acount = TRUE;
        let st = ftos(g.r(s).ammo_nails);
        g.sprint(o, &st);
        g.sprint(o, " nails");
    }
    if g.r(s).ammo_rockets != 0.0 {
        if acount != 0.0 {
            g.sprint(o, ", ");
        }
        acount = TRUE;
        let st = ftos(g.r(s).ammo_rockets);
        g.sprint(o, &st);
        g.sprint(o, " rockets");
    }
    if g.r(s).ammo_cells != 0.0 {
        if acount != 0.0 {
            g.sprint(o, ", ");
        }
        acount = TRUE;
        let st = ftos(g.r(s).ammo_cells);
        g.sprint(o, &st);
        g.sprint(o, " cells");
    }
    let _ = acount;

    g.sprint(o, "\n");
    // backpack touch sound
    g.sound(o, CHAN_ITEM, "weapons/lock4.wav", 1.0, ATTN_NORM);
    g.stuffcmd(o, "bf\n");

    // remove the backpack, change self to the player; as in id's code, self is not put back
    g.remove(s);
    g.self_ = g.other;
    let s = g.self_;

    // change to the weapon
    if g.deathmatch == 0.0 {
        g.v(s).weapon = new;
    } else {
        Deathmatch_Weapon(g, old as f32, new as f32);
    }

    W_SetCurrentAmmo(g);
}

pub fn DropBackpack(g: &mut Game) {
    let s = g.self_;

    if (g.r(s).ammo_shells + g.r(s).ammo_nails + g.r(s).ammo_rockets + g.r(s).ammo_cells) == 0.0 {
        return; // nothing in it
    }

    let item = g.spawn();
    g.v(item).origin = g.r(s).origin - Vec3::new(0.0, 0.0, 24.0);

    g.v(item).items = g.r(s).weapon;
    let it = g.r(item).items;
    g.v(item).netname = if it == IT_AXE {
        "Axe"
    } else if it == IT_SHOTGUN {
        "Shotgun"
    } else if it == IT_SUPER_SHOTGUN {
        "Double-barrelled Shotgun"
    } else if it == IT_NAILGUN {
        "Nailgun"
    } else if it == IT_SUPER_NAILGUN {
        "Super Nailgun"
    } else if it == IT_GRENADE_LAUNCHER {
        "Grenade Launcher"
    } else if it == IT_ROCKET_LAUNCHER {
        "Rocket Launcher"
    } else if it == IT_LIGHTNING {
        "Thunderbolt"
    } else {
        ""
    };

    g.v(item).ammo_shells = g.r(s).ammo_shells;
    g.v(item).ammo_nails = g.r(s).ammo_nails;
    g.v(item).ammo_rockets = g.r(s).ammo_rockets;
    g.v(item).ammo_cells = g.r(s).ammo_cells;

    g.v(item).velocity.z = 300.0;
    g.v(item).velocity.x = -100.0 + (g.random() * 200.0);
    g.v(item).velocity.y = -100.0 + (g.random() * 200.0);

    g.v(item).flags = FL_ITEM;
    g.v(item).solid = SOLID_TRIGGER;
    g.v(item).movetype = MOVETYPE_TOSS;
    g.setmodel(item, "progs/backpack.mdl");
    g.setsize(item, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    g.v(item).touch = Some(BackpackTouch);

    g.v(item).nextthink = g.time + 120.0; // remove after 2 minutes
    g.v(item).think = Some(SUB_Remove);
}
