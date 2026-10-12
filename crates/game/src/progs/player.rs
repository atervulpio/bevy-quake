//! `player.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;


// The model's frames, numbered as `$frame` numbers them: from nought, in order, in this file.
const F_axrun1: f32 = 0.0;
const F_rockrun1: f32 = 6.0;
const F_stand1: f32 = 12.0;
const F_axstnd1: f32 = 17.0;
const F_axpain1: f32 = 29.0;
const F_axpain2: f32 = 30.0;
const F_axpain3: f32 = 31.0;
const F_axpain4: f32 = 32.0;
const F_axpain5: f32 = 33.0;
const F_axpain6: f32 = 34.0;
const F_pain1: f32 = 35.0;
const F_pain2: f32 = 36.0;
const F_pain3: f32 = 37.0;
const F_pain4: f32 = 38.0;
const F_pain5: f32 = 39.0;
const F_pain6: f32 = 40.0;
const F_axdeth1: f32 = 41.0;
const F_axdeth2: f32 = 42.0;
const F_axdeth3: f32 = 43.0;
const F_axdeth4: f32 = 44.0;
const F_axdeth5: f32 = 45.0;
const F_axdeth6: f32 = 46.0;
const F_axdeth7: f32 = 47.0;
const F_axdeth8: f32 = 48.0;
const F_axdeth9: f32 = 49.0;
const F_deatha1: f32 = 50.0;
const F_deatha2: f32 = 51.0;
const F_deatha3: f32 = 52.0;
const F_deatha4: f32 = 53.0;
const F_deatha5: f32 = 54.0;
const F_deatha6: f32 = 55.0;
const F_deatha7: f32 = 56.0;
const F_deatha8: f32 = 57.0;
const F_deatha9: f32 = 58.0;
const F_deatha10: f32 = 59.0;
const F_deatha11: f32 = 60.0;
const F_deathb1: f32 = 61.0;
const F_deathb2: f32 = 62.0;
const F_deathb3: f32 = 63.0;
const F_deathb4: f32 = 64.0;
const F_deathb5: f32 = 65.0;
const F_deathb6: f32 = 66.0;
const F_deathb7: f32 = 67.0;
const F_deathb8: f32 = 68.0;
const F_deathb9: f32 = 69.0;
const F_deathc1: f32 = 70.0;
const F_deathc2: f32 = 71.0;
const F_deathc3: f32 = 72.0;
const F_deathc4: f32 = 73.0;
const F_deathc5: f32 = 74.0;
const F_deathc6: f32 = 75.0;
const F_deathc7: f32 = 76.0;
const F_deathc8: f32 = 77.0;
const F_deathc9: f32 = 78.0;
const F_deathc10: f32 = 79.0;
const F_deathc11: f32 = 80.0;
const F_deathc12: f32 = 81.0;
const F_deathc13: f32 = 82.0;
const F_deathc14: f32 = 83.0;
const F_deathc15: f32 = 84.0;
const F_deathd1: f32 = 85.0;
const F_deathd2: f32 = 86.0;
const F_deathd3: f32 = 87.0;
const F_deathd4: f32 = 88.0;
const F_deathd5: f32 = 89.0;
const F_deathd6: f32 = 90.0;
const F_deathd7: f32 = 91.0;
const F_deathd8: f32 = 92.0;
const F_deathd9: f32 = 93.0;
const F_deathe1: f32 = 94.0;
const F_deathe2: f32 = 95.0;
const F_deathe3: f32 = 96.0;
const F_deathe4: f32 = 97.0;
const F_deathe5: f32 = 98.0;
const F_deathe6: f32 = 99.0;
const F_deathe7: f32 = 100.0;
const F_deathe8: f32 = 101.0;
const F_deathe9: f32 = 102.0;
const F_nailatt1: f32 = 103.0;
const F_nailatt2: f32 = 104.0;
const F_light1: f32 = 105.0;
const F_light2: f32 = 106.0;
const F_rockatt1: f32 = 107.0;
const F_rockatt2: f32 = 108.0;
const F_rockatt3: f32 = 109.0;
const F_rockatt4: f32 = 110.0;
const F_rockatt5: f32 = 111.0;
const F_rockatt6: f32 = 112.0;
const F_shotatt1: f32 = 113.0;
const F_shotatt2: f32 = 114.0;
const F_shotatt3: f32 = 115.0;
const F_shotatt4: f32 = 116.0;
const F_shotatt5: f32 = 117.0;
const F_shotatt6: f32 = 118.0;
const F_axatt1: f32 = 119.0;
const F_axatt2: f32 = 120.0;
const F_axatt3: f32 = 121.0;
const F_axatt4: f32 = 122.0;
const F_axattb1: f32 = 125.0;
const F_axattb2: f32 = 126.0;
const F_axattb3: f32 = 127.0;
const F_axattb4: f32 = 128.0;
const F_axattc1: f32 = 131.0;
const F_axattc2: f32 = 132.0;
const F_axattc3: f32 = 133.0;
const F_axattc4: f32 = 134.0;
const F_axattd1: f32 = 137.0;
const F_axattd2: f32 = 138.0;
const F_axattd3: f32 = 139.0;
const F_axattd4: f32 = 140.0;


