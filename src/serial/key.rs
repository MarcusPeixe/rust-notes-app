use crate::serial::*;

const INITIAL: u64 = 0x3D9CA05D5422EDE9;
const SCRAMBLE: u64 = 0xECD96C09EEFC5F6D;
const MULT: u64 = 709;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Key {
  pub(crate) value: u64,
}

impl Key {
  pub fn from(password: &str) -> Key {
    let mut value: u64 = INITIAL;
    for c in password.bytes() {
      value = (value.wrapping_mul(MULT)).wrapping_add(
        (c as u64).wrapping_mul(SCRAMBLE)
      );
    }
    Key { value }
  }
}

impl Serializable for Key {
  fn serialize(&self, ostream: &mut OutputStream) -> Option<()> {
    if write_stream!(ostream, self.value, u64) == std::mem::size_of::<u64>() {
      Some(())
    }
    else {
      None
    }
  }

  fn deserialize(istream: &mut InputStream) -> Option<Key> {
    let mut key: Key = Key { value: 0 };
    if read_stream!(istream, key.value, u64) == std::mem::size_of::<u64>() {
      Some(key)
    }
    else {
      None
    }
  }
}