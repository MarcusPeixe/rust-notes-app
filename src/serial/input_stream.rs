use std::{fs::File, io::Read};

use crate::serial::*;

const SCRAMBLE: u64 = 0xECD96C09EEFC5F6D;
const MULT: u64 = 709;

pub struct InputStream {
  file: File,
  key: Key,
}

impl InputStream {
  pub fn new(filename: &str, key: Key) -> Option<InputStream> {
    let file = File::open(filename).ok()?;
    Some(InputStream { file, key })
  }

  pub fn read_raw(&mut self, output: &mut [u8]) -> usize {
    let bytes_read = self.file.read(output).unwrap_or(0);
    for i in 0..bytes_read {
      self.key.value = self.key.value.wrapping_mul(MULT);

      let idx = (i & 7) << 3;
      let x: u8 = ((self.key.value >> idx) & 0xFF) as u8;

      self.key.value = self.key.value.wrapping_add(
        (output[i] as u64).wrapping_mul(SCRAMBLE)
      );
      output[i] = output[i].wrapping_sub(x);
    }
    bytes_read
  }

  pub fn read<T: Serializable>(&mut self) -> Option<T> {
    T::deserialize(self)
  }
}

macro_rules! read_stream {
  ($istream:expr, $obj:expr, $typ:ty) => {{
    let mut buffer = [0; std::mem::size_of::<$typ>()];
    let size = $istream.read_raw(&mut buffer);
    $obj = unsafe { std::mem::transmute(buffer) };
    size
  }};
}

pub(crate) use read_stream;

use super::Serializable;