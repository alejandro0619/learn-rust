fn display_integers(){
  let int8b: i8 = - 5; // int 8 bits
  let int16b: i16 = - 15; // int 16 bits
  let int32b: i32 = - 69; // int 32 bits it's used by default.
  let int64b: i64 = 6000; // int 64 bits
  let int128b: i128 = 9000000; // int 128 bits
  let intisizeb: isize = 5; // depends on the arch
  let un8b: u8 = 5; // unsigned int 8 bits
  let un16b: u16 = 15; // unsigned int 16 bits
  let un32b: u32 = 555; // unsigned int 32 bits
  let un64b: u64 = 1444; // unsigned int 64 bits
  let un128b: u128 = 141141461458; // unsigned int 128 bits
  let unusizeb: usize = 5; // depends on the arch

  println!("Integers:\nBoth positives and negatives:\nInt8 {}\nInt16 {}\nInt32 {}\nInt64 {}\nInt128 {}\nIsize {}",int8b, int16b, int32b, int64b, int128b, intisizeb);
  println!("---------->");
  println!("Possitive:\nUnsigned 8{}\nUnsigned 16{}\nUnsigned 32{}\nUnsigned 64{}\nUnsigned 128{}\nUsize {}", un8b, un16b ,un32b, un64b, un128b, unusizeb);
}
fn display_float(){
  let floating_one : f64 = 1.0; // floating of double presicion used by default.
  let floating_two : f32 = 2.0; // floating of single presicion
  println!("---------->");
  println!("Double presicion:{}\nSingle presicion:{}", floating_one, floating_two);
}
fn display_bool(){
  let t = true; 
  let f: bool = false; // with explicit notation
  println!("---------->");
  println!("Possible values of a boolean: {} or {}", t, f);
}
fn display_character_type(){
  // They are 4 bytes in memory.
  let c = 'c'; // char literals are specified wit single quotes.
  let _x: char = 'x'; // with explicit notation
  let rocket = '🚀'; // Also supports unicode 
  println!("---------->");
  println!("\"C\" character {}\n rocket unicode: {}", c, rocket);
}
// * Scalar types.
// They represents a single value
fn display_scalar_types(){
  display_integers(); // display integers.
  display_float(); // display floating point values.
  display_bool(); // display boolean values.
  display_character_type();

}
fn display_tuple(){
  // Group together a variety of types into one compunt type
  // Fixed lenght.
  // Once it's declared they cannot grow or shrink in size.
  // Each position has a type.
  let tup: (i32, char, u8) = (500, '❤', 6); // We added here additional but optional notation.
  // The tup variable bind to the entire tuple because it's considered as single compount element.

  // To get a single value of the tuple we can use a pattern matchin to destructure a tuple:
  let tup = (4, -9, 5.3); // Note how we shadowed the original variable.
  // Note how the compiler can infer the type annotation of the values the tuple holds in
  let (x, y, z) = tup; // We are destructuring the tuple and assignating the value to a tern of variables that we can manipulate later one by one.
  println!("---------->");
  println!("The first value of the tuple is: {}\nThe second value of the tuple is: {}\nThe third value of the tuple is: {}",x, y, z);

  let _four = tup.0; // returns the first value of the tuple individually. 

  let _unit_type_tuple = (); // The tuple without any value, (), is a special type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don't return any other value.

}
fn display_array(){
  // every element of the array must have the same type.
  // Has fixed length
  // Comma separated in square brackets.
  
  let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

  let _array_one: [i8; 3] = [1, 2, 3]; // with explicit type declaration.
  // Is useful when we want to store our data in the stack rather than the heap.
  // Or if we want a fixed number of elements
  // For example we wouldn't like to change any element of the array because any program is likely to delete or add a new month.
  let _array_of_three = [3;3]; // This will initialize an array of lenght 3 and each element have the value of 3.

  let first = months[0]; // You will access the to the values within an array by writing the name of the variable that holds that array and then between squared braces the index you want to access to.

  println!("The first month of the year is: {}", first);
}
// * Compound types:
// Can group multiple values into one, Rust has two primitive compound data types: tuples and arrays.
fn display_compunt_types(){
  display_tuple();
  display_array();
}
pub fn display_types(){
  display_scalar_types();
  display_compunt_types();
}