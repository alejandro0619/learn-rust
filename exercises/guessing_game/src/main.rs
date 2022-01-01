use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100); // It will generate a number between 1 (start is inclusive by default) and 100 (the end es exclusive, we can use start.. =end or start..end+1) 

   loop { // If the loop don't exist we only will able to guess the number once, because after the match arm the program will stop, so that's why we are adding it. 
   //If the program reach the Ordering::Equal pattern it will display you win and after it comes the break; statement which will get the program flow out of the loop. 
   println!("Please input your guess.");
    let mut guess = String::new(); // creates a mutable variable
    
    io::stdin()
        .read_line(&mut guess) // reads the line and save the content in guess variable by changing it's value
        .expect("Failed to read line"); // read_line return a value, it's a io::Result enum, if it's (Err) expect method will cause the program to crash and display the message passed as an argument. 
        //If it return an (Ok) variant it will return the actual value of the input.
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // This will shadow the guess variable created before, and convert it to a number type not before getting rid of the possible whitespaces. 
        //As said before it will use parse method to convert to the type :i32 (it's a must-to-do because otherwise parse mathod wouldn't know what type it will convert to).
        //And the parsing will return a Result enum again, so we are going to evaluate:
        // If it returns the Ok variant, we'll return the number iself,
        // Other wise if it return an Err variant (no matter which error) we will skip to the next iteration of the loop so we can ask for the input again.

        // This will compare guess variable (the shadowed one) to the reference to secret_number (We don't want to copy the value) and it will return an enum, the Ordering enum which has 3 variants: Equal, Less or greater, the match will decide what to do next depending on the match pattern we wrote: If it's equal it will say "You win", if it's less it will say "Too small", and if it's greater, it will say "Too big" respectively.
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),        
        }
    }
}
