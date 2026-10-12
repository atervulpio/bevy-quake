//! `weapons.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;

/// Called by worldspawn.
pub fn W_Precache(g: &mut Game) {
    g.precache_sound("weapons/r_exp3.wav"); // new rocket explosion
    g.precache_sound("weapons/rocket1i.wav"); // spike gun
    g.precache_sound("weapons/sgun1.wav");
    g.precache_sound("weapons/guncock.wav"); // player shotgun
    g.precache_sound("weapons/ric1.wav"); // ricochet (used in C code)
    g.precache_sound("weapons/ric2.wav"); // ricochet (used in C code)
    g.precache_sound("weapons/ric3.wav"); // ricochet (used in C code)
    g.precache_sound("weapons/spike2.wav"); // super spikes
    g.precache_sound("weapons/tink1.wav"); // spikes tink (used in C code)
    g.precache_sound("weapons/grenade.wav"); // grenade launcher
    g.precache_sound("weapons/bounce.wav"); // grenade bounce
    g.precache_sound("weapons/shotgn2.wav"); // super shotgun
}

pub fn crandom(g: &mut Game) -> f32 {
    2.0 * (g.random() - 0.5)
}

pub fn W_FireAxe(g: &mut Game) {
    let s = g.self_;
    g.makevectors(g.r(s).v_angle);
    let source = g.r(s).origin + Vec3::new(0.0, 0.0, 16.0);
    g.traceline(source, source + g.v_forward * 64.0, FALSE, s);
    if g.trace_fraction == 1.0 {
        return;
    }

    let org = g.trace_endpos - g.v_forward * 4.0;

    if g.r(g.trace_ent).takedamage != 0 {
        g.v(g.trace_ent).axhitme = TRUE;
        SpawnBlood(g, org, Vec3::ZERO, 20.0);
        let t = g.trace_ent;
        T_Damage(g, t, s, s, 20.0);
    } else {
        // hit wall
        g.sound(s, CHAN_WEAPON, "player/axhit2.wav", 1.0, ATTN_NORM);
        g.temp_entity(TempEntity::Gunshot(org));
    }
}

//============================================================================

pub fn wall_velocity(g: &mut Game) -> Vec3 {
    let mut vel = normalize(g.r(g.self_).velocity);
    let up = g.v_up * (g.random() - 0.5);
    let right = g.v_right * (g.random() - 0.5);
    vel = normalize(vel + up + right);
    vel = vel + 2.0 * g.trace_plane_normal;
    vel = vel * 200.0;

    vel
}

pub fn SpawnMeatSpray(g: &mut Game, org: Vec3, vel: Vec3) {
    // QuakeC's `local vector org;` here names the parameter itself, as qcc finds the existing def.
    let s = g.self_;
    let missile = g.spawn();
    g.v(missile).owner = s;
    g.v(missile).movetype = MOVETYPE_BOUNCE;
    g.v(missile).solid = SOLID_NOT;

    g.makevectors(g.r(s).angles);

    g.v(missile).velocity = vel;
    let r = g.random();
    g.v(missile).velocity.z = g.r(missile).velocity.z + 250.0 + 50.0 * r;

    g.v(missile).avelocity = Vec3::new(3000.0, 1000.0, 2000.0);

    // set missile duration
    g.v(missile).nextthink = g.time + 1.0;
    g.v(missile).think = Some(SUB_Remove);

    g.setmodel(missile, "progs/zom_gib.mdl");
    g.setsize(missile, Vec3::ZERO, Vec3::ZERO);
    g.setorigin(missile, org);
}

pub fn SpawnBlood(g: &mut Game, org: Vec3, vel: Vec3, damage: f32) {
    g.particle(org, vel * 0.1, 73.0, damage * 2.0);
}

pub fn spawn_touchblood(g: &mut Game, damage: f32) {
    let vel = wall_velocity(g) * 0.2;
    let o = g.r(g.self_).origin;
    SpawnBlood(g, o + vel * 0.01, vel, damage);
}

pub fn SpawnChunk(g: &mut Game, org: Vec3, vel: Vec3) {
    g.particle(org, vel * 0.02, 0.0, 10.0);
}

// MULTI-DAMAGE: collects multiple small damages into a single damage.

pub fn ClearMultiDamage(g: &mut Game) {
    g.multi_ent = world;
    g.multi_damage = 0.0;
}

pub fn ApplyMultiDamage(g: &mut Game) {
    if g.multi_ent == world {
        return;
    }

    let (e, s, d) = (g.multi_ent, g.self_, g.multi_damage);
    T_Damage(g, e, s, s, d);
}

