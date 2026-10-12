//! `subs.qc`, id's QuakeC 1.06, in Rust.

#![allow(clippy::all)]

use super::*;

pub fn SUB_Null(_g: &mut Game) {}

pub fn SUB_Remove(g: &mut Game) {
    let s = g.self_;
    g.remove(s);
}

/// QuakeEd only writes a single float for angles (bad idea), so up and down are just constant
/// angles.
pub fn SetMovedir(g: &mut Game) {
    let s = g.self_;
    if g.r(s).angles == Vec3::new(0.0, -1.0, 0.0) {
        g.v(s).movedir = Vec3::new(0.0, 0.0, 1.0);
    } else if g.r(s).angles == Vec3::new(0.0, -2.0, 0.0) {
        g.v(s).movedir = Vec3::new(0.0, 0.0, -1.0);
    } else {
        g.makevectors(g.r(s).angles);
        g.v(s).movedir = g.v_forward;
    }

    g.v(s).angles = Vec3::ZERO;
}

pub fn InitTrigger(g: &mut Game) {
    let s = g.self_;
    // Trigger angles are used for one-way touches. An angle of 0 is assumed to mean no
    // restrictions, so use a yaw of 360 instead.
    if g.r(s).angles != Vec3::ZERO {
        SetMovedir(g);
    }
    g.v(s).solid = SOLID_TRIGGER;
    g.setmodel(s, g.r(s).model); // set size and link into world
    g.v(s).movetype = MOVETYPE_NONE;
    g.v(s).modelindex = 0.0;
    g.v(s).model = "";
}

pub fn SUB_CalcMoveEnt(g: &mut Game, ent: Ent, tdest: Vec3, tspeed: f32, func: fn(&mut Game)) {
    let stemp = g.self_;
    g.self_ = ent;

    SUB_CalcMove(g, tdest, tspeed, func);
    g.self_ = stemp;
}

/// Calculate self.velocity and self.nextthink to reach dest from self.origin traveling at speed.
pub fn SUB_CalcMove(g: &mut Game, tdest: Vec3, tspeed: f32, func: fn(&mut Game)) {
    let s = g.self_;
    if tspeed == 0.0 {
        g.objerror("No speed is defined!");
    }

    g.v(s).think1 = Some(func);
    g.v(s).finaldest = tdest;
    g.v(s).think = Some(SUB_CalcMoveDone);

    if tdest == g.r(s).origin {
        g.v(s).velocity = Vec3::ZERO;
        g.v(s).nextthink = g.r(s).ltime + 0.1;
        return;
    }

    // set destdelta to the vector needed to move
    let vdestdelta = tdest - g.r(s).origin;

    // calculate length of vector
    let len = vlen(vdestdelta);

    // divide by speed to get time to reach dest
    let traveltime = len / tspeed;

    if traveltime < 0.1 {
        g.v(s).velocity = Vec3::ZERO;
        g.v(s).nextthink = g.r(s).ltime + 0.1;
        return;
    }

    // set nextthink to trigger a think when dest is reached
    g.v(s).nextthink = g.r(s).ltime + traveltime;

    // scale the destdelta vector by the time spent traveling to get velocity
    g.v(s).velocity = vdestdelta * (1.0 / traveltime);
}

/// After moving, set origin to exact final destination.
pub fn SUB_CalcMoveDone(g: &mut Game) {
    let s = g.self_;
    g.setorigin(s, g.r(s).finaldest);
    g.v(s).velocity = Vec3::ZERO;
    g.v(s).nextthink = -1.0;
    let f = g.r(s).think1;
    if f.is_some() {
        g.call(f);
    }
}

/// The calling function should make sure self.think is valid.
pub fn SUB_CalcAngleMoveEnt(g: &mut Game, ent: Ent, destangle: Vec3, tspeed: f32, func: fn(&mut Game)) {
    let stemp = g.self_;
    g.self_ = ent;
    SUB_CalcAngleMove(g, destangle, tspeed, func);
    g.self_ = stemp;
}

