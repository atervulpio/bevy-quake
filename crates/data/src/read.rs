use std::fmt;

/// What went wrong reading a file, in words.
#[derive(Debug, Clone)]
pub struct Error(pub String);

impl Error {
    pub fn new(what: impl Into<String>) -> Error {
        Error(what.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// A cursor over little-endian bytes that refuses to read past the end.
pub struct Reader<'a> {
    pub bytes: &'a [u8],
    pub at: usize,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Reader { bytes, at: 0 }
    }
    pub fn at(bytes: &'a [u8], at: usize) -> Self {
        Reader { bytes, at }
    }
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.bytes.len()).ok_or_else(|| {
            Error::new(format!("read of {n} bytes at {} past the end ({})", self.at, self.bytes.len()))
        })?;
        let s = &self.bytes[self.at..end];
        self.at = end;
        Ok(s)
    }
    pub fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }
    pub fn i16(&mut self) -> Result<i16, Error> {
        Ok(i16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    pub fn u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    pub fn i32(&mut self) -> Result<i32, Error> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn vec3(&mut self) -> Result<glam::Vec3, Error> {
        Ok(glam::Vec3::new(self.f32()?, self.f32()?, self.f32()?))
    }
    /// A fixed-size name field, cut at its first nought.
    pub fn name(&mut self, n: usize) -> Result<String, Error> {
        Ok(cstr(self.take(n)?))
    }
}

pub fn cstr(b: &[u8]) -> String {
    let end = b.iter().position(|&c| c == 0).unwrap_or(b.len());
    String::from_utf8_lossy(&b[..end]).into_owned()
}