pub fn AddMultiDamage(g: &mut Game, hit: Ent, damage: f32) {
    if hit == world {
        return;
    }

    if hit != g.multi_ent {
        ApplyMultiDamage(g);
        g.multi_damage = damage;
        g.multi_ent = hit;
    } else {
        g.multi_damage = g.multi_damage + damage;
    }
}

// BULLETS

pub fn TraceAttack(g: &mut Game, damage: f32, dir: Vec3) {
    let up = g.v_up * crandom(g);
    let right = g.v_right * crandom(g);
    let mut vel = normalize(dir + up + right);
    vel = vel + 2.0 * g.trace_plane_normal;
    vel = vel * 200.0;

    let org = g.trace_endpos - dir * 4.0;

    if g.r(g.trace_ent).takedamage != 0 {
        SpawnBlood(g, org, vel * 0.2, damage);
        let t = g.trace_ent;
        AddMultiDamage(g, t, damage);
    } else {
        g.temp_entity(TempEntity::Gunshot(org));
    }
}

/// Used by shotgun, super shotgun, and enemy soldier firing. Goes to the trouble of combining
/// multiple pellets into a single damage call.
pub fn FireBullets(g: &mut Game, shotcount: f32, dir: Vec3, spread: Vec3) {
    let mut shotcount = shotcount;
    let s = g.self_;
    g.makevectors(g.r(s).v_angle);

    let mut src = g.r(s).origin + g.v_forward * 10.0;
    src.z = g.r(s).absmin.z + g.r(s).size.z * 0.7;

    ClearMultiDamage(g);
    while shotcount > 0.0 {
        let a = crandom(g) * spread.x * g.v_right;
        let b = crandom(g) * spread.y * g.v_up;
        let direction = dir + a + b;

        g.traceline(src, src + direction * 2048.0, FALSE, g.self_);
        if g.trace_fraction != 1.0 {
            TraceAttack(g, 4.0, direction);
        }

        shotcount = shotcount - 1.0;
    }
    ApplyMultiDamage(g);
}

pub fn W_FireShotgun(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_WEAPON, "weapons/guncock.wav", 1.0, ATTN_NORM);

    g.v(s).punchangle.x = -2.0;

    g.v(s).ammo_shells = g.r(s).ammo_shells - 1.0;
    g.v(s).currentammo = g.r(s).ammo_shells;
    let dir = g.aim(s, 100000.0);
    FireBullets(g, 6.0, dir, Vec3::new(0.04, 0.04, 0.0));
}

pub fn W_FireSuperShotgun(g: &mut Game) {
    let s = g.self_;
    if g.r(s).currentammo == 1.0 {
        W_FireShotgun(g);
        return;
    }

    g.sound(s, CHAN_WEAPON, "weapons/shotgn2.wav", 1.0, ATTN_NORM);

    g.v(s).punchangle.x = -4.0;

    g.v(s).ammo_shells = g.r(s).ammo_shells - 2.0;
    g.v(s).currentammo = g.r(s).ammo_shells;
    let dir = g.aim(s, 100000.0);
    FireBullets(g, 14.0, dir, Vec3::new(0.14, 0.08, 0.0));
}

// ROCKETS

pub fn s_explode1(g: &mut Game) {
    g.frame_state(0.0, s_explode2);
}

pub fn s_explode2(g: &mut Game) {
    g.frame_state(1.0, s_explode3);
}

pub fn s_explode3(g: &mut Game) {
    g.frame_state(2.0, s_explode4);
}

pub fn s_explode4(g: &mut Game) {
    g.frame_state(3.0, s_explode5);
}

pub fn s_explode5(g: &mut Game) {
    g.frame_state(4.0, s_explode6);
}

pub fn s_explode6(g: &mut Game) {
    g.frame_state(5.0, SUB_Remove);
}

pub fn BecomeExplosion(g: &mut Game) {
    let s = g.self_;
    g.v(s).movetype = MOVETYPE_NONE;
    g.v(s).velocity = Vec3::ZERO;
    g.v(s).touch = Some(SUB_Null);
    g.setmodel(s, "progs/s_explod.spr");
    g.v(s).solid = SOLID_NOT;
    s_explode1(g);
}

