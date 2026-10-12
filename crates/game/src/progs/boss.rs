//! `boss.qc`, id's QuakeC 1.06, in Rust.

#![allow(unused_variables, unused_mut, unreachable_code, clippy::all)]

use super::*;


// The model's frames, numbered as `$frame` numbers them: from nought, in order, in this file.
const F_rise1: f32 = 0.0;
const F_rise2: f32 = 1.0;
const F_rise3: f32 = 2.0;
const F_rise4: f32 = 3.0;
const F_rise5: f32 = 4.0;
const F_rise6: f32 = 5.0;
const F_rise7: f32 = 6.0;
const F_rise8: f32 = 7.0;
const F_rise9: f32 = 8.0;
const F_rise10: f32 = 9.0;
const F_rise11: f32 = 10.0;
const F_rise12: f32 = 11.0;
const F_rise13: f32 = 12.0;
const F_rise14: f32 = 13.0;
const F_rise15: f32 = 14.0;
const F_rise16: f32 = 15.0;
const F_rise17: f32 = 16.0;
const F_walk1: f32 = 17.0;
const F_walk2: f32 = 18.0;
const F_walk3: f32 = 19.0;
const F_walk4: f32 = 20.0;
const F_walk5: f32 = 21.0;
const F_walk6: f32 = 22.0;
const F_walk7: f32 = 23.0;
const F_walk8: f32 = 24.0;
const F_walk9: f32 = 25.0;
const F_walk10: f32 = 26.0;
const F_walk11: f32 = 27.0;
const F_walk12: f32 = 28.0;
const F_walk13: f32 = 29.0;
const F_walk14: f32 = 30.0;
const F_walk15: f32 = 31.0;
const F_walk16: f32 = 32.0;
const F_walk17: f32 = 33.0;
const F_walk18: f32 = 34.0;
const F_walk19: f32 = 35.0;
const F_walk20: f32 = 36.0;
const F_walk21: f32 = 37.0;
const F_walk22: f32 = 38.0;
const F_walk23: f32 = 39.0;
const F_walk24: f32 = 40.0;
const F_walk25: f32 = 41.0;
const F_walk26: f32 = 42.0;
const F_walk27: f32 = 43.0;
const F_walk28: f32 = 44.0;
const F_walk29: f32 = 45.0;
const F_walk30: f32 = 46.0;
const F_walk31: f32 = 47.0;
const F_death1: f32 = 48.0;
const F_death2: f32 = 49.0;
const F_death3: f32 = 50.0;
const F_death4: f32 = 51.0;
const F_death5: f32 = 52.0;
const F_death6: f32 = 53.0;
const F_death7: f32 = 54.0;
const F_death8: f32 = 55.0;
const F_death9: f32 = 56.0;
const F_attack1: f32 = 57.0;
const F_attack2: f32 = 58.0;
const F_attack3: f32 = 59.0;
const F_attack4: f32 = 60.0;
const F_attack5: f32 = 61.0;
const F_attack6: f32 = 62.0;
const F_attack7: f32 = 63.0;
const F_attack8: f32 = 64.0;
const F_attack9: f32 = 65.0;
const F_attack10: f32 = 66.0;
const F_attack11: f32 = 67.0;
const F_attack12: f32 = 68.0;
const F_attack13: f32 = 69.0;
const F_attack14: f32 = 70.0;
const F_attack15: f32 = 71.0;
const F_attack16: f32 = 72.0;
const F_attack17: f32 = 73.0;
const F_attack18: f32 = 74.0;
const F_attack19: f32 = 75.0;
const F_attack20: f32 = 76.0;
const F_attack21: f32 = 77.0;
const F_attack22: f32 = 78.0;
const F_attack23: f32 = 79.0;
const F_shocka1: f32 = 80.0;
const F_shocka2: f32 = 81.0;
const F_shocka3: f32 = 82.0;
const F_shocka4: f32 = 83.0;
const F_shocka5: f32 = 84.0;
const F_shocka6: f32 = 85.0;
const F_shocka7: f32 = 86.0;
const F_shocka8: f32 = 87.0;
const F_shocka9: f32 = 88.0;
const F_shocka10: f32 = 89.0;
const F_shockb1: f32 = 90.0;
const F_shockb2: f32 = 91.0;
const F_shockb3: f32 = 92.0;
const F_shockb4: f32 = 93.0;
const F_shockb5: f32 = 94.0;
const F_shockb6: f32 = 95.0;
const F_shockc1: f32 = 96.0;
const F_shockc2: f32 = 97.0;
const F_shockc3: f32 = 98.0;
const F_shockc4: f32 = 99.0;
const F_shockc5: f32 = 100.0;
const F_shockc6: f32 = 101.0;
const F_shockc7: f32 = 102.0;
const F_shockc8: f32 = 103.0;
const F_shockc9: f32 = 104.0;
const F_shockc10: f32 = 105.0;


