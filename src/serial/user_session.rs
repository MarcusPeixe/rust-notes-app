use std::fs::File;
use crate::serial::*;

pub struct UserSession {
  username: String,
  key: Key,
}

impl UserSession {
  pub fn authenticate(username: &str, password: &str) -> Option<UserSession> {
    let filename: &str = &format!("users/{}.data", username);

    let key = Key::from(password);
    let mut istream = InputStream::new(filename, key)?;
    let key_user = istream.read::<Key>()?;
    
    // Check if keys match (password is correct)
    if key == key_user {
      Some(UserSession {
        username: username.to_string(),
        key,
      })
    }
    else {
      None
    }
  }

  pub fn create(username: &str, password: &str) -> Option<UserSession> {
    let filename: &str = &format!("users/{}.data", username);
    
    // Check if account already exists
    let file = File::open(filename);
    if file.is_ok() {
      return None;
    }

    let key = Key::from(password);
    let mut ostream = OutputStream::new(filename, key)?;
    ostream.write(&key)?;

    Some(UserSession {
      username: username.to_string(),
      key,
    })
  }

  pub fn get_istream(&self) -> Option<InputStream> {
    let key: Key;
    let mut istream = InputStream::new(
      &format!("users/{}.data", self.username), self.key
    )?;
    key = istream.read()?;
    
    assert_eq!(key, self.key);
    Some(istream)
  }

  pub fn get_ostream(&self) -> Option<OutputStream> {
    let mut ostream = OutputStream::new(
      &format!("users/{}.data", self.username), self.key
    )?;
    ostream.write(&self.key)?;
    
    Some(ostream)
  }
}