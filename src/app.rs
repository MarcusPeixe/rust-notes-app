use std::io::Write;

use crate::serial::{self, UserSession};

fn readline() -> String {
  let mut line = String::new();
  loop {
    let res = std::io::stdin().read_line(&mut line).ok();
    if res.is_some() {
      return line.trim().to_string();
    }
    println!("\x1B[1;31mInvalid input!\x1B[m");
  }
}

fn scan<T>() -> T where T: std::str::FromStr {
  loop {
    let x: Option<T> = readline().parse().ok();
    if let Some(x) = x {
      return x;
    }
    println!("\x1B[1;31mInvalid input!\x1B[m");
  }
}

fn change_message(user: &UserSession) {
  print!("Type message: ");
  std::io::stdout().flush().unwrap();
  let message = readline();
  let mut ostream = user.ostream().unwrap();
  if message.len() > 0 {
    ostream.write(&message).unwrap();
  }
}

fn main_menu(user: &UserSession) {
  loop {
    let msg = user.istream().unwrap().read::<String>();
    print!(
      "\n\n\n=== MENU ===\n\
      \n\
      {}\n\
      \n\
      1. Change message\n\
      0. Logout\n\
      \n\
      Option: ",
      match msg {
        Some(msg) => format!("Message: '{}'", msg),
        None => "* No message *".to_string(),
      }
    );
    std::io::stdout().flush().unwrap();
    let option: i32 = scan();
    match option {
      1 => change_message(user),
      0 => break,
      _ => println!("\x1B[1;31mInvalid option!\x1B[m"),
    };
  }
}

fn login() {
  print!("Username: ");
  std::io::stdout().flush().unwrap();
  let username = readline();
  print!("Password: \x1B[8m");
  std::io::stdout().flush().unwrap();
  let password = readline();
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
  print!("Username: ");
  std::io::stdout().flush().unwrap();
  let username = readline();
  print!("Password: \x1B[8m");
  std::io::stdout().flush().unwrap();
  let password = readline();
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
      1. Login\n\
      2. Signup\n\
      0. Exit\n\
      \n\
      Option: "
    );
    std::io::stdout().flush().unwrap();
    let option: i32 = scan();
    match option {
      1 => login(),
      2 => signup(),
      0 => break,
      _ => println!("\x1B[1;31mInvalid option!\x1B[m"),
    };
  }
}