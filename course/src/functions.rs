pub fn run() {
  greetings("ALejandro", "Nice to meet you");
  let get_sum: i32 = add(5, 5);
  println!("Sum function {}", get_sum);

  // Closure:
  // We can get values from the outside of a function 
let n3: i32 = 5;
let add_nums = |n1: i32, n2:i32| n1 + n2 + n3;
println!("Closure sum function: {}", add_nums(1, 2));
}
fn greetings(greet: &str, name: &str) {
  println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  return n1 + n2;
}


