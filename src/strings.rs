/**
 * Primitive str =  Immutable fixed-length string somewhere in memory
 * String = Growable, heap-allocated data structure -
 *          Use when you need to modify or own string data
 */

pub fn run() {
  // str with double quotes
  let hello = "Hello";

  // Get length
  println!("Length str: {}", hello.len());
  println!("str: {}", hello);
  // String
  let ciao = String::from("Ciao");
  // Get length
  println!("Length String: {}", ciao.len());
  println!("String: {}", ciao);

  // String mutable with push (char)
  let mut greeting = String::from("Saluto");

  // push a char
  greeting.push(' ');
  greeting.push('W');

  // push a string
  greeting.push_str("orld!");

  // push unicode (emojii face)
  greeting.push('\u{1F600}');

  // Capacity in bites
  println!("Capacity: {}", greeting.capacity());
  // Check if is empty
  println!("Is Empty: {}", greeting.is_empty());

  // Check if contains
  println!("Contains 'World': {}", greeting.contains("World"));
  println!("Contains 'Mondo': {}", greeting.contains("Mondo"));

  println!("Greeting: {}", greeting);

  // Replace
  println!("Replace: {}", greeting.replace("World", "There"));
  println!("Greeting: {}", greeting);

  // Loop throught string by whitespice
  println!("Splitted:");
  for word in greeting.split_whitespace() {
    println!("\t{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing - show error if fails
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
  println!("String with capacity: {}", s)
}
