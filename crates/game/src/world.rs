//! What stops what (`world.c`, and the hulls `model.c` builds): the map's clipping hulls, the
//! boxes of everything else, the area tree that finds who might touch whom, and the trace.

use crate::{Ent, FL_ITEM, FL_MONSTER, Game, MOVE_MISSILE, MOVE_NOMONSTERS, MOVETYPE_PUSH, SOLID_BSP, SOLID_NOT, SOLID_TRIGGER};
use data::bsp::{Bsp, CONTENTS_EMPTY, CONTENTS_SOLID};
use glam::Vec3;

pub const CONTENTS_CURRENT_0: i32 = -9;
pub const CONTENTS_CURRENT_DOWN: i32 = -14;

#[derive(Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub dist: f32,
    /// 0, 1, 2: facing along that axis, so one coordinate is the distance.
    pub kind: i32,
}

#[derive(Clone, Copy)]
pub struct ClipNode {
    pub plane: usize,
    /// A clip node's number, or a contents value when negative.
    pub children: [i32; 2],
}

/// One of a model's clipping hulls: the tree a box of one size is traced through as a point.
#[derive(Clone, Copy)]
pub struct Hull {
    /// 0: the drawing tree (hull 0); 1: the clip nodes (hulls 1 and 2); 2: the box.
    pub tree: u8,
    pub first: i32,
    pub last: i32,
    pub clip_mins: Vec3,
    pub clip_maxs: Vec3,
}

pub struct SubModel {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub hulls: [Hull; 3],
}

/// A node of the drawing tree, for finding the leafs a box stands in.
pub struct Node {
    pub plane: usize,
    pub children: [i32; 2],
}

/// A node of the area tree (`areanode_t`): the level's box halved four times along its longer
/// side; each entity is kept at the deepest node it does not cross.
pub struct AreaNode {
    pub axis: i32,
    pub dist: f32,
    pub children: [usize; 2],
    pub triggers: Vec<Ent>,
    pub solids: Vec<Ent>,
}

pub struct World {
    pub planes: Vec<Plane>,
    /// Hull 0: the drawing tree read as clip nodes (`Mod_MakeHull0`).
    pub hull0: Vec<ClipNode>,
    pub clipnodes: Vec<ClipNode>,
    pub submodels: Vec<SubModel>,
    pub nodes: Vec<Node>,
    /// Each leaf's contents.
    pub leaf_contents: Vec<i32>,
    /// What each leaf can see, one bit a leaf from leaf 1 (`Mod_LeafPVS`).
    pub leaf_pvs: Vec<Vec<u8>>,
    pub areanodes: Vec<AreaNode>,
    /// The box hull's planes and nodes (`SV_InitBoxHull`), its distances set per use.
    box_planes: [Plane; 6],
    box_nodes: [ClipNode; 6],
}

#[derive(Clone, Copy, Debug)]
pub struct Trace {
    pub allsolid: bool,
    pub startsolid: bool,
    pub inopen: bool,
    pub inwater: bool,
    pub fraction: f32,
    pub endpos: Vec3,
    pub normal: Vec3,
    pub dist: f32,
    pub ent: Option<Ent>,
}

impl Trace {
    fn new(end: Vec3) -> Trace {
        Trace { allsolid: true, startsolid: false, inopen: false, inwater: false, fraction: 1.0, endpos: end, normal: Vec3::ZERO, dist: 0.0, ent: None }
    }
}

const DIST_EPSILON: f32 = 0.03125;
const AREA_DEPTH: i32 = 4;

impl World {
    /// An empty world with no map, for tests of the game's logic that need none.
    pub fn empty() -> World {
        let b = Bsp::parse(&empty_bsp()).expect("the empty map");
        World::from_bsp(&b)
    }