pub fn T_MissileTouch(g: &mut Game) {
    let s = g.self_;
    let other = g.other;
    if other == g.r(s).owner {
        return; // don't explode on owner
    }

    if g.pointcontents(g.r(s).origin) == CONTENT_SKY {
        g.remove(s);
        return;
    }

    let mut damg = 100.0 + g.random() * 20.0;

    if g.r(other).health != 0.0 {
        if g.r(other).classname == "monster_shambler" {
            damg = damg * 0.5; // mostly immune
        }
        let owner = g.r(s).owner;
        T_Damage(g, other, s, owner, damg);
    }

    // don't do radius damage to the other, because all the damage
    // was done in the impact
    let (s, other) = (g.self_, g.other);
    let owner = g.r(s).owner;
    T_RadiusDamage(g, s, owner, 120.0, other);

    g.v(s).origin = g.r(s).origin - 8.0 * normalize(g.r(s).velocity);

    g.temp_entity(TempEntity::Explosion(g.r(s).origin));

    BecomeExplosion(g);
}

pub fn W_FireRocket(g: &mut Game) {
    let s = g.self_;
    g.v(s).ammo_rockets = g.r(s).ammo_rockets - 1.0;
    g.v(s).currentammo = g.r(s).ammo_rockets;

    g.sound(s, CHAN_WEAPON, "weapons/sgun1.wav", 1.0, ATTN_NORM);

    g.v(s).punchangle.x = -2.0;

    let missile = g.spawn();
    g.v(missile).owner = s;
    g.v(missile).movetype = MOVETYPE_FLYMISSILE;
    g.v(missile).solid = SOLID_BBOX;
    g.v(missile).classname = "missile";

    // set missile speed
    g.makevectors(g.r(s).v_angle);
    g.v(missile).velocity = g.aim(s, 1000.0);
    g.v(missile).velocity = g.r(missile).velocity * 1000.0;
    g.v(missile).angles = vectoangles(g.r(missile).velocity);

    g.v(missile).touch = Some(T_MissileTouch);

    // set missile duration
    g.v(missile).nextthink = g.time + 5.0;
    g.v(missile).think = Some(SUB_Remove);

    g.setmodel(missile, "progs/missile.mdl");
    g.setsize(missile, Vec3::ZERO, Vec3::ZERO);
    let o = g.r(s).origin + g.v_forward * 8.0 + Vec3::new(0.0, 0.0, 16.0);
    g.setorigin(missile, o);
}

// LIGHTNING

pub fn LightningDamage(g: &mut Game, p1: Vec3, p2: Vec3, from: Ent, damage: f32) {
    let mut f = p2 - p1;
    let _ = normalize(f); // id's: the result is thrown away
    f.x = 0.0 - f.y;
    f.y = f.x;
    f.z = 0.0;
    f = f * 16.0;

    // id's `e1 = e2 = world;` is overwritten before any read.
    let e1;
    let e2;

    g.traceline(p1, p2, FALSE, g.self_);
    if g.r(g.trace_ent).takedamage != 0 {
        g.particle(g.trace_endpos, Vec3::new(0.0, 0.0, 100.0), 225.0, damage * 4.0);
        let t = g.trace_ent;
        T_Damage(g, t, from, from, damage);
        if g.r(g.self_).classname == "player" {
            if g.r(g.other).classname == "player" {
                g.v(g.trace_ent).velocity.z = g.r(g.trace_ent).velocity.z + 400.0;
            }
        }
    }
    e1 = g.trace_ent;

    g.traceline(p1 + f, p2 + f, FALSE, g.self_);
    if g.trace_ent != e1 && g.r(g.trace_ent).takedamage != 0 {
        g.particle(g.trace_endpos, Vec3::new(0.0, 0.0, 100.0), 225.0, damage * 4.0);
        let t = g.trace_ent;
        T_Damage(g, t, from, from, damage);
    }
    e2 = g.trace_ent;

    g.traceline(p1 - f, p2 - f, FALSE, g.self_);
    if g.trace_ent != e1 && g.trace_ent != e2 && g.r(g.trace_ent).takedamage != 0 {
        g.particle(g.trace_endpos, Vec3::new(0.0, 0.0, 100.0), 225.0, damage * 4.0);
        let t = g.trace_ent;
        T_Damage(g, t, from, from, damage);
    }
}

