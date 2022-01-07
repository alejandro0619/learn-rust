// We are going to discuss about Owenership using examples and comparisions so we are first introducing to String type and then using it while explaining a Rust-core concept.

// What´s the difference between string literals and String type?
// str are known and fixed size at compile time. Strings are growable and can be bound at runtime.
// Str lives in the stack memory while Strings are allocated in the heap.

// We can create string and push a chunk of string later:

fn display_string(){
  let mut s = String::from("Hello"); // Creates a String type variable using the method String::from() that takes a literal and return a String
  s.push_str(" , world!!"); // this method let us push a literal to a String

  println!("{}", s);
}

// The literals are hardcoded into the final binary, because we know the content and its size at compile time.
// But we can't hardcode an unkown amount of space for each variable into the final binary.
// So in order to make a String mutable and growable we need to allocate it in the heap, this means:
// * The memory must be requested from the memory allocator at runtime.
// * We need a way of returning this memory to the allocator when we're done with our string.

// The first part is done by the method String::from() because it's implementation is meant to requests the memory it needs.
// The second part is not that easy. In languages with Garbage Collector (GC)
// It keeps track and cleans up the memory when it's not being used anymore.
// We do not need to think about it.

// But when the programmer doesn't have a GC he needs to manually request the memory, use it and at the end: return it. 
//If you don't return the memory: You'll waste the memory. If you return it too early: you are trying to access to an invalid variable. And these are the famous bugs that some low-level programmers faces to.

// Rust takes a different approach: Memory is automatically returned once the variable that owns it goes out of scope.

// {
//  let s = String::from("Hello"); // Is valid
// We can use it.
// } it is no longer valid so the memory is free now

// ! WAYS THAT VARIABLES AND DATA INTERACT: MOVE.
// Multiples variables can interact with the same data in differente ways:
let y = 5; // bind the value 5 to the variable y.
let x = y; // Copy the value of variable y and bind it to y.
// Both are equal to 5, and that's right because integers aren't complex and they are well known and its size in compilet time.

// What will happen here?
// It won't copy exactly the value as in the previous example.
// A string, under the covers, is made up of three parts, shown on the left:
// A poninter to the memory that  hold the content of the string, a length and a capacity.
// And in the other hand, is the memory on the heap, that holds the content.
// When we assign s1 to s2 (s2 = s1) daa is copied, meaning we copy the pointer, length and the capacity that are on the stack. We don't copy the data on the heap that the pointer refers to.

// But if both refers to the same memory address and they both go out of scope
// It's a problem called double free error and it's a big safety bug.
// Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
// So to avoid this error Rust decides that after s2 = s1 the variable s1 will no longer be availabe, thus, won't be valid anymore so Rust doesn't need to free anything after s1 goes out of scope.

// If you've heard of the terms shallow copy and deep copy, while working on the below example, copying the pointerm lenght and capacity, it probably sound like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, we call it move. SO we say that s1 moved into s2
fn move_strings(){
  let s1 = String::from("Hello");
  let s2 = s1;
}

// ! Clone:
// If we want to deeply copy the heap data of the string, not just stack data, we can use a common a method called clone:
fn display_clone_method(){
  let s1 = String::from("Hellow");
  let s2 = s1.clone();
  println!("s1 = {}, s2 = {}", s1, s2);
}
// This will produce exactly what we would expect when we just "Copy the variables into other one and maintain both alive"
// But this code may be expensive if we are copying a large amount of data
// ! Stack-only data: Copy
// This code using integers is completely valid:
fn stack_only_data_copy(){
  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);
}
// This seems to contradict what we just learned: we don't a call to clone, but x is still valid and wasn't moved into y.
// The reason is that types such as integers have a known size at compile time
// and are stored completely on the stack. SO copies are quick to make.
// So there's no reason to prevent x being valid after the y assignment.

//  Basically there's no difference between shaloow and deep copy here.
// Calling clone wouldn't do anything differente from the usual shallow copying and we can leave it out.

// Rust has a special annotation called the Copy trait that we can place on types like integers that are stored on the stack. 
// If a type implements the Copy trait, an older variable is still usable after assigment. Rus won't let us annotate a type with the Copy trait if the type or any of its parts has implemented the Drop trait.
// If the type need something special to happen when the value goes out of scope and we add the Copy annotation to that type, we'll get a compile-time error.
// The types that implements the Copy trait:
// Integers, boolean, floating, char, tuples (only if contain types also implements Copy)

// * Ownership and functions
// Passing a value into a function will move or copy, just like an assigment.
fn takes_ownership(some_string: String){
  println!("Some string: {}", some_string);
}
fn makes_copy(some_integer: i32){
  println!("Some integer: {}", some_integer);
}
fn test_one(){
  let s = String::from("Hello") // comes into scope.
  takes_ownership(s);
  let x = 7;
  makes_copy(x);
}
// It will move the value stored in s because it is not a type that implements the Copy trait.
// But x will remain available even after passing that value to the function.

fn test_two() {
  let s1 = gives_ownership();         // gives_ownership moves its return
                                      // value into s1
  let s2 = String::from("hello");     // s2 comes into scope
  let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                      // takes_and_gives_back, which also
                                      // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                           // return value into the function
                                           // that calls it

  let some_string = String::from("yours"); // some_string comes into scope

  some_string                              // some_string is returned and
                                           // moves out to the calling
                                           // function
}
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                    // scope

  a_string  // a_string is returned and moves out to the calling function
}

// And it is possible to return multiples values using a tuple:
fn test_three(){
  let s1 = String::from("Hello world from Rust");
  let (s2, length) = calculate_length(s1);
}
fn calculate_length(s: String) -> (String, usize){
  let length = s.len();
  (s, length)
}