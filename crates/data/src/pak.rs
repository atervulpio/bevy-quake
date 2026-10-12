//! The pack file (`common.c`, `dpackheader_t`): "PACK", the directory's offset and length, and
//! entries of a 56-byte name, an offset and a length.

use crate::read::{Error, Reader};
use std::collections::HashMap;
use std::path::Path;

pub struct Pak {
    bytes: Vec<u8>,
    files: HashMap<String, (usize, usize)>,
    order: Vec<String>,
}

impl Pak {
    pub fn open(path: &Path) -> Result<Pak, Error> {
        let bytes = std::fs::read(path).map_err(|e| Error::new(format!("{}: {e}", path.display())))?;
        Pak::parse(bytes)
    }

    pub fn parse(bytes: Vec<u8>) -> Result<Pak, Error> {
        let mut r = Reader::new(&bytes);
        if r.take(4)? != b"PACK" {
            return Err(Error::new("not a pack: no PACK at its head"));
        }
        let dirofs = r.i32()? as usize;
        let dirlen = r.i32()? as usize;
        let mut files = HashMap::new();
        let mut order = Vec::new();
        let mut d = Reader::at(&bytes, dirofs);
        for _ in 0..dirlen / 64 {
            let name = d.name(56)?.to_ascii_lowercase();
            let ofs = d.i32()? as usize;
            let len = d.i32()? as usize;
            if ofs + len > bytes.len() {
                return Err(Error::new(format!("{name} runs past the end of the pack")));
            }
            order.push(name.clone());
            files.insert(name, (ofs, len));
        }
        Ok(Pak { bytes, files, order })
    }

    pub fn get(&self, name: &str) -> Option<&[u8]> {
        let (o, l) = *self.files.get(&name.to_ascii_lowercase())?;
        Some(&self.bytes[o..o + l])
    }

    pub fn names(&self) -> &[String] {
        &self.order
    }
}

/// Every pack of a game folder in the order the engine searches them: `pak0.pak`, `pak1.pak`, …,
/// a later one shadowing an earlier one.
pub struct Packs(pub Vec<Pak>);

impl Packs {
    pub fn open_folder(dir: &Path) -> Result<Packs, Error> {
        let mut packs = Vec::new();
        for i in 0.. {
            let p = dir.join(format!("pak{i}.pak"));
            if !p.exists() {
                break;
            }
            packs.push(Pak::open(&p)?);
        }
        if packs.is_empty() {
            return Err(Error::new(format!("no pak0.pak in {}", dir.display())));
        }
        Ok(Packs(packs))
    }

    pub fn get(&self, name: &str) -> Option<&[u8]> {
        self.0.iter().rev().find_map(|p| p.get(name))
    }

    pub fn need(&self, name: &str) -> Result<&[u8], Error> {
        self.get(name).ok_or_else(|| Error::new(format!("{name} is in no pack")))
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.0.iter().flat_map(|p| p.names().iter())
    }
}