    pub fn from_bsp(b: &Bsp) -> World {
        let planes: Vec<Plane> = b.planes.iter().map(|p| Plane { normal: p.normal, dist: p.dist, kind: p.kind }).collect();
        let hull0 = b
            .nodes
            .iter()
            .map(|n| ClipNode {
                plane: n.plane as usize,
                children: n.children.map(|c| if c >= 0 { c } else { b.leafs[(-1 - c) as usize].contents }),
            })
            .collect();
        let clipnodes: Vec<ClipNode> = b.clipnodes.iter().map(|c| ClipNode { plane: c.plane as usize, children: c.children }).collect();
        let nn = b.nodes.len() as i32;
        let nc = clipnodes.len() as i32;
        let submodels = b
            .models
            .iter()
            .map(|m| SubModel {
                // `Mod_LoadSubmodels` widens every brush model's box by a unit each way.
                mins: m.mins - Vec3::ONE,
                maxs: m.maxs + Vec3::ONE,
                hulls: [
                    Hull { tree: 0, first: m.head_nodes[0], last: nn - 1, clip_mins: Vec3::ZERO, clip_maxs: Vec3::ZERO },
                    Hull { tree: 1, first: m.head_nodes[1], last: nc - 1, clip_mins: Vec3::new(-16.0, -16.0, -24.0), clip_maxs: Vec3::new(16.0, 16.0, 32.0) },
                    Hull { tree: 1, first: m.head_nodes[2], last: nc - 1, clip_mins: Vec3::new(-32.0, -32.0, -24.0), clip_maxs: Vec3::new(32.0, 32.0, 64.0) },
                ],
            })
            .collect();
        let nodes = b.nodes.iter().map(|n| Node { plane: n.plane as usize, children: n.children }).collect();
        let leaf_contents = b.leafs.iter().map(|l| l.contents).collect();
        let leaf_pvs = (0..b.leafs.len()).map(|i| b.leaf_sees(i)).collect();
        let mut box_planes = [Plane { normal: Vec3::ZERO, dist: 0.0, kind: 0 }; 6];
        let mut box_nodes = [ClipNode { plane: 0, children: [0, 0] }; 6];
        for i in 0..6 {
            let side = i & 1;
            box_nodes[i].plane = i;
            box_nodes[i].children[side] = CONTENTS_EMPTY;
            box_nodes[i].children[side ^ 1] = if i != 5 { i as i32 + 1 } else { CONTENTS_SOLID };
            box_planes[i].kind = (i >> 1) as i32;
            box_planes[i].normal[i >> 1] = 1.0;
        }
        let mut w = World { planes, hull0, clipnodes, submodels, nodes, leaf_contents, leaf_pvs, areanodes: Vec::new(), box_planes, box_nodes };
        if let Some(m) = w.submodels.first() {
            let (mins, maxs) = (m.mins, m.maxs);
            w.create_area_node(0, mins, maxs);
        }
        w
    }

    fn create_area_node(&mut self, depth: i32, mins: Vec3, maxs: Vec3) -> usize {
        let i = self.areanodes.len();
        self.areanodes.push(AreaNode { axis: -1, dist: 0.0, children: [0, 0], triggers: Vec::new(), solids: Vec::new() });
        if depth == AREA_DEPTH {
            return i;
        }
        let size = maxs - mins;
        let axis = if size.x > size.y { 0 } else { 1 };
        let dist = 0.5 * (maxs[axis] + mins[axis]);
        let (mut maxs1, mut mins2) = (maxs, mins);
        maxs1[axis] = dist;
        mins2[axis] = dist;
        let c0 = self.create_area_node(depth + 1, mins2, maxs);
        let c1 = self.create_area_node(depth + 1, mins, maxs1);
        let n = &mut self.areanodes[i];
        n.axis = axis as i32;
        n.dist = dist;
        n.children = [c0, c1];
        i
    }

    fn tree(&self, h: &Hull) -> (&[ClipNode], &[Plane]) {
        match h.tree {
            0 => (&self.hull0, &self.planes),
            1 => (&self.clipnodes, &self.planes),
            _ => (&self.box_nodes, &self.box_planes),
        }
    }

