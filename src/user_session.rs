//! # User Session
//! 
//! This module contains the UserSession struct, which is used to authenticate
//! users and create new accounts. It also handles reading and writing user
//! data from its file, with automatic encryption.
//! 
//! ## Usage
//! 
//! To log in as a user, simply call the `authenticate` function:
//! 
//! ```
//! let username: String;
//! let password: String;
//! let user = UserSession::authenticate(&username, &password);
//! ```
//! 
//! To create a new user account, call the `create` function:
//! 
//! ```
//! let username: String;
//! let password: String;
//! let user = UserSession::create(&username, &password);
//! ```

use std::fs::File;
use crate::serial::*;

/// # UserSession
/// 
/// This struct represents a user session. A user can log in or create a new
/// account, both generating a user session. It exposes streams for reading
/// and writing data to the user's file.
pub struct UserSession {
  username: String,
  key: Key,
}

impl UserSession {
  /// ## authenticate
  /// 
  /// This function authenticates a user. It takes a username and password,
  /// and returns a UserSession if the user exists and its password is correct.
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

  /// ## create
  /// 
  /// This function creates a new user account. It takes a username and
  /// password, and returns a UserSession if the account was created
  /// successfully (username was not taken).
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

  /// ## get_istream
  /// 
  /// This function returns an InputStream for reading data from the user's
  /// file.
  pub fn get_istream(&self) -> Option<InputStream> {
    let key: Key;
    let mut istream = InputStream::new(
      &format!("users/{}.data", self.username), self.key
    )?;
    key = istream.read()?;
    
    assert_eq!(key, self.key);
    Some(istream)
  }

  /// ## get_ostream
  /// 
  /// This function returns an OutputStream for writing data to the user's
  /// file.
  pub fn get_ostream(&self) -> Option<OutputStream> {
    let mut ostream = OutputStream::new(
      &format!("users/{}.data", self.username), self.key
    )?;
    ostream.write(&self.key)?;
    
    Some(ostream)
  }
}