/// Calculate self.avelocity and self.nextthink to reach destangle from self.angles rotating.
pub fn SUB_CalcAngleMove(g: &mut Game, destangle: Vec3, tspeed: f32, func: fn(&mut Game)) {
    let s = g.self_;
    if tspeed == 0.0 {
        g.objerror("No speed is defined!");
    }

    // set destdelta to the vector needed to move
    let destdelta = destangle - g.r(s).angles;

    // calculate length of vector
    let len = vlen(destdelta);

    // divide by speed to get time to reach dest
    let traveltime = len / tspeed;

    // set nextthink to trigger a think when dest is reached
    g.v(s).nextthink = g.r(s).ltime + traveltime;

    // scale the destdelta vector by the time spent traveling to get velocity
    g.v(s).avelocity = destdelta * (1.0 / traveltime);

    g.v(s).think1 = Some(func);
    g.v(s).finalangle = destangle;
    g.v(s).think = Some(SUB_CalcAngleMoveDone);
}

/// After rotating, set angle to exact final angle.
pub fn SUB_CalcAngleMoveDone(g: &mut Game) {
    let s = g.self_;
    g.v(s).angles = g.r(s).finalangle;
    g.v(s).avelocity = Vec3::ZERO;
    g.v(s).nextthink = -1.0;
    let f = g.r(s).think1;
    if f.is_some() {
        g.call(f);
    }
}

pub fn DelayThink(g: &mut Game) {
    g.activator = g.r(g.self_).enemy;
    SUB_UseTargets(g);
    let s = g.self_;
    g.remove(s);
}

/// The global `activator` should be set to the entity that initiated the firing.
///
/// If self.delay is set, a DelayedUse entity will be created that will actually do the
/// SUB_UseTargets after that many seconds have passed.
///
/// Centerprints any self.message to the activator.
///
/// Removes all entities with a targetname that match self.killtarget, so some events can remove
/// other triggers.
///
/// Searches for (string)targetname in all entities that match (string)self.target and calls
/// their .use function.
#[allow(unpredictable_function_pointer_comparisons)]
pub fn SUB_UseTargets(g: &mut Game) {
    let s = g.self_;

    // check for a delay
    if g.r(s).delay != 0.0 {
        // create a temp object to fire at a later time
        let t = g.spawn();
        g.v(t).classname = "DelayedUse";
        g.v(t).nextthink = g.time + g.r(s).delay;
        g.v(t).think = Some(DelayThink);
        g.v(t).enemy = g.activator;
        g.v(t).message = g.r(s).message;
        g.v(t).killtarget = g.r(s).killtarget;
        g.v(t).target = g.r(s).target;
        return;
    }

    // print the message
    let act = g.activator;
    if g.r(act).classname == "player" && g.r(s).message != "" {
        g.centerprint(act, g.r(s).message);
        if g.r(s).noise == "" {
            g.sound(act, CHAN_VOICE, "misc/talk.wav", 1.0, ATTN_NORM);
        }
    }

    // kill the killtargets; once none is left, return, even if there is a target to fire
    if g.r(s).killtarget != "" {
        let mut t = world;
        loop {
            t = g.find(t, |v: &EntVars| v.targetname, g.r(s).killtarget);
            if t == world {
                return;
            }

            g.remove(t);
        }
    }

    // fire targets
    if g.r(s).target != "" {
        let act = g.activator;
        let mut t = world;
        loop {
            t = g.find(t, |v: &EntVars| v.targetname, g.r(s).target);
            if t == world {
                return;
            }

            let stemp = g.self_;
            let otemp = g.other;
            g.self_ = t;
            g.other = stemp;
            let u = g.r(g.self_).use_;
            if u != Some(SUB_Null as fn(&mut Game)) {
                if u.is_some() {
                    g.call(u);
                }
            }
            g.self_ = stemp;
            g.other = otemp;
            g.activator = act;
        }
    }
}

/// In nightmare mode, all attack_finished times become 0; some monsters refire twice
/// automatically.
pub fn SUB_AttackFinished(g: &mut Game, normal: f32) {
    let s = g.self_;
    g.v(s).cnt = 0.0; // refire count for nightmare
    if g.skill != 3.0 {
        g.v(s).attack_finished = g.time + normal;
    }
}

pub fn SUB_CheckRefire(g: &mut Game, thinkst: fn(&mut Game)) {
    let s = g.self_;
    if g.skill != 3.0 {
        return;
    }

    if g.r(s).cnt == 1.0 {
        return;
    }

    if visible(g, g.r(s).enemy) == 0.0 {
        return;
    }

    g.v(s).cnt = 1.0;
    g.v(s).think = Some(thinkst);
}
