//! `client.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;

// ---- level changing and intermission -----------------------------------------------------------

/// The camera point for the intermission. It uses `mangle` instead of `angle`, so pitch and roll
/// can be set as well as yaw.
pub fn info_intermission(g: &mut Game) {}

pub fn SetChangeParms(g: &mut Game) {
    let s = g.self_;
    if g.r(s).health <= 0.0 {
        SetNewParms(g);
        return;
    }

    // remove items
    g.v(s).items = g.r(s).items
        - (g.r(s).items & (IT_KEY1 | IT_KEY2 | IT_INVISIBILITY | IT_INVULNERABILITY | IT_SUIT | IT_QUAD));

    // cap super health
    if g.r(s).health > 100.0 {
        g.v(s).health = 100.0;
    } else if g.r(s).health < 50.0 {
        g.v(s).health = 50.0;
    }

    g.parm[0] = g.r(s).items as f32;
    g.parm[1] = g.r(s).health;
    g.parm[2] = g.r(s).armorvalue;
    if g.r(s).ammo_shells < 25.0 {
        g.parm[3] = 25.0;
    } else {
        g.parm[3] = g.r(s).ammo_shells;
    }
    g.parm[4] = g.r(s).ammo_nails;
    g.parm[5] = g.r(s).ammo_rockets;
    g.parm[6] = g.r(s).ammo_cells;
    g.parm[7] = g.r(s).weapon as f32;
    g.parm[8] = g.r(s).armortype * 100.0;
}

pub fn SetNewParms(g: &mut Game) {
    g.parm[0] = (IT_SHOTGUN | IT_AXE) as f32;
    g.parm[1] = 100.0;
    g.parm[2] = 0.0;
    g.parm[3] = 25.0;
    g.parm[4] = 0.0;
    g.parm[5] = 0.0;
    g.parm[6] = 0.0;
    g.parm[7] = 1.0;
    g.parm[8] = 0.0;
}

pub fn DecodeLevelParms(g: &mut Game) {
    if g.serverflags != 0 {
        if g.r(world).model == "maps/start.bsp" {
            SetNewParms(g); // take away all stuff on starting new episode
        }
    }

    let s = g.self_;
    g.v(s).items = g.parm[0] as i32;
    g.v(s).health = g.parm[1];
    g.v(s).armorvalue = g.parm[2];
    g.v(s).ammo_shells = g.parm[3];
    g.v(s).ammo_nails = g.parm[4];
    g.v(s).ammo_rockets = g.parm[5];
    g.v(s).ammo_cells = g.parm[6];
    g.v(s).weapon = g.parm[7] as i32;
    g.v(s).armortype = g.parm[8] * 0.01;
}

/// The entity to view the intermission from.
pub fn FindIntermission(g: &mut Game) -> Ent {
    // look for info_intermission first
    let mut spot = g.find(world, |v: &EntVars| v.classname, "info_intermission");
    if spot != world {
        // pick a random one
        let mut cyc = g.random() * 4.0;
        while cyc > 1.0 {
            spot = g.find(spot, |v: &EntVars| v.classname, "info_intermission");
            if spot == world {
                spot = g.find(spot, |v: &EntVars| v.classname, "info_intermission");
            }
            cyc = cyc - 1.0;
        }
        return spot;
    }

    // then look for the start position
    spot = g.find(world, |v: &EntVars| v.classname, "info_player_start");
    if spot != world {
        return spot;
    }

    // testinfo_player_start is only found in regioned levels
    spot = g.find(world, |v: &EntVars| v.classname, "testplayerstart");
    if spot != world {
        return spot;
    }

    g.objerror("FindIntermission: no spot");
}

pub fn GotoNextMap(g: &mut Game) {
    if g.cvar("samelevel") != 0.0 {
        // if samelevel is set, stay on same level
        let m = g.mapname;
        g.changelevel(m);
    } else {
        let m = g.nextmap;
        g.changelevel(m);
    }
}

