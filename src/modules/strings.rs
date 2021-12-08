pub fn run () {
  let mut hello = String::from("Hello ");
  println!("{}", hello);

  // Get length
  println!("Length: {}", hello.len());

  // Concatenate strings
  hello.push_str("world!");
  println!("{}", hello);

  // Get capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Is empty
  println!("Is empty: {}", hello.is_empty());

  // Contains "world"
  println!("Contains word 'world': {}", hello.contains("world"));

  // Replace "world" with "there"
  println!("Replace word 'world': {}", hello.replace("world", "there"));

  // Loop through strings by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push_str("Capacity!");

  println!("{}", s);
}