pub fn boss_face(g: &mut Game) {
    let s = g.self_;
    // go for another player if multi player
    // Both sides evaluated, as QuakeC does: random() is called every time.
    let dead = g.r(g.r(s).enemy).health <= 0.0;
    let roll = g.random() < 0.02;
    if dead | roll {
        g.v(s).enemy = g.find(g.r(s).enemy, |v: &EntVars| v.classname, "player");
        if g.r(s).enemy == world {
            g.v(s).enemy = g.find(g.r(s).enemy, |v: &EntVars| v.classname, "player");
        }
    }
    ai_face(g);
}

pub fn boss_rise1(g: &mut Game) {
    g.frame_state(F_rise1, boss_rise2);
    g.sound(g.self_, CHAN_WEAPON, "boss1/out1.wav", 1.0, ATTN_NORM);
}

pub fn boss_rise2(g: &mut Game) {
    g.frame_state(F_rise2, boss_rise3);
    g.sound(g.self_, CHAN_VOICE, "boss1/sight1.wav", 1.0, ATTN_NORM);
}

pub fn boss_rise3(g: &mut Game) {
    g.frame_state(F_rise3, boss_rise4);
}

pub fn boss_rise4(g: &mut Game) {
    g.frame_state(F_rise4, boss_rise5);
}

pub fn boss_rise5(g: &mut Game) {
    g.frame_state(F_rise5, boss_rise6);
}

pub fn boss_rise6(g: &mut Game) {
    g.frame_state(F_rise6, boss_rise7);
}

pub fn boss_rise7(g: &mut Game) {
    g.frame_state(F_rise7, boss_rise8);
}

pub fn boss_rise8(g: &mut Game) {
    g.frame_state(F_rise8, boss_rise9);
}

pub fn boss_rise9(g: &mut Game) {
    g.frame_state(F_rise9, boss_rise10);
}

pub fn boss_rise10(g: &mut Game) {
    g.frame_state(F_rise10, boss_rise11);
}

pub fn boss_rise11(g: &mut Game) {
    g.frame_state(F_rise11, boss_rise12);
}

pub fn boss_rise12(g: &mut Game) {
    g.frame_state(F_rise12, boss_rise13);
}

pub fn boss_rise13(g: &mut Game) {
    g.frame_state(F_rise13, boss_rise14);
}

pub fn boss_rise14(g: &mut Game) {
    g.frame_state(F_rise14, boss_rise15);
}

pub fn boss_rise15(g: &mut Game) {
    g.frame_state(F_rise15, boss_rise16);
}

pub fn boss_rise16(g: &mut Game) {
    g.frame_state(F_rise16, boss_rise17);
}

pub fn boss_rise17(g: &mut Game) {
    g.frame_state(F_rise17, boss_missile1);
}

pub fn boss_idle1(g: &mut Game) {
    g.frame_state(F_walk1, boss_idle2);
    // look for other players
}

pub fn boss_idle2(g: &mut Game) {
    g.frame_state(F_walk2, boss_idle3);
    boss_face(g);
}

pub fn boss_idle3(g: &mut Game) {
    g.frame_state(F_walk3, boss_idle4);
    boss_face(g);
}

pub fn boss_idle4(g: &mut Game) {
    g.frame_state(F_walk4, boss_idle5);
    boss_face(g);
}

pub fn boss_idle5(g: &mut Game) {
    g.frame_state(F_walk5, boss_idle6);
    boss_face(g);
}

pub fn boss_idle6(g: &mut Game) {
    g.frame_state(F_walk6, boss_idle7);
    boss_face(g);
}

pub fn boss_idle7(g: &mut Game) {
    g.frame_state(F_walk7, boss_idle8);
    boss_face(g);
}

pub fn boss_idle8(g: &mut Game) {
    g.frame_state(F_walk8, boss_idle9);
    boss_face(g);
}