pub fn ExitIntermission(g: &mut Game) {
    // skip any text in deathmatch
    if g.deathmatch != 0.0 {
        GotoNextMap(g);
        return;
    }

    g.intermission_exittime = g.time + 1.0;
    g.intermission_running = g.intermission_running + 1.0;

    // run some text if at the end of an episode
    if g.intermission_running == 2.0 {
        if g.r(world).model == "maps/e1m7.bsp" {
            g.msg_cdtrack(2.0, 3.0);
            if g.cvar("registered") == 0.0 {
                g.msg_finale("As the corpse of the monstrous entity\nChthon sinks back into the lava whence\nit rose, you grip the Rune of Earth\nMagic tightly. Now that you have\nconquered the Dimension of the Doomed,\nrealm of Earth Magic, you are ready to\ncomplete your task in the other three\nhaunted lands of Quake. Or are you? If\nyou don't register Quake, you'll never\nknow what awaits you in the Realm of\nBlack Magic, the Netherworld, and the\nElder World!");
            } else {
                g.msg_finale("As the corpse of the monstrous entity\nChthon sinks back into the lava whence\nit rose, you grip the Rune of Earth\nMagic tightly. Now that you have\nconquered the Dimension of the Doomed,\nrealm of Earth Magic, you are ready to\ncomplete your task. A Rune of magic\npower lies at the end of each haunted\nland of Quake. Go forth, seek the\ntotality of the four Runes!");
            }
            return;
        } else if g.r(world).model == "maps/e2m6.bsp" {
            g.msg_cdtrack(2.0, 3.0);

            g.msg_finale("The Rune of Black Magic throbs evilly in\nyour hand and whispers dark thoughts\ninto your brain. You learn the inmost\nlore of the Hell-Mother; Shub-Niggurath!\nYou now know that she is behind all the\nterrible plotting which has led to so\nmuch death and horror. But she is not\ninviolate! Armed with this Rune, you\nrealize that once all four Runes are\ncombined, the gate to Shub-Niggurath's\nPit will open, and you can face the\nWitch-Goddess herself in her frightful\notherworld cathedral.");
            return;
        } else if g.r(world).model == "maps/e3m6.bsp" {
            g.msg_cdtrack(2.0, 3.0);

            g.msg_finale("The charred viscera of diabolic horrors\nbubble viscously as you seize the Rune\nof Hell Magic. Its heat scorches your\nhand, and its terrible secrets blight\nyour mind. Gathering the shreds of your\ncourage, you shake the devil's shackles\nfrom your soul, and become ever more\nhard and determined to destroy the\nhideous creatures whose mere existence\nthreatens the souls and psyches of all\nthe population of Earth.");
            return;
        } else if g.r(world).model == "maps/e4m7.bsp" {
            g.msg_cdtrack(2.0, 3.0);

            g.msg_finale("Despite the awful might of the Elder\nWorld, you have achieved the Rune of\nElder Magic, capstone of all types of\narcane wisdom. Beyond good and evil,\nbeyond life and death, the Rune\npulsates, heavy with import. Patient and\npotent, the Elder Being Shub-Niggurath\nweaves her dire plans to clear off all\nlife from the Earth, and bring her own\nfoul offspring to our world! For all the\ndwellers in these nightmare dimensions\nare her descendants! Once all Runes of\nmagic power are united, the energy\nbehind them will blast open the Gateway\nto Shub-Niggurath, and you can travel\nthere to foil the Hell-Mother's plots\nin person.");
            return;
        }

        // id's code falls through to the GotoNextMap below as well and so calls it twice;
        // kept, as the second changelevel is ignored by the engine.
        GotoNextMap(g);
    }

    if g.intermission_running == 3.0 {
        if g.cvar("registered") == 0.0 {
            // shareware episode has been completed, go to sell screen
            g.msg_sellscreen();
            return;
        }

        if (g.serverflags & 15) == 15 {
            g.msg_finale("Now, you have all four Runes. You sense\ntremendous invisible forces moving to\nunseal ancient barriers. Shub-Niggurath\nhad hoped to use the Runes Herself to\nclear off the Earth, but now instead,\nyou will use them to enter her home and\nconfront her as an avatar of avenging\nEarth-life. If you defeat her, you will\nbe remembered forever as the savior of\nthe planet. If she conquers, it will be\nas if you had never been born.");
            return;
        }
    }

    GotoNextMap(g);
}

/// When the player presses attack or jump, change to the next level.
pub fn IntermissionThink(g: &mut Game) {
    if g.time < g.intermission_exittime {
        return;
    }

    let s = g.self_;
    if g.r(s).button0 == 0 && g.r(s).button1 == 0 && g.r(s).button2 == 0 {
        return;
    }

    ExitIntermission(g);
}

pub fn execute_changelevel(g: &mut Game) {
    g.intermission_running = 1.0;

    // enforce a wait time before allowing changelevel
    if g.deathmatch != 0.0 {
        g.intermission_exittime = g.time + 5.0;
    } else {
        g.intermission_exittime = g.time + 2.0;
    }

    g.msg_cdtrack(3.0, 3.0);

    let pos = FindIntermission(g);

    g.other = g.find(world, |v: &EntVars| v.classname, "player");
    while g.other != world {
        let o = g.other;
        g.v(o).view_ofs = Vec3::ZERO;
        g.v(o).v_angle = g.r(pos).mangle;
        g.v(o).angles = g.r(o).v_angle;
        g.v(o).fixangle = TRUE as i32; // turn this way immediately
        g.v(o).nextthink = g.time + 0.5;
        g.v(o).takedamage = DAMAGE_NO;
        g.v(o).solid = SOLID_NOT;
        g.v(o).movetype = MOVETYPE_NONE;
        g.v(o).modelindex = 0.0;
        g.setorigin(o, g.r(pos).origin);
        g.other = g.find(o, |v: &EntVars| v.classname, "player");
    }

    g.msg_intermission();
}

pub fn changelevel_touch(g: &mut Game) {
    let s = g.self_;
    if g.r(g.other).classname != "player" {
        return;
    }

    // both sides evaluated, as QuakeC does
    let noexit1 = g.cvar("noexit") == 1.0;
    let noexit2 = g.cvar("noexit") == 2.0 && g.mapname != "start";
    if noexit1 | noexit2 {
        T_Damage(g, g.other, s, s, 50000.0);
        return;
    }

    if g.coop != 0.0 || g.deathmatch != 0.0 {
        let o = g.other;
        g.bprint(g.r(o).netname);
        g.bprint(" exited the level\n");
    }

    g.nextmap = g.r(s).map;

    SUB_UseTargets(g);

    let s = g.self_;
    if (g.r(s).spawnflags & 1) != 0 && g.deathmatch == 0.0 {
        // NO_INTERMISSION
        GotoNextMap(g);
        return;
    }

    g.v(s).touch = Some(SUB_Null);

    // we can't move people right now, because touch functions are called
    // in the middle of C movement code, so set a think time to do it
    g.v(s).think = Some(execute_changelevel);
    g.v(s).nextthink = g.time + 0.1;
}

