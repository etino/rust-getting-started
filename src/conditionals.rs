/**
 * Conditionals - Used to check the condition of something and act on the result
 */

pub fn run() {
  let mut age: u8 = 18;
  let check_id: bool = false;
  let knows_person_of_age: bool = true;

  // If/Else
  if age >= 21 && check_id || knows_person_of_age {
    println!("Bartender: Whats would yuo like to dring?")
  } else if age < 21 && check_id {
    println!("Bartender: Sorry you have to leave")
  } else {
    println!("Bartender: I'll need to see your ID")
  }

  // Shorthand If - No ternary operation is available in Rust
  let mut is_of_age = if age >= 21 { true } else { false };
  println!("Is Of Age: {}", is_of_age);
  age = 22;
  is_of_age = if age >= 21 { true } else { false };
  println!("Is Of Age: {}", is_of_age)
}
