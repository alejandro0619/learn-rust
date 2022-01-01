pub fn display_shadowing(){
  let x = 4; // creates a variable and bind it a value of 4
  let x = x + 1; // It will shadow the first value and it's new value will be:
  // Original value + 1, and bind it to the new variable named x
  // Btw we can change the type of the variable, and after the shadowing is completed, that value will still be inmutable
  println!("The value of x is: {}", {x});
}