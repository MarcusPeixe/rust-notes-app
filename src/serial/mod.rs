//! # Serial
//! 
//! This module is used for serializing and deserializing user data. It
//! contains stream classes for reading and writing encrypted data to files.
//! This cryptography is achieved through keys, generated from user passwords.
//! It also contains a trait for serializing and deserializing custom data.
//! 
//! ## Usage
//! 
//! To read data from an user's file, use the `InputStream` class:
//! 
//! ```
//! let username: String;
//! let password: String;
//! 
//! let filename: &str = &format!("users/{}.data", username);
//! let key = Key::from(password);
//! 
//! let mut istream = InputStream::new(filename, key)
//!   .expect("Failed to open file");
//! 
//! let data: String = istream.read()
//!   .expect("Failed to read data");
//! ```
//! 
//! To write data to an user's file, use the `OutputStream` class:
//! 
//! ```
//! let username: String;
//! let password: String;
//! 
//! let filename: &str = &format!("users/{}.data", username);
//! let key = Key::from(password);
//! 
//! let mut ostream = OutputStream::new(filename, key)
//!   .expect("Failed to open file");
//! 
//! let data = String::from("Hello, world!");
//! ostream.write(&data)
//!   .expect("Failed to write data");
//! ```

#[doc(hidden)]
mod key;
pub use key::*;
#[doc(hidden)]
mod input_stream;
pub use input_stream::*;
#[doc(hidden)]
mod output_stream;
pub use output_stream::*;
#[doc(hidden)]
mod serializable;
pub use serializable::*;