pub fn W_FireLightning(g: &mut Game) {
    let s = g.self_;
    if g.r(s).ammo_cells < 1.0 {
        g.v(s).weapon = W_BestWeapon(g) as i32;
        W_SetCurrentAmmo(g);
        return;
    }

    // explode if under water
    if g.r(s).waterlevel > 1 {
        let cells = g.r(s).ammo_cells;
        g.v(s).ammo_cells = 0.0;
        W_SetCurrentAmmo(g);
        T_RadiusDamage(g, s, s, 35.0 * cells, world);
        return;
    }

    if g.r(s).t_width < g.time {
        g.sound(s, CHAN_WEAPON, "weapons/lhit.wav", 1.0, ATTN_NORM);
        g.v(s).t_width = g.time + 0.6;
    }

    g.v(s).punchangle.x = -2.0;
    g.v(s).ammo_cells = g.r(s).ammo_cells - 1.0;
    g.v(s).currentammo = g.r(s).ammo_cells;

    let org = g.r(s).origin + Vec3::new(0.0, 0.0, 16.0);
    g.traceline(org, org + g.v_forward * 600.0, TRUE, s);

    g.temp_entity(TempEntity::Lightning2 { ent: s, start: org, end: g.trace_endpos });

    let (o, end) = (g.r(s).origin, g.trace_endpos + g.v_forward * 4.0);
    LightningDamage(g, o, end, s, 30.0);
}

//=============================================================================

pub fn GrenadeExplode(g: &mut Game) {
    let s = g.self_;
    let owner = g.r(s).owner;
    T_RadiusDamage(g, s, owner, 120.0, world);

    g.temp_entity(TempEntity::Explosion(g.r(s).origin));

    BecomeExplosion(g);
}

pub fn GrenadeTouch(g: &mut Game) {
    let s = g.self_;
    if g.other == g.r(s).owner {
        return; // don't explode on owner
    }

    if g.r(g.other).takedamage == DAMAGE_AIM {
        GrenadeExplode(g);
        return;
    }

    g.sound(s, CHAN_WEAPON, "weapons/bounce.wav", 1.0, ATTN_NORM); // bounce sound

    if g.r(s).velocity == Vec3::ZERO {
        g.v(s).avelocity = Vec3::ZERO;
    }
}

pub fn W_FireGrenade(g: &mut Game) {
    let s = g.self_;
    g.v(s).ammo_rockets = g.r(s).ammo_rockets - 1.0;
    g.v(s).currentammo = g.r(s).ammo_rockets;

    g.sound(s, CHAN_WEAPON, "weapons/grenade.wav", 1.0, ATTN_NORM);

    g.v(s).punchangle.x = -2.0;

    let missile = g.spawn();
    g.v(missile).owner = s;
    g.v(missile).movetype = MOVETYPE_BOUNCE;
    g.v(missile).solid = SOLID_BBOX;
    g.v(missile).classname = "grenade";

    // set missile speed
    g.makevectors(g.r(s).v_angle);
    if g.r(s).v_angle.x != 0.0 {
        let base = g.v_forward * 600.0 + g.v_up * 200.0;
        let a = crandom(g) * g.v_right * 10.0;
        let b = crandom(g) * g.v_up * 10.0;
        g.v(missile).velocity = base + a + b;
    } else {
        g.v(missile).velocity = g.aim(s, 10000.0);
        g.v(missile).velocity = g.r(missile).velocity * 600.0;
        g.v(missile).velocity.z = 200.0;
    }

    g.v(missile).avelocity = Vec3::new(300.0, 300.0, 300.0);

    g.v(missile).angles = vectoangles(g.r(missile).velocity);

    g.v(missile).touch = Some(GrenadeTouch);

    // set missile duration
    g.v(missile).nextthink = g.time + 2.5;
    g.v(missile).think = Some(GrenadeExplode);

    g.setmodel(missile, "progs/grenade.mdl");
    g.setsize(missile, Vec3::ZERO, Vec3::ZERO);
    let o = g.r(s).origin;
    g.setorigin(missile, o);
}

//=============================================================================

/// Used for both the player and the ogre.
pub fn launch_spike(g: &mut Game, org: Vec3, dir: Vec3) {
    let s = g.self_;
    g.newmis = g.spawn();
    let m = g.newmis;
    g.v(m).owner = s;
    g.v(m).movetype = MOVETYPE_FLYMISSILE;
    g.v(m).solid = SOLID_BBOX;

    g.v(m).angles = vectoangles(dir);

    g.v(m).touch = Some(spike_touch);
    g.v(m).classname = "spike";
    g.v(m).think = Some(SUB_Remove);
    g.v(m).nextthink = g.time + 6.0;
    g.setmodel(m, "progs/spike.mdl");
    g.setsize(m, VEC_ORIGIN, VEC_ORIGIN);
    g.setorigin(m, org);

    g.v(m).velocity = dir * 1000.0;
}

