use std::io::Write;

use crate::serial::{self, UserSession};

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

fn input<T>(prompt: &str) -> T where T: std::str::FromStr {
  loop {
    let x = readline(prompt).parse::<T>().ok();
    if let Some(x) = x {
      return x;
    }
    println!("\x1B[0;1;31mInvalid input!\x1B[m");
  }
}

fn change_message(user: &UserSession) {
  let message = readline("Type message: ");
  let mut ostream = user.get_ostream().unwrap();
  if message != "" {
    ostream.write(&message).unwrap();
  }
}

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

fn login() {
  let username = readline("Username: ");
  let password = readline("Password: \x1B[8m");
  println!("\x1B[m");
  
  let user = match serial::UserSession::authenticate(&username, &password) {
    Some(user) => user,
    None => {
      println!("\x1B[1;31mInvalid username or password!\x1B[m");
      return;
    }
  };

  main_menu(&user);
}

fn signup() {
  let username = readline("Username: ");
  let password = readline("Password: \x1B[8m");
  println!("\x1B[m");
  
  let user = match serial::UserSession::create(&username, &password) {
    Some(user) => user,
    None => {
      println!("\x1B[1;31mUsername already exists!\x1B[m");
      return;
    }
  };

  main_menu(&user);
}

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