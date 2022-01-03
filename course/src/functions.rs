// fn is the keyword designated to tell the compiler we are declaring a function.
// a_function is the name of the function, which we call later by.
// the parentheses are means "parameters" and it's the input that the function receives. It's optional: it can be empty
// between the pair of curly braces is placed the body of the function: the code that will be executed.
fn a_function(){
  println!("A function!");
}

a_function(); // we are calling the function.

// square function will receive a param called n with the type of i32.
// Will return ad i32 value type. 
fn square(n: i32) -> i32 {
  return n * n
}

// functions are made up of statements and expressions: So that makes us
// flexible to manipulate and do a lot of operation inside of it.