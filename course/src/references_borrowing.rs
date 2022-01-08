// We can share a variable to a chunk of code (function) without that function taking ownership of the value you just passed.
// Let's use something called references.

fn test_one(){
  let s1 = String::from("Hello");
  let length = calculate_length(&s1); // & means "address of"
  println!("{}", length);
  let mut s2 = String::from("Hello!!");
  modify_string(&mut s2);

}
// The ampersant preceding the String type tell the function that it will receive the address where a String value is stored
// But it is not the owner of that value.
fn calculate_length(s: &String) -> usize {
  s.len()
}

// To modify the value we are borrowing we need to:
// &mut means we are borrowing the value of the variable and we are going to modify it.
// We can only use one mutable references to a value once at time.
// The benefit of using this restriction is that rust prevents data races at compile time. Data race is similar to race condition and happens when these three behaviors occurÑ
// - Two or more pointer access the same data at the same time.
// - At least one of the pointers is being used to write to the data.
// - Thres's mechanims being used to synchronize access to the data.
// Data races cause undefined behavior and can be difficult to track down at runtime.
fn modify_string(s: &mut String) {
  s.push_str(", World");
}
// By the way we can create a new scope using curly brackets and defined a mutable borrow inside of it.

// Another error will be thrown if we use a inmutable reference and then we use a mutable one:
fn _this_code_will_panic(){
  let mut s = String::from("new");
  let r1 = &s; // No problem
  let r2 = &s; // No problem
  let r3 = &mut s;
  println!("{}{}{}", r1, r2, r3);
  // We cannot have a mutable reference while we have an inmutable one
}
fn _this_code_will_compile(){
  let mut s = String::from("new");
  let r1 = &s; // No problem
  let r2 = &s; // No problem
  println!("{} and {}", r1, r2);
  let r3 = &mut s;
  println!("{}", r3);
  // The scopes of the inmutable references r1 and r2 end after the println! where they are last used, which is before the mutable references r3 is created,
  // These scopes don't overlap, so this code is allowed. The ability of the compiler to tell that a references is no longer beig used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short)
}

// * Dangling references
// In languages with pointer, it's easy to erroneously create a dangling pointer, a pointer that refereces a location in memory 
// that may have been given to someone else
// By freeing some memory while preserving a pointer to that memmory. In Rust, by contrast, the compiler guarantees that references will never be dangiling references, if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

// This is a dangling reference that won't compile.
fn test_dangle(){
  let reference_to_nothing  = dangle_will_panic();
}
fn dangle_will_panic() -> &String { // returns a reference to a string
  let s = String::from("Hello!"); // s is the string
  &s // we return the reference to the string, s
} // here the function end and s goes out of scope and s is dropped
// The best solution is to return the string itself, the ownership will be moved and nothing is deallocated

// So basically:
// - At any given time, you can have either one mutable reference or any number of immutable references.
// - References must always be valid.