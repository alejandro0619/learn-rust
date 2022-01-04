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