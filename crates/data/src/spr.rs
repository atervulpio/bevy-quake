//! A sprite (`spritegn.h`, version 1): flat pictures facing the eye, alone or in timed groups.

use crate::read::{Error, Reader};

pub struct SprPicture {
    pub origin: [i32; 2],
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

pub struct SprFrame {
    pub pictures: Vec<SprPicture>,
    pub intervals: Vec<f32>,
}

pub struct Spr {
    pub kind: i32,
    pub radius: f32,
    pub width: u32,
    pub height: u32,
    pub frames: Vec<SprFrame>,
}

impl Spr {
    pub fn parse(bytes: &[u8]) -> Result<Spr, Error> {
        let mut r = Reader::new(bytes);
        if r.take(4)? != b"IDSP" {
            return Err(Error::new("not a sprite: no IDSP at its head"));
        }
        let version = r.i32()?;
        if version != 1 {
            return Err(Error::new(format!("sprite version {version}, not 1")));
        }
        let kind = r.i32()?;
        let radius = r.f32()?;
        let width = r.i32()? as u32;
        let height = r.i32()? as u32;
        let n = r.i32()?;
        let _beam = r.f32()?;
        let _sync = r.i32()?;
        let picture = |r: &mut Reader| -> Result<SprPicture, Error> {
            let origin = [r.i32()?, r.i32()?];
            let w = r.i32()? as u32;
            let h = r.i32()? as u32;
            Ok(SprPicture { origin, width: w, height: h, pixels: r.take((w * h) as usize)?.to_vec() })
        };
        let mut frames = Vec::new();
        for _ in 0..n {
            if r.i32()? == 0 {
                frames.push(SprFrame { pictures: vec![picture(&mut r)?], intervals: vec![] });
            } else {
                let k = r.i32()? as usize;
                let intervals = (0..k).map(|_| r.f32()).collect::<Result<Vec<_>, _>>()?;
                let pictures = (0..k).map(|_| picture(&mut r)).collect::<Result<Vec<_>, _>>()?;
                frames.push(SprFrame { pictures, intervals });
            }
        }
        Ok(Spr { kind, radius, width, height, frames })
    }
}
