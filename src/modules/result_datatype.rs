pub fn run() {
  #[derive(Debug)]
  enum MenuChoice {
    MainMenu,
    Start,
    Quit
  }

  fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
      "main menu" => Ok(MenuChoice::MainMenu),
      "start" => Ok(MenuChoice::Start),
      "quit" => Ok(MenuChoice::Quit),
      _ => Err("Option not found".to_owned()),
    }
  }

  let choice = get_choice("main menu");
  println!("Choice: {:?}", choice);

  // Demo activity
  struct Customer {
    age: i32
  }

  fn can_purchase(customer: &Customer) -> Result<bool, bool> {
    if customer.age < 21 {
      return Err(false);
    } else {
      return Ok(true);
    }
  }

  let customer = Customer {
    age: 20
  };

  let is_purchased = can_purchase(&customer);

  println!("Is purchased: {:?}", is_purchased);

  // Demo activity 2
  enum Position {
    Marketing,
    Manager,
    Maintenance
  }

  enum Status {
    Active,
    Terminated
  }

  struct Employee {
    position: Position,
    status: Status
  }

  fn try_access(employee: &Employee) -> Result<bool, bool> {
    match employee.status {
      Status::Terminated => return Err(false),
      _ => (),
    }

    match employee.position {
      Position::Manager => Ok(true),
      _ => Err(false),
    }
  }

  let manager = Employee {
    position: Position::Manager,
    status: Status::Active
  };

  let marketing = Employee {
    position: Position::Marketing,
    status: Status::Terminated,
  };

  let maintenance = Employee {
    position: Position::Maintenance,
    status: Status::Active,
  };

  println!("Can access Manager: {:?}", try_access(&manager));
  println!("Can try_access Marketing: {:?}", try_access(&marketing));
  println!("Can try_access Maintenance: {:?}", try_access(&maintenance));
}