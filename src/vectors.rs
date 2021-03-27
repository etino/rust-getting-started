/**
 * Vectors are Resizable arrays
 */
use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  println!("{:?}", numbers);

  // Get single val
  println!("Single value: {}", numbers[0]);

  // Get array lenght
  println!("Vector Lenght: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector occpies {} bytes", mem::size_of_val(&numbers));

  // Re-assign value
  numbers[2] = 30;

  // Add on to vector
  numbers.push(6);
  numbers.push(7);

  // Pop off last value
  numbers.pop();

  println!("Mutated numbers: {:?}", numbers);
  println!("Vector occpies {} bytes", mem::size_of_val(&numbers));

  // Get Slice from position 0 to 2 of mutable_numbers
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  println!("Loop Numbers");
  for x in numbers.iter() {
    println!("\tNumber: {}", x)
  }

  // Loop and mutate values
  println!("Loop and Mutate Numbers");
  for x in numbers.iter_mut() {
    *x *= 2
  }
  println!("Mutated Vector with loop: {:?}", numbers)
}