/// When the player touches this, he is sent to the map in its `map` key. Unless the
/// NO_INTERMISSION flag (1) is set, the view goes to the info_intermission spot and shows the
/// tally.
pub fn trigger_changelevel(g: &mut Game) {
    if g.r(g.self_).map == "" {
        g.objerror("changelevel trigger doesn't have map");
    }

    InitTrigger(g);
    g.v(g.self_).touch = Some(changelevel_touch);
}

// ---- the player's game edge functions ---------------------------------------------------------

/// Called by ClientKill and DeadThink.
pub fn respawn(g: &mut Game) {
    if g.coop != 0.0 {
        // make a copy of the dead body for appearances sake
        CopyToBodyQueue(g, g.self_);
        // get the spawn parms as they were at level start
        g.setspawnparms(g.self_);
        // respawn
        PutClientInServer(g);
    } else if g.deathmatch != 0.0 {
        // make a copy of the dead body for appearances sake
        CopyToBodyQueue(g, g.self_);
        // set default spawn parms
        SetNewParms(g);
        // respawn
        PutClientInServer(g);
    } else {
        // restart the entire server
        g.localcmd("restart\n");
    }
}

/// The player entered the suicide command.
pub fn ClientKill(g: &mut Game) {
    g.bprint(g.r(g.self_).netname);
    g.bprint(" suicides\n");
    set_suicide_frame(g);
    let s = g.self_;
    g.v(s).modelindex = g.modelindex_player;
    g.v(s).frags = g.r(s).frags - 2.0; // extra penalty
    respawn(g);
}

pub fn CheckSpawnPoint(g: &mut Game, v: Vec3) -> f32 {
    FALSE
}

/// The entity to spawn at.
pub fn SelectSpawnPoint(g: &mut Game) -> Ent {
    // testinfo_player_start is only found in regioned levels
    let mut spot = g.find(world, |v: &EntVars| v.classname, "testplayerstart");
    if spot != world {
        return spot;
    }

    // choose a info_player_deathmatch point
    if g.coop != 0.0 {
        g.lastspawn = g.find(g.lastspawn, |v: &EntVars| v.classname, "info_player_coop");
        if g.lastspawn == world {
            g.lastspawn = g.find(g.lastspawn, |v: &EntVars| v.classname, "info_player_start");
        }
        if g.lastspawn != world {
            return g.lastspawn;
        }
    } else if g.deathmatch != 0.0 {
        spot = g.lastspawn;
        loop {
            spot = g.find(spot, |v: &EntVars| v.classname, "info_player_deathmatch");
            if spot != world {
                if spot == g.lastspawn {
                    return g.lastspawn;
                }

                let mut pcount = 0.0;
                let mut thing = g.findradius(g.r(spot).origin, 32.0);
                while thing != world {
                    if g.r(thing).classname == "player" {
                        pcount = pcount + 1.0;
                    }
                    thing = g.r(thing).chain;
                }
                if pcount == 0.0 {
                    g.lastspawn = spot;
                    return spot;
                }
            }
        }
    }

    if g.serverflags != 0 {
        // return with a rune to start
        spot = g.find(world, |v: &EntVars| v.classname, "info_player_start2");
        if spot != world {
            return spot;
        }
    }

    spot = g.find(world, |v: &EntVars| v.classname, "info_player_start");
    if spot == world {
        g.error("PutClientInServer: no info_player_start on level");
    }

    spot
}

/// Called each time a player is spawned.
pub fn PutClientInServer(g: &mut Game) {
    let spot = SelectSpawnPoint(g);

    let s = g.self_;
    g.v(s).classname = "player";
    g.v(s).health = 100.0;
    g.v(s).takedamage = DAMAGE_AIM;
    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_WALK;
    g.v(s).show_hostile = 0.0;
    g.v(s).max_health = 100.0;
    g.v(s).flags = FL_CLIENT;
    g.v(s).air_finished = g.time + 12.0;
    g.v(s).dmg = 2.0; // initial water damage
    g.v(s).super_damage_finished = 0.0;
    g.v(s).radsuit_finished = 0.0;
    g.v(s).invisible_finished = 0.0;
    g.v(s).invincible_finished = 0.0;
    g.v(s).effects = 0;
    g.v(s).invincible_time = 0.0;

    DecodeLevelParms(g);

    W_SetCurrentAmmo(g);

    let s = g.self_;
    g.v(s).attack_finished = g.time;
    g.v(s).th_pain = Some(player_pain);
    g.v(s).th_die = Some(PlayerDie);

    g.v(s).deadflag = DEAD_NO;
    // pausetime is set by teleporters to keep the player from moving a while
    g.v(s).pausetime = 0.0;

    g.v(s).origin = g.r(spot).origin + Vec3::new(0.0, 0.0, 1.0);
    g.v(s).angles = g.r(spot).angles;
    g.v(s).fixangle = TRUE as i32; // turn this way immediately

    // oh, this is a hack!
    g.setmodel(s, "progs/eyes.mdl");
    g.modelindex_eyes = g.r(s).modelindex;

    g.setmodel(s, "progs/player.mdl");
    g.modelindex_player = g.r(s).modelindex;

    g.setsize(s, VEC_HULL_MIN, VEC_HULL_MAX);

    g.v(s).view_ofs = Vec3::new(0.0, 0.0, 22.0);

    player_stand1(g);

    if g.deathmatch != 0.0 || g.coop != 0.0 {
        let s = g.self_;
        g.makevectors(g.r(s).angles);
        spawn_tfog(g, g.r(s).origin + g.v_forward * 20.0);
    }

    let s = g.self_;
    spawn_tdeath(g, g.r(s).origin, s);
}