    /// `SV_HullPointContents`.
    pub fn hull_point_contents(&self, h: &Hull, mut num: i32, p: Vec3) -> i32 {
        let (nodes, planes) = self.tree(h);
        while num >= 0 {
            assert!(num >= h.first && num <= h.last, "SV_HullPointContents: bad node number");
            let node = &nodes[num as usize];
            let plane = &planes[node.plane];
            let d = if plane.kind < 3 { p[plane.kind as usize] - plane.dist } else { plane.normal.dot(p) - plane.dist };
            num = if d < 0.0 { node.children[1] } else { node.children[0] };
        }
        num
    }

    /// `SV_RecursiveHullCheck`: false when the trace stopped in this branch.
    pub fn recursive_hull_check(&self, h: &Hull, num: i32, p1f: f32, p2f: f32, p1: Vec3, p2: Vec3, trace: &mut Trace) -> bool {
        if num < 0 {
            if num != CONTENTS_SOLID {
                trace.allsolid = false;
                if num == CONTENTS_EMPTY {
                    trace.inopen = true;
                } else {
                    trace.inwater = true;
                }
            } else {
                trace.startsolid = true;
            }
            return true;
        }
        assert!(num >= h.first && num <= h.last, "SV_RecursiveHullCheck: bad node number");
        let (nodes, planes) = self.tree(h);
        let node = nodes[num as usize];
        let plane = planes[node.plane];
        let (t1, t2) = if plane.kind < 3 {
            (p1[plane.kind as usize] - plane.dist, p2[plane.kind as usize] - plane.dist)
        } else {
            (plane.normal.dot(p1) - plane.dist, plane.normal.dot(p2) - plane.dist)
        };
        if t1 >= 0.0 && t2 >= 0.0 {
            return self.recursive_hull_check(h, node.children[0], p1f, p2f, p1, p2, trace);
        }
        if t1 < 0.0 && t2 < 0.0 {
            return self.recursive_hull_check(h, node.children[1], p1f, p2f, p1, p2, trace);
        }
        let mut frac = if t1 < 0.0 { (t1 + DIST_EPSILON) / (t1 - t2) } else { (t1 - DIST_EPSILON) / (t1 - t2) };
        frac = frac.clamp(0.0, 1.0);
        let mut midf = p1f + (p2f - p1f) * frac;
        let mut mid = p1 + frac * (p2 - p1);
        let side = (t1 < 0.0) as usize;
        if !self.recursive_hull_check(h, node.children[side], p1f, midf, p1, mid, trace) {
            return false;
        }
        if self.hull_point_contents(h, node.children[side ^ 1], mid) != CONTENTS_SOLID {
            return self.recursive_hull_check(h, node.children[side ^ 1], midf, p2f, mid, p2, trace);
        }
        if trace.allsolid {
            return false;
        }
        if side == 0 {
            trace.normal = plane.normal;
            trace.dist = plane.dist;
        } else {
            trace.normal = -plane.normal;
            trace.dist = -plane.dist;
        }
        while self.hull_point_contents(h, h.first, mid) == CONTENTS_SOLID {
            frac -= 0.1;
            if frac < 0.0 {
                trace.fraction = midf;
                trace.endpos = mid;
                return false;
            }
            midf = p1f + (p2f - p1f) * frac;
            mid = p1 + frac * (p2 - p1);
        }
        trace.fraction = midf;
        trace.endpos = mid;
        false
    }

    /// The contents at a point of the world (`SV_PointContents`): a current counts as water.
    /// Whether the line from `a` to `b` crosses no solid of the map itself (its first model, its
    /// hull for a point): what a lamp at `a` lights at `b` (row 34), read without touching the
    /// game's own state.
    pub fn sees(&self, a: Vec3, b: Vec3) -> bool {
        let h = &self.submodels[0].hulls[0];
        let mut t = Trace::new(b);
        self.recursive_hull_check(h, h.first, 0.0, 1.0, a, b, &mut t);
        !t.startsolid && t.fraction >= 1.0
    }

    pub fn point_contents(&self, p: Vec3) -> i32 {
        let c = self.true_point_contents(p);
        if (CONTENTS_CURRENT_DOWN..=CONTENTS_CURRENT_0).contains(&c) { crate::CONTENT_WATER } else { c }
    }

