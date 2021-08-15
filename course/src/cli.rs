use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();
  let status = "100%";

  if command == "hello" {
    println!("Hey, how are you?");
  } else if command == "status" {
    println!("Your status is :{}", status);
  } else {
    println!("That isn't a valid command.");
  }

  
}