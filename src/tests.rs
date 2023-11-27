use std::fs::remove_file;

use crate::serial::*;

#[test]
fn test_key1() {
  let key1 = Key::from("123456");
  let key2 = Key::from("123456");
  let key3 = Key::from("1234567");
  let key4 = Key::from("123457");

  assert_eq!(key1, key2);
  assert_ne!(key1, key3);
  assert_ne!(key1, key4);
  assert_ne!(key2, key3);
  assert_ne!(key2, key4);
  assert_ne!(key3, key4);
}

#[test]
fn test_stream1() {
  const PATH: &str = "users/test-stream-1.tmp";
  let key = Key::from("123456");
  let key2: Key;

  {
    let mut ostream = OutputStream::new(PATH, key).unwrap();
    ostream.write(&key).unwrap();
  }

  {
    let mut istream = InputStream::new(PATH, key).unwrap();
    key2 = istream.read().unwrap();
  }

  assert_eq!(key, key2);

  remove_file(PATH).unwrap();
}

#[test]
fn test_stream2() {
  const PATH: &str = "users/test-stream-2.tmp";
  let key1 = Key::from("123456");
  let key2 = Key::from("654321");
  let key3: Key;

  {
    let mut ostream = OutputStream::new(PATH, key1).unwrap();
    ostream.write(&key1).unwrap();
  }

  {
    let mut istream = InputStream::new(PATH, key2).unwrap();
    key3 = istream.read().unwrap();
  }

  assert_ne!(key1, key2);
  assert_ne!(key1, key3);
  assert_ne!(key2, key3);

  remove_file(PATH).unwrap();
}

#[test]
fn test_stream3() {
  const PATH: &str = "users/test-stream-3.tmp";
  let msg: String = String::from("Hello, world!");
  let msg2: String;

  {
    let mut ostream = OutputStream::new(PATH, Key::from("123456")).unwrap();
    ostream.write(&msg).unwrap();
  }

  {
    let mut istream = InputStream::new(PATH, Key::from("123456")).unwrap();
    msg2 = istream.read().unwrap();
  }
  
  assert_eq!(msg, msg2);

  remove_file(PATH).unwrap();
}

#[test]
fn test_auth1() {
  const USERNAME: &str = "test-authenticate-1";
  const PASSWORD: &str = "123456";
  
  {
    // User not created yet
    assert!(UserSession::authenticate(USERNAME, PASSWORD).is_none());
  }
  {
    // Create user
    assert!(UserSession::create(USERNAME, PASSWORD).is_some());
  }
  {
    // Assert user exists
    assert!(UserSession::authenticate(USERNAME, PASSWORD).is_some());
  }
  {
    // Can't create user twice
    assert!(UserSession::create(USERNAME, PASSWORD).is_none());
  }

  remove_file(&format!("users/{}.data", USERNAME)).unwrap();
}