// ---- the QUAKED functions ---------------------------------------------------------------------

/// The normal starting point for a level.
pub fn info_player_start(g: &mut Game) {}

/// Only used on the start map, for the return point from an episode.
pub fn info_player_start2(g: &mut Game) {}

/// Saved out by QuakeEd in region mode.
pub fn testplayerstart(g: &mut Game) {}

/// A potential spawning position for deathmatch games.
pub fn info_player_deathmatch(g: &mut Game) {}

/// A potential spawning position for coop games.
pub fn info_player_coop(g: &mut Game) {}

// ---- rules ------------------------------------------------------------------------------------

/// Goes to the next level for deathmatch; only called if a time or frag limit has expired.
pub fn NextLevel(g: &mut Game) {
    let mut o: Ent;

    if g.mapname == "start" {
        if g.cvar("registered") == 0.0 {
            g.mapname = "e1m1";
        } else if (g.serverflags & 1) == 0 {
            g.mapname = "e1m1";
            g.serverflags = g.serverflags | 1;
        } else if (g.serverflags & 2) == 0 {
            g.mapname = "e2m1";
            g.serverflags = g.serverflags | 2;
        } else if (g.serverflags & 4) == 0 {
            g.mapname = "e3m1";
            g.serverflags = g.serverflags | 4;
        } else if (g.serverflags & 8) == 0 {
            g.mapname = "e4m1";
            g.serverflags = g.serverflags - 7;
        }

        o = g.spawn();
        g.v(o).map = g.mapname;
    } else {
        // find a trigger_changelevel
        o = g.find(world, |v: &EntVars| v.classname, "trigger_changelevel");

        // go back to start if no trigger_changelevel
        if o == world {
            g.mapname = "start";
            o = g.spawn();
            g.v(o).map = g.mapname;
        }
    }

    g.nextmap = g.r(o).map;
    g.gameover = TRUE;

    if g.r(o).nextthink < g.time {
        g.v(o).think = Some(execute_changelevel);
        g.v(o).nextthink = g.time + 0.1;
    }
}

/// Exits deathmatch games upon conditions.
pub fn CheckRules(g: &mut Game) {
    if g.gameover != 0.0 {
        // someone else quit the game already
        return;
    }

    let timelimit = g.cvar("timelimit") * 60.0;
    let fraglimit = g.cvar("fraglimit");

    if timelimit != 0.0 && g.time >= timelimit {
        NextLevel(g);
        return;
    }

    if fraglimit != 0.0 && g.r(g.self_).frags >= fraglimit {
        NextLevel(g);
        return;
    }
}

pub fn PlayerDeathThink(g: &mut Game) {
    let s = g.self_;
    if (g.r(s).flags & FL_ONGROUND) != 0 {
        let mut forward = vlen(g.r(s).velocity);
        forward = forward - 20.0;
        if forward <= 0.0 {
            g.v(s).velocity = Vec3::ZERO;
        } else {
            g.v(s).velocity = forward * normalize(g.r(s).velocity);
        }
    }

    // wait for all buttons released
    if g.r(s).deadflag == DEAD_DEAD {
        if g.r(s).button2 != 0 || g.r(s).button1 != 0 || g.r(s).button0 != 0 {
            return;
        }

        g.v(s).deadflag = DEAD_RESPAWNABLE;
        return;
    }

    // wait for any button down
    if g.r(s).button2 == 0 && g.r(s).button1 == 0 && g.r(s).button0 == 0 {
        return;
    }

    g.v(s).button0 = 0;
    g.v(s).button1 = 0;
    g.v(s).button2 = 0;
    respawn(g);
}

pub fn PlayerJump(g: &mut Game) {
    let s = g.self_;
    if (g.r(s).flags & FL_WATERJUMP) != 0 {
        return;
    }

    if g.r(s).waterlevel >= 2 {
        if g.r(s).watertype == CONTENT_WATER {
            g.v(s).velocity.z = 100.0;
        } else if g.r(s).watertype == CONTENT_SLIME {
            g.v(s).velocity.z = 80.0;
        } else {
            g.v(s).velocity.z = 50.0;
        }

        // play swimming sound
        if g.r(s).swim_flag < g.time {
            g.v(s).swim_flag = g.time + 1.0;
            if g.random() < 0.5 {
                g.sound(s, CHAN_BODY, "misc/water1.wav", 1.0, ATTN_NORM);
            } else {
                g.sound(s, CHAN_BODY, "misc/water2.wav", 1.0, ATTN_NORM);
            }
        }

        return;
    }

    if (g.r(s).flags & FL_ONGROUND) == 0 {
        return;
    }

    if (g.r(s).flags & FL_JUMPRELEASED) == 0 {
        return; // don't pogo stick
    }

    g.v(s).flags = g.r(s).flags - (g.r(s).flags & FL_JUMPRELEASED);

    g.v(s).flags = g.r(s).flags - FL_ONGROUND; // don't stairwalk

    g.v(s).button2 = 0;
    // player jumping sound
    g.sound(s, CHAN_BODY, "player/plyrjmp8.wav", 1.0, ATTN_NORM);
    g.v(s).velocity.z = g.r(s).velocity.z + 270.0;
}

