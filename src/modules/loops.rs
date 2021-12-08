pub fn run () {
  let mut counter = 0;

  loop {
    if counter > 5 {
      break;
    }

    println!("Counter: {}", counter);
    counter += 1;
  }

  while counter > 0 {
    println!("Counter: {}", counter);
    counter -= 1;
  }
}