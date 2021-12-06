pub fn run() {
  enum Color {
    Red,
    Green,
    Blue
  }

  fn get_hex_color(color: Color) {
    match color {
      Color::Red => println!("#FF0000"),
      Color::Green => println!("#00FF00"),
      Color::Blue => println!("#0000FF")
    }
  }

  get_hex_color(Color::Red);
  get_hex_color(Color::Green);
  get_hex_color(Color::Blue);
}