pub fn run() {
  enum Flavour {
    Sweet,
    Acid,
    Bitter
  }

  struct Drink {
    name: String,
    fluid_oz: f64,
    flavour: Flavour
  }

  fn print_drink(drink: Drink) {
    println!("Name: {}", drink.name);
    println!("Fluid OZ: {}", drink.fluid_oz);

    match drink.flavour {
      Flavour::Sweet => println!("Flavour: {}", "Sweet"),
      Flavour::Acid => println!("Flavour: {}", "Acid"),
      Flavour::Bitter => println!("Flavour: {}", "Bitter"),
    }
  }

  let martini_cocktail = Drink {
    name: String::from("Martini"),
    fluid_oz: 2.2,
    flavour: Flavour::Sweet,
  };

  let daikiri_cocktail = Drink {
    name: String::from("Daikiri"),
    fluid_oz: 4.0,
    flavour: Flavour::Acid,
  };

  let old_fashioned_cocktail = Drink {
    name: String::from("Old Fashioned"),
    fluid_oz: 3.1,
    flavour: Flavour::Bitter,
  };

  print_drink(martini_cocktail);
  print_drink(daikiri_cocktail);
  print_drink(old_fashioned_cocktail);
}