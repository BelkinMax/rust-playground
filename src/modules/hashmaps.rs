use std::collections::HashMap;

pub fn run() {
  #[derive(Debug)]
  struct Owner {
    name: String,
    password: Option<i32>,
  }

  let mut lockers = HashMap::new();
  lockers.insert(1, Owner {
    name: "Max".to_owned(),
    password: Some(1422),
  });
  lockers.insert(5, Owner {
    name: "Bob".to_owned(),
    password: None,
  });
  lockers.insert(3, Owner {
    name: "Ana".to_owned(),
    password: Some(6644),
  });

  for (key, value) in lockers {
    println!("Key: {:?}, Value: {:?}", key, value);
  }

}