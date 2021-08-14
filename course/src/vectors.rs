// vectors - resizable arrays.

use std::mem;

pub fn run() {
let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
// access to a single value:
// get index 0, element 1.
println!("{}", numbers[0]);

// mutate elements:
numbers[1] = 5;

// add to vector:
numbers.push(6);

//pop off the last value:
numbers.pop();

// get the length:
numbers.len();

// Loop through vectors velue:
for x in numbers.iter() {
  println!("Number: {}", x );
}
// vector are stack allocated so:
println!("this vector takes {} bytes ", mem::size_of_val(&numbers));

// pretty similar to map() in javascript
// Loop and mutate values:
for x in numbers.iter_mut() {
  *x *= 2;
}
// get slice:
let slice: &[i32] = &numbers[0..2];

println!("{:?}", slice);
println!("{:?}", numbers);
}