pub fn WaterMove(g: &mut Game) {
    let s = g.self_;
    if g.r(s).movetype == MOVETYPE_NOCLIP {
        return;
    }
    if g.r(s).health < 0.0 {
        return;
    }

    if g.r(s).waterlevel != 3 {
        if g.r(s).air_finished < g.time {
            g.sound(s, CHAN_VOICE, "player/gasp2.wav", 1.0, ATTN_NORM);
        } else if g.r(s).air_finished < (g.time + 9.0) {
            g.sound(s, CHAN_VOICE, "player/gasp1.wav", 1.0, ATTN_NORM);
        }
        g.v(s).air_finished = g.time + 12.0;
        g.v(s).dmg = 2.0;
    } else if g.r(s).air_finished < g.time {
        // drown!
        if g.r(s).pain_finished < g.time {
            g.v(s).dmg = g.r(s).dmg + 2.0;
            if g.r(s).dmg > 15.0 {
                g.v(s).dmg = 10.0;
            }
            T_Damage(g, s, world, world, g.r(s).dmg);
            let s = g.self_;
            g.v(s).pain_finished = g.time + 1.0;
        }
    }

    let s = g.self_;
    if g.r(s).waterlevel == 0 {
        if (g.r(s).flags & FL_INWATER) != 0 {
            // play leave water sound
            g.sound(s, CHAN_BODY, "misc/outwater.wav", 1.0, ATTN_NORM);
            g.v(s).flags = g.r(s).flags - FL_INWATER;
        }
        return;
    }

    if g.r(s).watertype == CONTENT_LAVA {
        // do damage
        if g.r(s).dmgtime < g.time {
            if g.r(s).radsuit_finished > g.time {
                g.v(s).dmgtime = g.time + 1.0;
            } else {
                g.v(s).dmgtime = g.time + 0.2;
            }

            T_Damage(g, s, world, world, 10.0 * g.r(s).waterlevel as f32);
        }
    } else if g.r(s).watertype == CONTENT_SLIME {
        // do damage
        if g.r(s).dmgtime < g.time && g.r(s).radsuit_finished < g.time {
            g.v(s).dmgtime = g.time + 1.0;
            T_Damage(g, s, world, world, 4.0 * g.r(s).waterlevel as f32);
        }
    }

    let s = g.self_;
    if (g.r(s).flags & FL_INWATER) == 0 {
        // player enter water sound
        if g.r(s).watertype == CONTENT_LAVA {
            g.sound(s, CHAN_BODY, "player/inlava.wav", 1.0, ATTN_NORM);
        } else if g.r(s).watertype == CONTENT_WATER {
            g.sound(s, CHAN_BODY, "player/inh2o.wav", 1.0, ATTN_NORM);
        } else if g.r(s).watertype == CONTENT_SLIME {
            g.sound(s, CHAN_BODY, "player/slimbrn2.wav", 1.0, ATTN_NORM);
        }

        g.v(s).flags = g.r(s).flags + FL_INWATER;
        g.v(s).dmgtime = 0.0;
    }

    if (g.r(s).flags & FL_WATERJUMP) == 0 {
        g.v(s).velocity = g.r(s).velocity - 0.8 * g.r(s).waterlevel as f32 * g.frametime * g.r(s).velocity;
    }
}

pub fn CheckWaterJump(g: &mut Game) {
    let s = g.self_;

    // check for a jump-out-of-water
    g.makevectors(g.r(s).angles);
    let mut start = g.r(s).origin;
    start.z = start.z + 8.0;
    g.v_forward.z = 0.0;
    // id's code discards the result, so v_forward stays as it is
    let _ = normalize(g.v_forward);
    let mut end = start + g.v_forward * 24.0;
    g.traceline(start, end, TRUE, s);
    if g.trace_fraction < 1.0 {
        // solid at waist
        start.z = start.z + g.r(s).maxs.z - 8.0;
        end = start + g.v_forward * 24.0;
        g.v(s).movedir = g.trace_plane_normal * -50.0;
        g.traceline(start, end, TRUE, s);
        if g.trace_fraction == 1.0 {
            // open at eye level
            g.v(s).flags = g.r(s).flags | FL_WATERJUMP;
            g.v(s).velocity.z = 225.0;
            g.v(s).flags = g.r(s).flags - (g.r(s).flags & FL_JUMPRELEASED);
            g.v(s).teleport_time = g.time + 2.0; // safety net
            return;
        }
    }
}

