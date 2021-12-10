use std::io;

fn format_input(input: &str) -> String {
  input.trim().to_lowercase()
}

fn get_input() -> io::Result<String> {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer)?;
  Ok(format_input(&buffer).to_owned())
}

enum Commands {
  Off,
  Sleep,
  Reboot,
  Shutdown,
  Hibernate,
}

pub fn run() {
  match get_input() {
    Ok(command) => {
      
    },
    Err(err) => println!("Error: {:?}", err),
  }
}