use std::{fs::File, io::Write};

use crate::serial::*;

#[doc(hidden)]
const SCRAMBLE: u64 = 0xECD96C09EEFC5F6D;
#[doc(hidden)]
const MULT: u64 = 709;

/// # OutputStream
/// 
/// This struct is used to write data to a file. The data written is
/// automatically encrypted using the key.
pub struct OutputStream {
  file: File,
  key: Key,
}

impl OutputStream {
  /// ## new
  /// 
  /// This function creates a new OutputStream from a filename and a key.
  pub fn new(filename: &str, key: Key) -> Option<OutputStream> {
    let file = File::create(filename).ok()?;
    Some(OutputStream { file, key })
  }

  /// ## write_raw
  /// 
  /// This function encrypts the raw data and writes it to the file.
  pub(super) fn write_raw(&mut self, input: &[u8]) -> usize {
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

  /// ## write
  /// 
  /// This function encrypts the data and writes it to the file. It is
  /// templated, and the type of the data to be written must implement the
  /// Serializable trait.
  pub fn write<T: Serializable>(&mut self, obj: &T) -> Option<()> {
    obj.serialize(self)
  }
}

#[doc(hidden)]
macro_rules! write_stream {
  ($istream:expr, $obj:expr, $typ:ty) => {{
    let buffer = unsafe {
      std::mem::transmute::<_, [u8; std::mem::size_of::<$typ>()]>($obj)
    };
    $istream.write_raw(&buffer)
  }};
}

pub(super) use write_stream;