//! The picture pack, WAD2 (`wad.h`): named lumps, each a picture, a palette or a texture.

use crate::read::{Error, Reader};
use std::collections::HashMap;

pub struct Wad {
    pub lumps: HashMap<String, (u8, Vec<u8>)>,
}

/// A picture: its size and palette indices (`qpic_t`).
pub struct Pic {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Pic {
    pub fn parse(bytes: &[u8]) -> Result<Pic, Error> {
        let mut r = Reader::new(bytes);
        let width = r.i32()? as u32;
        let height = r.i32()? as u32;
        Ok(Pic { width, height, pixels: r.take((width * height) as usize)?.to_vec() })
    }
}

impl Wad {
    pub fn parse(bytes: &[u8]) -> Result<Wad, Error> {
        let mut r = Reader::new(bytes);
        if r.take(4)? != b"WAD2" {
            return Err(Error::new("not a WAD2"));
        }
        let n = r.i32()?;
        let table = r.i32()? as usize;
        let mut t = Reader::at(bytes, table);
        let mut lumps = HashMap::new();
        for _ in 0..n {
            let pos = t.i32()? as usize;
            let disk = t.i32()? as usize;
            let _size = t.i32()?;
            let kind = t.u8()?;
            let compression = t.u8()?;
            t.take(2)?;
            let name = t.name(16)?.to_ascii_lowercase();
            if compression != 0 {
                return Err(Error::new(format!("{name} is compressed, which no shipped WAD is")));
            }
            lumps.insert(name, (kind, Reader::at(bytes, pos).take(disk)?.to_vec()));
        }
        Ok(Wad { lumps })
    }
}