pub fn player_stand1(g: &mut Game) {
    g.frame_state(F_axstnd1, player_stand1);
    let s = g.self_;
    g.v(s).weaponframe = 0.0;

    if g.r(s).velocity.x != 0.0 || g.r(s).velocity.y != 0.0 {
        g.v(s).walkframe = 0.0;
        player_run(g);
        return;
    }

    let v = g.v(s);
    if v.weapon == IT_AXE {
        if v.walkframe >= 12.0 {
            v.walkframe = 0.0;
        }
        v.frame = F_axstnd1 + v.walkframe;
    } else {
        if v.walkframe >= 5.0 {
            v.walkframe = 0.0;
        }
        v.frame = F_stand1 + v.walkframe;
    }
    v.walkframe = v.walkframe + 1.0;
}

pub fn player_run(g: &mut Game) {
    g.frame_state(F_rockrun1, player_run);
    let s = g.self_;
    g.v(s).weaponframe = 0.0;

    if g.r(s).velocity.x == 0.0 && g.r(s).velocity.y == 0.0 {
        g.v(s).walkframe = 0.0;
        player_stand1(g);
        return;
    }

    let v = g.v(s);
    if v.weapon == IT_AXE {
        if v.walkframe == 6.0 {
            v.walkframe = 0.0;
        }
        v.frame = F_axrun1 + v.walkframe;
    } else {
        if v.walkframe == 6.0 {
            v.walkframe = 0.0;
        }
        v.frame = v.frame + v.walkframe;
    }
    v.walkframe = v.walkframe + 1.0;
}

pub fn player_shot1(g: &mut Game) {
    g.frame_state(F_shotatt1, player_shot2);
    let s = g.self_;
    g.v(s).weaponframe = 1.0;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;
}

pub fn player_shot2(g: &mut Game) {
    g.frame_state(F_shotatt2, player_shot3);
    g.v(g.self_).weaponframe = 2.0;
}

pub fn player_shot3(g: &mut Game) {
    g.frame_state(F_shotatt3, player_shot4);
    g.v(g.self_).weaponframe = 3.0;
}

pub fn player_shot4(g: &mut Game) {
    g.frame_state(F_shotatt4, player_shot5);
    g.v(g.self_).weaponframe = 4.0;
}

pub fn player_shot5(g: &mut Game) {
    g.frame_state(F_shotatt5, player_shot6);
    g.v(g.self_).weaponframe = 5.0;
}

pub fn player_shot6(g: &mut Game) {
    g.frame_state(F_shotatt6, player_run);
    g.v(g.self_).weaponframe = 6.0;
}

pub fn player_axe1(g: &mut Game) {
    g.frame_state(F_axatt1, player_axe2);
    g.v(g.self_).weaponframe = 1.0;
}

pub fn player_axe2(g: &mut Game) {
    g.frame_state(F_axatt2, player_axe3);
    g.v(g.self_).weaponframe = 2.0;
}

pub fn player_axe3(g: &mut Game) {
    g.frame_state(F_axatt3, player_axe4);
    g.v(g.self_).weaponframe = 3.0;
    W_FireAxe(g);
}

pub fn player_axe4(g: &mut Game) {
    g.frame_state(F_axatt4, player_run);
    g.v(g.self_).weaponframe = 4.0;
}

pub fn player_axeb1(g: &mut Game) {
    g.frame_state(F_axattb1, player_axeb2);
    g.v(g.self_).weaponframe = 5.0;
}

