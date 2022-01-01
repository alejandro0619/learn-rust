pub fn display_constants() {
  const MY_CONSTANT: i32 = 5; // Unlike variables, constants are'nt allowed to be mutable, It needs to be declared having a explicit type annotation.
  // It's name, by convention, will be in UPPER_SNAKE_CASE.
  // And it's computed ahead of runtime.
  println!("My constant is: {}", MY_CONSTANT);

}