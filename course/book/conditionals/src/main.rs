// *If expressions:

fn check_age(age: i32) {
    // The if statement will only check booleans
    if age >= 18 { // checks if the variable age is greater than 18
      println!("You are 18 or older");
    } else { // Otherwise do:
      println!("You are not allowed, you are under 18.");
    }
  }
  
  // *If inside a let statement:
  fn check_if_even(n: i32) {
    // Evaluates the expressions and returns a vlaue, that value is bound to the variable even.
    // Both of the possible return types needs to be the same.
    let even = if n % 2 == 0 { true } else { false };
    println!("{}", even)
  }
  
  
  
fn main() {
    check_age(25);

    check_if_even(4);
}
