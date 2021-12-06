pub fn run() {
  enum Light {
    Bright,
    Dull
  }

  fn display_light(light: &Light) {
    match light {
      Light::Bright => println!("Bright"),
      Light::Dull => println!("Dull"),
    }
  }

  fn hello_light(light: &Light) {
    match light {
      Light::Bright => println!("Hello Bright"),
      Light::Dull => println!("Hello Dull"),
    }
  }

  let dull = Light::Dull;
  let bright = Light::Bright;

  display_light(&dull);
  hello_light(&dull);
  hello_light(&bright);
}