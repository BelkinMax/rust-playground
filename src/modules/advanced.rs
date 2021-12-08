pub fn run() {
  // Advanced enums
  enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
  }

  let tickets = vec![
    Ticket::Backstage(49.99, String::from("Max")),
    Ticket::Standard(14.99),
    Ticket::Vip(29.99, String::from("Bobby"))
  ];

  for t in tickets {
    match t {
      Ticket::Backstage(price, holder) => {
        println!("[ BACKSTAGE ] Holder: {:?}, Price: ${:?}", holder, price)
      },
      Ticket::Standard(price) => {
        println!("[ STANDARD ] Price: ${:?}", price)
      },
      Ticket::Vip(price, holder) => {
        println!("[ VIP ] Holder: {:?}, Price: ${:?}", holder, price)
      },
    }
  }

  // Option
  struct Customer {
    age: Option<i32>,
    email: String,
  }

  let max = Customer {
    age: Some(27),
    email: String::from("hellomax@world.com"),
  };

  let bob = Customer {
    age: None,
    email: String::from("bobbybob@hello.com"),
  };

  let customers_list = vec![max, bob];

  for customer in customers_list {
    match customer.age {
      Some(age) => println!("Customer is {:?} years old", age),
      None => println!("Customer age not provided")
    }
  }
}