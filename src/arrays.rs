/**
 * Arrays - Fixed list where elements are the same data types
 */
// Instead of use std::mem in the code, now I can use only mem
use std::mem;

pub fn run() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];

  println!("{:?}", numbers);

  // Get single val
  println!("Single value: {}", numbers[0]);

  // Get array lenght
  println!("Array Lenght: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occpies {} bytes", mem::size_of_val(&numbers));

  let mut mutable_numbers: [i32; 4] = [6, 7, 8, 9];
  println!("Mutable numbers: {:?}", mutable_numbers);
  println!("Array occpies {} bytes", mem::size_of_val(&mutable_numbers));
  // Re-assign value
  mutable_numbers[2] = 70;
  println!("Mutated numbers: {:?}", mutable_numbers);
  println!("Array occpies {} bytes", mem::size_of_val(&mutable_numbers));

  // Get Slice from position 0 to 2 of mutable_numbers
  let slice: &[i32] = &mutable_numbers[0..2];
  println!("Slice: {:?}", slice);
}
