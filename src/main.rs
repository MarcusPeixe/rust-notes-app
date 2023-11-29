//! # Rust Notes App
//! 
//! This repo is just a little experiment, where I try to learn and improve my
//! Rust. I also try to make the best out of cargo, learning all its options
//! and functionalities in the way.
//! 
//! ## Usage
//! 
//! + `cargo r` or `cargo run` runs the project.
//! + `cargo t` or `cargo test` runs unit tests.
//! + `cargo d` or `cargo doc` builds the docs.

#![warn(missing_docs)]

#[cfg(test)]
mod tests;
pub mod serial;
mod user_session;
mod app;

#[doc(hidden)]
fn main() {
  app::start();
}
