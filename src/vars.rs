pub fn run() {
  let (first_name, last_name) = ("Max", "Belkin");
  const ID: i32 = 001;
  let mut age = 26;

  println!(
    "{id}: My name is {first_name} {last_name} and I am {age}.",
    id = ID,
    first_name = first_name,
    last_name = last_name,
    age = age
  );

  age = 27;

  println!(
    "{id}: My name is {first_name} {last_name} and I am {age}.",
    id = ID,
    first_name = first_name,
    last_name = last_name,
    age = age
  );
}
