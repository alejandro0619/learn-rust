/*
Well, yes you saw it from the last section (comments), but here I will dig a bit into what every piece of code does:

1. fn is a reserved word, you are telling the compiler that you will use a function.

2. hello_world is the name of the function, you can call it later by its name.

3. println! is a macro, we will see it later in deep, for now keep in mind that this println! macro is responsable of printing not only text but barely everything through the console.

4. As you can see in the line 22 I wrote hello_world(), that is a function calling, you can create a function but until you call it, it won't be able to execute the code inside of it.


For short: hello_world is function that prints out in a console the text "hello world"
*/
 // run it online: 
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=baac3300f5a56e3249d1bea7d2108af8
pub fn hello_world(){
  println!("hello world");
}

