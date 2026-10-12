//! `world.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;


pub fn main(g: &mut Game) {
    g.dprint("main function\n");

    // these are just commands for the prog compiler to copy these files
    for f in ["progs.dat", "gfx.wad", "quake.rc", "default.cfg", "end1.bin"] {
        g.precache_file(f);
    }
    g.precache_file2("end2.bin");

    for f in ["demo1.dem", "demo2.dem", "demo3.dem"] {
        g.precache_file(f);
    }

    // these are all of the lumps from the cached.ls files
    g.precache_file("gfx/palette.lmp");
    g.precache_file("gfx/colormap.lmp");

    g.precache_file2("gfx/pop.lmp");

    for f in [
        "gfx/complete.lmp",
        "gfx/inter.lmp",
        "gfx/ranking.lmp",
        "gfx/vidmodes.lmp",
        "gfx/finale.lmp",
        "gfx/conback.lmp",
        "gfx/qplaque.lmp",
        "gfx/menudot1.lmp",
        "gfx/menudot2.lmp",
        "gfx/menudot3.lmp",
        "gfx/menudot4.lmp",
        "gfx/menudot5.lmp",
        "gfx/menudot6.lmp",
        "gfx/menuplyr.lmp",
        "gfx/bigbox.lmp",
        "gfx/dim_modm.lmp",
        "gfx/dim_drct.lmp",
        "gfx/dim_ipx.lmp",
        "gfx/dim_tcp.lmp",
        "gfx/dim_mult.lmp",
        "gfx/mainmenu.lmp",
        "gfx/box_tl.lmp",
        "gfx/box_tm.lmp",
        "gfx/box_tr.lmp",
        "gfx/box_ml.lmp",
        "gfx/box_mm.lmp",
        "gfx/box_mm2.lmp",
        "gfx/box_mr.lmp",
        "gfx/box_bl.lmp",
        "gfx/box_bm.lmp",
        "gfx/box_br.lmp",
        "gfx/sp_menu.lmp",
        "gfx/ttl_sgl.lmp",
        "gfx/ttl_main.lmp",
        "gfx/ttl_cstm.lmp",
        "gfx/mp_menu.lmp",
        "gfx/netmen1.lmp",
        "gfx/netmen2.lmp",
        "gfx/netmen3.lmp",
        "gfx/netmen4.lmp",
        "gfx/netmen5.lmp",
        "gfx/sell.lmp",
        "gfx/help0.lmp",
        "gfx/help1.lmp",
        "gfx/help2.lmp",
        "gfx/help3.lmp",
        "gfx/help4.lmp",
        "gfx/help5.lmp",
        "gfx/pause.lmp",
        "gfx/loading.lmp",
        "gfx/p_option.lmp",
        "gfx/p_load.lmp",
        "gfx/p_save.lmp",
        "gfx/p_multi.lmp",
    ] {
        g.precache_file(f);
    }

    // sounds loaded by C code
    g.precache_sound("misc/menu1.wav");
    g.precache_sound("misc/menu2.wav");
    g.precache_sound("misc/menu3.wav");

    g.precache_sound("ambience/water1.wav");
    g.precache_sound("ambience/wind2.wav");

    // shareware
    for f in [
        "maps/start.bsp",
        "maps/e1m1.bsp",
        "maps/e1m2.bsp",
        "maps/e1m3.bsp",
        "maps/e1m4.bsp",
        "maps/e1m5.bsp",
        "maps/e1m6.bsp",
        "maps/e1m7.bsp",
        "maps/e1m8.bsp",
    ] {
        g.precache_file(f);
    }

    // registered
    for f in [
        "gfx/pop.lmp",
        "maps/e2m1.bsp",
        "maps/e2m2.bsp",
        "maps/e2m3.bsp",
        "maps/e2m4.bsp",
        "maps/e2m5.bsp",
        "maps/e2m6.bsp",
        "maps/e2m7.bsp",
        "maps/e3m1.bsp",
        "maps/e3m2.bsp",
        "maps/e3m3.bsp",
        "maps/e3m4.bsp",
        "maps/e3m5.bsp",
        "maps/e3m6.bsp",
        "maps/e3m7.bsp",
        "maps/e4m1.bsp",
        "maps/e4m2.bsp",
        "maps/e4m3.bsp",
        "maps/e4m4.bsp",
        "maps/e4m5.bsp",
        "maps/e4m6.bsp",
        "maps/e4m7.bsp",
        "maps/e4m8.bsp",
        "maps/end.bsp",
        "maps/dm1.bsp",
        "maps/dm2.bsp",
        "maps/dm3.bsp",
        "maps/dm4.bsp",
        "maps/dm5.bsp",
        "maps/dm6.bsp",
    ] {
        g.precache_file2(f);
    }
}

