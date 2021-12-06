use rand::Rng;

pub fn run() {
  let mut rng = rand::thread_rng();
  let n1: i32 = rng.gen_range(0..10);
  let n2: i32 = rng.gen_range(0..10);

  fn subtract (a: i32, b: i32) -> i32 {
    return a - b;
  }

  let subtract_result = subtract(n1, n2);

  if subtract_result == 0 {
    println!("Zero result: {}", subtract_result);
  } else if subtract_result > 0 {
    println!("Positive result: {}", subtract_result);
  } else {
    println!("Negative result: {}", subtract_result);
  }

  // Loops
  let mut counter = 1;

  println!("Loop");
  loop {
    println!("{}", counter);

    if counter >= 5 {
      break;
    }

    counter = counter + 1;
  }

  println!("While");
  while counter > 0 {
    println!("{}", counter);
    counter = counter - 1;
  }
} 