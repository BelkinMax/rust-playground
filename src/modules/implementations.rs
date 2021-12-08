pub fn run() {
  enum Color {
    Brown,
    Red
  }

  impl Color {
    fn print(&self) {
      match self {
        Color::Brown => println!("Color: {}", "Brown"),
        Color::Red => println!("Color: {}", "Red")
      }
    }
  }

  struct Dimensions {
    height: i32,
    width: i32,
    depth: i32
  }

  impl Dimensions {
    fn print(&self) {
      println!("Height: {}", &self.height);
      println!("Width: {}", &self.width);
      println!("Depth: {}", &self.depth);
    }
  }

  struct Box {
    name: String,
    dimensions: Dimensions,
    color: Color,
    price: f64
  }

  impl Box {
    fn new(name: String, dimensions: Dimensions, color: Color, price: f64) -> Self {
      return Self {
        name,
        dimensions,
        color,
        price
      };
    }

    fn print(&self) {
      println!("Name: {}", self.name);
      println!("Price: {}", self.price);
      self.dimensions.print();
      self.color.print();
    }
  }

  let small_dimensions = Dimensions {
    height: 2,
    width: 3,
    depth: 3
  };

  let small_box = Box::new(
    String::from("Small Box"),
    small_dimensions,
    Color::Brown,
    12.99
  );

  small_box.print();
}