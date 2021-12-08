pub fn run() {
  struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<char>,
  }

  let response = Survey {
    q1: Some(12),
    q2: None,
    q3: Some('A'.to_owned()),
  };

  match response.q1 {
    Some(answer) => println!("Q1: {:?}", answer),
    None => println!("Q1: No response")
  }
  match response.q2 {
    Some(answer) => println!("Q2: {:?}", answer),
    None => println!("Q2: No response")
  }
  match response.q3 {
    Some(answer) => println!("Q3: {:?}", answer),
    None => println!("Q3: No response")
  }
}