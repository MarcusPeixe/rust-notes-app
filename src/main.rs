//! # Rust Notes App
//! 
//! This repo is just a little experiment, where I try to learn and improve my
//! Rust.
//! 
//! ## Usage
//! 
//! + `cargo r` or `cargo run` runs the project.
//! + `cargo t` or `cargo test` runs unit tests.

#![warn(missing_docs)]

#[cfg(test)]
mod tests;
pub mod serial;
pub mod user_session;
pub mod app;

#[doc(hidden)]
fn main() {
  app::start();
}