pub fn player_axeb2(g: &mut Game) {
    g.frame_state(F_axattb2, player_axeb3);
    g.v(g.self_).weaponframe = 6.0;
}

pub fn player_axeb3(g: &mut Game) {
    g.frame_state(F_axattb3, player_axeb4);
    g.v(g.self_).weaponframe = 7.0;
    W_FireAxe(g);
}

pub fn player_axeb4(g: &mut Game) {
    g.frame_state(F_axattb4, player_run);
    g.v(g.self_).weaponframe = 8.0;
}

pub fn player_axec1(g: &mut Game) {
    g.frame_state(F_axattc1, player_axec2);
    g.v(g.self_).weaponframe = 1.0;
}

pub fn player_axec2(g: &mut Game) {
    g.frame_state(F_axattc2, player_axec3);
    g.v(g.self_).weaponframe = 2.0;
}

pub fn player_axec3(g: &mut Game) {
    g.frame_state(F_axattc3, player_axec4);
    g.v(g.self_).weaponframe = 3.0;
    W_FireAxe(g);
}

pub fn player_axec4(g: &mut Game) {
    g.frame_state(F_axattc4, player_run);
    g.v(g.self_).weaponframe = 4.0;
}

pub fn player_axed1(g: &mut Game) {
    g.frame_state(F_axattd1, player_axed2);
    g.v(g.self_).weaponframe = 5.0;
}

pub fn player_axed2(g: &mut Game) {
    g.frame_state(F_axattd2, player_axed3);
    g.v(g.self_).weaponframe = 6.0;
}

pub fn player_axed3(g: &mut Game) {
    g.frame_state(F_axattd3, player_axed4);
    g.v(g.self_).weaponframe = 7.0;
    W_FireAxe(g);
}

pub fn player_axed4(g: &mut Game) {
    g.frame_state(F_axattd4, player_run);
    g.v(g.self_).weaponframe = 8.0;
}

//============================================================================

pub fn player_nail1(g: &mut Game) {
    g.frame_state(F_nailatt1, player_nail2);
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;

    if g.r(s).button0 == 0 {
        player_run(g);
        return;
    }

    g.v(s).weaponframe = g.r(s).weaponframe + 1.0;
    if g.r(s).weaponframe == 9.0 {
        g.v(s).weaponframe = 1.0;
    }

    SuperDamageSound(g);
    W_FireSpikes(g, 4.0);
    g.v(s).attack_finished = g.time + 0.2;
}

pub fn player_nail2(g: &mut Game) {
    g.frame_state(F_nailatt2, player_nail1);
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;

    if g.r(s).button0 == 0 {
        player_run(g);
        return;
    }

    g.v(s).weaponframe = g.r(s).weaponframe + 1.0;
    if g.r(s).weaponframe == 9.0 {
        g.v(s).weaponframe = 1.0;
    }

    SuperDamageSound(g);
    W_FireSpikes(g, -4.0);
    g.v(s).attack_finished = g.time + 0.2;
}

//============================================================================

pub fn player_light1(g: &mut Game) {
    g.frame_state(F_light1, player_light2);
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;

    if g.r(s).button0 == 0 {
        player_run(g);
        return;
    }

    g.v(s).weaponframe = g.r(s).weaponframe + 1.0;
    if g.r(s).weaponframe == 5.0 {
        g.v(s).weaponframe = 1.0;
    }

    SuperDamageSound(g);
    W_FireLightning(g);
    g.v(s).attack_finished = g.time + 0.2;
}

pub fn player_light2(g: &mut Game) {
    g.frame_state(F_light2, player_light1);
    let s = g.self_;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;

    if g.r(s).button0 == 0 {
        player_run(g);
        return;
    }

    g.v(s).weaponframe = g.r(s).weaponframe + 1.0;
    if g.r(s).weaponframe == 5.0 {
        g.v(s).weaponframe = 1.0;
    }

    SuperDamageSound(g);
    W_FireLightning(g);
    g.v(s).attack_finished = g.time + 0.2;
}

//============================================================================

pub fn player_rocket1(g: &mut Game) {
    g.frame_state(F_rockatt1, player_rocket2);
    let s = g.self_;
    g.v(s).weaponframe = 1.0;
    g.v(s).effects = g.r(s).effects | EF_MUZZLEFLASH;
}