pub fn W_FireSuperSpikes(g: &mut Game) {
    let s = g.self_;
    g.sound(s, CHAN_WEAPON, "weapons/spike2.wav", 1.0, ATTN_NORM);
    g.v(s).attack_finished = g.time + 0.2;
    g.v(s).ammo_nails = g.r(s).ammo_nails - 2.0;
    g.v(s).currentammo = g.r(s).ammo_nails;
    let dir = g.aim(s, 1000.0);
    let o = g.r(s).origin + Vec3::new(0.0, 0.0, 16.0);
    launch_spike(g, o, dir);
    let m = g.newmis;
    g.v(m).touch = Some(superspike_touch);
    g.setmodel(m, "progs/s_spike.mdl");
    g.setsize(m, VEC_ORIGIN, VEC_ORIGIN);
    g.v(s).punchangle.x = -2.0;
}

pub fn W_FireSpikes(g: &mut Game, ox: f32) {
    let s = g.self_;
    g.makevectors(g.r(s).v_angle);

    if g.r(s).ammo_nails >= 2.0 && g.r(s).weapon == IT_SUPER_NAILGUN {
        W_FireSuperSpikes(g);
        return;
    }

    if g.r(s).ammo_nails < 1.0 {
        g.v(s).weapon = W_BestWeapon(g) as i32;
        W_SetCurrentAmmo(g);
        return;
    }

    g.sound(s, CHAN_WEAPON, "weapons/rocket1i.wav", 1.0, ATTN_NORM);
    g.v(s).attack_finished = g.time + 0.2;
    g.v(s).ammo_nails = g.r(s).ammo_nails - 1.0;
    g.v(s).currentammo = g.r(s).ammo_nails;
    let dir = g.aim(s, 1000.0);
    let o = g.r(s).origin + Vec3::new(0.0, 0.0, 16.0) + g.v_right * ox;
    launch_spike(g, o, dir);

    g.v(s).punchangle.x = -2.0;
}

pub fn spike_touch(g: &mut Game) {
    let s = g.self_;
    let other = g.other;
    if other == g.r(s).owner {
        return;
    }
    if g.r(other).solid == SOLID_TRIGGER {
        return; // trigger field, do nothing
    }

    if g.pointcontents(g.r(s).origin) == CONTENT_SKY {
        g.remove(s);
        return;
    }

    // hit something that bleeds
    if g.r(other).takedamage != 0 {
        spawn_touchblood(g, 9.0);
        let owner = g.r(s).owner;
        T_Damage(g, other, s, owner, 9.0);
    } else {
        let o = g.r(s).origin;
        let t = if g.r(s).classname == "wizspike" {
            TempEntity::WizSpike(o)
        } else if g.r(s).classname == "knightspike" {
            TempEntity::KnightSpike(o)
        } else {
            TempEntity::Spike(o)
        };
        g.temp_entity(t);
    }

    g.remove(s);
}

pub fn superspike_touch(g: &mut Game) {
    let s = g.self_;
    let other = g.other;
    if other == g.r(s).owner {
        return;
    }
    if g.r(other).solid == SOLID_TRIGGER {
        return; // trigger field, do nothing
    }

    if g.pointcontents(g.r(s).origin) == CONTENT_SKY {
        g.remove(s);
        return;
    }

    // hit something that bleeds
    if g.r(other).takedamage != 0 {
        spawn_touchblood(g, 18.0);
        let owner = g.r(s).owner;
        T_Damage(g, other, s, owner, 18.0);
    } else {
        g.temp_entity(TempEntity::SuperSpike(g.r(s).origin));
    }

    g.remove(s);
}

// PLAYER WEAPON USE

