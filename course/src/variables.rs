pub fn display_variables(){
  let x = 5; // It's inmutabble by default and the compiler can infer it's type.
  let mut mut_variable = 9;
  println!("Initial value of mut_variable: {}", mut_variable);
  mut_variable = 3;

  println!("Inmutable variable: {}\nmutable one: {}", x, mut_variable);
}