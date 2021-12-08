pub fn run() {
  // Print to console
  println!("Hello from print.rs file.");

  // Formatted print
  println!("Hello! My name is {}. I am {} years old.", "Max", 26);

  // Positional arguments
  println!("{0} is from {1}. {0} has a {2}.", "Max", "Russia", "guitar");

  // Named arguments
  println!(
    "{name} is from {country}. {name} has a {object}.",
    name = "Max",
    country = "Russia",
    object = "guitar"
  );

  // Placeholder traits
  println!("Number {}, Binary: {:b}, Hex: {:x}, Octal {:o}.", 10, 10, 10, 10);

  // Placeholder for debug traits
  println!("{:?}", (12, true, "Hello"));
}
