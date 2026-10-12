//! Rays through the map's triangles: a bounding volume hierarchy built by binned surface-area
//! cost, and the two questions light asks of it, "is anything in the way" and "what is hit first".

use glam::Vec3;

/// A triangle the light meets, and what it is.
#[derive(Clone, Copy)]
pub struct Tri {
    pub a: Vec3,
    pub e1: Vec3,
    pub e2: Vec3,
    /// The face it belongs to.
    pub face: u32,
    /// Whether it stops light (a liquid's surface does not, as `light.exe` traced only solids).
    pub blocks: bool,
}

#[derive(Clone, Copy)]
struct Node {
    min: Vec3,
    max: Vec3,
    /// A leaf: the first triangle and the count; an inner node: the left child (right is +1),
    /// and a count of nought.
    first: u32,
    count: u32,
}

pub struct Bvh {
    nodes: Vec<Node>,
    pub tris: Vec<Tri>,
}

/// What a ray met first.
#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub tri: u32,
    pub u: f32,
    pub v: f32,
}

const BINS: usize = 12;
const LEAF: usize = 4;

fn tri_bounds(t: &Tri) -> (Vec3, Vec3) {
    let b = t.a + t.e1;
    let c = t.a + t.e2;
    (t.a.min(b).min(c), t.a.max(b).max(c))
}

fn area(min: Vec3, max: Vec3) -> f32 {
    let d = (max - min).max(Vec3::ZERO);
    2.0 * (d.x * d.y + d.y * d.z + d.z * d.x)
}

