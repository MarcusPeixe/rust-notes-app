use std::{fs::File, io::Read};

use crate::serial::*;

#[doc(hidden)]
const SCRAMBLE: u64 = 0xECD96C09EEFC5F6D;
#[doc(hidden)]
const MULT: u64 = 709;

/// # InputStream
/// 
/// This struct is used to read data from a file. The data read is
/// automatically decrypted using the key.
pub struct InputStream {
  file: File,
  key: Key,
}

impl InputStream {
  /// ## new
  /// 
  /// This function creates a new InputStream from a filename and a key.
  pub fn new(filename: &str, key: Key) -> Option<InputStream> {
    let file = File::open(filename).ok()?;
    Some(InputStream { file, key })
  }

  /// ## read_raw
  /// 
  /// This function reads raw data from the file and decrypts it.
  pub(super) fn read_raw(&mut self, output: &mut [u8]) -> usize {
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

  /// ## read
  /// 
  /// This function reads data from the file and decrypts it. It is templated,
  /// and the type of the data to be read must implement the Serializable
  /// trait.
  pub fn read<T: Serializable>(&mut self) -> Option<T> {
    T::deserialize(self)
  }
}

#[doc(hidden)]
macro_rules! read_stream {
  ($istream:expr, $obj:expr, $typ:ty) => {{
    let mut buffer = [0; std::mem::size_of::<$typ>()];
    let size = $istream.read_raw(&mut buffer);
    $obj = unsafe { std::mem::transmute(buffer) };
    size
  }};
}

pub(super) use read_stream;
