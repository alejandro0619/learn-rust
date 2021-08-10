extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number! ");
  let secret_number:u32 = rand::thread_rng()
    .gen_range(1, 51);

    loop{
        println!("Introduce the first number");

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed at read the line");

        let guess:u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Equal!");
                println!("the secret number is {}", secret_number);
                break;
            }
        }
    }
}