pub fn boss_idle9(g: &mut Game) {
    g.frame_state(F_walk9, boss_idle10);
    boss_face(g);
}

pub fn boss_idle10(g: &mut Game) {
    g.frame_state(F_walk10, boss_idle11);
    boss_face(g);
}

pub fn boss_idle11(g: &mut Game) {
    g.frame_state(F_walk11, boss_idle12);
    boss_face(g);
}

pub fn boss_idle12(g: &mut Game) {
    g.frame_state(F_walk12, boss_idle13);
    boss_face(g);
}

pub fn boss_idle13(g: &mut Game) {
    g.frame_state(F_walk13, boss_idle14);
    boss_face(g);
}

pub fn boss_idle14(g: &mut Game) {
    g.frame_state(F_walk14, boss_idle15);
    boss_face(g);
}

pub fn boss_idle15(g: &mut Game) {
    g.frame_state(F_walk15, boss_idle16);
    boss_face(g);
}

pub fn boss_idle16(g: &mut Game) {
    g.frame_state(F_walk16, boss_idle17);
    boss_face(g);
}

pub fn boss_idle17(g: &mut Game) {
    g.frame_state(F_walk17, boss_idle18);
    boss_face(g);
}

pub fn boss_idle18(g: &mut Game) {
    g.frame_state(F_walk18, boss_idle19);
    boss_face(g);
}

pub fn boss_idle19(g: &mut Game) {
    g.frame_state(F_walk19, boss_idle20);
    boss_face(g);
}

pub fn boss_idle20(g: &mut Game) {
    g.frame_state(F_walk20, boss_idle21);
    boss_face(g);
}

pub fn boss_idle21(g: &mut Game) {
    g.frame_state(F_walk21, boss_idle22);
    boss_face(g);
}

pub fn boss_idle22(g: &mut Game) {
    g.frame_state(F_walk22, boss_idle23);
    boss_face(g);
}

pub fn boss_idle23(g: &mut Game) {
    g.frame_state(F_walk23, boss_idle24);
    boss_face(g);
}

pub fn boss_idle24(g: &mut Game) {
    g.frame_state(F_walk24, boss_idle25);
    boss_face(g);
}

pub fn boss_idle25(g: &mut Game) {
    g.frame_state(F_walk25, boss_idle26);
    boss_face(g);
}

pub fn boss_idle26(g: &mut Game) {
    g.frame_state(F_walk26, boss_idle27);
    boss_face(g);
}

pub fn boss_idle27(g: &mut Game) {
    g.frame_state(F_walk27, boss_idle28);
    boss_face(g);
}

pub fn boss_idle28(g: &mut Game) {
    g.frame_state(F_walk28, boss_idle29);
    boss_face(g);
}

pub fn boss_idle29(g: &mut Game) {
    g.frame_state(F_walk29, boss_idle30);
    boss_face(g);
}

pub fn boss_idle30(g: &mut Game) {
    g.frame_state(F_walk30, boss_idle31);
    boss_face(g);
}

pub fn boss_idle31(g: &mut Game) {
    g.frame_state(F_walk31, boss_idle1);
    boss_face(g);
}

pub fn boss_missile1(g: &mut Game) {
    g.frame_state(F_attack1, boss_missile2);
    boss_face(g);
}

pub fn boss_missile2(g: &mut Game) {
    g.frame_state(F_attack2, boss_missile3);
    boss_face(g);
}

pub fn boss_missile3(g: &mut Game) {
    g.frame_state(F_attack3, boss_missile4);
    boss_face(g);
}

pub fn boss_missile4(g: &mut Game) {
    g.frame_state(F_attack4, boss_missile5);
    boss_face(g);
}

pub fn boss_missile5(g: &mut Game) {
    g.frame_state(F_attack5, boss_missile6);
    boss_face(g);
}

pub fn boss_missile6(g: &mut Game) {
    g.frame_state(F_attack6, boss_missile7);
    boss_face(g);
}

pub fn boss_missile7(g: &mut Game) {
    g.frame_state(F_attack7, boss_missile8);
    boss_face(g);
}

pub fn boss_missile8(g: &mut Game) {
    g.frame_state(F_attack8, boss_missile9);
    boss_face(g);
}

pub fn boss_missile9(g: &mut Game) {
    g.frame_state(F_attack9, boss_missile10);
    boss_missile(g, Vec3::new(100.0, 100.0, 200.0));
}