pub fn W_SetCurrentAmmo(g: &mut Game) {
    player_run(g); // get out of any weapon firing states

    let s = g.self_;
    g.v(s).items = g.r(s).items - (g.r(s).items & (IT_SHELLS | IT_NAILS | IT_ROCKETS | IT_CELLS));

    let w = g.r(s).weapon;
    let v = g.v(s);
    if w == IT_AXE {
        v.currentammo = 0.0;
        v.weaponmodel = "progs/v_axe.mdl";
        v.weaponframe = 0.0;
    } else if w == IT_SHOTGUN {
        v.currentammo = v.ammo_shells;
        v.weaponmodel = "progs/v_shot.mdl";
        v.weaponframe = 0.0;
        v.items = v.items | IT_SHELLS;
    } else if w == IT_SUPER_SHOTGUN {
        v.currentammo = v.ammo_shells;
        v.weaponmodel = "progs/v_shot2.mdl";
        v.weaponframe = 0.0;
        v.items = v.items | IT_SHELLS;
    } else if w == IT_NAILGUN {
        v.currentammo = v.ammo_nails;
        v.weaponmodel = "progs/v_nail.mdl";
        v.weaponframe = 0.0;
        v.items = v.items | IT_NAILS;
    } else if w == IT_SUPER_NAILGUN {
        v.currentammo = v.ammo_nails;
        v.weaponmodel = "progs/v_nail2.mdl";
        v.weaponframe = 0.0;
        v.items = v.items | IT_NAILS;
    } else if w == IT_GRENADE_LAUNCHER {
        v.currentammo = v.ammo_rockets;
        v.weaponmodel = "progs/v_rock.mdl";
        v.weaponframe = 0.0;
        v.items = v.items | IT_ROCKETS;
    } else if w == IT_ROCKET_LAUNCHER {
        v.currentammo = v.ammo_rockets;
        v.weaponmodel = "progs/v_rock2.mdl";
        v.weaponframe = 0.0;
        v.items = v.items | IT_ROCKETS;
    } else if w == IT_LIGHTNING {
        v.currentammo = v.ammo_cells;
        v.weaponmodel = "progs/v_light.mdl";
        v.weaponframe = 0.0;
        v.items = v.items | IT_CELLS;
    } else {
        v.currentammo = 0.0;
        v.weaponmodel = "";
        v.weaponframe = 0.0;
    }
}

pub fn W_BestWeapon(g: &mut Game) -> f32 {
    let v = g.r(g.self_);
    let it = v.items;

    if v.waterlevel <= 1 && v.ammo_cells >= 1.0 && (it & IT_LIGHTNING) != 0 {
        return IT_LIGHTNING as f32;
    }
    if v.ammo_nails >= 2.0 && (it & IT_SUPER_NAILGUN) != 0 {
        return IT_SUPER_NAILGUN as f32;
    }
    if v.ammo_shells >= 2.0 && (it & IT_SUPER_SHOTGUN) != 0 {
        return IT_SUPER_SHOTGUN as f32;
    }
    if v.ammo_nails >= 1.0 && (it & IT_NAILGUN) != 0 {
        return IT_NAILGUN as f32;
    }
    if v.ammo_shells >= 1.0 && (it & IT_SHOTGUN) != 0 {
        return IT_SHOTGUN as f32;
    }

    IT_AXE as f32
}

pub fn W_CheckNoAmmo(g: &mut Game) -> f32 {
    let s = g.self_;
    if g.r(s).currentammo > 0.0 {
        return TRUE; // has at least one ammo
    }

    if g.r(s).weapon == IT_AXE {
        return TRUE; // weapon doesn't need ammo
    }

    g.v(s).weapon = W_BestWeapon(g) as i32;

    W_SetCurrentAmmo(g);

    // drop the weapon down
    FALSE
}

/// An attack impulse can be triggered now.
pub fn W_Attack(g: &mut Game) {
    if W_CheckNoAmmo(g) == 0.0 {
        return;
    }

    let s = g.self_;
    g.makevectors(g.r(s).v_angle); // calculate forward angle for velocity
    g.v(s).show_hostile = g.time + 1.0; // wake monsters up

    let w = g.r(s).weapon;
    if w == IT_AXE {
        g.sound(s, CHAN_WEAPON, "weapons/ax1.wav", 1.0, ATTN_NORM);
        let r = g.random();
        if r < 0.25 {
            player_axe1(g);
        } else if r < 0.5 {
            player_axeb1(g);
        } else if r < 0.75 {
            player_axec1(g);
        } else {
            player_axed1(g);
        }
        g.v(s).attack_finished = g.time + 0.5;
    } else if w == IT_SHOTGUN {
        player_shot1(g);
        W_FireShotgun(g);
        g.v(s).attack_finished = g.time + 0.5;
    } else if w == IT_SUPER_SHOTGUN {
        player_shot1(g);
        W_FireSuperShotgun(g);
        g.v(s).attack_finished = g.time + 0.7;
    } else if w == IT_NAILGUN {
        player_nail1(g);
    } else if w == IT_SUPER_NAILGUN {
        player_nail1(g);
    } else if w == IT_GRENADE_LAUNCHER {
        player_rocket1(g);
        W_FireGrenade(g);
        g.v(s).attack_finished = g.time + 0.6;
    } else if w == IT_ROCKET_LAUNCHER {
        player_rocket1(g);
        W_FireRocket(g);
        g.v(s).attack_finished = g.time + 0.8;
    } else if w == IT_LIGHTNING {
        player_light1(g);
        g.v(s).attack_finished = g.time + 0.1;
        g.sound(s, CHAN_AUTO, "weapons/lstart.wav", 1.0, ATTN_NORM);
    }
}

