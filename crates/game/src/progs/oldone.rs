//! `oldone.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;


// The model's frames, numbered as `$frame` numbers them: from nought, in order, in this file.
const F_old1: f32 = 0.0;
const F_old2: f32 = 1.0;
const F_old3: f32 = 2.0;
const F_old4: f32 = 3.0;
const F_old5: f32 = 4.0;
const F_old6: f32 = 5.0;
const F_old7: f32 = 6.0;
const F_old8: f32 = 7.0;
const F_old9: f32 = 8.0;
const F_old10: f32 = 9.0;
const F_old11: f32 = 10.0;
const F_old12: f32 = 11.0;
const F_old13: f32 = 12.0;
const F_old14: f32 = 13.0;
const F_old15: f32 = 14.0;
const F_old16: f32 = 15.0;
const F_old17: f32 = 16.0;
const F_old18: f32 = 17.0;
const F_old19: f32 = 18.0;
const F_old20: f32 = 19.0;
const F_old21: f32 = 20.0;
const F_old22: f32 = 21.0;
const F_old23: f32 = 22.0;
const F_old24: f32 = 23.0;
const F_old25: f32 = 24.0;
const F_old26: f32 = 25.0;
const F_old27: f32 = 26.0;
const F_old28: f32 = 27.0;
const F_old29: f32 = 28.0;
const F_old30: f32 = 29.0;
const F_old31: f32 = 30.0;
const F_old32: f32 = 31.0;
const F_old33: f32 = 32.0;
const F_old34: f32 = 33.0;
const F_old35: f32 = 34.0;
const F_old36: f32 = 35.0;
const F_old37: f32 = 36.0;
const F_old38: f32 = 37.0;
const F_old39: f32 = 38.0;
const F_old40: f32 = 39.0;
const F_old41: f32 = 40.0;
const F_old42: f32 = 41.0;
const F_old43: f32 = 42.0;
const F_old44: f32 = 43.0;
const F_old45: f32 = 44.0;
const F_old46: f32 = 45.0;
const F_shake1: f32 = 46.0;
const F_shake2: f32 = 47.0;
const F_shake3: f32 = 48.0;
const F_shake4: f32 = 49.0;
const F_shake5: f32 = 50.0;
const F_shake6: f32 = 51.0;
const F_shake7: f32 = 52.0;
const F_shake8: f32 = 53.0;
const F_shake9: f32 = 54.0;
const F_shake10: f32 = 55.0;
const F_shake11: f32 = 56.0;
const F_shake12: f32 = 57.0;
const F_shake13: f32 = 59.0;
const F_shake14: f32 = 60.0;
const F_shake15: f32 = 61.0;
const F_shake16: f32 = 62.0;
const F_shake17: f32 = 63.0;
const F_shake18: f32 = 64.0;
const F_shake19: f32 = 65.0;
const F_shake20: f32 = 66.0;


pub fn old_idle1(g: &mut Game) {
    g.frame_state(F_old1, old_idle2);
}

pub fn old_idle2(g: &mut Game) {
    g.frame_state(F_old2, old_idle3);
}

pub fn old_idle3(g: &mut Game) {
    g.frame_state(F_old3, old_idle4);
}

pub fn old_idle4(g: &mut Game) {
    g.frame_state(F_old4, old_idle5);
}

pub fn old_idle5(g: &mut Game) {
    g.frame_state(F_old5, old_idle6);
}

pub fn old_idle6(g: &mut Game) {
    g.frame_state(F_old6, old_idle7);
}

pub fn old_idle7(g: &mut Game) {
    g.frame_state(F_old7, old_idle8);
}

pub fn old_idle8(g: &mut Game) {
    g.frame_state(F_old8, old_idle9);
}

pub fn old_idle9(g: &mut Game) {
    g.frame_state(F_old9, old_idle10);
}

pub fn old_idle10(g: &mut Game) {
    g.frame_state(F_old10, old_idle11);
}

pub fn old_idle11(g: &mut Game) {
    g.frame_state(F_old11, old_idle12);
}

pub fn old_idle12(g: &mut Game) {
    g.frame_state(F_old12, old_idle13);
}

