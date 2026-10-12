//! A sound (`snd_mem.c`, `GetWavinfo`): PCM in a RIFF/WAVE file, with its loop's start from a
//! `cue ` chunk and its length from a `LIST`'s `mark` when one follows.

use crate::read::{Error, Reader};

pub struct Wav {
    pub rate: u32,
    /// The samples as numbers from -1 to 1, one channel (the game's sounds are all mono).
    pub samples: Vec<f32>,
    /// The bytes per sample the file held: 1 for the game's 8-bit sounds.
    pub width: u32,
    pub loop_start: Option<usize>,
}

fn find<'a>(bytes: &'a [u8], from: usize, id: &[u8; 4]) -> Option<(usize, &'a [u8])> {
    let mut at = from;
    while at + 8 <= bytes.len() {
        let len = u32::from_le_bytes(bytes[at + 4..at + 8].try_into().unwrap()) as usize;
        if &bytes[at..at + 4] == id {
            let end = (at + 8 + len).min(bytes.len());
            return Some((at, &bytes[at + 8..end]));
        }
        at += 8 + ((len + 1) & !1);
    }
    None
}

impl Wav {
    pub fn parse(bytes: &[u8]) -> Result<Wav, Error> {
        if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return Err(Error::new("missing RIFF/WAVE chunks"));
        }
        let (_, fmt) = find(bytes, 12, b"fmt ").ok_or_else(|| Error::new("missing fmt chunk"))?;
        let mut r = Reader::new(fmt);
        if r.i16()? != 1 {
            return Err(Error::new("not PCM"));
        }
        let channels = r.i16()? as usize;
        let rate = r.u32()?;
        r.take(6)?;
        let width = (r.i16()? / 8) as u32;
        let mut loop_start = None;
        let mut loop_end = None;
        if let Some((at, cue)) = find(bytes, 12, b"cue ") {
            loop_start = Some(Reader::at(cue, 24).u32()? as usize);
            let next = at + 8 + cue.len();
            if let Some((_, list)) = find(bytes, next, b"LIST")
                && list.len() >= 24 && &list[20..24] == b"mark" {
                    loop_end = Some(loop_start.unwrap() + Reader::at(list, 16).u32()? as usize);
                }
        }
        let (_, data) = find(bytes, 12, b"data").ok_or_else(|| Error::new("missing data chunk"))?;
        let frame = width as usize * channels;
        let mut count = data.len() / frame;
        if let Some(e) = loop_end {
            if e > count {
                return Err(Error::new("loop runs past the sound's end"));
            }
            count = e;
        }
        let samples = (0..count)
            .map(|i| {
                let at = i * frame;
                match width {
                    1 => (data[at] as f32 - 128.0) / 128.0,
                    _ => i16::from_le_bytes([data[at], data[at + 1]]) as f32 / 32768.0,
                }
            })
            .collect();
        Ok(Wav { rate, samples, width, loop_start })
    }
}
