//! A model (`modelgen.h`, version 6): skins of palette indices, texture coordinates, triangles,
//! and frames of vertices packed in bytes, each frame alone or in a timed group.

use crate::read::{Error, Reader};
use glam::Vec3;

pub struct Skin {
    /// One picture, or the pictures of a timed group with the time each ends at.
    pub pictures: Vec<Vec<u8>>,
    pub intervals: Vec<f32>,
}

pub struct StVert {
    pub on_seam: bool,
    pub s: i32,
    pub t: i32,
}

pub struct Triangle {
    pub faces_front: bool,
    pub verts: [u32; 3],
}

pub struct Pose {
    pub name: String,
    /// The vertices unpacked into model space: `byte * scale + origin`.
    pub verts: Vec<Vec3>,
    /// Each vertex's normal, an index into the table of 162 (`anorms.h`).
    pub normals: Vec<u8>,
}

pub struct Frame {
    /// One pose, or a group's poses with the time each ends at.
    pub poses: Vec<Pose>,
    pub intervals: Vec<f32>,
}

pub struct Mdl {
    pub scale: Vec3,
    pub origin: Vec3,
    pub radius: f32,
    pub eye: Vec3,
    pub skin_width: u32,
    pub skin_height: u32,
    pub skins: Vec<Skin>,
    pub st: Vec<StVert>,
    pub triangles: Vec<Triangle>,
    pub frames: Vec<Frame>,
    /// Random start in a frame group (`ST_RAND`), not in step with the others.
    pub sync_random: bool,
    /// The trail and rotation flags (`EF_ROCKET` and the rest).
    pub flags: i32,
}

impl Mdl {
    pub fn parse(bytes: &[u8]) -> Result<Mdl, Error> {
        let mut r = Reader::new(bytes);
        if r.take(4)? != b"IDPO" {
            return Err(Error::new("not a model: no IDPO at its head"));
        }
        let version = r.i32()?;
        if version != 6 {
            return Err(Error::new(format!("model version {version}, not 6")));
        }
        let scale = r.vec3()?;
        let origin = r.vec3()?;
        let radius = r.f32()?;
        let eye = r.vec3()?;
        let num_skins = r.i32()?;
        let skin_width = r.i32()? as u32;
        let skin_height = r.i32()? as u32;
        let num_verts = r.i32()? as usize;
        let num_tris = r.i32()? as usize;
        let num_frames = r.i32()?;
        let sync_random = r.i32()? == 1;
        let flags = r.i32()?;
        let _size = r.f32()?;
        let pic = (skin_width * skin_height) as usize;
        let mut skins = Vec::new();
        for _ in 0..num_skins {
            if r.i32()? == 0 {
                skins.push(Skin { pictures: vec![r.take(pic)?.to_vec()], intervals: vec![] });
            } else {
                let n = r.i32()? as usize;
                let intervals = (0..n).map(|_| r.f32()).collect::<Result<Vec<_>, _>>()?;
                let pictures = (0..n).map(|_| Ok(r.take(pic)?.to_vec())).collect::<Result<Vec<_>, Error>>()?;
                skins.push(Skin { pictures, intervals });
            }
        }
        let st = (0..num_verts)
            .map(|_| Ok(StVert { on_seam: r.i32()? != 0, s: r.i32()?, t: r.i32()? }))
            .collect::<Result<Vec<_>, Error>>()?;
        let triangles = (0..num_tris)
            .map(|_| Ok(Triangle { faces_front: r.i32()? != 0, verts: [r.i32()? as u32, r.i32()? as u32, r.i32()? as u32] }))
            .collect::<Result<Vec<_>, Error>>()?;
        let pose = |r: &mut Reader| -> Result<Pose, Error> {
            r.take(8)?; // the frame's box, which the vertices themselves give
            let name = r.name(16)?;
            let mut verts = Vec::with_capacity(num_verts);
            let mut normals = Vec::with_capacity(num_verts);
            for _ in 0..num_verts {
                let b = r.take(4)?;
                verts.push(Vec3::new(b[0] as f32, b[1] as f32, b[2] as f32) * scale + origin);
                normals.push(b[3]);
            }
            Ok(Pose { name, verts, normals })
        };
        let mut frames = Vec::new();
        for _ in 0..num_frames {
            if r.i32()? == 0 {
                frames.push(Frame { poses: vec![pose(&mut r)?], intervals: vec![] });
            } else {
                let n = r.i32()? as usize;
                r.take(8)?;
                let intervals = (0..n).map(|_| r.f32()).collect::<Result<Vec<_>, _>>()?;
                let poses = (0..n).map(|_| pose(&mut r)).collect::<Result<Vec<_>, _>>()?;
                frames.push(Frame { poses, intervals });
            }
        }
        Ok(Mdl { scale, origin, radius, eye, skin_width, skin_height, skins, st, triangles, frames, sync_random, flags })
    }
}