pub fn boss_missile10(g: &mut Game) {
    g.frame_state(F_attack10, boss_missile11);
    boss_face(g);
}

pub fn boss_missile11(g: &mut Game) {
    g.frame_state(F_attack11, boss_missile12);
    boss_face(g);
}

pub fn boss_missile12(g: &mut Game) {
    g.frame_state(F_attack12, boss_missile13);
    boss_face(g);
}

pub fn boss_missile13(g: &mut Game) {
    g.frame_state(F_attack13, boss_missile14);
    boss_face(g);
}

pub fn boss_missile14(g: &mut Game) {
    g.frame_state(F_attack14, boss_missile15);
    boss_face(g);
}

pub fn boss_missile15(g: &mut Game) {
    g.frame_state(F_attack15, boss_missile16);
    boss_face(g);
}

pub fn boss_missile16(g: &mut Game) {
    g.frame_state(F_attack16, boss_missile17);
    boss_face(g);
}

pub fn boss_missile17(g: &mut Game) {
    g.frame_state(F_attack17, boss_missile18);
    boss_face(g);
}

pub fn boss_missile18(g: &mut Game) {
    g.frame_state(F_attack18, boss_missile19);
    boss_face(g);
}

pub fn boss_missile19(g: &mut Game) {
    g.frame_state(F_attack19, boss_missile20);
    boss_face(g);
}

pub fn boss_missile20(g: &mut Game) {
    g.frame_state(F_attack20, boss_missile21);
    boss_missile(g, Vec3::new(100.0, -100.0, 200.0));
}

pub fn boss_missile21(g: &mut Game) {
    g.frame_state(F_attack21, boss_missile22);
    boss_face(g);
}

pub fn boss_missile22(g: &mut Game) {
    g.frame_state(F_attack22, boss_missile23);
    boss_face(g);
}

pub fn boss_missile23(g: &mut Game) {
    g.frame_state(F_attack23, boss_missile1);
    boss_face(g);
}

pub fn boss_shocka1(g: &mut Game) {
    g.frame_state(F_shocka1, boss_shocka2);
}

pub fn boss_shocka2(g: &mut Game) {
    g.frame_state(F_shocka2, boss_shocka3);
}

pub fn boss_shocka3(g: &mut Game) {
    g.frame_state(F_shocka3, boss_shocka4);
}

pub fn boss_shocka4(g: &mut Game) {
    g.frame_state(F_shocka4, boss_shocka5);
}

pub fn boss_shocka5(g: &mut Game) {
    g.frame_state(F_shocka5, boss_shocka6);
}

pub fn boss_shocka6(g: &mut Game) {
    g.frame_state(F_shocka6, boss_shocka7);
}

pub fn boss_shocka7(g: &mut Game) {
    g.frame_state(F_shocka7, boss_shocka8);
}

pub fn boss_shocka8(g: &mut Game) {
    g.frame_state(F_shocka8, boss_shocka9);
}

pub fn boss_shocka9(g: &mut Game) {
    g.frame_state(F_shocka9, boss_shocka10);
}

pub fn boss_shocka10(g: &mut Game) {
    g.frame_state(F_shocka10, boss_missile1);
}

pub fn boss_shockb1(g: &mut Game) {
    g.frame_state(F_shockb1, boss_shockb2);
}

pub fn boss_shockb2(g: &mut Game) {
    g.frame_state(F_shockb2, boss_shockb3);
}

pub fn boss_shockb3(g: &mut Game) {
    g.frame_state(F_shockb3, boss_shockb4);
}

pub fn boss_shockb4(g: &mut Game) {
    g.frame_state(F_shockb4, boss_shockb5);
}

pub fn boss_shockb5(g: &mut Game) {
    g.frame_state(F_shockb5, boss_shockb6);
}

pub fn boss_shockb6(g: &mut Game) {
    g.frame_state(F_shockb6, boss_shockb7);
}

pub fn boss_shockb7(g: &mut Game) {
    g.frame_state(F_shockb1, boss_shockb8);
}

pub fn boss_shockb8(g: &mut Game) {
    g.frame_state(F_shockb2, boss_shockb9);
}

pub fn boss_shockb9(g: &mut Game) {
    g.frame_state(F_shockb3, boss_shockb10);
}

pub fn boss_shockb10(g: &mut Game) {
    g.frame_state(F_shockb4, boss_missile1);
}

