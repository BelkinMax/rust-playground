pub fn run() {
  // Vectors can hold only the same type variables

  let mut my_numbers = Vec::new();
  my_numbers.push(1);
  my_numbers.push(2);
  my_numbers.push(3);
  
  for num in &my_numbers {
    println!("Number is: {}", num);
  }

  println!("Length is: {}", my_numbers.len());
}