pub fn player_rocket2(g: &mut Game) {
    g.frame_state(F_rockatt2, player_rocket3);
    g.v(g.self_).weaponframe = 2.0;
}

pub fn player_rocket3(g: &mut Game) {
    g.frame_state(F_rockatt3, player_rocket4);
    g.v(g.self_).weaponframe = 3.0;
}

pub fn player_rocket4(g: &mut Game) {
    g.frame_state(F_rockatt4, player_rocket5);
    g.v(g.self_).weaponframe = 4.0;
}

pub fn player_rocket5(g: &mut Game) {
    g.frame_state(F_rockatt5, player_rocket6);
    g.v(g.self_).weaponframe = 5.0;
}

pub fn player_rocket6(g: &mut Game) {
    g.frame_state(F_rockatt6, player_run);
    g.v(g.self_).weaponframe = 6.0;
}

pub fn PainSound(g: &mut Game) {
    let s = g.self_;
    if g.r(s).health < 0.0 {
        return;
    }

    if g.r(g.damage_attacker).classname == "teledeath" {
        g.sound(s, CHAN_VOICE, "player/teledth1.wav", 1.0, ATTN_NONE);
        return;
    }

    // water pain sounds
    if g.r(s).watertype == CONTENT_WATER && g.r(s).waterlevel == 3 {
        DeathBubbles(g, 1.0);
        if g.random() > 0.5 {
            g.sound(s, CHAN_VOICE, "player/drown1.wav", 1.0, ATTN_NORM);
        } else {
            g.sound(s, CHAN_VOICE, "player/drown2.wav", 1.0, ATTN_NORM);
        }
        return;
    }

    // slime pain sounds
    if g.r(s).watertype == CONTENT_SLIME {
        // FIX ME put in some steam here
        if g.random() > 0.5 {
            g.sound(s, CHAN_VOICE, "player/lburn1.wav", 1.0, ATTN_NORM);
        } else {
            g.sound(s, CHAN_VOICE, "player/lburn2.wav", 1.0, ATTN_NORM);
        }
        return;
    }

    if g.r(s).watertype == CONTENT_LAVA {
        if g.random() > 0.5 {
            g.sound(s, CHAN_VOICE, "player/lburn1.wav", 1.0, ATTN_NORM);
        } else {
            g.sound(s, CHAN_VOICE, "player/lburn2.wav", 1.0, ATTN_NORM);
        }
        return;
    }

    if g.r(s).pain_finished > g.time {
        g.v(s).axhitme = FALSE;
        return;
    }

    g.v(s).pain_finished = g.time + 0.5;

    // don't make multiple pain sounds right after each other

    // axe pain sound
    if g.r(s).axhitme == TRUE {
        g.v(s).axhitme = FALSE;
        g.sound(s, CHAN_VOICE, "player/axhit1.wav", 1.0, ATTN_NORM);
        return;
    }

    let rs = rint((g.random() * 5.0) + 1.0);

    g.v(s).noise = "";
    g.v(s).noise = if rs == 1.0 {
        "player/pain1.wav"
    } else if rs == 2.0 {
        "player/pain2.wav"
    } else if rs == 3.0 {
        "player/pain3.wav"
    } else if rs == 4.0 {
        "player/pain4.wav"
    } else if rs == 5.0 {
        "player/pain5.wav"
    } else {
        "player/pain6.wav"
    };

    let noise = g.r(s).noise;
    g.sound(s, CHAN_VOICE, noise, 1.0, ATTN_NORM);
}

pub fn player_pain1(g: &mut Game) {
    g.frame_state(F_pain1, player_pain2);
    PainSound(g);
    g.v(g.self_).weaponframe = 0.0;
}

pub fn player_pain2(g: &mut Game) {
    g.frame_state(F_pain2, player_pain3);
}

pub fn player_pain3(g: &mut Game) {
    g.frame_state(F_pain3, player_pain4);
}

pub fn player_pain4(g: &mut Game) {
    g.frame_state(F_pain4, player_pain5);
}

pub fn player_pain5(g: &mut Game) {
    g.frame_state(F_pain5, player_pain6);
}

pub fn player_pain6(g: &mut Game) {
    g.frame_state(F_pain6, player_run);
}