impl Bvh {
    pub fn build(mut tris: Vec<Tri>) -> Bvh {
        let mut nodes = Vec::with_capacity(tris.len() * 2);
        let centroids: Vec<Vec3> = tris.iter().map(|t| t.a + (t.e1 + t.e2) / 3.0).collect();
        let mut order: Vec<u32> = (0..tris.len() as u32).collect();
        nodes.push(Node { min: Vec3::ZERO, max: Vec3::ZERO, first: 0, count: 0 });
        let mut stack = vec![(0usize, 0usize, tris.len())];
        while let Some((ni, start, end)) = stack.pop() {
            let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
            let (mut cmin, mut cmax) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
            for &i in &order[start..end] {
                let (a, b) = tri_bounds(&tris[i as usize]);
                min = min.min(a);
                max = max.max(b);
                cmin = cmin.min(centroids[i as usize]);
                cmax = cmax.max(centroids[i as usize]);
            }
            nodes[ni].min = min;
            nodes[ni].max = max;
            let n = end - start;
            let extent = cmax - cmin;
            let axis = if extent.x >= extent.y && extent.x >= extent.z { 0 } else if extent.y >= extent.z { 1 } else { 2 };
            if n <= LEAF || extent[axis] <= 0.0 {
                nodes[ni].first = start as u32;
                nodes[ni].count = n as u32;
                continue;
            }
            // binned surface-area split along the longest axis of the centroids
            let mut bins = [(Vec3::splat(f32::MAX), Vec3::splat(f32::MIN), 0usize); BINS];
            let scale = BINS as f32 / extent[axis];
            let bin_of = |c: Vec3| (((c[axis] - cmin[axis]) * scale) as usize).min(BINS - 1);
            for &i in &order[start..end] {
                let b = &mut bins[bin_of(centroids[i as usize])];
                let (lo, hi) = tri_bounds(&tris[i as usize]);
                b.0 = b.0.min(lo);
                b.1 = b.1.max(hi);
                b.2 += 1;
            }
            let mut best = (f32::MAX, 0);
            for split in 1..BINS {
                let (mut lmin, mut lmax, mut ln) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN), 0);
                for b in &bins[..split] {
                    lmin = lmin.min(b.0);
                    lmax = lmax.max(b.1);
                    ln += b.2;
                }
                let (mut rmin, mut rmax, mut rn) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN), 0);
                for b in &bins[split..] {
                    rmin = rmin.min(b.0);
                    rmax = rmax.max(b.1);
                    rn += b.2;
                }
                if ln == 0 || rn == 0 {
                    continue;
                }
                let cost = area(lmin, lmax) * ln as f32 + area(rmin, rmax) * rn as f32;
                if cost < best.0 {
                    best = (cost, split);
                }
            }
            if best.0 == f32::MAX {
                nodes[ni].first = start as u32;
                nodes[ni].count = n as u32;
                continue;
            }
            let slice = &mut order[start..end];
            let mid = {
                let mut i = 0;
                for j in 0..slice.len() {
                    if bin_of(centroids[slice[j] as usize]) < best.1 {
                        slice.swap(i, j);
                        i += 1;
                    }
                }
                start + i
            };
            let left = nodes.len();
            nodes.push(Node { min: Vec3::ZERO, max: Vec3::ZERO, first: 0, count: 0 });
            nodes.push(Node { min: Vec3::ZERO, max: Vec3::ZERO, first: 0, count: 0 });
            nodes[ni].first = left as u32;
            stack.push((left, start, mid));
            stack.push((left + 1, mid, end));
        }
        let sorted: Vec<Tri> = order.iter().map(|&i| tris[i as usize]).collect();
        tris = sorted;
        Bvh { nodes, tris }
    }

    #[inline]
    fn slab(n: &Node, o: Vec3, inv: Vec3, tmax: f32) -> Option<f32> {
        let t0 = (n.min - o) * inv;
        let t1 = (n.max - o) * inv;
        let tn = t0.min(t1).max_element().max(0.0);
        let tf = t0.max(t1).min_element().min(tmax);
        (tn <= tf).then_some(tn)
    }

    /// Möller–Trumbore: the distance along the ray, and the hit's place on the triangle.
    #[inline]
    fn intersect(t: &Tri, o: Vec3, d: Vec3) -> Option<(f32, f32, f32)> {
        let p = d.cross(t.e2);
        let det = t.e1.dot(p);
        if det.abs() < 1e-9 {
            return None;
        }
        let inv = 1.0 / det;
        let s = o - t.a;
        let u = s.dot(p) * inv;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = s.cross(t.e1);
        let v = d.dot(q) * inv;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let dist = t.e2.dot(q) * inv;
        (dist > 1e-4).then_some((dist, u, v))
    }

    /// Whether anything that blocks light lies on the segment from `o` along `d` for `tmax`.
    pub fn occluded(&self, o: Vec3, d: Vec3, tmax: f32) -> bool {
        let inv = d.recip();
        let mut stack = [0u32; 64];
        let mut sp = 1;
        while sp > 0 {
            sp -= 1;
            let n = &self.nodes[stack[sp] as usize];
            if Self::slab(n, o, inv, tmax).is_none() {
                continue;
            }
            if n.count > 0 {
                for t in &self.tris[n.first as usize..(n.first + n.count) as usize] {
                    if t.blocks
                        && let Some((dist, _, _)) = Self::intersect(t, o, d)
                            && dist < tmax {
                                return true;
                            }
                }
            } else {
                stack[sp] = n.first;
                stack[sp + 1] = n.first + 1;
                sp += 2;
            }
        }
        false
    }

    /// The first triangle the ray meets within `tmax`, those not blocking light passed through.
    pub fn closest(&self, o: Vec3, d: Vec3, tmax: f32) -> Option<Hit> {
        let inv = d.recip();
        let mut best: Option<Hit> = None;
        let mut limit = tmax;
        let mut stack = [0u32; 64];
        let mut sp = 1;
        while sp > 0 {
            sp -= 1;
            let n = &self.nodes[stack[sp] as usize];
            if Self::slab(n, o, inv, limit).is_none() {
                continue;
            }
            if n.count > 0 {
                for (k, t) in self.tris[n.first as usize..(n.first + n.count) as usize].iter().enumerate() {
                    if !t.blocks {
                        continue;
                    }
                    if let Some((dist, u, v)) = Self::intersect(t, o, d)
                        && dist < limit {
                            limit = dist;
                            best = Some(Hit { t: dist, tri: n.first + k as u32, u, v });
                        }
                }
            } else {
                // nearer child first
                let (a, b) = (n.first, n.first + 1);
                let ta = Self::slab(&self.nodes[a as usize], o, inv, limit);
                let tb = Self::slab(&self.nodes[b as usize], o, inv, limit);
                match (ta, tb) {
                    (Some(x), Some(y)) => {
                        let (near, far) = if x <= y { (a, b) } else { (b, a) };
                        stack[sp] = far;
                        stack[sp + 1] = near;
                        sp += 2;
                    }
                    (Some(_), None) => {
                        stack[sp] = a;
                        sp += 1;
                    }
                    (None, Some(_)) => {
                        stack[sp] = b;
                        sp += 1;
                    }
                    (None, None) => {}
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quad(z: f32, face: u32) -> [Tri; 2] {
        let a = Vec3::new(-10.0, -10.0, z);
        [
            Tri { a, e1: Vec3::new(20.0, 0.0, 0.0), e2: Vec3::new(0.0, 20.0, 0.0), face, blocks: true },
            Tri { a: Vec3::new(10.0, 10.0, z), e1: Vec3::new(-20.0, 0.0, 0.0), e2: Vec3::new(0.0, -20.0, 0.0), face, blocks: true },
        ]
    }

    #[test]
    fn the_nearer_of_two_floors_is_hit() {
        let mut tris = quad(0.0, 0).to_vec();
        tris.extend(quad(5.0, 1));
        let b = Bvh::build(tris);
        let h = b.closest(Vec3::new(1.0, 2.0, 20.0), Vec3::NEG_Z, 100.0).unwrap();
        assert!((h.t - 15.0).abs() < 1e-4);
        assert_eq!(b.tris[h.tri as usize].face, 1);
        assert!(b.occluded(Vec3::new(1.0, 2.0, 20.0), Vec3::NEG_Z, 16.0));
        assert!(!b.occluded(Vec3::new(1.0, 2.0, 20.0), Vec3::NEG_Z, 14.0));
        assert!(!b.occluded(Vec3::new(30.0, 2.0, 20.0), Vec3::NEG_Z, 100.0));
    }
}
