// Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful whenyou have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

// Creating a vector

pub fn vectors(){
  let my_vec: Vec<i32> = Vec::new(); // new empty vector of i32 binded to my_vec
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


}