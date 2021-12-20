pub fn run() {
  // Closures
  let add = |a: i32, b: i32| -> i32 { a + b };
  let short_add = |a, b| a + b;

  let are_equal = short_add(12, 52) == add(12, 52);
  println!("Sum are equal: {:?}", are_equal);

  // Map combinator
  
}