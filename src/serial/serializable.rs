use crate::serial::*;

/// # Serializable
/// 
/// This trait is used to serialize and deserialize data. It is used with the
/// InputStream and OutputStream structs.
pub trait Serializable {
  /// ## serialize
  /// 
  /// This function serializes the data and writes it to the OutputStream.
  fn serialize(&self, ostream: &mut OutputStream) -> Option<()>;

  /// ## deserialize
  /// 
  /// This function deserializes the data and reads it from the InputStream.
  fn deserialize(istream: &mut InputStream) -> Option<Self>
  where Self: Sized;
}

impl Serializable for String {
  fn serialize(&self, ostream: &mut OutputStream) -> Option<()> {
    if write_stream!(ostream, self.len(), usize) !=
      std::mem::size_of::<usize>() { return None; }
    for i in 0..self.len() {
      if write_stream!(ostream, self.as_bytes()[i], u8) !=
        std::mem::size_of::<u8>() { return None; }
    }
    Some(())
  }

  fn deserialize(istream: &mut InputStream) -> Option<String> {
    let len: usize;
    if read_stream!(istream, len, usize) !=
      std::mem::size_of::<usize>() { return None; }
    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    for _ in 0..len {
      let byte: u8;
      if read_stream!(istream, byte, u8) !=
        std::mem::size_of::<u8>() { return None; }
      bytes.push(byte);
    }
    Some(String::from_utf8(bytes).unwrap())
  }
}