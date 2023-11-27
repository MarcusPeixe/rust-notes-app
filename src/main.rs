#[cfg(test)]
mod tests;
mod serial;
mod app;

fn main() {
  println!("Hello, world!");

  app::start();
}
