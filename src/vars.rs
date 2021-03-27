// Variables hold primiteve data or references to data
// Variable are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Brad";

  // Mutable variable
  let mut age = 37;
  println!("My name is {} and I'm {}", name, age);

  age = 38;
  println!("My name is {} and I'm {}", name, age);

  // Define a constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Brad", 37);
  println!("{} is {}", my_name, my_age);
}