    pub fn true_point_contents(&self, p: Vec3) -> i32 {
        let h = self.submodels[0].hulls[0];
        self.hull_point_contents(&h, h.first, p)
    }

    /// The leaf a point is in (`Mod_PointInLeaf`).
    pub fn point_in_leaf(&self, p: Vec3) -> usize {
        let mut n = 0i32;
        loop {
            let node = &self.nodes[n as usize];
            let plane = &self.planes[node.plane];
            let d = plane.normal.dot(p) - plane.dist;
            n = if d > 0.0 { node.children[0] } else { node.children[1] };
            if n < 0 {
                return (-1 - n) as usize;
            }
        }
    }
}

impl Game {
    /// `SV_HullForEntity`: the hull to trace a box of `mins`–`maxs` against `ent`, and the offset
    /// to take from the trace's points to stand in the hull's own space.
    fn hull_for_entity(&mut self, ent: Ent, mins: Vec3, maxs: Vec3) -> (Hull, Vec3) {
        let v = &self.ents[ent].v;
        if v.solid == SOLID_BSP {
            assert!(v.movetype == MOVETYPE_PUSH, "SOLID_BSP without MOVETYPE_PUSH");
            let brush = self.models.get(v.modelindex as usize).and_then(|m| m.brush).expect("MOVETYPE_PUSH with a non bsp model");
            let size = maxs - mins;
            let sub = &self.map.submodels[brush];
            let h = if size.x < 3.0 { sub.hulls[0] } else if size.x <= 32.0 { sub.hulls[1] } else { sub.hulls[2] };
            (h, h.clip_mins - mins + v.origin)
        } else {
            let hmins = v.mins - maxs;
            let hmaxs = v.maxs - mins;
            let w = &mut self.map;
            w.box_planes[0].dist = hmaxs.x;
            w.box_planes[1].dist = hmins.x;
            w.box_planes[2].dist = hmaxs.y;
            w.box_planes[3].dist = hmins.y;
            w.box_planes[4].dist = hmaxs.z;
            w.box_planes[5].dist = hmins.z;
            (Hull { tree: 2, first: 0, last: 5, clip_mins: Vec3::ZERO, clip_maxs: Vec3::ZERO }, v.origin)
        }
    }

    /// `SV_ClipMoveToEntity`.
    pub fn clip_move_to_entity(&mut self, ent: Ent, start: Vec3, mins: Vec3, maxs: Vec3, end: Vec3) -> Trace {
        let mut trace = Trace::new(end);
        let (hull, offset) = self.hull_for_entity(ent, mins, maxs);
        self.map.recursive_hull_check(&hull, hull.first, 0.0, 1.0, start - offset, end - offset, &mut trace);
        if trace.fraction != 1.0 {
            trace.endpos += offset;
        }
        if trace.fraction < 1.0 || trace.startsolid {
            trace.ent = Some(ent);
        }
        trace
    }

    /// `SV_UnlinkEdict`.
    pub fn unlink_edict(&mut self, e: Ent) {
        if let Some((node, trigger)) = self.ents[e].area.take() {
            let list = if trigger { &mut self.map.areanodes[node].triggers } else { &mut self.map.areanodes[node].solids };
            if let Some(i) = list.iter().position(|&x| x == e) {
                list.remove(i);
            }
        }
    }

