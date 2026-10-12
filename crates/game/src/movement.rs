//! How monsters walk (`sv_move.c`): a step up and down stairs, never off an edge, and the chase
//! that turns by eighths of a circle towards the goal.

use crate::{CONTENT_EMPTY, CONTENT_SOLID, Ent, FL_FLY, FL_ONGROUND, FL_PARTIALGROUND, FL_SWIM, Game};
use glam::Vec3;

/// The height a walker steps up without jumping (`STEPSIZE`).
pub const STEPSIZE: f32 = 18.0;

/// `anglemod`: an angle into 0–360, to the 65,536 steps the original keeps.
pub fn anglemod(a: f32) -> f32 {
    (360.0 / 65536.0) * (((a * (65536.0 / 360.0)) as i32) & 65535) as f32
}

impl Game {
    /// `SV_CheckBottom`: false if a corner of the entity's base is over an edge that is not a
    /// stair.
    pub fn check_bottom(&mut self, ent: Ent) -> bool {
        let v = &self.ents[ent].v;
        let mins = v.origin + v.mins;
        let maxs = v.origin + v.maxs;
        let mut start = Vec3::new(0.0, 0.0, mins.z - 1.0);
        let mut easy = true;
        'corners: for x in 0..=1 {
            for y in 0..=1 {
                start.x = if x == 1 { maxs.x } else { mins.x };
                start.y = if y == 1 { maxs.y } else { mins.y };
                if self.point_contents(start) != CONTENT_SOLID {
                    easy = false;
                    break 'corners;
                }
            }
        }
        if easy {
            return true;
        }
        start.z = mins.z;
        start.x = (mins.x + maxs.x) * 0.5;
        start.y = (mins.y + maxs.y) * 0.5;
        let mut stop = start;
        stop.z = start.z - 2.0 * STEPSIZE;
        let t = self.trace_move(start, Vec3::ZERO, Vec3::ZERO, stop, 1, ent);
        if t.fraction == 1.0 {
            return false;
        }
        let mid = t.endpos.z;
        for x in 0..=1 {
            for y in 0..=1 {
                start.x = if x == 1 { maxs.x } else { mins.x };
                start.y = if y == 1 { maxs.y } else { mins.y };
                stop.x = start.x;
                stop.y = start.y;
                let t = self.trace_move(start, Vec3::ZERO, Vec3::ZERO, stop, 1, ent);
                if t.fraction == 1.0 || mid - t.endpos.z > STEPSIZE {
                    return false;
                }
            }
        }
        true
    }

    /// `SV_movestep`: the move made if it can be, up a stair or down one; false and no move if
    /// not.
    pub fn movestep(&mut self, ent: Ent, mv: Vec3, relink: bool) -> bool {
        let oldorg = self.ents[ent].v.origin;
        let flags = self.ents[ent].v.flags;
        let (mins, maxs) = (self.ents[ent].v.mins, self.ents[ent].v.maxs);
        if flags & (FL_SWIM | FL_FLY) != 0 {
            for i in 0..2 {
                let mut neworg = self.ents[ent].v.origin + mv;
                let enemy = self.ents[ent].v.enemy;
                if i == 0 && enemy != 0 {
                    let dz = self.ents[ent].v.origin.z - self.ents[enemy].v.origin.z;
                    if dz > 40.0 {
                        neworg.z -= 8.0;
                    }
                    if dz < 30.0 {
                        neworg.z += 8.0;
                    }
                }
                let o = self.ents[ent].v.origin;
                let t = self.trace_move(o, mins, maxs, neworg, 0, ent);
                if t.fraction == 1.0 {
                    if flags & FL_SWIM != 0 && self.point_contents(t.endpos) == CONTENT_EMPTY {
                        return false; // a swimmer may not leave the water
                    }
                    self.ents[ent].v.origin = t.endpos;
                    if relink {
                        self.link_edict(ent, true);
                    }
                    return true;
                }
                if enemy == 0 {
                    break;
                }
            }
            return false;
        }
        let mut neworg = oldorg + mv;
        neworg.z += STEPSIZE;
        let mut end = neworg;
        end.z -= STEPSIZE * 2.0;
        let mut t = self.trace_move(neworg, mins, maxs, end, 0, ent);
        if t.allsolid {
            return false;
        }
        if t.startsolid {
            neworg.z -= STEPSIZE;
            t = self.trace_move(neworg, mins, maxs, end, 0, ent);
            if t.allsolid || t.startsolid {
                return false;
            }
        }
        if t.fraction == 1.0 {
            if flags & FL_PARTIALGROUND != 0 {
                self.ents[ent].v.origin += mv;
                if relink {
                    self.link_edict(ent, true);
                }
                self.ents[ent].v.flags &= !FL_ONGROUND;
                return true;
            }
            return false; // walked off an edge
        }
        self.ents[ent].v.origin = t.endpos;
        if !self.check_bottom(ent) {
            if self.ents[ent].v.flags & FL_PARTIALGROUND != 0 {
                if relink {
                    self.link_edict(ent, true);
                }
                return true;
            }
            self.ents[ent].v.origin = oldorg;
            return false;
        }
        self.ents[ent].v.flags &= !FL_PARTIALGROUND;
        self.ents[ent].v.groundentity = t.ent.unwrap_or(0);
        if relink {
            self.link_edict(ent, true);
        }
        true
    }

    /// `PF_changeyaw` for an entity: its yaw turned towards `ideal_yaw` by at most `yaw_speed`.
    pub fn change_yaw(&mut self, ent: Ent) {
        let v = &mut self.ents[ent].v;
        let current = anglemod(v.angles.y);
        let ideal = v.ideal_yaw;
        let speed = v.yaw_speed;
        if current == ideal {
            return;
        }
        let mut mv = ideal - current;
        if ideal > current {
            if mv >= 180.0 {
                mv -= 360.0;
            }
        } else if mv <= -180.0 {
            mv += 360.0;
        }
        mv = if mv > 0.0 { mv.min(speed) } else { mv.max(-speed) };
        v.angles.y = anglemod(current + mv);
    }

    /// `SV_StepDirection`: turn to the direction, and step if now facing within 45°.
    fn step_direction(&mut self, ent: Ent, yaw: f32, dist: f32) -> bool {
        self.ents[ent].v.ideal_yaw = yaw;
        self.change_yaw(ent);
        let r = yaw * std::f32::consts::PI * 2.0 / 360.0;
        let mv = Vec3::new(r.cos() * dist, r.sin() * dist, 0.0);
        let oldorigin = self.ents[ent].v.origin;
        if self.movestep(ent, mv, false) {
            let delta = self.ents[ent].v.angles.y - self.ents[ent].v.ideal_yaw;
            if delta > 45.0 && delta < 315.0 {
                self.ents[ent].v.origin = oldorigin; // not turned far enough to step
            }
            self.link_edict(ent, true);
            return true;
        }
        self.link_edict(ent, true);
        false
    }

    /// `SV_NewChaseDir`.
    fn new_chase_dir(&mut self, actor: Ent, enemy: Ent, dist: f32) {
        const NODIR: f32 = -1.0;
        let olddir = anglemod(((self.ents[actor].v.ideal_yaw / 45.0) as i32 * 45) as f32);
        let turnaround = anglemod(olddir - 180.0);
        let deltax = self.ents[enemy].v.origin.x - self.ents[actor].v.origin.x;
        let deltay = self.ents[enemy].v.origin.y - self.ents[actor].v.origin.y;
        let mut d1 = if deltax > 10.0 { 0.0 } else if deltax < -10.0 { 180.0 } else { NODIR };
        let mut d2 = if deltay < -10.0 { 270.0 } else if deltay > 10.0 { 90.0 } else { NODIR };
        if d1 != NODIR && d2 != NODIR {
            let tdir = if d1 == 0.0 { if d2 == 90.0 { 45.0 } else { 315.0 } } else if d2 == 90.0 { 135.0 } else { 215.0 };
            if tdir != turnaround && self.step_direction(actor, tdir, dist) {
                return;
            }
        }
        // C's abs of the float deltas cut to integers, as the original's `abs` does.
        if (self.rand() & 3) & 1 != 0 || (deltay as i32).abs() > (deltax as i32).abs() {
            std::mem::swap(&mut d1, &mut d2);
        }
        if d1 != NODIR && d1 != turnaround && self.step_direction(actor, d1, dist) {
            return;
        }
        if d2 != NODIR && d2 != turnaround && self.step_direction(actor, d2, dist) {
            return;
        }
        if olddir != NODIR && self.step_direction(actor, olddir, dist) {
            return;
        }
        if self.rand() & 1 != 0 {
            let mut tdir = 0.0;
            while tdir <= 315.0 {
                if tdir != turnaround && self.step_direction(actor, tdir, dist) {
                    return;
                }
                tdir += 45.0;
            }
        } else {
            let mut tdir = 315.0;
            while tdir >= 0.0 {
                if tdir != turnaround && self.step_direction(actor, tdir, dist) {
                    return;
                }
                tdir -= 45.0;
            }
        }
        if turnaround != NODIR && self.step_direction(actor, turnaround, dist) {
            return;
        }
        self.ents[actor].v.ideal_yaw = olddir; // cannot move
        if !self.check_bottom(actor) {
            self.ents[actor].v.flags |= FL_PARTIALGROUND;
        }
    }

    // as `SV_CloseEnough` writes it: a NaN in a box counts as close, so the negations stay
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn close_enough(&self, ent: Ent, goal: Ent, dist: f32) -> bool {
        let (e, g) = (&self.ents[ent].v, &self.ents[goal].v);
        (0..3).all(|i| !(g.absmin[i] > e.absmax[i] + dist) && !(g.absmax[i] < e.absmin[i] - dist))
    }

    /// `SV_MoveToGoal`: `movetogoal(dist)` for `self`.
    pub fn movetogoal(&mut self, dist: f32) {
        let ent = self.self_;
        let goal = self.ents[ent].v.goalentity;
        if self.ents[ent].v.flags & (FL_ONGROUND | FL_FLY | FL_SWIM) == 0 {
            return;
        }
        if self.ents[ent].v.enemy != 0 && self.close_enough(ent, goal, dist) {
            return;
        }
        let ideal = self.ents[ent].v.ideal_yaw;
        if self.rand() & 3 == 1 || !self.step_direction(ent, ideal, dist) {
            self.new_chase_dir(ent, goal, dist);
        }
    }
}
