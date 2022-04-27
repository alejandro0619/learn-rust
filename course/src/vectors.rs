// Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. 
// Vectors can only store values of the same type. They are useful whencyou have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

// Creating a vector

pub fn vectors(){
  let my_vec: Vec<i32> = Vec::new(); // new empty vector of i32's binded to my_vec
  // We added the type annotation because we aren't inserting values into the vector
  // So Rust needs to know what kind of elements we intend to store. This is an import point because Vectors are implemented using generics.

  // If we add initial values and Rust will infer the type of value you want to store
  // So you rarely need to do this type of annotation
  let v = vec![1, 2, 3];
  // Rust can inferthe type based of the values it stores

  // Updating a vector
  // Let push values into my_vec_2
  let mut my_vec_2 = Vec::new();
  my_vec_2.push(5);

  //Dropping a vector:
  //When dropping a vector you drop its elements.
  //Like a struct, a vector is freed when it goes out of scope.

  {
      let v = vec![1, 2, 3, 4];
      // Do some stuff with v
  } // It is dropped here.

  // Reading elements of vectors.
  let third_el = &v[2];
  println!("The third element is: {}", third_el);

  // If you prefer the safety you can access by:
  match v.get(2){
      Some(third) => println!("The third element accessed safety: {}", third),
      None => println!("No third element");
  }

  // Iterating over the values in a vector:
  // To access each element in a vector in turn, we would iterate through
  // all of the elements rather than use indics one at time.
  // How to use a for loop to get inmutable references to each element in a vec.

  for i in &v {
      println!("{}", v);
  }

  // Or if you wanna iterate over mutable references:

  let v2 = vec![1, 2, 4, 6];

  for i in &mut v2 {
      *i += 50;
  }
}

fn _enum_to_store_mul_types() {
    // Vectoes can only store values that are the same type.
    // This can be inconvenient; there are definitely use cases
    // for needing to store a list of item of different types.
    // Fortunately, the variants of an enum are defied under the same 
    // enum type. So, when we need one type to represent elements of 
    // different types, we can defined and use an enum! 


    // For an example, say we want to get values from a row in a spreadsheet 
    // in which some of the columns contain integers, some floating, and some strings.
    // We can define an enum whose variants will hold the different values type and
    // all the enum variants will be considered the same type: that of the enum.
    // Then we can create a vector to hold that enum and so, ultimately
    // holds different types. We've demostrated this below:
    
    enum spreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        spreadsheetCell::Int(3),
        spreadsheetCell::Float(3.14),
        spreadsheetCell::Text(String::from("Blue")),
    ];

}