/// Only used for the world entity. `message` is the level's name, `sounds` its CD track,
/// `worldtype` 0 medieval, 1 metal, 2 base.
pub fn worldspawn(g: &mut Game) {
    g.lastspawn = world;
    InitBodyQueue(g);

    // custom map attributes
    if g.r(g.self_).model == "maps/e1m8.bsp" {
        g.cvar_set("sv_gravity", "100");
    } else {
        g.cvar_set("sv_gravity", "800");
    }

    // the area based ambient sounds MUST be the first precache_sounds

    // player precaches
    W_Precache(g); // get weapon precaches

    // sounds used from C physics code
    g.precache_sound("demon/dland2.wav"); // landing thud
    g.precache_sound("misc/h2ohit1.wav"); // landing splash

    // setup precaches always needed
    g.precache_sound("items/itembk2.wav"); // item respawn sound
    g.precache_sound("player/plyrjmp8.wav"); // player jump
    g.precache_sound("player/land.wav"); // player landing
    g.precache_sound("player/land2.wav"); // player hurt landing
    g.precache_sound("player/drown1.wav"); // drowning pain
    g.precache_sound("player/drown2.wav"); // drowning pain
    g.precache_sound("player/gasp1.wav"); // gasping for air
    g.precache_sound("player/gasp2.wav"); // taking breath
    g.precache_sound("player/h2odeath.wav"); // drowning death

    g.precache_sound("misc/talk.wav"); // talk
    g.precache_sound("player/teledth1.wav"); // telefrag
    g.precache_sound("misc/r_tele1.wav"); // teleport sounds
    g.precache_sound("misc/r_tele2.wav");
    g.precache_sound("misc/r_tele3.wav");
    g.precache_sound("misc/r_tele4.wav");
    g.precache_sound("misc/r_tele5.wav");
    g.precache_sound("weapons/lock4.wav"); // ammo pick up
    g.precache_sound("weapons/pkup.wav"); // weapon up
    g.precache_sound("items/armor1.wav"); // armor up
    g.precache_sound("weapons/lhit.wav"); // lightning
    g.precache_sound("weapons/lstart.wav"); // lightning start
    g.precache_sound("items/damage3.wav");

    g.precache_sound("misc/power.wav"); // lightning for boss

    // player gib sounds
    g.precache_sound("player/gib.wav"); // player gib sound
    g.precache_sound("player/udeath.wav"); // player gib sound
    g.precache_sound("player/tornoff2.wav"); // gib sound

    // player pain sounds
    g.precache_sound("player/pain1.wav");
    g.precache_sound("player/pain2.wav");
    g.precache_sound("player/pain3.wav");
    g.precache_sound("player/pain4.wav");
    g.precache_sound("player/pain5.wav");
    g.precache_sound("player/pain6.wav");

    // player death sounds
    g.precache_sound("player/death1.wav");
    g.precache_sound("player/death2.wav");
    g.precache_sound("player/death3.wav");
    g.precache_sound("player/death4.wav");
    g.precache_sound("player/death5.wav");

    // axe sounds
    g.precache_sound("weapons/ax1.wav"); // axe swoosh
    g.precache_sound("player/axhit1.wav"); // axe hit meat
    g.precache_sound("player/axhit2.wav"); // axe hit world

    g.precache_sound("player/h2ojump.wav"); // player jumping into water
    g.precache_sound("player/slimbrn2.wav"); // player enter slime
    g.precache_sound("player/inh2o.wav"); // player enter water
    g.precache_sound("player/inlava.wav"); // player enter lava
    g.precache_sound("misc/outwater.wav"); // leaving water sound

    g.precache_sound("player/lburn1.wav"); // lava burn
    g.precache_sound("player/lburn2.wav"); // lava burn

    g.precache_sound("misc/water1.wav"); // swimming
    g.precache_sound("misc/water2.wav"); // swimming

    g.precache_model("progs/player.mdl");
    g.precache_model("progs/eyes.mdl");
    g.precache_model("progs/h_player.mdl");
    g.precache_model("progs/gib1.mdl");
    g.precache_model("progs/gib2.mdl");
    g.precache_model("progs/gib3.mdl");

    g.precache_model("progs/s_bubble.spr"); // drowning bubbles
    g.precache_model("progs/s_explod.spr"); // sprite explosion

    g.precache_model("progs/v_axe.mdl");
    g.precache_model("progs/v_shot.mdl");
    g.precache_model("progs/v_nail.mdl");
    g.precache_model("progs/v_rock.mdl");
    g.precache_model("progs/v_shot2.mdl");
    g.precache_model("progs/v_nail2.mdl");
    g.precache_model("progs/v_rock2.mdl");

    g.precache_model("progs/bolt.mdl"); // for lightning gun
    g.precache_model("progs/bolt2.mdl"); // for lightning gun
    g.precache_model("progs/bolt3.mdl"); // for boss shock
    g.precache_model("progs/lavaball.mdl"); // for testing

    g.precache_model("progs/missile.mdl");
    g.precache_model("progs/grenade.mdl");
    g.precache_model("progs/spike.mdl");
    g.precache_model("progs/s_spike.mdl");

    g.precache_model("progs/backpack.mdl");

    g.precache_model("progs/zom_gib.mdl");

    g.precache_model("progs/v_light.mdl");

    // Setup light animation tables. 'a' is total darkness, 'z' is maxbright.

    // 0 normal
    g.lightstyle(0.0, "m");

    // 1 FLICKER (first variety)
    g.lightstyle(1.0, "mmnmmommommnonmmonqnmmo");

    // 2 SLOW STRONG PULSE
    g.lightstyle(2.0, "abcdefghijklmnopqrstuvwxyzyxwvutsrqponmlkjihgfedcba");

    // 3 CANDLE (first variety)
    g.lightstyle(3.0, "mmmmmaaaaammmmmaaaaaabcdefgabcdefg");

    // 4 FAST STROBE
    g.lightstyle(4.0, "mamamamamama");

    // 5 GENTLE PULSE 1
    g.lightstyle(5.0, "jklmnopqrstuvwxyzyxwvutsrqponmlkj");

    // 6 FLICKER (second variety)
    g.lightstyle(6.0, "nmonqnmomnmomomno");

    // 7 CANDLE (second variety)
    g.lightstyle(7.0, "mmmaaaabcdefgmmmmaaaammmaamm");

    // 8 CANDLE (third variety)
    g.lightstyle(8.0, "mmmaaammmaaammmabcdefaaaammmmabcdefmmmaaaa");

    // 9 SLOW STROBE (fourth variety)
    g.lightstyle(9.0, "aaaaaaaazzzzzzzz");

    // 10 FLUORESCENT FLICKER
    g.lightstyle(10.0, "mmamammmmammamamaaamammma");

    // 11 SLOW PULSE NOT FADE TO BLACK
    g.lightstyle(11.0, "abcdefghijklmnopqrrqponmlkjihgfedcba");

    // styles 32-62 are assigned by the light program for switchable lights

    // 63 testing
    g.lightstyle(63.0, "a");
}

