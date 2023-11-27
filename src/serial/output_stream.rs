use std::{fs::File, io::Write};

use crate::serial::*;

const SCRAMBLE: u64 = 0xECD96C09EEFC5F6D;
const MULT: u64 = 709;

pub struct OutputStream {
  file: File,
  key: Key,
}

impl OutputStream {
  pub fn new(filename: &str, key: Key) -> Option<OutputStream> {
    let file = File::create(filename).ok()?;
    Some(OutputStream { file, key })
  }

  pub fn write_raw(&mut self, input: &[u8]) -> usize {
    let mut buffer = Vec::from(input);
    for i in 0..input.len() {
      self.key.value = self.key.value.wrapping_mul(MULT);

      let idx = (i & 7) << 3;
      let x: u8 = ((self.key.value >> idx) & 0xFF) as u8;

      buffer[i] = buffer[i].wrapping_add(x);
      self.key.value = self.key.value.wrapping_add(
        (buffer[i] as u64).wrapping_mul(SCRAMBLE)
      );
    }
    self.file.write(&buffer).unwrap_or(0)
  }

  pub fn write<T: Serializable>(&mut self, obj: &T) -> Option<()> {
    obj.serialize(self)
  }
}

macro_rules! write_stream {
  ($istream:expr, $obj:expr, $typ:ty) => {{
    let buffer = unsafe {
      std::mem::transmute::<_, [u8; std::mem::size_of::<$typ>()]>($obj)
    };
    $istream.write_raw(&buffer)
  }};
}

pub(crate) use write_stream;