pub fn old_idle13(g: &mut Game) {
    g.frame_state(F_old13, old_idle14);
}

pub fn old_idle14(g: &mut Game) {
    g.frame_state(F_old14, old_idle15);
}

pub fn old_idle15(g: &mut Game) {
    g.frame_state(F_old15, old_idle16);
}

pub fn old_idle16(g: &mut Game) {
    g.frame_state(F_old16, old_idle17);
}

pub fn old_idle17(g: &mut Game) {
    g.frame_state(F_old17, old_idle18);
}

pub fn old_idle18(g: &mut Game) {
    g.frame_state(F_old18, old_idle19);
}

pub fn old_idle19(g: &mut Game) {
    g.frame_state(F_old19, old_idle20);
}

pub fn old_idle20(g: &mut Game) {
    g.frame_state(F_old20, old_idle21);
}

pub fn old_idle21(g: &mut Game) {
    g.frame_state(F_old21, old_idle22);
}

pub fn old_idle22(g: &mut Game) {
    g.frame_state(F_old22, old_idle23);
}

pub fn old_idle23(g: &mut Game) {
    g.frame_state(F_old23, old_idle24);
}

pub fn old_idle24(g: &mut Game) {
    g.frame_state(F_old24, old_idle25);
}

pub fn old_idle25(g: &mut Game) {
    g.frame_state(F_old25, old_idle26);
}

pub fn old_idle26(g: &mut Game) {
    g.frame_state(F_old26, old_idle27);
}

pub fn old_idle27(g: &mut Game) {
    g.frame_state(F_old27, old_idle28);
}

pub fn old_idle28(g: &mut Game) {
    g.frame_state(F_old28, old_idle29);
}

pub fn old_idle29(g: &mut Game) {
    g.frame_state(F_old29, old_idle30);
}

pub fn old_idle30(g: &mut Game) {
    g.frame_state(F_old30, old_idle31);
}

pub fn old_idle31(g: &mut Game) {
    g.frame_state(F_old31, old_idle32);
}

pub fn old_idle32(g: &mut Game) {
    g.frame_state(F_old32, old_idle33);
}

pub fn old_idle33(g: &mut Game) {
    g.frame_state(F_old33, old_idle34);
}

pub fn old_idle34(g: &mut Game) {
    g.frame_state(F_old34, old_idle35);
}

pub fn old_idle35(g: &mut Game) {
    g.frame_state(F_old35, old_idle36);
}

pub fn old_idle36(g: &mut Game) {
    g.frame_state(F_old36, old_idle37);
}

pub fn old_idle37(g: &mut Game) {
    g.frame_state(F_old37, old_idle38);
}

pub fn old_idle38(g: &mut Game) {
    g.frame_state(F_old38, old_idle39);
}

pub fn old_idle39(g: &mut Game) {
    g.frame_state(F_old39, old_idle40);
}

pub fn old_idle40(g: &mut Game) {
    g.frame_state(F_old40, old_idle41);
}

pub fn old_idle41(g: &mut Game) {
    g.frame_state(F_old41, old_idle42);
}

pub fn old_idle42(g: &mut Game) {
    g.frame_state(F_old42, old_idle43);
}

pub fn old_idle43(g: &mut Game) {
    g.frame_state(F_old43, old_idle44);
}

pub fn old_idle44(g: &mut Game) {
    g.frame_state(F_old44, old_idle45);
}

pub fn old_idle45(g: &mut Game) {
    g.frame_state(F_old45, old_idle46);
}

pub fn old_idle46(g: &mut Game) {
    g.frame_state(F_old46, old_idle1);
}

pub fn old_thrash1(g: &mut Game) {
    g.frame_state(F_shake1, old_thrash2);
    g.lightstyle(0.0, "m");
}

pub fn old_thrash2(g: &mut Game) {
    g.frame_state(F_shake2, old_thrash3);
    g.lightstyle(0.0, "k");
}

pub fn old_thrash3(g: &mut Game) {
    g.frame_state(F_shake3, old_thrash4);
    g.lightstyle(0.0, "k");
}