pub fn W_ChangeWeapon(g: &mut Game) {
    let s = g.self_;
    let mut am = false;
    // Called only for impulses 1 to 8, so `fl` is always set below.
    let mut fl: i32 = 0;

    let v = g.r(s);
    let imp = v.impulse;
    if imp == 1 {
        fl = IT_AXE;
    } else if imp == 2 {
        fl = IT_SHOTGUN;
        if v.ammo_shells < 1.0 {
            am = true;
        }
    } else if imp == 3 {
        fl = IT_SUPER_SHOTGUN;
        if v.ammo_shells < 2.0 {
            am = true;
        }
    } else if imp == 4 {
        fl = IT_NAILGUN;
        if v.ammo_nails < 1.0 {
            am = true;
        }
    } else if imp == 5 {
        fl = IT_SUPER_NAILGUN;
        if v.ammo_nails < 2.0 {
            am = true;
        }
    } else if imp == 6 {
        fl = IT_GRENADE_LAUNCHER;
        if v.ammo_rockets < 1.0 {
            am = true;
        }
    } else if imp == 7 {
        fl = IT_ROCKET_LAUNCHER;
        if v.ammo_rockets < 1.0 {
            am = true;
        }
    } else if imp == 8 {
        fl = IT_LIGHTNING;
        if v.ammo_cells < 1.0 {
            am = true;
        }
    }

    g.v(s).impulse = 0;

    if (g.r(s).items & fl) == 0 {
        // don't have the weapon or the ammo
        g.sprint(s, "no weapon.\n");
        return;
    }

    if am {
        // don't have the ammo
        g.sprint(s, "not enough ammo.\n");
        return;
    }

    // set weapon, set ammo
    g.v(s).weapon = fl;
    W_SetCurrentAmmo(g);
}

pub fn CheatCommand(g: &mut Game) {
    if g.deathmatch != 0.0 || g.coop != 0.0 {
        return;
    }

    let s = g.self_;
    let v = g.v(s);
    v.ammo_rockets = 100.0;
    v.ammo_nails = 200.0;
    v.ammo_shells = 100.0;
    v.items = v.items
        | IT_AXE
        | IT_SHOTGUN
        | IT_SUPER_SHOTGUN
        | IT_NAILGUN
        | IT_SUPER_NAILGUN
        | IT_GRENADE_LAUNCHER
        | IT_ROCKET_LAUNCHER
        | IT_KEY1
        | IT_KEY2;

    v.ammo_cells = 200.0;
    v.items = v.items | IT_LIGHTNING;

    v.weapon = IT_ROCKET_LAUNCHER;
    v.impulse = 0;
    W_SetCurrentAmmo(g);
}

/// QuakeC's interpreter stops a loop that runs past 100000 instructions ("runaway loop
/// error"); the weapon cycles below can only loop for ever if the player owns none of the eight
/// weapons, and then they stop the game as the original did.
fn runaway(n: &mut u32) {
    *n += 1;
    if *n > 100000 {
        panic!("runaway loop error");
    }
}

/// Go to the next weapon with ammo.
pub fn CycleWeaponCommand(g: &mut Game) {
    let s = g.self_;
    let it = g.r(s).items;
    g.v(s).impulse = 0;

    let mut n = 0;
    loop {
        runaway(&mut n);
        let mut am = false;
        let v = g.v(s);

        if v.weapon == IT_LIGHTNING {
            v.weapon = IT_AXE;
        } else if v.weapon == IT_AXE {
            v.weapon = IT_SHOTGUN;
            if v.ammo_shells < 1.0 {
                am = true;
            }
        } else if v.weapon == IT_SHOTGUN {
            v.weapon = IT_SUPER_SHOTGUN;
            if v.ammo_shells < 2.0 {
                am = true;
            }
        } else if v.weapon == IT_SUPER_SHOTGUN {
            v.weapon = IT_NAILGUN;
            if v.ammo_nails < 1.0 {
                am = true;
            }
        } else if v.weapon == IT_NAILGUN {
            v.weapon = IT_SUPER_NAILGUN;
            if v.ammo_nails < 2.0 {
                am = true;
            }
        } else if v.weapon == IT_SUPER_NAILGUN {
            v.weapon = IT_GRENADE_LAUNCHER;
            if v.ammo_rockets < 1.0 {
                am = true;
            }
        } else if v.weapon == IT_GRENADE_LAUNCHER {
            v.weapon = IT_ROCKET_LAUNCHER;
            if v.ammo_rockets < 1.0 {
                am = true;
            }
        } else if v.weapon == IT_ROCKET_LAUNCHER {
            v.weapon = IT_LIGHTNING;
            if v.ammo_cells < 1.0 {
                am = true;
            }
        }

        if (it & v.weapon) != 0 && !am {
            W_SetCurrentAmmo(g);
            return;
        }
    }
}

