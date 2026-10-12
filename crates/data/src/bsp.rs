//! A map, BSP version 29 (`bspfile.h`): fifteen lumps, each read into its own list.

use crate::read::{Error, Reader, cstr};
use glam::Vec3;

pub const CONTENTS_EMPTY: i32 = -1;
pub const CONTENTS_SOLID: i32 = -2;
pub const CONTENTS_WATER: i32 = -3;
pub const CONTENTS_SLIME: i32 = -4;
pub const CONTENTS_LAVA: i32 = -5;
pub const CONTENTS_SKY: i32 = -6;

/// `TEX_SPECIAL`: sky or a liquid, with no lightmap.
pub const TEX_SPECIAL: i32 = 1;

pub struct Plane {
    pub normal: Vec3,
    pub dist: f32,
    pub kind: i32,
}

/// A texture as the map carries it: its name and size, and its four mip levels of palette
/// indices. The pixels are read for what they say about the surface (its colour, its period);
/// they are never drawn.
pub struct MipTex {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

pub struct Node {
    pub plane: u32,
    /// Children: a node's number, or a leaf's `-(leaf + 1)`.
    pub children: [i32; 2],
    pub mins: [i16; 3],
    pub maxs: [i16; 3],
    pub first_face: u16,
    pub num_faces: u16,
}

pub struct ClipNode {
    pub plane: u32,
    /// A clip node's number, or a contents value when negative.
    pub children: [i32; 2],
}

pub struct TexInfo {
    pub s: [f32; 4],
    pub t: [f32; 4],
    pub miptex: i32,
    pub flags: i32,
}

pub struct Face {
    pub plane: u16,
    pub side: u16,
    pub first_edge: i32,
    pub num_edges: i16,
    pub texinfo: i16,
    pub styles: [u8; 4],
    pub light_ofs: i32,
}

pub struct Leaf {
    pub contents: i32,
    pub vis_ofs: i32,
    pub mins: [i16; 3],
    pub maxs: [i16; 3],
    pub first_mark: u16,
    pub num_marks: u16,
    pub ambient: [u8; 4],
}

pub struct Model {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub origin: Vec3,
    pub head_nodes: [i32; 4],
    pub vis_leafs: i32,
    pub first_face: i32,
    pub num_faces: i32,
}

pub struct Bsp {
    pub entities: String,
    pub planes: Vec<Plane>,
    /// `None` where the map's texture lump has an empty slot (offset -1).
    pub textures: Vec<Option<MipTex>>,
    pub vertices: Vec<Vec3>,
    pub visibility: Vec<u8>,
    pub nodes: Vec<Node>,
    pub texinfo: Vec<TexInfo>,
    pub faces: Vec<Face>,
    pub lighting: Vec<u8>,
    pub clipnodes: Vec<ClipNode>,
    pub leafs: Vec<Leaf>,
    pub marksurfaces: Vec<u16>,
    pub edges: Vec<[u16; 2]>,
    pub surfedges: Vec<i32>,
    pub models: Vec<Model>,
}

fn lump<'a>(bytes: &'a [u8], lumps: &[(usize, usize)], i: usize, size: usize) -> Result<(&'a [u8], usize), Error> {
    let (o, l) = lumps[i];
    if o + l > bytes.len() {
        return Err(Error::new(format!("lump {i} runs past the end of the map")));
    }
    if size > 0 && l % size != 0 {
        return Err(Error::new(format!("lump {i}'s length {l} is not a multiple of {size}")));
    }
    Ok((&bytes[o..o + l], l.checked_div(size).unwrap_or(l)))
}

fn list<T>(bytes: &[u8], lumps: &[(usize, usize)], i: usize, size: usize, mut f: impl FnMut(&mut Reader) -> Result<T, Error>) -> Result<Vec<T>, Error> {
    let (b, n) = lump(bytes, lumps, i, size)?;
    let mut r = Reader::new(b);
    (0..n).map(|_| f(&mut r)).collect()
}