pub fn old_thrash4(g: &mut Game) {
    g.frame_state(F_shake4, old_thrash5);
    g.lightstyle(0.0, "i");
}

pub fn old_thrash5(g: &mut Game) {
    g.frame_state(F_shake5, old_thrash6);
    g.lightstyle(0.0, "g");
}

pub fn old_thrash6(g: &mut Game) {
    g.frame_state(F_shake6, old_thrash7);
    g.lightstyle(0.0, "e");
}

pub fn old_thrash7(g: &mut Game) {
    g.frame_state(F_shake7, old_thrash8);
    g.lightstyle(0.0, "c");
}

pub fn old_thrash8(g: &mut Game) {
    g.frame_state(F_shake8, old_thrash9);
    g.lightstyle(0.0, "a");
}

pub fn old_thrash9(g: &mut Game) {
    g.frame_state(F_shake9, old_thrash10);
    g.lightstyle(0.0, "c");
}

pub fn old_thrash10(g: &mut Game) {
    g.frame_state(F_shake10, old_thrash11);
    g.lightstyle(0.0, "e");
}

pub fn old_thrash11(g: &mut Game) {
    g.frame_state(F_shake11, old_thrash12);
    g.lightstyle(0.0, "g");
}

pub fn old_thrash12(g: &mut Game) {
    g.frame_state(F_shake12, old_thrash13);
    g.lightstyle(0.0, "i");
}

pub fn old_thrash13(g: &mut Game) {
    g.frame_state(F_shake13, old_thrash14);
    g.lightstyle(0.0, "k");
}

pub fn old_thrash14(g: &mut Game) {
    g.frame_state(F_shake14, old_thrash15);
    g.lightstyle(0.0, "m");
}

pub fn old_thrash15(g: &mut Game) {
    g.frame_state(F_shake15, old_thrash16);
    g.lightstyle(0.0, "m");
    let s = g.self_;
    g.v(s).cnt = g.r(s).cnt + 1.0;
    if g.r(s).cnt != 3.0 {
        g.v(s).think = Some(old_thrash1);
    }
}

pub fn old_thrash16(g: &mut Game) {
    g.frame_state(F_shake16, old_thrash17);
    g.lightstyle(0.0, "g");
}

pub fn old_thrash17(g: &mut Game) {
    g.frame_state(F_shake17, old_thrash18);
    g.lightstyle(0.0, "c");
}

pub fn old_thrash18(g: &mut Game) {
    g.frame_state(F_shake18, old_thrash19);
    g.lightstyle(0.0, "b");
}

pub fn old_thrash19(g: &mut Game) {
    g.frame_state(F_shake19, old_thrash20);
    g.lightstyle(0.0, "a");
}

pub fn old_thrash20(g: &mut Game) {
    g.frame_state(F_shake20, old_thrash20);
    finale_4(g);
}

pub fn finale_1(g: &mut Game) {
    g.intermission_exittime = g.time + 10000000.0; // never allow exit
    g.intermission_running = 1.0;

    // find the intermission spot
    let pos = g.find(world, |v: &EntVars| v.classname, "info_intermission");
    if pos == world {
        g.error("no info_intermission");
    }

    let mut pl = g.find(world, |v: &EntVars| v.classname, "misc_teleporttrain");
    if pl == world {
        g.error("no teleporttrain");
    }

    g.remove(pl);

    g.msg_finale("");

    pl = g.find(world, |v: &EntVars| v.classname, "player");
    while pl != world {
        g.v(pl).view_ofs = Vec3::ZERO;
        // `pl.angles = other.v_angle = pos.mangle;`: id's code sets `other`'s v_angle, not pl's.
        let mangle = g.r(pos).mangle;
        let other = g.other;
        g.v(other).v_angle = mangle;
        g.v(pl).angles = mangle;
        g.v(pl).fixangle = 1; // turn this way immediately
        g.v(pl).map = g.r(g.self_).map;
        g.v(pl).nextthink = g.time + 0.5;
        g.v(pl).takedamage = DAMAGE_NO;
        g.v(pl).solid = SOLID_NOT;
        g.v(pl).movetype = MOVETYPE_NONE;
        g.v(pl).modelindex = 0.0;
        g.setorigin(pl, g.r(pos).origin);
        pl = g.find(pl, |v: &EntVars| v.classname, "player");
    }

    // make fake versions of all players as standins, and move the real
    // players to the intermission spot

    // wait for 1 second
    let timer = g.spawn();
    g.v(timer).nextthink = g.time + 1.0;
    g.v(timer).think = Some(finale_2);
}