pub fn player_axpain1(g: &mut Game) {
    g.frame_state(F_axpain1, player_axpain2);
    PainSound(g);
    g.v(g.self_).weaponframe = 0.0;
}

pub fn player_axpain2(g: &mut Game) {
    g.frame_state(F_axpain2, player_axpain3);
}

pub fn player_axpain3(g: &mut Game) {
    g.frame_state(F_axpain3, player_axpain4);
}

pub fn player_axpain4(g: &mut Game) {
    g.frame_state(F_axpain4, player_axpain5);
}

pub fn player_axpain5(g: &mut Game) {
    g.frame_state(F_axpain5, player_axpain6);
}

pub fn player_axpain6(g: &mut Game) {
    g.frame_state(F_axpain6, player_run);
}

pub fn player_pain(g: &mut Game, attacker: Ent, damage: f32) {
    let s = g.self_;
    if g.r(s).weaponframe != 0.0 {
        return;
    }
    if g.r(s).invisible_finished > g.time {
        return; // eyes don't have pain frames
    }

    if g.r(s).weapon == IT_AXE {
        player_axpain1(g);
    } else {
        player_pain1(g);
    }
}

pub fn DeathBubblesSpawn(g: &mut Game) {
    let s = g.self_;
    let owner = g.r(s).owner;
    if g.r(owner).waterlevel != 3 {
        return;
    }

    let bubble = g.spawn();
    g.setmodel(bubble, "progs/s_bubble.spr");
    let o = g.r(owner).origin + Vec3::new(0.0, 0.0, 24.0);
    g.setorigin(bubble, o);
    g.v(bubble).movetype = MOVETYPE_NOCLIP;
    g.v(bubble).solid = SOLID_NOT;
    g.v(bubble).velocity = Vec3::new(0.0, 0.0, 15.0);
    g.v(bubble).nextthink = g.time + 0.5;
    g.v(bubble).think = Some(bubble_bob);
    g.v(bubble).classname = "bubble";
    g.v(bubble).frame = 0.0;
    g.v(bubble).cnt = 0.0;
    g.setsize(bubble, Vec3::new(-8.0, -8.0, -8.0), Vec3::new(8.0, 8.0, 8.0));
    g.v(s).nextthink = g.time + 0.1;
    g.v(s).think = Some(DeathBubblesSpawn);
    g.v(s).air_finished = g.r(s).air_finished + 1.0;
    if g.r(s).air_finished >= g.r(s).bubble_count {
        g.remove(s);
    }
}

pub fn DeathBubbles(g: &mut Game, num_bubbles: f32) {
    let s = g.self_;
    let bubble_spawner = g.spawn();
    let o = g.r(s).origin;
    g.setorigin(bubble_spawner, o);
    g.v(bubble_spawner).movetype = MOVETYPE_NONE;
    g.v(bubble_spawner).solid = SOLID_NOT;
    g.v(bubble_spawner).nextthink = g.time + 0.1;
    g.v(bubble_spawner).think = Some(DeathBubblesSpawn);
    g.v(bubble_spawner).air_finished = 0.0;
    g.v(bubble_spawner).owner = s;
    g.v(bubble_spawner).bubble_count = num_bubbles;
}

pub fn DeathSound(g: &mut Game) {
    let s = g.self_;
    // water death sounds
    if g.r(s).waterlevel == 3 {
        DeathBubbles(g, 20.0);
        g.sound(s, CHAN_VOICE, "player/h2odeath.wav", 1.0, ATTN_NONE);
        return;
    }

    let rs = rint((g.random() * 4.0) + 1.0);
    if rs == 1.0 {
        g.v(s).noise = "player/death1.wav";
    } else if rs == 2.0 {
        g.v(s).noise = "player/death2.wav";
    } else if rs == 3.0 {
        g.v(s).noise = "player/death3.wav";
    } else if rs == 4.0 {
        g.v(s).noise = "player/death4.wav";
    } else if rs == 5.0 {
        g.v(s).noise = "player/death5.wav";
    }

    let noise = g.r(s).noise;
    g.sound(s, CHAN_VOICE, noise, 1.0, ATTN_NONE);
}

pub fn PlayerDead(g: &mut Game) {
    let s = g.self_;
    g.v(s).nextthink = -1.0;
    // allow respawn after a certain time
    g.v(s).deadflag = DEAD_DEAD;
}