/// Called every frame before physics are run.
pub fn PlayerPreThink(g: &mut Game) {
    if g.intermission_running != 0.0 {
        IntermissionThink(g); // otherwise a button could be missed between
        return; // the think tics
    }

    if g.r(g.self_).view_ofs == Vec3::ZERO {
        return; // intermission or finale
    }

    g.makevectors(g.r(g.self_).v_angle); // is this still used

    CheckRules(g);
    WaterMove(g);

    if g.r(g.self_).waterlevel == 2 {
        CheckWaterJump(g);
    }

    if g.r(g.self_).deadflag >= DEAD_DEAD {
        PlayerDeathThink(g);
        return;
    }

    if g.r(g.self_).deadflag == DEAD_DYING {
        return; // dying, so do nothing
    }

    if g.r(g.self_).button2 != 0 {
        PlayerJump(g);
    } else {
        let s = g.self_;
        g.v(s).flags = g.r(s).flags | FL_JUMPRELEASED;
    }

    // teleporters can force a non-moving pause time
    let s = g.self_;
    if g.time < g.r(s).pausetime {
        g.v(s).velocity = Vec3::ZERO;
    }

    if g.time > g.r(s).attack_finished && g.r(s).currentammo == 0.0 && g.r(s).weapon != IT_AXE {
        let w = W_BestWeapon(g);
        g.v(g.self_).weapon = w as i32;
        W_SetCurrentAmmo(g);
    }
}

/// Checks for turning off powerups.
pub fn CheckPowerups(g: &mut Game) {
    let s = g.self_;
    if g.r(s).health <= 0.0 {
        return;
    }

    // invisibility
    if g.r(s).invisible_finished != 0.0 {
        // sound and screen flash when item starts to run out
        if g.r(s).invisible_sound < g.time {
            g.sound(s, CHAN_AUTO, "items/inv3.wav", 0.5, ATTN_IDLE);
            g.v(s).invisible_sound = g.time + ((g.random() * 3.0) + 1.0);
        }

        if g.r(s).invisible_finished < (g.time + 3.0) {
            if g.r(s).invisible_time == 1.0 {
                g.sprint(s, "Ring of Shadows magic is fading\n");
                g.stuffcmd(s, "bf\n");
                g.sound(s, CHAN_AUTO, "items/inv2.wav", 1.0, ATTN_NORM);
                g.v(s).invisible_time = g.time + 1.0;
            } else if g.r(s).invisible_time < g.time {
                g.v(s).invisible_time = g.time + 1.0;
                g.stuffcmd(s, "bf\n");
            }
        }

        if g.r(s).invisible_finished < g.time {
            // just stopped
            g.v(s).items = g.r(s).items - IT_INVISIBILITY;
            g.v(s).invisible_finished = 0.0;
            g.v(s).invisible_time = 0.0;
        }

        // use the eyes
        g.v(s).frame = 0.0;
        g.v(s).modelindex = g.modelindex_eyes;
    } else {
        g.v(s).modelindex = g.modelindex_player; // don't use eyes
    }

    // invincibility
    if g.r(s).invincible_finished != 0.0 {
        // sound and screen flash when item starts to run out
        if g.r(s).invincible_finished < (g.time + 3.0) {
            if g.r(s).invincible_time == 1.0 {
                g.sprint(s, "Protection is almost burned out\n");
                g.stuffcmd(s, "bf\n");
                g.sound(s, CHAN_AUTO, "items/protect2.wav", 1.0, ATTN_NORM);
                g.v(s).invincible_time = g.time + 1.0;
            } else if g.r(s).invincible_time < g.time {
                g.v(s).invincible_time = g.time + 1.0;
                g.stuffcmd(s, "bf\n");
            }
        }

        if g.r(s).invincible_finished < g.time {
            // just stopped
            g.v(s).items = g.r(s).items - IT_INVULNERABILITY;
            g.v(s).invincible_time = 0.0;
            g.v(s).invincible_finished = 0.0;
        }
        if g.r(s).invincible_finished > g.time {
            g.v(s).effects = g.r(s).effects | EF_DIMLIGHT;
        } else {
            g.v(s).effects = g.r(s).effects - (g.r(s).effects & EF_DIMLIGHT);
        }
    }

    // super damage
    if g.r(s).super_damage_finished != 0.0 {
        // sound and screen flash when item starts to run out
        if g.r(s).super_damage_finished < (g.time + 3.0) {
            if g.r(s).super_time == 1.0 {
                g.sprint(s, "Quad Damage is wearing off\n");
                g.stuffcmd(s, "bf\n");
                g.sound(s, CHAN_AUTO, "items/damage2.wav", 1.0, ATTN_NORM);
                g.v(s).super_time = g.time + 1.0;
            } else if g.r(s).super_time < g.time {
                g.v(s).super_time = g.time + 1.0;
                g.stuffcmd(s, "bf\n");
            }
        }

        if g.r(s).super_damage_finished < g.time {
            // just stopped
            g.v(s).items = g.r(s).items - IT_QUAD;
            g.v(s).super_damage_finished = 0.0;
            g.v(s).super_time = 0.0;
        }
        if g.r(s).super_damage_finished > g.time {
            g.v(s).effects = g.r(s).effects | EF_DIMLIGHT;
        } else {
            g.v(s).effects = g.r(s).effects - (g.r(s).effects & EF_DIMLIGHT);
        }
    }

    // suit
    if g.r(s).radsuit_finished != 0.0 {
        g.v(s).air_finished = g.time + 12.0; // don't drown

        // sound and screen flash when item starts to run out
        if g.r(s).radsuit_finished < (g.time + 3.0) {
            if g.r(s).rad_time == 1.0 {
                g.sprint(s, "Air supply in Biosuit expiring\n");
                g.stuffcmd(s, "bf\n");
                g.sound(s, CHAN_AUTO, "items/suit2.wav", 1.0, ATTN_NORM);
                g.v(s).rad_time = g.time + 1.0;
            } else if g.r(s).rad_time < g.time {
                g.v(s).rad_time = g.time + 1.0;
                g.stuffcmd(s, "bf\n");
            }
        }

        if g.r(s).radsuit_finished < g.time {
            // just stopped
            g.v(s).items = g.r(s).items - IT_SUIT;
            g.v(s).rad_time = 0.0;
            g.v(s).radsuit_finished = 0.0;
        }
    }
}