pub fn finale_2(g: &mut Game) {
    // start a teleport splash inside shub
    let o = g.r(g.shub).origin - Vec3::new(0.0, 100.0, 0.0);
    g.temp_entity(TempEntity::Teleport(o));

    g.sound(g.shub, CHAN_VOICE, "misc/r_tele1.wav", 1.0, ATTN_NORM);

    let s = g.self_;
    g.v(s).nextthink = g.time + 2.0;
    g.v(s).think = Some(finale_3);
}

pub fn finale_3(g: &mut Game) {
    // start shub thrashing wildly
    g.v(g.shub).think = Some(old_thrash1);
    g.sound(g.shub, CHAN_VOICE, "boss2/death.wav", 1.0, ATTN_NORM);
    g.lightstyle(0.0, "abcdefghijklmlkjihgfedcb");
}

pub fn finale_4(g: &mut Game) {
    // throw tons of meat chunks
    let s = g.self_;
    g.sound(s, CHAN_VOICE, "boss2/pop2.wav", 1.0, ATTN_NORM);

    let mut oldo = g.r(s).origin;

    let mut z = 16.0;
    while z <= 144.0 {
        let mut x = -64.0;
        while x <= 64.0 {
            let mut y = -64.0;
            while y <= 64.0 {
                let s = g.self_;
                g.v(s).origin.x = oldo.x + x;
                g.v(s).origin.y = oldo.y + y;
                g.v(s).origin.z = oldo.z + z;

                let r = g.random();
                if r < 0.3 {
                    ThrowGib(g, "progs/gib1.mdl", -999.0);
                } else if r < 0.6 {
                    ThrowGib(g, "progs/gib2.mdl", -999.0);
                } else {
                    ThrowGib(g, "progs/gib3.mdl", -999.0);
                }

                y = y + 32.0;
            }
            x = x + 32.0;
        }
        z = z + 96.0;
    }

    // start the end text
    g.msg_finale("Congratulations and well done! You have\nbeaten the hideous Shub-Niggurath, and\nher hundreds of ugly changelings and\nmonsters. You have proven that your\nskill and your cunning are greater than\nall the powers of Quake. You are the\nmaster now. Id Software salutes you.");

    // put a player model down
    let n = g.spawn();
    g.setmodel(n, "progs/player.mdl");
    oldo = oldo - Vec3::new(32.0, 264.0, 0.0);
    g.setorigin(n, oldo);
    g.v(n).angles = Vec3::new(0.0, 290.0, 0.0);
    g.v(n).frame = 1.0;

    g.remove(g.self_);

    // switch cd track
    g.msg_cdtrack(3.0, 3.0);
    g.lightstyle(0.0, "m");
}

pub fn nopain(g: &mut Game, attacker: Ent, damage: f32) {
    g.v(g.self_).health = 40000.0;
}

pub fn monster_oldone(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model2("progs/oldone.mdl");

    g.precache_sound2("boss2/death.wav");
    g.precache_sound2("boss2/idle.wav");
    g.precache_sound2("boss2/sight.wav");
    g.precache_sound2("boss2/pop2.wav");

    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;

    g.setmodel(s, "progs/oldone.mdl");
    g.setsize(s, Vec3::new(-160.0, -128.0, -24.0), Vec3::new(160.0, 128.0, 256.0));

    g.v(s).health = 40000.0; // kill by telefrag
    g.v(s).think = Some(old_idle1);
    g.v(s).nextthink = g.time + 0.1;
    g.v(s).takedamage = DAMAGE_YES;
    g.v(s).th_pain = Some(nopain);
    g.v(s).th_die = Some(finale_1);
    g.shub = s;

    g.total_monsters = g.total_monsters + 1.0;
}
