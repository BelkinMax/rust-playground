pub fn run() {
  let coordinates = (2, 3);
  println!("{}", coordinates.0);

  let (x, y) = coordinates;
  println!("X: {}, Y: {}", x, y);
}