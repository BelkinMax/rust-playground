pub fn run() {
  let first_name = String::from("Max");
  let last_name = String::from("Belkin");

  fn say_full_name(first: String, last: String) -> String {
    [String::from("My name is"), first, last].join(" ")
  }

  println!("{}", say_full_name(first_name, last_name));
}