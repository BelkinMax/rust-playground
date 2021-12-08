pub fn run() {
  /// All possible colors list.
  enum Color {
    Red,
    Blue,
  }
  
  /// User provided email address.
  struct Mail {
    /// User email.
    address: String,
  }

  /// Sum two numbers.
  fn add(a: i32, b: i32) -> i32 {
    return a + b
  }

  let result: i32 = add(14, 10);

  println!("Result: {}", result);
}