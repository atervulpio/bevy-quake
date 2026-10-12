//! `misc.qc`, id's QuakeC 1.06, in Rust.

#![allow(clippy::all)]

use super::*;

/// QUAKED info_null: used as a positional target for spotlights, etc.
pub fn info_null(g: &mut Game) {
    let s = g.self_;
    g.remove(s);
}

/// QUAKED info_notnull: used as a positional target for lightning.
pub fn info_notnull(g: &mut Game) {
    let _ = g;
}

//============================================================================

const START_OFF: i32 = 1;

pub fn light_use(g: &mut Game) {
    let s = g.self_;
    if (g.r(s).spawnflags & START_OFF) != 0 {
        g.lightstyle(g.r(s).style, "m");
        g.v(s).spawnflags = g.r(s).spawnflags - START_OFF;
    } else {
        g.lightstyle(g.r(s).style, "a");
        g.v(s).spawnflags = g.r(s).spawnflags + START_OFF;
    }
}

/// QUAKED light: non-displayed light. Default light value is 300, default style 0. If
/// targeted, it will toggle between on or off.
pub fn light(g: &mut Game) {
    let s = g.self_;
    if g.r(s).targetname == "" {
        // inert light
        g.remove(s);
        return;
    }

    if g.r(s).style >= 32.0 {
        g.v(s).use_ = Some(light_use);
        if (g.r(s).spawnflags & START_OFF) != 0 {
            g.lightstyle(g.r(s).style, "a");
        } else {
            g.lightstyle(g.r(s).style, "m");
        }
    }
}

/// QUAKED light_fluoro: as `light`, and makes a steady fluorescent humming sound.
pub fn light_fluoro(g: &mut Game) {
    let s = g.self_;
    if g.r(s).style >= 32.0 {
        g.v(s).use_ = Some(light_use);
        if (g.r(s).spawnflags & START_OFF) != 0 {
            g.lightstyle(g.r(s).style, "a");
        } else {
            g.lightstyle(g.r(s).style, "m");
        }
    }

    g.precache_sound("ambience/fl_hum1.wav");
    g.ambientsound(g.r(s).origin, "ambience/fl_hum1.wav", 0.5, ATTN_STATIC);
}

/// QUAKED light_fluorospark: default style 10; makes a sparking, broken fluorescent sound.
pub fn light_fluorospark(g: &mut Game) {
    let s = g.self_;
    if g.r(s).style == 0.0 {
        g.v(s).style = 10.0;
    }

    g.precache_sound("ambience/buzz1.wav");
    g.ambientsound(g.r(s).origin, "ambience/buzz1.wav", 0.5, ATTN_STATIC);
}

/// QUAKED light_globe: sphere globe light.
pub fn light_globe(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/s_light.spr");
    g.setmodel(s, "progs/s_light.spr");
    g.makestatic(s);
}

pub fn FireAmbient(g: &mut Game) {
    let s = g.self_;
    g.precache_sound("ambience/fire1.wav");
    // attenuate fast
    g.ambientsound(g.r(s).origin, "ambience/fire1.wav", 0.5, ATTN_STATIC);
}

/// QUAKED light_torch_small_walltorch: short wall torch.
pub fn light_torch_small_walltorch(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/flame.mdl");
    g.setmodel(s, "progs/flame.mdl");
    FireAmbient(g);
    g.makestatic(g.self_);
}

/// QUAKED light_flame_large_yellow: large yellow flame ball.
pub fn light_flame_large_yellow(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/flame2.mdl");
    g.setmodel(s, "progs/flame2.mdl");
    g.v(s).frame = 1.0;
    FireAmbient(g);
    g.makestatic(g.self_);
}

/// QUAKED light_flame_small_yellow: small yellow flame ball.
pub fn light_flame_small_yellow(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/flame2.mdl");
    g.setmodel(s, "progs/flame2.mdl");
    FireAmbient(g);
    g.makestatic(g.self_);
}

/// QUAKED light_flame_small_white: small white flame ball.
pub fn light_flame_small_white(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/flame2.mdl");
    g.setmodel(s, "progs/flame2.mdl");
    FireAmbient(g);
    g.makestatic(g.self_);
}

//============================================================================

