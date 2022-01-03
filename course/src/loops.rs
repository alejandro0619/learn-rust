// Loops statement:
// Sometimes wee ned to repeat certain task several times
// Rust gives us 3 options to approach this.

// *Loop:

// Basic loop
fn using_loop(){
  loop {
    let mut n = 0;
    if n >= 10 {
      break; // This keyword will let us to get out of the loop.
    } else {
      println!("Again!");
    }
  }
}

// Using labeled loops
pub fn nested_loops(){
  let mut count = 0;
  'counting_up: loop { // we can add labels to loops in order to skip or break a determined loop.
    println!("Count: {}", count);
    let mut remaining = 20;
      loop {
        println!("Remaining: {}",remaining );
        if remaining == 9{ // If remaining is = 9, we'll get out of the loop
          break;
        } 
        if count == 2 { // If count is = 2 we'll get out of the loop labeled counting_up
          break 'counting_up;
        }
        remaining -= 1; // remaining -1

      }
    count +=1; // coun +1

  }
  println!("End of count = {}", count);
}

// Returning values from loops

fn returning_values_loop(){
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2; // we use break statement followed by the expression we wanna return. PS it wll be bound to result variable
    }
  };
}