pub fn boss_shockc1(g: &mut Game) {
    g.frame_state(F_shockc1, boss_shockc2);
}

pub fn boss_shockc2(g: &mut Game) {
    g.frame_state(F_shockc2, boss_shockc3);
}

pub fn boss_shockc3(g: &mut Game) {
    g.frame_state(F_shockc3, boss_shockc4);
}

pub fn boss_shockc4(g: &mut Game) {
    g.frame_state(F_shockc4, boss_shockc5);
}

pub fn boss_shockc5(g: &mut Game) {
    g.frame_state(F_shockc5, boss_shockc6);
}

pub fn boss_shockc6(g: &mut Game) {
    g.frame_state(F_shockc6, boss_shockc7);
}

pub fn boss_shockc7(g: &mut Game) {
    g.frame_state(F_shockc7, boss_shockc8);
}

pub fn boss_shockc8(g: &mut Game) {
    g.frame_state(F_shockc8, boss_shockc9);
}

pub fn boss_shockc9(g: &mut Game) {
    g.frame_state(F_shockc9, boss_shockc10);
}

pub fn boss_shockc10(g: &mut Game) {
    g.frame_state(F_shockc10, boss_death1);
}

pub fn boss_death1(g: &mut Game) {
    g.frame_state(F_death1, boss_death2);
    g.sound(g.self_, CHAN_VOICE, "boss1/death.wav", 1.0, ATTN_NORM);
}

pub fn boss_death2(g: &mut Game) {
    g.frame_state(F_death2, boss_death3);
}

pub fn boss_death3(g: &mut Game) {
    g.frame_state(F_death3, boss_death4);
}

pub fn boss_death4(g: &mut Game) {
    g.frame_state(F_death4, boss_death5);
}

pub fn boss_death5(g: &mut Game) {
    g.frame_state(F_death5, boss_death6);
}

pub fn boss_death6(g: &mut Game) {
    g.frame_state(F_death6, boss_death7);
}

pub fn boss_death7(g: &mut Game) {
    g.frame_state(F_death7, boss_death8);
}

pub fn boss_death8(g: &mut Game) {
    g.frame_state(F_death8, boss_death9);
}

pub fn boss_death9(g: &mut Game) {
    g.frame_state(F_death9, boss_death10);
    let s = g.self_;
    g.sound(s, CHAN_BODY, "boss1/out1.wav", 1.0, ATTN_NORM);
    g.temp_entity(TempEntity::LavaSplash(g.r(s).origin));
}

pub fn boss_death10(g: &mut Game) {
    g.frame_state(F_death9, boss_death10);
    g.killed_monsters = g.killed_monsters + 1.0;
    g.msg_killed_monster(); // FIXME: reliable broadcast
    SUB_UseTargets(g);
    g.remove(g.self_);
}

pub fn boss_missile(g: &mut Game, p: Vec3) {
    let s = g.self_;
    let enemy = g.r(s).enemy;
    let offang = vectoangles(g.r(enemy).origin - g.r(s).origin);
    g.makevectors(offang);

    let org = g.r(s).origin + p.x * g.v_forward + p.y * g.v_right + p.z * Vec3::new(0.0, 0.0, 1.0);

    // lead the player on hard mode
    let d;
    if g.skill > 1.0 {
        let t = vlen(g.r(enemy).origin - org) / 300.0;
        let mut vec = g.r(enemy).velocity;
        vec.z = 0.0;
        d = g.r(enemy).origin + t * vec;
    } else {
        d = g.r(enemy).origin;
    }

    let vec = normalize(d - org);

    launch_spike(g, org, vec);
    let newmis = g.newmis;
    g.setmodel(newmis, "progs/lavaball.mdl");
    g.v(newmis).avelocity = Vec3::new(200.0, 100.0, 300.0);
    g.setsize(newmis, VEC_ORIGIN, VEC_ORIGIN);
    g.v(newmis).velocity = vec * 300.0;
    g.v(newmis).touch = Some(T_MissileTouch); // rocket explosion
    let s = g.self_;
    g.sound(s, CHAN_WEAPON, "boss1/throw.wav", 1.0, ATTN_NORM);

    // check for dead enemy
    if g.r(g.r(s).enemy).health <= 0.0 {
        boss_idle1(g);
    }
}

