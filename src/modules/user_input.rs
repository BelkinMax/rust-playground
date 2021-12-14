use std::io::stdin;

enum Commands {
  Off,
  Sleep,
  Reboot,
  Shutdown,
  Hibernate,
}

impl Commands {
  fn new(state: &str) -> Option<Self> {
    // to_lowercase creates owned string
    let formatted_state = state
      .trim()
      .to_lowercase();
    
    match formatted_state.as_str() {
      "off" => Some(Commands::Off),
      "sleep" => Some(Commands::Sleep),
      "reboot" => Some(Commands::Reboot),
      "shutdown" => Some(Commands::Shutdown),
      "hibernate" => Some(Commands::Hibernate),
      _ => None,
    }
  }
}

fn print_command_action(state: Commands) {
  use Commands::*;

  match state {
    Off => println!("Turning off"),
    Sleep => println!("Sleeping"),
    Reboot => println!("Rebooting"),
    Shutdown => println!("Shutting down"),
    Hibernate => println!("Hibernating"),
  }
}

pub fn run() {
  let mut buffer = String::new();
  let user_input = stdin().read_line(&mut buffer);

  if user_input.is_ok() {
    match Commands::new(&buffer) {
      Some(state) => print_command_action(state),
      None => println!("Invalid command")
    }
  } else {
    println!("Error on reading input");
  }
}