/// Called every frame after physics are run.
pub fn PlayerPostThink(g: &mut Game) {
    let s = g.self_;
    if g.r(s).view_ofs == Vec3::ZERO {
        return; // intermission or finale
    }
    if g.r(s).deadflag != 0 {
        return;
    }

    // do weapon stuff
    W_WeaponFrame(g);

    // check to see if player landed and play landing sound
    let s = g.self_;
    if g.r(s).jump_flag < -300.0 && (g.r(s).flags & FL_ONGROUND) != 0 && g.r(s).health > 0.0 {
        if g.r(s).watertype == CONTENT_WATER {
            g.sound(s, CHAN_BODY, "player/h2ojump.wav", 1.0, ATTN_NORM);
        } else if g.r(s).jump_flag < -650.0 {
            T_Damage(g, s, world, world, 5.0);
            let s = g.self_;
            g.sound(s, CHAN_VOICE, "player/land2.wav", 1.0, ATTN_NORM);
            g.v(s).deathtype = "falling";
        } else {
            g.sound(s, CHAN_VOICE, "player/land.wav", 1.0, ATTN_NORM);
        }

        g.v(g.self_).jump_flag = 0.0;
    }

    let s = g.self_;
    if (g.r(s).flags & FL_ONGROUND) == 0 {
        g.v(s).jump_flag = g.r(s).velocity.z;
    }

    CheckPowerups(g);
}

/// Called when a player connects to a server.
pub fn ClientConnect(g: &mut Game) {
    g.bprint(g.r(g.self_).netname);
    g.bprint(" entered the game\n");

    // a client connecting during an intermission can cause problems
    if g.intermission_running != 0.0 {
        ExitIntermission(g);
    }
}

/// Called when a player disconnects from a server.
pub fn ClientDisconnect(g: &mut Game) {
    if g.gameover != 0.0 {
        // if the level end trigger has been activated, just return
        // since they aren't *really* leaving
        return;
    }

    // let everyone else know
    let s = g.self_;
    g.bprint(g.r(s).netname);
    g.bprint(" left the game with ");
    g.bprint(&ftos(g.r(s).frags));
    g.bprint(" frags\n");
    g.sound(s, CHAN_BODY, "player/tornoff2.wav", 1.0, ATTN_NONE);
    set_suicide_frame(g);
}