pub fn StartFrame(g: &mut Game) {
    g.teamplay = g.cvar("teamplay");
    g.skill = g.cvar("skill");
    g.framecount = g.framecount + 1.0;
}

// ---- the body queue ---------------------------------------------------------------------------

pub fn bodyqueue(g: &mut Game) {
    // just here so spawn functions don't complain after the world creates bodyqueues
}

pub fn InitBodyQueue(g: &mut Game) {
    let e = g.spawn();
    g.bodyqueue_head = e;
    g.v(e).classname = "bodyqueue";
    let e1 = g.spawn();
    g.v(g.bodyqueue_head).owner = e1;
    g.v(e1).classname = "bodyqueue";
    let e2 = g.spawn();
    g.v(e1).owner = e2;
    g.v(e2).classname = "bodyqueue";
    let e3 = g.spawn();
    g.v(e2).owner = e3;
    g.v(e3).classname = "bodyqueue";
    g.v(e3).owner = g.bodyqueue_head;
}

/// Makes a body queue entry for the given ent, so the ent can be respawned elsewhere.
pub fn CopyToBodyQueue(g: &mut Game, ent: Ent) {
    let h = g.bodyqueue_head;
    g.v(h).angles = g.r(ent).angles;
    g.v(h).model = g.r(ent).model;
    g.v(h).modelindex = g.r(ent).modelindex;
    g.v(h).frame = g.r(ent).frame;
    g.v(h).colormap = g.r(ent).colormap;
    g.v(h).movetype = g.r(ent).movetype;
    g.v(h).velocity = g.r(ent).velocity;
    g.v(h).flags = 0;
    g.setorigin(h, g.r(ent).origin);
    g.setsize(h, g.r(ent).mins, g.r(ent).maxs);
    g.bodyqueue_head = g.r(h).owner;
}
