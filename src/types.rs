/*
 * Primitive Types--
 * Integer: u8, i8, u16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
 *    u -> unsiged (no negative values)
 *    i -> can be positive or negative number
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (char) -> 1 character different from String
 * Tuples
 * Arrays
 */

/**
 * Rust is a statically typed language, which means that it must know the types
 * of all variables at compile time, however, the compiler can usally infer
 * what type we want to use based on the value and how we use it.
 **/

pub fn run() {
  // Default is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 456546546546542;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active = true;
  let is_red: bool = false;

  // Get boolean from expressoin
  let is_greater: bool = 10 > 5;

  // char with single quote
  let a1 = 'a';

  // with unicode (print emoji)
  let face = '\u{1F600}';

  println!("{:?}", (x, y, z, is_active, is_red, is_greater, a1, face))
}