    /// `SV_LinkEdict`: the entity's box worked out, its leafs found, and it put into the area tree;
    /// with `touch_triggers`, every trigger it now overlaps is touched.
    pub fn link_edict(&mut self, e: Ent, touch_triggers: bool) {
        self.unlink_edict(e);
        if e == 0 || self.ents[e].free {
            return;
        }
        let v = &mut self.ents[e].v;
        v.absmin = v.origin + v.mins;
        v.absmax = v.origin + v.maxs;
        if v.flags & FL_ITEM != 0 {
            v.absmin -= Vec3::new(15.0, 15.0, 0.0);
            v.absmax += Vec3::new(15.0, 15.0, 0.0);
        } else {
            v.absmin -= Vec3::ONE;
            v.absmax += Vec3::ONE;
        }
        let (absmin, absmax, modelindex, solid) = (v.absmin, v.absmax, v.modelindex, v.solid);
        let mut leafs = Vec::new();
        if modelindex != 0.0 && !self.map.nodes.is_empty() {
            self.find_touched_leafs(0, absmin, absmax, &mut leafs);
        }
        self.ents[e].leafs = leafs;
        if solid == SOLID_NOT {
            return;
        }
        let mut node = 0;
        loop {
            let n = &self.map.areanodes[node];
            if n.axis == -1 {
                break;
            }
            let a = n.axis as usize;
            if absmin[a] > n.dist {
                node = n.children[0];
            } else if absmax[a] < n.dist {
                node = n.children[1];
            } else {
                break;
            }
        }
        let trigger = solid == SOLID_TRIGGER;
        if trigger {
            self.map.areanodes[node].triggers.push(e);
        } else {
            self.map.areanodes[node].solids.push(e);
        }
        self.ents[e].area = Some((node, trigger));
        if touch_triggers {
            self.touch_links(e, 0);
        }
    }

    /// `SV_FindTouchedLeafs`: the leafs of the world a box stands in, at most sixteen
    /// (`MAX_ENT_LEAFS`).
    fn find_touched_leafs(&self, node: i32, absmin: Vec3, absmax: Vec3, out: &mut Vec<u16>) {
        if node < 0 {
            let leaf = (-1 - node) as usize;
            if self.map.leaf_contents[leaf] == CONTENTS_SOLID || out.len() == 16 {
                return;
            }
            out.push((leaf - 1) as u16);
            return;
        }
        let n = &self.map.nodes[node as usize];
        let sides = box_on_plane_side(absmin, absmax, &self.map.planes[n.plane]);
        if sides & 1 != 0 {
            self.find_touched_leafs(n.children[0], absmin, absmax, out);
        }
        if sides & 2 != 0 {
            self.find_touched_leafs(n.children[1], absmin, absmax, out);
        }
    }

    /// `SV_TouchLinks`: every trigger whose box overlaps the entity's is touched by it.
    fn touch_links(&mut self, e: Ent, node: usize) {
        let list = self.map.areanodes[node].triggers.clone();
        for touch in list {
            if touch == e || self.ents[touch].free {
                continue;
            }
            let t = &self.ents[touch].v;
            let Some(f) = t.touch else { continue };
            if t.solid != SOLID_TRIGGER {
                continue;
            }
            let v = &self.ents[e].v;
            if v.absmin.x > t.absmax.x || v.absmin.y > t.absmax.y || v.absmin.z > t.absmax.z || v.absmax.x < t.absmin.x || v.absmax.y < t.absmin.y || v.absmax.z < t.absmin.z {
                continue;
            }
            self.call_as(touch, e, f);
        }
        let n = &self.map.areanodes[node];
        if n.axis == -1 {
            return;
        }
        let (a, dist, c) = (n.axis as usize, n.dist, n.children);
        if self.ents[e].v.absmax[a] > dist {
            self.touch_links(e, c[0]);
        }
        if self.ents[e].v.absmin[a] < dist {
            self.touch_links(e, c[1]);
        }
    }

    /// `SV_TestEntityPosition`: whether the entity stands inside something solid.
    pub fn test_entity_position(&mut self, e: Ent) -> bool {
        let v = &self.ents[e].v;
        let (o, mins, maxs) = (v.origin, v.mins, v.maxs);
        self.trace_move(o, mins, maxs, o, 0, e).startsolid
    }

