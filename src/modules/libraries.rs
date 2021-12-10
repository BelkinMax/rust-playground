pub fn run() {
  let message = String::from("Hello World!");
  let uppercase_message = message.to_uppercase();
  let lowercase_message = message.to_lowercase();

  println!("{}", uppercase_message);
  println!("{}", lowercase_message);

  let upper_contains_text = uppercase_message.contains("HELLO");
  let lower_contains_text = lowercase_message.contains("HELLO");

  println!("Uppercase contains HELLO: {}", upper_contains_text);
  println!("Lowercase contains HELLO: {}", lower_contains_text);
}