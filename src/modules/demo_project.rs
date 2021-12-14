use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
  name: String,
  amount: f64
}

struct Bills {
  inner: HashMap<String,Bill>
}

impl Bills {
  fn new() -> Self {
    Self { inner: HashMap::new() }
  }

  fn add(&mut self, bill: Bill) {
    self.inner.insert(bill.name.clone(), bill);
  }

  fn remove(&mut self, name: &str) -> bool {
    self.inner.remove(name).is_some()
  }

  fn get_all(&self) -> Vec<Bill> {
    let mut bills = vec![];

    for bill in self.inner.values() {
      bills.push(bill.clone());
    }
    bills
  }
}

fn get_input() -> String {
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_err() {
    println!("Please type a valid input");
  }
  return buffer.trim().to_owned();
}

fn get_bill_amount() -> f64 {
  println!("");
  println!("Enter bill amount:");

  loop {
    let input: String = get_input();
    let parsed_input: Result<f64, _> = input.parse();

    match parsed_input {
      Ok(amount) => return amount,
      Err(_) => println!("Please enter a valid amount")
    }
  }
}

fn get_bill_name() -> String {
  println!("");
  println!("Enter bill name:");
  get_input()
}

fn view_bill_menu(bills: &Bills) {
  for bill in bills.get_all() {
    println!("{:?}", bill);
  }
}

fn add_bill_menu(bills: &mut Bills) {
  let name = get_bill_name();
  let amount = get_bill_amount();
  let bill = Bill {
    name,
    amount
  };
  bills.add(bill);
  println!("");
  println!("[!] A new bill was successfully added!");
  println!("");
}

fn remove_bill_menu(bills: &mut Bills) {
  for bill in bills.get_all() {
    println!("{:?}", bill);
  }

  println!("");
  println!("Type bill name to remove:");
  let input = get_input();

  if bills.remove(&input) {
    println!("");
    println!("[!] Bill was removed.");
    println!("");
  } else {
    println!("");
    println!("[!] Bill not found.");
    println!("");
  }
}

fn main_menu() {
  fn show() {
    println!("");
    println!("== Manage Bills ==");
    println!("1. Add new bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("");
    println!("Enter action number:");
  }

  let mut bills = Bills::new();

  loop {
    show();
    let input = get_input();
    match input.as_str() {
      "1" => {
        println!("Creating a new bill...");
        add_bill_menu(&mut bills)
      },
      "2" => {
        println!("All your bills list:");
        view_bill_menu(&bills)
      },
      "3" => {
        println!("Remove bill:");
        remove_bill_menu(&mut bills)
      },
      _ => break
    }
  }
}

pub fn run() {
  main_menu();
}
