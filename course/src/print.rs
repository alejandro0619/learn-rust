pub fn print_to_terminal(){
  // You can use {} inside these macros to introduce text inside of it, actually, you can enter as many parameters
  // It is used as:
  println!("Hey, I'm Alejandro and I'm {} years old", 16);
  
  //Remember you can enter as many params as you want, so this snippet below will also works as well.
  println!("Hey, I'm {} and I'm {}", "Alejandro", 16);

  // arguments can be used, for example posotional arguments:

  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // As can named arguments.
  println!("{subject} {verb} {object}",
  object="the lazy dog",
  subject="the quick brown fox",
  verb="jumps over");

  
}