pub fn VelocityForDamage(g: &mut Game, dm: f32) -> Vec3 {
    let mut v = Vec3::ZERO;
    v.x = 100.0 * crandom(g);
    v.y = 100.0 * crandom(g);
    v.z = 200.0 + 100.0 * g.random();

    if dm > -50.0 {
        v = v * 0.7;
    } else if dm > -200.0 {
        v = v * 2.0;
    } else {
        v = v * 10.0;
    }

    v
}

pub fn ThrowGib(g: &mut Game, gibname: Str, dm: f32) {
    let s = g.self_;
    let new = g.spawn();
    g.v(new).origin = g.r(s).origin;
    g.setmodel(new, gibname);
    g.setsize(new, Vec3::ZERO, Vec3::ZERO);
    g.v(new).velocity = VelocityForDamage(g, dm);
    g.v(new).movetype = MOVETYPE_BOUNCE;
    g.v(new).solid = SOLID_NOT;
    g.v(new).avelocity.x = g.random() * 600.0;
    g.v(new).avelocity.y = g.random() * 600.0;
    g.v(new).avelocity.z = g.random() * 600.0;
    g.v(new).think = Some(SUB_Remove);
    g.v(new).ltime = g.time;
    g.v(new).nextthink = g.time + 10.0 + g.random() * 10.0;
    g.v(new).frame = 0.0;
    g.v(new).flags = 0;
}

pub fn ThrowHead(g: &mut Game, gibname: Str, dm: f32) {
    let s = g.self_;
    g.setmodel(s, gibname);
    g.v(s).frame = 0.0;
    g.v(s).nextthink = -1.0;
    g.v(s).movetype = MOVETYPE_BOUNCE;
    g.v(s).takedamage = DAMAGE_NO;
    g.v(s).solid = SOLID_NOT;
    g.v(s).view_ofs = Vec3::new(0.0, 0.0, 8.0);
    g.setsize(s, Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 56.0));
    g.v(s).velocity = VelocityForDamage(g, dm);
    g.v(s).origin.z = g.r(s).origin.z - 24.0;
    g.v(s).flags = g.r(s).flags - (g.r(s).flags & FL_ONGROUND);
    g.v(s).avelocity = crandom(g) * Vec3::new(0.0, 600.0, 0.0);
}

pub fn GibPlayer(g: &mut Game) {
    let s = g.self_;
    let h = g.r(s).health;
    ThrowHead(g, "progs/h_player.mdl", h);
    let h = g.r(s).health;
    ThrowGib(g, "progs/gib1.mdl", h);
    let h = g.r(s).health;
    ThrowGib(g, "progs/gib2.mdl", h);
    let h = g.r(s).health;
    ThrowGib(g, "progs/gib3.mdl", h);

    g.v(s).deadflag = DEAD_DEAD;

    if g.r(g.damage_attacker).classname == "teledeath" {
        g.sound(s, CHAN_VOICE, "player/teledth1.wav", 1.0, ATTN_NONE);
        return;
    }

    if g.r(g.damage_attacker).classname == "teledeath2" {
        g.sound(s, CHAN_VOICE, "player/teledth1.wav", 1.0, ATTN_NONE);
        return;
    }

    if g.random() < 0.5 {
        g.sound(s, CHAN_VOICE, "player/gib.wav", 1.0, ATTN_NONE);
    } else {
        g.sound(s, CHAN_VOICE, "player/udeath.wav", 1.0, ATTN_NONE);
    }
}

