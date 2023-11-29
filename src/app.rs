//! # App module
//! 
//! This module contains the main application logic. It is responsible for
//! displaying the menu and handling user input. The menu state is represented
//! by the call stack itself.
//! 
//! The `start` function is the entry point of the application. It displays the
//! login screen and handles user input Each subsequent screen is a new
//! function call on the stack, allowing for easy navigation between screens
//! and managing the context.
//! 
//! ## Design
//! 
//! Most functions are of the form:
//! 
//! ```
//! pub fn menu_name() {
//!   loop {
//!     println!("MENU TEXT AND LIST OF OPTIONS");
//! 
//!     let option: i32 = input("Option: ");
//!     match option {
//!       1 => option1(),
//!       2 => option2(),
//!       0 => break,
//!       _ => println!("Invalid option!"),
//!     };
//!   }
//! }
//! ```
//! 
//! This design makes it extremely easy to add new screens and options to the
//! application.

use std::io::Write;

use crate::user_session::*;

/// ## readline
/// 
/// Helper function for reading a line from stdin. It takes a prompt and
/// attempts to read a line from stdin. If the input is invalid, it prints
/// an error message and tries again.
fn readline(prompt: &str) -> String {
  let mut line = String::new();
  loop {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let res = std::io::stdin().read_line(&mut line).ok();
    if res.is_some() {
      return line.trim().to_string();
    }
    println!("\x1B[0;1;31mInvalid input!\x1B[m");
  }
}

/// ## input
/// 
/// Helper function for reading a value from stdin. It takes a prompt and
/// attempts to read and parse a value from stdin. If the input is invalid,
/// it prints an error message and tries again.
fn input<T>(prompt: &str) -> T where T: std::str::FromStr {
  loop {
    let x = readline(prompt).parse::<T>().ok();
    if let Some(x) = x {
      return x;
    }
    println!("\x1B[0;1;31mInvalid input!\x1B[m");
  }
}

#[doc(hidden)]
fn change_message(user: &UserSession) {
  let message = readline("Type message: ");
  let mut ostream = user.get_ostream().unwrap();
  if message != "" {
    ostream.write(&message).unwrap();
  }
}

#[doc(hidden)]
fn main_menu(user: &UserSession) {
  loop {
    let msg = match user.get_istream().unwrap().read::<String>() {
      Some(text) => format!("Message: '{}'", text),
      None => String::from("* No message *"),
    };
    print!(
      "\n\n\n=== MENU ===\n\
      \n\
      {}\n\
      \n\
      1. Change message\n\
      0. Logout\n\
      \n",
      msg
    );
    let option: i32 = input("Option: ");
    match option {
      1 => change_message(user),
      0 => break,
      _ => println!("\x1B[1;31mInvalid option!\x1B[m"),
    };
  }
}

#[doc(hidden)]
fn login() {
  let username = readline("Username: ");
  let password = readline("Password: \x1B[8m");
  println!("\x1B[m");
  
  let user = match UserSession::authenticate(&username, &password) {
    Some(user) => user,
    None => {
      println!("\x1B[1;31mInvalid username or password!\x1B[m");
      return;
    }
  };

  main_menu(&user);
}

#[doc(hidden)]
fn signup() {
  let username = readline("Username: ");
  let password = readline("Password: \x1B[8m");
  println!("\x1B[m");
  
  let user = match UserSession::create(&username, &password) {
    Some(user) => user,
    None => {
      println!("\x1B[1;31mUsername already exists!\x1B[m");
      return;
    }
  };

  main_menu(&user);
}

/// ## start
/// 
/// This function is the entry point of the application.
pub fn start() {
  loop {
    print!(
      "\n\n\n=== MENU ===\n\
      \n\
      1. Log in\n\
      2. Sign up\n\
      0. Exit\n\
      \n"
    );
    let option: i32 = input("Option: ");
    match option {
      1 => login(),
      2 => signup(),
      0 => break,
      _ => println!("\x1B[1;31mInvalid option!\x1B[m"),
    };
  }
}