/// QUAKED misc_fireball: lava balls.
pub fn misc_fireball(g: &mut Game) {
    let s = g.self_;
    g.precache_model("progs/lavaball.mdl");
    g.v(s).classname = "fireball";
    g.v(s).nextthink = g.time + (g.random() * 5.0);
    g.v(s).think = Some(fire_fly);
    if g.r(s).speed == 0.0 {
        // id wrote `self.speed == 1000;`, a comparison that sets nothing: speed stays 0.
    }
}

pub fn fire_fly(g: &mut Game) {
    let s = g.self_;
    let fireball = g.spawn();
    g.v(fireball).solid = SOLID_TRIGGER;
    g.v(fireball).movetype = MOVETYPE_TOSS;
    g.v(fireball).velocity = Vec3::new(0.0, 0.0, 1000.0);
    g.v(fireball).velocity.x = (g.random() * 100.0) - 50.0;
    g.v(fireball).velocity.y = (g.random() * 100.0) - 50.0;
    g.v(fireball).velocity.z = g.r(s).speed + (g.random() * 200.0);
    g.v(fireball).classname = "fireball";
    g.setmodel(fireball, "progs/lavaball.mdl");
    g.setsize(fireball, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    g.setorigin(fireball, g.r(s).origin);
    g.v(fireball).nextthink = g.time + 5.0;
    g.v(fireball).think = Some(SUB_Remove);
    g.v(fireball).touch = Some(fire_touch);

    g.v(s).nextthink = g.time + (g.random() * 5.0) + 3.0;
    g.v(s).think = Some(fire_fly);
}

pub fn fire_touch(g: &mut Game) {
    let (s, o) = (g.self_, g.other);
    T_Damage(g, o, s, s, 20.0);
    g.remove(g.self_);
}

//============================================================================

pub fn barrel_explode(g: &mut Game) {
    let s = g.self_;
    g.v(s).takedamage = DAMAGE_NO;
    g.v(s).classname = "explo_box";
    // did say self.owner
    T_RadiusDamage(g, s, s, 160.0, world);
    let s = g.self_;
    g.sound(s, CHAN_VOICE, "weapons/r_exp3.wav", 1.0, ATTN_NORM);
    g.particle(g.r(s).origin, Vec3::new(0.0, 0.0, 0.0), 75.0, 255.0);

    g.v(s).origin.z = g.r(s).origin.z + 32.0;
    BecomeExplosion(g);
}

/// QUAKED misc_explobox: TESTING THING.
pub fn misc_explobox(g: &mut Game) {
    let s = g.self_;
    g.v(s).solid = SOLID_BBOX;
    g.v(s).movetype = MOVETYPE_NONE;
    g.precache_model("maps/b_explob.bsp");
    g.setmodel(s, "maps/b_explob.bsp");
    g.precache_sound("weapons/r_exp3.wav");
    g.v(s).health = 20.0;
    g.v(s).th_die = Some(barrel_explode);
    g.v(s).takedamage = DAMAGE_AIM;

    g.v(s).origin.z = g.r(s).origin.z + 2.0;
    let oldz = g.r(s).origin.z;
    g.droptofloor();
    if (oldz - g.r(s).origin.z) > 250.0 {
        g.dprint("item fell out of level at ");
        g.dprint(&vtos(g.r(s).origin));
        g.dprint("\n");
        g.remove(s);
    }
}

/// QUAKED misc_explobox2: smaller exploding box, REGISTERED ONLY.
pub fn misc_explobox2(g: &mut Game) {
    let s = g.self_;
    g.v(s).solid = SOLID_BBOX;
    g.v(s).movetype = MOVETYPE_NONE;
    g.precache_model2("maps/b_exbox2.bsp");
    g.setmodel(s, "maps/b_exbox2.bsp");
    g.precache_sound("weapons/r_exp3.wav");
    g.v(s).health = 20.0;
    g.v(s).th_die = Some(barrel_explode);
    g.v(s).takedamage = DAMAGE_AIM;

    g.v(s).origin.z = g.r(s).origin.z + 2.0;
    let oldz = g.r(s).origin.z;
    g.droptofloor();
    if (oldz - g.r(s).origin.z) > 250.0 {
        g.dprint("item fell out of level at ");
        g.dprint(&vtos(g.r(s).origin));
        g.dprint("\n");
        g.remove(s);
    }
}

//============================================================================

const SPAWNFLAG_SUPERSPIKE: i32 = 1;
const SPAWNFLAG_LASER: i32 = 2;

pub fn spikeshooter_use(g: &mut Game) {
    let s = g.self_;
    if (g.r(s).spawnflags & SPAWNFLAG_LASER) != 0 {
        g.sound(s, CHAN_VOICE, "enforcer/enfire.wav", 1.0, ATTN_NORM);
        let (o, d) = (g.r(s).origin, g.r(s).movedir);
        LaunchLaser(g, o, d);
    } else {
        g.sound(s, CHAN_VOICE, "weapons/spike2.wav", 1.0, ATTN_NORM);
        let (o, d) = (g.r(s).origin, g.r(s).movedir);
        launch_spike(g, o, d);
        let s = g.self_;
        g.v(g.newmis).velocity = g.r(s).movedir * 500.0;
        if (g.r(s).spawnflags & SPAWNFLAG_SUPERSPIKE) != 0 {
            g.v(g.newmis).touch = Some(superspike_touch);
        }
    }
}

pub fn shooter_think(g: &mut Game) {
    spikeshooter_use(g);
    let s = g.self_;
    g.v(s).nextthink = g.time + g.r(s).wait;
    g.v(g.newmis).velocity = g.r(s).movedir * 500.0;
}

/// QUAKED trap_spikeshooter: when triggered, fires a spike in the direction set in QuakeEd.
/// Laser is only for REGISTERED.
pub fn trap_spikeshooter(g: &mut Game) {
    SetMovedir(g);
    let s = g.self_;
    g.v(s).use_ = Some(spikeshooter_use);
    if (g.r(s).spawnflags & SPAWNFLAG_LASER) != 0 {
        g.precache_model2("progs/laser.mdl");

        g.precache_sound2("enforcer/enfire.wav");
        g.precache_sound2("enforcer/enfstop.wav");
    } else {
        g.precache_sound("weapons/spike2.wav");
    }
}

/// QUAKED trap_shooter: continuously fires spikes. "wait" is the time between spikes (default
/// 1.0); "nextthink" the delay before the first, so multiple shooters can be staggered.
pub fn trap_shooter(g: &mut Game) {
    trap_spikeshooter(g);

    let s = g.self_;
    if g.r(s).wait == 0.0 {
        g.v(s).wait = 1.0;
    }
    g.v(s).nextthink = g.r(s).nextthink + g.r(s).wait + g.r(s).ltime;
    g.v(s).think = Some(shooter_think);
}

//===============================================================================

/// QUAKED air_bubbles: testing air bubbles.
pub fn air_bubbles(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model("progs/s_bubble.spr");
    g.v(s).nextthink = g.time + 1.0;
    g.v(s).think = Some(make_bubbles);
}

pub fn make_bubbles(g: &mut Game) {
    let s = g.self_;
    let bubble = g.spawn();
    g.setmodel(bubble, "progs/s_bubble.spr");
    g.setorigin(bubble, g.r(s).origin);
    g.v(bubble).movetype = MOVETYPE_NOCLIP;
    g.v(bubble).solid = SOLID_NOT;
    g.v(bubble).velocity = Vec3::new(0.0, 0.0, 15.0);
    g.v(bubble).nextthink = g.time + 0.5;
    g.v(bubble).think = Some(bubble_bob);
    g.v(bubble).touch = Some(bubble_remove);
    g.v(bubble).classname = "bubble";
    g.v(bubble).frame = 0.0;
    g.v(bubble).cnt = 0.0;
    g.setsize(bubble, Vec3::new(-8.0, -8.0, -8.0), Vec3::new(8.0, 8.0, 8.0));
    g.v(s).nextthink = g.time + g.random() + 0.5;
    g.v(s).think = Some(make_bubbles);
}

pub fn bubble_split(g: &mut Game) {
    let s = g.self_;
    let bubble = g.spawn();
    g.setmodel(bubble, "progs/s_bubble.spr");
    g.setorigin(bubble, g.r(s).origin);
    g.v(bubble).movetype = MOVETYPE_NOCLIP;
    g.v(bubble).solid = SOLID_NOT;
    g.v(bubble).velocity = g.r(s).velocity;
    g.v(bubble).nextthink = g.time + 0.5;
    g.v(bubble).think = Some(bubble_bob);
    g.v(bubble).touch = Some(bubble_remove);
    g.v(bubble).classname = "bubble";
    g.v(bubble).frame = 1.0;
    g.v(bubble).cnt = 10.0;
    g.setsize(bubble, Vec3::new(-8.0, -8.0, -8.0), Vec3::new(8.0, 8.0, 8.0));
    g.v(s).frame = 1.0;
    g.v(s).cnt = 10.0;
    if g.r(s).waterlevel != 3 {
        g.remove(s);
    }
}

pub fn bubble_remove(g: &mut Game) {
    let s = g.self_;
    if g.r(g.other).classname == g.r(s).classname {
        return;
    }

    g.remove(s);
}

pub fn bubble_bob(g: &mut Game) {
    let s = g.self_;
    g.v(s).cnt = g.r(s).cnt + 1.0;
    if g.r(s).cnt == 4.0 {
        bubble_split(g);
    } else if g.r(s).cnt == 20.0 {
        g.remove(s);
    }

    let s = g.self_;
    // As in the original, a bubble just removed still has its fields written below.
    let mut rnd1 = g.r(s).velocity.x + (-10.0 + (g.random() * 20.0));
    let mut rnd2 = g.r(s).velocity.y + (-10.0 + (g.random() * 20.0));
    let mut rnd3 = g.r(s).velocity.z + 10.0 + g.random() * 10.0;

    if rnd1 > 10.0 {
        rnd1 = 5.0;
    } else if rnd1 < -10.0 {
        rnd1 = -5.0;
    }

    if rnd2 > 10.0 {
        rnd2 = 5.0;
    } else if rnd2 < -10.0 {
        rnd2 = -5.0;
    }

    if rnd3 < 10.0 {
        rnd3 = 15.0;
    } else if rnd3 > 30.0 {
        rnd3 = 25.0;
    }

    g.v(s).velocity.x = rnd1;
    g.v(s).velocity.y = rnd2;
    g.v(s).velocity.z = rnd3;

    g.v(s).nextthink = g.time + 0.5;
    g.v(s).think = Some(bubble_bob);
}

/// QUAKED viewthing: just for the debugging level. Don't use.
pub fn viewthing(g: &mut Game) {
    let s = g.self_;
    g.v(s).movetype = MOVETYPE_NONE;
    g.v(s).solid = SOLID_NOT;
    g.precache_model("progs/player.mdl");
    g.setmodel(s, "progs/player.mdl");
}

// ---- simple bmodels -------------------------------------------------------------------------

pub fn func_wall_use(g: &mut Game) {
    let s = g.self_;
    // change to alternate textures
    g.v(s).frame = 1.0 - g.r(s).frame;
}

/// QUAKED func_wall: just a solid wall if not inhibited.
pub fn func_wall(g: &mut Game) {
    let s = g.self_;
    g.v(s).angles = Vec3::new(0.0, 0.0, 0.0);
    g.v(s).movetype = MOVETYPE_PUSH; // so it doesn't get pushed by anything
    g.v(s).solid = SOLID_BSP;
    g.v(s).use_ = Some(func_wall_use);
    g.setmodel(s, g.r(s).model);
}

/// QUAKED func_illusionary: looks solid but lets you walk through it.
pub fn func_illusionary(g: &mut Game) {
    let s = g.self_;
    g.v(s).angles = Vec3::new(0.0, 0.0, 0.0);
    g.v(s).movetype = MOVETYPE_NONE;
    g.v(s).solid = SOLID_NOT;
    g.setmodel(s, g.r(s).model);
    // id calls `makestatic()` with no argument; the builtin reads its first parameter slot,
    // which still holds self from the setmodel call just before.
    g.makestatic(s);
}

/// QUAKED func_episodegate: appears if the episode has already been completed, so players
/// can't reenter it.
pub fn func_episodegate(g: &mut Game) {
    let s = g.self_;
    if !((g.serverflags & g.r(s).spawnflags) != 0) {
        return; // can still enter episode
    }

    g.v(s).angles = Vec3::new(0.0, 0.0, 0.0);
    g.v(s).movetype = MOVETYPE_PUSH; // so it doesn't get pushed by anything
    g.v(s).solid = SOLID_BSP;
    g.v(s).use_ = Some(func_wall_use);
    g.setmodel(s, g.r(s).model);
}

/// QUAKED func_bossgate: appears unless players have all of the episode sigils.
pub fn func_bossgate(g: &mut Game) {
    let s = g.self_;
    if (g.serverflags & 15) == 15 {
        return; // all episodes completed
    }

    g.v(s).angles = Vec3::new(0.0, 0.0, 0.0);
    g.v(s).movetype = MOVETYPE_PUSH; // so it doesn't get pushed by anything
    g.v(s).solid = SOLID_BSP;
    g.v(s).use_ = Some(func_wall_use);
    g.setmodel(s, g.r(s).model);
}

//============================================================================

pub fn ambient_suck_wind(g: &mut Game) {
    g.precache_sound("ambience/suck1.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/suck1.wav", 1.0, ATTN_STATIC);
}

pub fn ambient_drone(g: &mut Game) {
    g.precache_sound("ambience/drone6.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/drone6.wav", 0.5, ATTN_STATIC);
}

pub fn ambient_flouro_buzz(g: &mut Game) {
    g.precache_sound("ambience/buzz1.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/buzz1.wav", 1.0, ATTN_STATIC);
}

pub fn ambient_drip(g: &mut Game) {
    g.precache_sound("ambience/drip1.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/drip1.wav", 0.5, ATTN_STATIC);
}

pub fn ambient_comp_hum(g: &mut Game) {
    g.precache_sound("ambience/comp1.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/comp1.wav", 1.0, ATTN_STATIC);
}

pub fn ambient_thunder(g: &mut Game) {
    g.precache_sound("ambience/thunder1.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/thunder1.wav", 0.5, ATTN_STATIC);
}

pub fn ambient_light_buzz(g: &mut Game) {
    g.precache_sound("ambience/fl_hum1.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/fl_hum1.wav", 0.5, ATTN_STATIC);
}

pub fn ambient_swamp1(g: &mut Game) {
    g.precache_sound("ambience/swamp1.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/swamp1.wav", 0.5, ATTN_STATIC);
}

pub fn ambient_swamp2(g: &mut Game) {
    g.precache_sound("ambience/swamp2.wav");
    g.ambientsound(g.r(g.self_).origin, "ambience/swamp2.wav", 0.5, ATTN_STATIC);
}

//============================================================================

pub fn noise_think(g: &mut Game) {
    let s = g.self_;
    g.v(s).nextthink = g.time + 0.5;
    g.sound(s, 1, "enforcer/enfire.wav", 1.0, ATTN_NORM);
    g.sound(s, 2, "enforcer/enfstop.wav", 1.0, ATTN_NORM);
    g.sound(s, 3, "enforcer/sight1.wav", 1.0, ATTN_NORM);
    g.sound(s, 4, "enforcer/sight2.wav", 1.0, ATTN_NORM);
    g.sound(s, 5, "enforcer/sight3.wav", 1.0, ATTN_NORM);
    g.sound(s, 6, "enforcer/sight4.wav", 1.0, ATTN_NORM);
    g.sound(s, 7, "enforcer/pain1.wav", 1.0, ATTN_NORM);
}

/// QUAKED misc_noisemaker: for optimization testing, starts a lot of sounds.
pub fn misc_noisemaker(g: &mut Game) {
    let s = g.self_;
    g.precache_sound2("enforcer/enfire.wav");
    g.precache_sound2("enforcer/enfstop.wav");
    g.precache_sound2("enforcer/sight1.wav");
    g.precache_sound2("enforcer/sight2.wav");
    g.precache_sound2("enforcer/sight3.wav");
    g.precache_sound2("enforcer/sight4.wav");
    g.precache_sound2("enforcer/pain1.wav");
    g.precache_sound2("enforcer/pain2.wav");
    g.precache_sound2("enforcer/death1.wav");
    g.precache_sound2("enforcer/idle1.wav");

    g.v(s).nextthink = g.time + 0.1 + g.random();
    g.v(s).think = Some(noise_think);
}
