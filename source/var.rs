pub fn run(){
  // define let 
  let name = "Alejandro";
  // mutable constant
  let mut age = 16;
  age = 17;


  // define const:
  // I must define a type
  const ID: i32 = 45;
  println!(" My name is {}", name );

  // define multiple vars:
  let (my_name, my_age) = ("Alejandro", 16);
  println!("This is my name: {}, this is my age: {}", my_name, my_age);

}