impl Bsp {
    pub fn parse(bytes: &[u8]) -> Result<Bsp, Error> {
        let mut r = Reader::new(bytes);
        let version = r.i32()?;
        if version != 29 {
            return Err(Error::new(format!("map version {version}, not 29")));
        }
        let mut lumps = Vec::with_capacity(15);
        for _ in 0..15 {
            lumps.push((r.i32()? as usize, r.i32()? as usize));
        }
        let l = &lumps;
        let entities = cstr(lump(bytes, l, 0, 0)?.0);
        let planes = list(bytes, l, 1, 20, |r| Ok(Plane { normal: r.vec3()?, dist: r.f32()?, kind: r.i32()? }))?;
        let textures = {
            let (b, _) = lump(bytes, l, 2, 0)?;
            let mut out = Vec::new();
            if !b.is_empty() {
                let mut r = Reader::new(b);
                let n = r.i32()?;
                for _ in 0..n {
                    let ofs = r.i32()?;
                    if ofs < 0 {
                        out.push(None);
                        continue;
                    }
                    let mut m = Reader::at(b, ofs as usize);
                    let name = m.name(16)?.to_ascii_lowercase();
                    let width = m.u32()?;
                    let height = m.u32()?;
                    let o0 = m.u32()? as usize;
                    let size = (width * height) as usize;
                    let pixels = Reader::at(b, ofs as usize + o0).take(size)?.to_vec();
                    out.push(Some(MipTex { name, width, height, pixels }));
                }
            }
            out
        };
        let vertices = list(bytes, l, 3, 12, |r| r.vec3())?;
        let visibility = lump(bytes, l, 4, 0)?.0.to_vec();
        let nodes = list(bytes, l, 5, 24, |r| {
            Ok(Node {
                plane: r.i32()? as u32,
                children: [r.i16()? as i32, r.i16()? as i32],
                mins: [r.i16()?, r.i16()?, r.i16()?],
                maxs: [r.i16()?, r.i16()?, r.i16()?],
                first_face: r.u16()?,
                num_faces: r.u16()?,
            })
        })?;
        let texinfo = list(bytes, l, 6, 40, |r| {
            Ok(TexInfo {
                s: [r.f32()?, r.f32()?, r.f32()?, r.f32()?],
                t: [r.f32()?, r.f32()?, r.f32()?, r.f32()?],
                miptex: r.i32()?,
                flags: r.i32()?,
            })
        })?;
        let faces = list(bytes, l, 7, 20, |r| {
            Ok(Face {
                plane: r.u16()?,
                side: r.u16()?,
                first_edge: r.i32()?,
                num_edges: r.i16()?,
                texinfo: r.i16()?,
                styles: [r.u8()?, r.u8()?, r.u8()?, r.u8()?],
                light_ofs: r.i32()?,
            })
        })?;
        let lighting = lump(bytes, l, 8, 0)?.0.to_vec();
        let clipnodes = list(bytes, l, 9, 8, |r| Ok(ClipNode { plane: r.i32()? as u32, children: [r.i16()? as i32, r.i16()? as i32] }))?;
        let leafs = list(bytes, l, 10, 28, |r| {
            Ok(Leaf {
                contents: r.i32()?,
                vis_ofs: r.i32()?,
                mins: [r.i16()?, r.i16()?, r.i16()?],
                maxs: [r.i16()?, r.i16()?, r.i16()?],
                first_mark: r.u16()?,
                num_marks: r.u16()?,
                ambient: [r.u8()?, r.u8()?, r.u8()?, r.u8()?],
            })
        })?;
        let marksurfaces = list(bytes, l, 11, 2, |r| r.u16())?;
        let edges = list(bytes, l, 12, 4, |r| Ok([r.u16()?, r.u16()?]))?;
        let surfedges = list(bytes, l, 13, 4, |r| r.i32())?;
        let models = list(bytes, l, 14, 64, |r| {
            Ok(Model {
                mins: r.vec3()?,
                maxs: r.vec3()?,
                origin: r.vec3()?,
                head_nodes: [r.i32()?, r.i32()?, r.i32()?, r.i32()?],
                vis_leafs: r.i32()?,
                first_face: r.i32()?,
                num_faces: r.i32()?,
            })
        })?;
        // Clip nodes and nodes keep their child numbers as the file's signed shorts; a node's
        // child above 32767 cannot occur within MAX_MAP_NODES, so the sign is the file's meaning.
        Ok(Bsp { entities, planes, textures, vertices, visibility, nodes, texinfo, faces, lighting, clipnodes, leafs, marksurfaces, edges, surfedges, models })
    }

    /// The corners of a face in order, from its edges (`surfedges`: a negative number walks an
    /// edge backwards).
    pub fn face_points(&self, f: &Face) -> Vec<Vec3> {
        (0..f.num_edges as i32)
            .map(|i| {
                let se = self.surfedges[(f.first_edge + i) as usize];
                let v = if se >= 0 { self.edges[se as usize][0] } else { self.edges[(-se) as usize][1] };
                self.vertices[v as usize]
            })
            .collect()
    }

    /// The leafs a leaf can see (`Mod_DecompressVis`): a run-length code where a nought byte is
    /// followed by the count of nought bytes it stands for. Bit `i` is leaf `i + 1`.
    pub fn leaf_sees(&self, leaf: usize) -> Vec<u8> {
        let row = (self.models[0].vis_leafs as usize).div_ceil(8);
        let ofs = self.leafs[leaf].vis_ofs;
        if ofs < 0 || self.visibility.is_empty() {
            return vec![0xff; row];
        }
        let mut out = Vec::with_capacity(row);
        let mut i = ofs as usize;
        while out.len() < row && i < self.visibility.len() {
            let b = self.visibility[i];
            i += 1;
            if b != 0 {
                out.push(b);
            } else {
                let n = self.visibility.get(i).copied().unwrap_or(0) as usize;
                i += 1;
                out.extend(std::iter::repeat_n(0, n));
            }
        }
        out.resize(row, 0);
        out
    }
}