pub fn PlayerDie(g: &mut Game) {
    let s = g.self_;
    g.v(s).items = g.r(s).items - (g.r(s).items & IT_INVISIBILITY);
    g.v(s).invisible_finished = 0.0; // don't die as eyes
    g.v(s).invincible_finished = 0.0;
    g.v(s).super_damage_finished = 0.0;
    g.v(s).radsuit_finished = 0.0;
    g.v(s).modelindex = g.modelindex_player; // don't use eyes

    if g.deathmatch != 0.0 || g.coop != 0.0 {
        DropBackpack(g);
    }

    g.v(s).weaponmodel = "";
    g.v(s).view_ofs = Vec3::new(0.0, 0.0, -8.0);
    g.v(s).deadflag = DEAD_DYING;
    g.v(s).solid = SOLID_NOT;
    g.v(s).flags = g.r(s).flags - (g.r(s).flags & FL_ONGROUND);
    g.v(s).movetype = MOVETYPE_TOSS;
    if g.r(s).velocity.z < 10.0 {
        let r = g.random();
        g.v(s).velocity.z = g.r(s).velocity.z + r * 300.0;
    }

    if g.r(s).health < -40.0 {
        GibPlayer(g);
        return;
    }

    DeathSound(g);

    g.v(s).angles.x = 0.0;
    g.v(s).angles.z = 0.0;

    if g.r(s).weapon == IT_AXE {
        player_die_ax1(g);
        return;
    }

    let mut i = g.cvar("temp1"); // FIXME/Maddes: remove/disable old testing code
    if i == 0.0 {
        i = 1.0 + floor(g.random() * 6.0);
    }
    if i == 1.0 {
        player_diea1(g);
    } else if i == 2.0 {
        player_dieb1(g);
    } else if i == 3.0 {
        player_diec1(g);
    } else if i == 4.0 {
        player_died1(g);
    } else {
        player_diee1(g);
    }
}

/// Used by the kill command and the disconnect command.
pub fn set_suicide_frame(g: &mut Game) {
    let s = g.self_;
    if g.r(s).model != "progs/player.mdl" {
        return; // already gibbed
    }

    g.v(s).frame = F_deatha11;
    g.v(s).solid = SOLID_NOT;
    g.v(s).movetype = MOVETYPE_TOSS;
    g.v(s).deadflag = DEAD_DEAD;
    g.v(s).nextthink = -1.0;
}

pub fn player_diea1(g: &mut Game) {
    g.frame_state(F_deatha1, player_diea2);
}

pub fn player_diea2(g: &mut Game) {
    g.frame_state(F_deatha2, player_diea3);
}

pub fn player_diea3(g: &mut Game) {
    g.frame_state(F_deatha3, player_diea4);
}

pub fn player_diea4(g: &mut Game) {
    g.frame_state(F_deatha4, player_diea5);
}

pub fn player_diea5(g: &mut Game) {
    g.frame_state(F_deatha5, player_diea6);
}

pub fn player_diea6(g: &mut Game) {
    g.frame_state(F_deatha6, player_diea7);
}

pub fn player_diea7(g: &mut Game) {
    g.frame_state(F_deatha7, player_diea8);
}

pub fn player_diea8(g: &mut Game) {
    g.frame_state(F_deatha8, player_diea9);
}

pub fn player_diea9(g: &mut Game) {
    g.frame_state(F_deatha9, player_diea10);
}

pub fn player_diea10(g: &mut Game) {
    g.frame_state(F_deatha10, player_diea11);
}

pub fn player_diea11(g: &mut Game) {
    g.frame_state(F_deatha11, player_diea11);
    PlayerDead(g);
}

pub fn player_dieb1(g: &mut Game) {
    g.frame_state(F_deathb1, player_dieb2);
}

pub fn player_dieb2(g: &mut Game) {
    g.frame_state(F_deathb2, player_dieb3);
}

pub fn player_dieb3(g: &mut Game) {
    g.frame_state(F_deathb3, player_dieb4);
}

pub fn player_dieb4(g: &mut Game) {
    g.frame_state(F_deathb4, player_dieb5);
}

pub fn player_dieb5(g: &mut Game) {
    g.frame_state(F_deathb5, player_dieb6);
}

pub fn player_dieb6(g: &mut Game) {
    g.frame_state(F_deathb6, player_dieb7);
}

pub fn player_dieb7(g: &mut Game) {
    g.frame_state(F_deathb7, player_dieb8);
}

pub fn player_dieb8(g: &mut Game) {
    g.frame_state(F_deathb8, player_dieb9);
}

pub fn player_dieb9(g: &mut Game) {
    g.frame_state(F_deathb9, player_dieb9);
    PlayerDead(g);
}

pub fn player_diec1(g: &mut Game) {
    g.frame_state(F_deathc1, player_diec2);
}

pub fn player_diec2(g: &mut Game) {
    g.frame_state(F_deathc2, player_diec3);
}

pub fn player_diec3(g: &mut Game) {
    g.frame_state(F_deathc3, player_diec4);
}

pub fn player_diec4(g: &mut Game) {
    g.frame_state(F_deathc4, player_diec5);
}

pub fn player_diec5(g: &mut Game) {
    g.frame_state(F_deathc5, player_diec6);
}