    /// `SV_Move`: a box swept from `start` to `end` against the world and every solid entity
    /// but `pass`, its own missiles and its owner.
    pub fn trace_move(&mut self, start: Vec3, mins: Vec3, maxs: Vec3, end: Vec3, kind: i32, pass: Ent) -> Trace {
        let mut clip = self.clip_move_to_entity(0, start, mins, maxs, end);
        let (mins2, maxs2) = if kind == MOVE_MISSILE { (Vec3::splat(-15.0), Vec3::splat(15.0)) } else { (mins, maxs) };
        let mut boxmins = Vec3::ZERO;
        let mut boxmaxs = Vec3::ZERO;
        for i in 0..3 {
            if end[i] > start[i] {
                boxmins[i] = start[i] + mins2[i] - 1.0;
                boxmaxs[i] = end[i] + maxs2[i] + 1.0;
            } else {
                boxmins[i] = end[i] + mins2[i] - 1.0;
                boxmaxs[i] = start[i] + maxs2[i] + 1.0;
            }
        }
        let pass = if pass == 0 { None } else { Some(pass) };
        self.clip_to_links(0, &mut clip, start, end, mins, maxs, mins2, maxs2, boxmins, boxmaxs, kind, pass);
        clip
    }

    /// `SV_ClipToLinks`.
    fn clip_to_links(&mut self, node: usize, clip: &mut Trace, start: Vec3, end: Vec3, mins: Vec3, maxs: Vec3, mins2: Vec3, maxs2: Vec3, boxmins: Vec3, boxmaxs: Vec3, kind: i32, pass: Option<Ent>) {
        let list = self.map.areanodes[node].solids.clone();
        for touch in list {
            let t = &self.ents[touch].v;
            if t.solid == SOLID_NOT || Some(touch) == pass {
                continue;
            }
            assert!(t.solid != SOLID_TRIGGER, "Trigger in clipping list");
            if kind == MOVE_NOMONSTERS && t.solid != SOLID_BSP {
                continue;
            }
            if boxmins.x > t.absmax.x || boxmins.y > t.absmax.y || boxmins.z > t.absmax.z || boxmaxs.x < t.absmin.x || boxmaxs.y < t.absmin.y || boxmaxs.z < t.absmin.z {
                continue;
            }
            if let Some(p) = pass
                && self.ents[p].v.size.x != 0.0 && t.size.x == 0.0 {
                    continue; // points never interact
                }
            if clip.allsolid {
                return;
            }
            if let Some(p) = pass
                && (t.owner == p || self.ents[p].v.owner == touch) {
                    continue; // its own missiles, or its owner
                }
            let trace = if t.flags & FL_MONSTER != 0 {
                self.clip_move_to_entity(touch, start, mins2, maxs2, end)
            } else {
                self.clip_move_to_entity(touch, start, mins, maxs, end)
            };
            if trace.allsolid || trace.startsolid || trace.fraction < clip.fraction {
                let was_startsolid = clip.startsolid;
                *clip = trace;
                clip.ent = Some(touch);
                if was_startsolid {
                    clip.startsolid = true;
                }
            } else if trace.startsolid {
                clip.startsolid = true;
            }
        }
        let n = &self.map.areanodes[node];
        if n.axis == -1 {
            return;
        }
        let (a, dist, c) = (n.axis as usize, n.dist, n.children);
        if boxmaxs[a] > dist {
            self.clip_to_links(c[0], clip, start, end, mins, maxs, mins2, maxs2, boxmins, boxmaxs, kind, pass);
        }
        if boxmins[a] < dist {
            self.clip_to_links(c[1], clip, start, end, mins, maxs, mins2, maxs2, boxmins, boxmaxs, kind, pass);
        }
    }

    /// The contents at a point of the world.
    pub fn point_contents(&self, p: Vec3) -> i32 {
        self.map.point_contents(p)
    }
}

/// `BoxOnPlaneSide`: 1 if the box is in front of the plane, 2 if behind, 3 if across it.
pub fn box_on_plane_side(mins: Vec3, maxs: Vec3, p: &Plane) -> i32 {
    if p.kind < 3 {
        let a = p.kind as usize;
        if p.dist <= mins[a] {
            return 1;
        }
        if p.dist >= maxs[a] {
            return 2;
        }
        return 3;
    }
    let mut near = Vec3::ZERO;
    let mut far = Vec3::ZERO;
    for i in 0..3 {
        if p.normal[i] >= 0.0 {
            far[i] = maxs[i];
            near[i] = mins[i];
        } else {
            far[i] = mins[i];
            near[i] = maxs[i];
        }
    }
    let mut sides = 0;
    if p.normal.dot(far) >= p.dist {
        sides = 1;
    }
    if p.normal.dot(near) < p.dist {
        sides |= 2;
    }
    sides
}

