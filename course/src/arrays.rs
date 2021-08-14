// Arrays - fixed list where elements are the same data type
use std::mem;

pub fn run() {
let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
// access to a single value:
// get index 0, element 1.
println!("{}", numbers[0]);

// mutate elements:
numbers[1] = 5;

// get the length:
numbers.len();

// arrays are stack allocated so:
println!("this arrays takes {} bytes ", mem::size_of_val(&numbers));

// get slice:
let slice: &[i32] = &numbers[0..2];

println!("{:?}", slice);
println!("{:?}", numbers);
}