pub fn player_diec6(g: &mut Game) {
    g.frame_state(F_deathc6, player_diec7);
}

pub fn player_diec7(g: &mut Game) {
    g.frame_state(F_deathc7, player_diec8);
}

pub fn player_diec8(g: &mut Game) {
    g.frame_state(F_deathc8, player_diec9);
}

pub fn player_diec9(g: &mut Game) {
    g.frame_state(F_deathc9, player_diec10);
}

pub fn player_diec10(g: &mut Game) {
    g.frame_state(F_deathc10, player_diec11);
}

pub fn player_diec11(g: &mut Game) {
    g.frame_state(F_deathc11, player_diec12);
}

pub fn player_diec12(g: &mut Game) {
    g.frame_state(F_deathc12, player_diec13);
}

pub fn player_diec13(g: &mut Game) {
    g.frame_state(F_deathc13, player_diec14);
}

pub fn player_diec14(g: &mut Game) {
    g.frame_state(F_deathc14, player_diec15);
}

pub fn player_diec15(g: &mut Game) {
    g.frame_state(F_deathc15, player_diec15);
    PlayerDead(g);
}

pub fn player_died1(g: &mut Game) {
    g.frame_state(F_deathd1, player_died2);
}

pub fn player_died2(g: &mut Game) {
    g.frame_state(F_deathd2, player_died3);
}

pub fn player_died3(g: &mut Game) {
    g.frame_state(F_deathd3, player_died4);
}

pub fn player_died4(g: &mut Game) {
    g.frame_state(F_deathd4, player_died5);
}

pub fn player_died5(g: &mut Game) {
    g.frame_state(F_deathd5, player_died6);
}

pub fn player_died6(g: &mut Game) {
    g.frame_state(F_deathd6, player_died7);
}

pub fn player_died7(g: &mut Game) {
    g.frame_state(F_deathd7, player_died8);
}

pub fn player_died8(g: &mut Game) {
    g.frame_state(F_deathd8, player_died9);
}

pub fn player_died9(g: &mut Game) {
    g.frame_state(F_deathd9, player_died9);
    PlayerDead(g);
}

pub fn player_diee1(g: &mut Game) {
    g.frame_state(F_deathe1, player_diee2);
}

pub fn player_diee2(g: &mut Game) {
    g.frame_state(F_deathe2, player_diee3);
}

pub fn player_diee3(g: &mut Game) {
    g.frame_state(F_deathe3, player_diee4);
}

pub fn player_diee4(g: &mut Game) {
    g.frame_state(F_deathe4, player_diee5);
}

pub fn player_diee5(g: &mut Game) {
    g.frame_state(F_deathe5, player_diee6);
}

pub fn player_diee6(g: &mut Game) {
    g.frame_state(F_deathe6, player_diee7);
}

pub fn player_diee7(g: &mut Game) {
    g.frame_state(F_deathe7, player_diee8);
}

pub fn player_diee8(g: &mut Game) {
    g.frame_state(F_deathe8, player_diee9);
}

pub fn player_diee9(g: &mut Game) {
    g.frame_state(F_deathe9, player_diee9);
    PlayerDead(g);
}

pub fn player_die_ax1(g: &mut Game) {
    g.frame_state(F_axdeth1, player_die_ax2);
}

pub fn player_die_ax2(g: &mut Game) {
    g.frame_state(F_axdeth2, player_die_ax3);
}

pub fn player_die_ax3(g: &mut Game) {
    g.frame_state(F_axdeth3, player_die_ax4);
}

pub fn player_die_ax4(g: &mut Game) {
    g.frame_state(F_axdeth4, player_die_ax5);
}

pub fn player_die_ax5(g: &mut Game) {
    g.frame_state(F_axdeth5, player_die_ax6);
}

pub fn player_die_ax6(g: &mut Game) {
    g.frame_state(F_axdeth6, player_die_ax7);
}

pub fn player_die_ax7(g: &mut Game) {
    g.frame_state(F_axdeth7, player_die_ax8);
}

pub fn player_die_ax8(g: &mut Game) {
    g.frame_state(F_axdeth8, player_die_ax9);
}

pub fn player_die_ax9(g: &mut Game) {
    g.frame_state(F_axdeth9, player_die_ax9);
    PlayerDead(g);
}
