pub fn run() {
  // Primitive str = Inmutable fixed-length string somewhere in memory
  // String = Growable, heap=allocated data structure. Use when you need to modify your data.

  let mut hello = String::from("Hello");
  println!("lenght of  hello variable is {}", hello.len());

  // To push char:
  hello.push('a');

  // push a string;
  hello.push_str(" A new world pushed");

  // capacity in bytes;
  println!("Capacity {}", hello.capacity());

  //Is empty?
  println!("Is empty? {}", hello.is_empty());

  // contains:
  println!("Does it contains the word world: {}", hello.contains("world"));

  // replace:
  println!("replace: {}", hello.replace("Hello", "Hey"));

  // Loop through string by whitespaces:
  for word in hello.split_whitespace() {
    println!("{}", word );
  }

  // Create string with capacity:
  let mut string = String::with_capacity(10);
  string.push('a');
  string.push('b');
  println!("{}", string );

  // Assertion testing:
  assert_eq!(10, string.capacity());

}