pub fn boss_awake(g: &mut Game) {
    let s = g.self_;
    g.v(s).solid = SOLID_SLIDEBOX;
    g.v(s).movetype = MOVETYPE_STEP;
    g.v(s).takedamage = DAMAGE_NO;

    g.setmodel(s, "progs/boss.mdl");
    g.setsize(s, Vec3::new(-128.0, -128.0, -24.0), Vec3::new(128.0, 128.0, 256.0));

    if g.skill == 0.0 {
        g.v(s).health = 1.0;
    } else {
        g.v(s).health = 3.0;
    }

    g.v(s).enemy = g.activator;

    g.temp_entity(TempEntity::LavaSplash(g.r(s).origin));

    g.v(s).yaw_speed = 20.0;
    boss_rise1(g);
}

pub fn monster_boss(g: &mut Game) {
    let s = g.self_;
    if g.deathmatch != 0.0 {
        g.remove(s);
        return;
    }

    g.precache_model("progs/boss.mdl");
    g.precache_model("progs/lavaball.mdl");

    g.precache_sound("weapons/rocket1i.wav");
    g.precache_sound("boss1/out1.wav");
    g.precache_sound("boss1/sight1.wav");
    g.precache_sound("misc/power.wav");
    g.precache_sound("boss1/throw.wav");
    g.precache_sound("boss1/pain.wav");
    g.precache_sound("boss1/death.wav");

    g.total_monsters = g.total_monsters + 1.0;

    g.v(s).use_ = Some(boss_awake);
}

pub fn lightning_fire(g: &mut Game) {
    if g.time >= g.lightning_end {
        // done here, put the terminals back up
        g.self_ = g.le1;
        door_go_down(g);
        g.self_ = g.le2;
        door_go_down(g);
        return;
    }

    let (le1, le2) = (g.le1, g.le2);
    let mut p1 = (g.r(le1).mins + g.r(le1).maxs) * 0.5;
    p1.z = g.r(le1).absmin.z - 16.0;

    let mut p2 = (g.r(le2).mins + g.r(le2).maxs) * 0.5;
    p2.z = g.r(le2).absmin.z - 16.0;

    // compensate for length of bolt
    p2 = p2 - normalize(p2 - p1) * 100.0;

    let s = g.self_;
    g.v(s).nextthink = g.time + 0.1;
    g.v(s).think = Some(lightning_fire);

    g.temp_entity(TempEntity::Lightning3 { ent: world, start: p1, end: p2 });
}

pub fn lightning_use(g: &mut Game) {
    if g.lightning_end >= (g.time + 1.0) {
        return;
    }

    g.le1 = g.find(world, |v: &EntVars| v.target, "lightning");
    g.le2 = g.find(g.le1, |v: &EntVars| v.target, "lightning");
    let (le1, le2) = (g.le1, g.le2);
    if le1 == world || le2 == world {
        g.dprint("missing lightning targets\n");
        return;
    }

    let (st1, st2) = (g.r(le1).state, g.r(le2).state);
    if (st1 != STATE_TOP && st1 != STATE_BOTTOM)
        || (st2 != STATE_TOP && st2 != STATE_BOTTOM)
        || (st1 != st2)
    {
        return;
    }

    // don't let the electrodes go back up until the bolt is done
    g.v(le1).nextthink = -1.0;
    g.v(le2).nextthink = -1.0;
    g.lightning_end = g.time + 1.0;

    g.sound(g.self_, CHAN_VOICE, "misc/power.wav", 1.0, ATTN_NORM);
    lightning_fire(g);

    // advance the boss pain if down
    g.self_ = g.find(world, |v: &EntVars| v.classname, "monster_boss");
    let s = g.self_;
    if s == world {
        return;
    }

    g.v(s).enemy = g.activator;

    if g.r(g.le1).state == STATE_TOP && g.r(s).health > 0.0 {
        g.sound(s, CHAN_VOICE, "boss1/pain.wav", 1.0, ATTN_NORM);
        g.v(s).health = g.r(s).health - 1.0;
        if g.r(s).health >= 2.0 {
            boss_shocka1(g);
        } else if g.r(s).health == 1.0 {
            boss_shockb1(g);
        } else if g.r(s).health == 0.0 {
            boss_shockc1(g);
        }
    }
}

// Just for boss level.
pub fn event_lightning(g: &mut Game) {
    g.v(g.self_).use_ = Some(lightning_use);
}