/// Go to the previous weapon with ammo.
pub fn CycleWeaponReverseCommand(g: &mut Game) {
    let s = g.self_;
    let it = g.r(s).items;
    g.v(s).impulse = 0;

    let mut n = 0;
    loop {
        runaway(&mut n);
        let mut am = false;
        let v = g.v(s);

        if v.weapon == IT_LIGHTNING {
            v.weapon = IT_ROCKET_LAUNCHER;
            if v.ammo_rockets < 1.0 {
                am = true;
            }
        } else if v.weapon == IT_ROCKET_LAUNCHER {
            v.weapon = IT_GRENADE_LAUNCHER;
            if v.ammo_rockets < 1.0 {
                am = true;
            }
        } else if v.weapon == IT_GRENADE_LAUNCHER {
            v.weapon = IT_SUPER_NAILGUN;
            if v.ammo_nails < 2.0 {
                am = true;
            }
        } else if v.weapon == IT_SUPER_NAILGUN {
            v.weapon = IT_NAILGUN;
            if v.ammo_nails < 1.0 {
                am = true;
            }
        } else if v.weapon == IT_NAILGUN {
            v.weapon = IT_SUPER_SHOTGUN;
            if v.ammo_shells < 2.0 {
                am = true;
            }
        } else if v.weapon == IT_SUPER_SHOTGUN {
            v.weapon = IT_SHOTGUN;
            if v.ammo_shells < 1.0 {
                am = true;
            }
        } else if v.weapon == IT_SHOTGUN {
            v.weapon = IT_AXE;
        } else if v.weapon == IT_AXE {
            v.weapon = IT_LIGHTNING;
            if v.ammo_cells < 1.0 {
                am = true;
            }
        }

        if (it & v.weapon) != 0 && !am {
            W_SetCurrentAmmo(g);
            return;
        }
    }
}

/// Just for development.
pub fn ServerflagsCommand(g: &mut Game) {
    g.serverflags = g.serverflags * 2 + 1;
}

pub fn QuadCheat(g: &mut Game) {
    if g.deathmatch != 0.0 || g.coop != 0.0 {
        return;
    }

    let s = g.self_;
    g.v(s).super_time = 1.0;
    g.v(s).super_damage_finished = g.time + 30.0;
    g.v(s).items = g.r(s).items | IT_QUAD;
    g.dprint("quad cheat\n");
}

pub fn ImpulseCommands(g: &mut Game) {
    let imp = g.r(g.self_).impulse;
    if imp >= 1 && imp <= 8 {
        W_ChangeWeapon(g);
    } else if imp == 9 {
        CheatCommand(g);
    } else if imp == 10 {
        CycleWeaponCommand(g);
    } else if imp == 11 {
        ServerflagsCommand(g);
    } else if imp == 12 {
        CycleWeaponReverseCommand(g);
    } else if imp == 255 {
        QuadCheat(g);
    }

    g.v(g.self_).impulse = 0;
}

/// Called every frame so impulse events can be handled as well as possible.
pub fn W_WeaponFrame(g: &mut Game) {
    if g.time < g.r(g.self_).attack_finished {
        return;
    }

    ImpulseCommands(g);

    // check for attack
    if g.r(g.self_).button0 != 0 {
        SuperDamageSound(g);
        W_Attack(g);
    }
}

/// Plays sound if needed.
pub fn SuperDamageSound(g: &mut Game) {
    let s = g.self_;
    if g.r(s).super_damage_finished > g.time {
        if g.r(s).super_sound < g.time {
            g.v(s).super_sound = g.time + 1.0;
            g.sound(s, CHAN_BODY, "items/damage3.wav", 1.0, ATTN_NORM);
        }
    }
}
