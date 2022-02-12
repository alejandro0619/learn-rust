// gets the definitions from the body of front_of_house module.
//To continue with out example and extract the  hosting module to its own file as well,
// we change src/front_of_house.rs to contain only the declaration of the hosting module

/*pub mod hosting {
  pub fn add_to_waitlist(){
    println!("Adding to waitlist");
  }
}*/

pub mod hosting;
// And we create a src/front_of_house directory and a file 
// src/front_of_house/hosting.rs