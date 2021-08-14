// conditioans - Used to check the condition of something and act accordingly:

pub fn run(){

  let age = 22;
  let check_id = true;
  let knows_person_of_age = true;
  if age >= 21  && check_id || knows_person_of_age {
    println!("Bartender: What do you wanna drink?");
  } else if age < 21 && check_id {
    println!("Sorry, you gotta leave");
  } else {
    println!("Sorry, I need to see your ID")
  }

  // shorthand :
  let is_of_age = if age >= 21 { true } else { false };
  println!("is over age? {}", is_of_age);
}