/// Called when a player dies.
pub fn ClientObituary(g: &mut Game, targ: Ent, attacker: Ent) {
    // QuakeC's locals are not cleared between calls; an unmatched weapon below would print
    // whatever the last call left. Here they start empty.
    let mut deathstring: Str = "";
    let mut deathstring2: Str = "";

    let mut rnum = g.random();

    if g.r(targ).classname == "player" {
        if g.r(attacker).classname == "teledeath" {
            g.bprint(g.r(targ).netname);
            g.bprint(" was telefragged by ");
            g.bprint(g.r(g.r(attacker).owner).netname);
            g.bprint("\n");

            let o = g.r(attacker).owner;
            g.v(o).frags = g.r(o).frags + 1.0;
            return;
        }

        if g.r(attacker).classname == "teledeath2" {
            g.bprint("Satan's power deflects ");
            g.bprint(g.r(targ).netname);
            g.bprint("'s telefrag\n");

            g.v(targ).frags = g.r(targ).frags - 1.0;
            return;
        }

        if g.r(attacker).classname == "player" {
            // killed by a player (incl. itself)
            if targ == attacker {
                // killed self
                g.v(attacker).frags = g.r(attacker).frags - 1.0;

                g.bprint(g.r(targ).netname);
                if g.r(targ).weapon == IT_LIGHTNING && g.r(targ).waterlevel > 1 {
                    g.bprint(" discharges into the water.\n");
                    return;
                }

                if g.r(targ).weapon == IT_GRENADE_LAUNCHER {
                    g.bprint(" tries to put the pin back in\n");
                } else {
                    g.bprint(" becomes bored with life\n");
                }
                return;
            } else if g.teamplay == 2.0 && g.r(targ).team > 0.0 && g.r(targ).team == g.r(attacker).team {
                if rnum < 0.25 {
                    deathstring = " mows down a teammate\n";
                } else if rnum < 0.50 {
                    deathstring = " checks his glasses\n";
                } else if rnum < 0.75 {
                    deathstring = " gets a frag for the other team\n";
                } else {
                    deathstring = " loses another friend\n";
                }
                g.bprint(g.r(attacker).netname);
                g.bprint(deathstring);

                g.v(attacker).frags = g.r(attacker).frags - 1.0;
                return;
            } else {
                g.v(attacker).frags = g.r(attacker).frags + 1.0;

                rnum = g.r(attacker).weapon as f32;
                if rnum == IT_AXE as f32 {
                    deathstring = " was axe-murdered by ";
                    deathstring2 = "\n";
                } else if rnum == IT_SHOTGUN as f32 {
                    deathstring = " chewed on ";
                    deathstring2 = "'s boomstick\n";
                } else if rnum == IT_SUPER_SHOTGUN as f32 {
                    deathstring = " ate 2 loads of ";
                    deathstring2 = "'s buckshot\n";
                } else if rnum == IT_NAILGUN as f32 {
                    deathstring = " was nailed by ";
                    deathstring2 = "\n";
                } else if rnum == IT_SUPER_NAILGUN as f32 {
                    deathstring = " was punctured by ";
                    deathstring2 = "\n";
                } else if rnum == IT_GRENADE_LAUNCHER as f32 {
                    deathstring = " eats ";
                    deathstring2 = "'s pineapple\n";
                    if g.r(targ).health < -40.0 {
                        deathstring = " was gibbed by ";
                        deathstring2 = "'s grenade\n";
                    }
                } else if rnum == IT_ROCKET_LAUNCHER as f32 {
                    deathstring = " rides ";
                    deathstring2 = "'s rocket\n";
                    if g.r(targ).health < -40.0 {
                        deathstring = " was gibbed by ";
                        deathstring2 = "'s rocket\n";
                    }
                } else if rnum == IT_LIGHTNING as f32 {
                    deathstring = " accepts ";
                    if g.r(attacker).waterlevel > 1 {
                        deathstring2 = "'s discharge\n";
                    } else {
                        deathstring2 = "'s shaft\n";
                    }
                }
                g.bprint(g.r(targ).netname);
                g.bprint(deathstring);
                g.bprint(g.r(attacker).netname);
                g.bprint(deathstring2);
            }
            return;
        } else {
            // killed by something else, which was not an attack by any player (incl. itself)
            g.v(targ).frags = g.r(targ).frags - 1.0;

            g.bprint(g.r(targ).netname);

            // killed by a monster?
            if (g.r(attacker).flags & FL_MONSTER) != 0 {
                let line = match g.r(attacker).classname {
                    "monster_army" => Some(" was shot by a Grunt\n"),
                    "monster_demon1" => Some(" was eviscerated by a Fiend\n"),
                    "monster_dog" => Some(" was mauled by a Rottweiler\n"),
                    "monster_dragon" => Some(" was fried by a Dragon\n"),
                    "monster_enforcer" => Some(" was blasted by an Enforcer\n"),
                    "monster_fish" => Some(" was fed to the Rotfish\n"),
                    "monster_hell_knight" => Some(" was slain by a Death Knight\n"),
                    "monster_knight" => Some(" was slashed by a Knight\n"),
                    "monster_ogre" => Some(" was destroyed by an Ogre\n"),
                    "monster_oldone" => Some(" became one with Shub-Niggurath\n"),
                    "monster_shalrath" => Some(" was exploded by a Vore\n"),
                    "monster_shambler" => Some(" was smashed by a Shambler\n"),
                    "monster_tarbaby" => Some(" was slimed by a Spawn\n"),
                    "monster_vomit" => Some(" was vomited on by a Vomitus\n"),
                    "monster_wizard" => Some(" was scragged by a Scrag\n"),
                    "monster_zombie" => Some(" joins the Zombies\n"),
                    _ => None,
                };
                if let Some(line) = line {
                    g.bprint(line);
                }

                return;
            }

            // tricks and traps
            if g.r(attacker).classname == "explo_box" {
                g.bprint(" blew up\n");
                return;
            }
            if g.r(attacker).solid == SOLID_BSP && attacker != world {
                g.bprint(" was squished\n");
                return;
            }
            if g.r(attacker).classname == "trap_shooter" || g.r(attacker).classname == "trap_spikeshooter" {
                g.bprint(" was spiked\n");
                return;
            }
            if g.r(attacker).classname == "fireball" {
                g.bprint(" ate a lavaball\n");
                return;
            }
            if g.r(attacker).classname == "trigger_changelevel" {
                g.bprint(" tried to leave\n");
                return;
            }

            // in-water deaths
            rnum = g.r(targ).watertype as f32;
            if rnum == CONTENT_WATER as f32 {
                if g.random() < 0.5 {
                    g.bprint(" sleeps with the fishes\n");
                } else {
                    g.bprint(" sucks it down\n");
                }
                return;
            } else if rnum == CONTENT_SLIME as f32 {
                if g.random() < 0.5 {
                    g.bprint(" gulped a load of slime\n");
                } else {
                    g.bprint(" can't exist on slime alone\n");
                }
                return;
            } else if rnum == CONTENT_LAVA as f32 {
                if g.r(targ).health < -15.0 {
                    g.bprint(" burst into flames\n");
                    return;
                }

                if g.random() < 0.5 {
                    g.bprint(" turned into hot slag\n");
                } else {
                    g.bprint(" visits the Volcano God\n");
                }
                return;
            }

            // fell to their death?
            if g.r(targ).deathtype == "falling" {
                g.v(targ).deathtype = "";
                g.bprint(" fell to his death\n");
                return;
            }

            // hell if I know; he's just dead!!!
            g.bprint(" died\n");
        }
    }
}