/// The smallest map the reader accepts: one plane, one empty leaf and one model, so the game's
/// logic can be tried with nothing around it. Built in code: no file of id's.
fn empty_bsp() -> Vec<u8> {
    let mut lumps: Vec<Vec<u8>> = vec![Vec::new(); 15];
    lumps[0] = b"{\n\"classname\" \"worldspawn\"\n}\n\0".to_vec();
    let mut plane = Vec::new();
    for f in [0.0f32, 0.0, 1.0, -4096.0] {
        plane.extend_from_slice(&f.to_le_bytes());
    }
    plane.extend_from_slice(&2i32.to_le_bytes());
    lumps[1] = plane;
    // One node: in front of the plane far below is empty (leaf 1), behind it solid (leaf 0).
    let mut node = Vec::new();
    node.extend_from_slice(&0i32.to_le_bytes());
    node.extend_from_slice(&(-2i16).to_le_bytes());
    node.extend_from_slice(&(-1i16).to_le_bytes());
    node.extend_from_slice(&[0u8; 16]);
    lumps[5] = node;
    let mut clip = Vec::new();
    clip.extend_from_slice(&0i32.to_le_bytes());
    clip.extend_from_slice(&(CONTENTS_EMPTY as i16).to_le_bytes());
    clip.extend_from_slice(&(CONTENTS_SOLID as i16).to_le_bytes());
    lumps[9] = clip;
    let mut leafs = Vec::new();
    for contents in [CONTENTS_SOLID, CONTENTS_EMPTY] {
        leafs.extend_from_slice(&contents.to_le_bytes());
        leafs.extend_from_slice(&(-1i32).to_le_bytes());
        leafs.extend_from_slice(&[0u8; 20]);
    }
    lumps[10] = leafs;
    let mut model = Vec::new();
    for f in [-4096.0f32, -4096.0, -4096.0, 4096.0, 4096.0, 4096.0, 0.0, 0.0, 0.0] {
        model.extend_from_slice(&f.to_le_bytes());
    }
    for h in [0i32, 0, 0, 0, 1, 0, 0] {
        model.extend_from_slice(&h.to_le_bytes());
    }
    lumps[14] = model;
    let mut out = Vec::new();
    out.extend_from_slice(&29i32.to_le_bytes());
    let mut at = 4 + 15 * 8;
    for l in &lumps {
        out.extend_from_slice(&(at as i32).to_le_bytes());
        out.extend_from_slice(&(l.len() as i32).to_le_bytes());
        at += l.len();
    }
    for l in &lumps {
        out.extend_from_slice(l);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_empty_map_is_empty_above_its_floor() {
        let w = World::empty();
        assert_eq!(w.point_contents(Vec3::new(0.0, 0.0, 0.0)), CONTENTS_EMPTY);
        assert_eq!(w.point_contents(Vec3::new(0.0, 0.0, -5000.0)), CONTENTS_SOLID);
    }

    #[test]
    fn a_trace_stops_at_the_floor_by_the_epsilon() {
        let w = World::empty();
        let h = w.submodels[0].hulls[1];
        let mut t = Trace::new(Vec3::new(0.0, 0.0, -5000.0));
        w.recursive_hull_check(&h, h.first, 0.0, 1.0, Vec3::new(0.0, 0.0, -4000.0), Vec3::new(0.0, 0.0, -5000.0), &mut t);
        assert!((t.endpos.z - (-4096.0 + DIST_EPSILON)).abs() < 1e-3, "{:?}", t.endpos);
        assert_eq!(t.normal, Vec3::Z);
    }
}
