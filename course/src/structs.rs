// Structs - Used to create data type:
struct Color {
  red: u8,
  green: u8,
  blue: u8
}
// tuple struct:
struct newColor(i8,i8,i8);

struct Person {
  first_name: String,
  second_name: String
}
impl Person {
  // construct person:
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: String::from(first),
      second_name: String::from(last)
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.second_name)
  }
  fn set_last_name(&mut self, last: &str){
    self.second_name = String::from(last);

  }
  fn name_to_tuple(self) ->  (String, String){
    (self.first_name, self.second_name)
  }
}

pub fn run(){

  let mut color = Color {
    red: 255,
    green: 0,
    blue: 0
  };
  // we can mutate it:
  color.red = 200;

  println!("Colors: {} {} {}", color.red, color.green, color.blue);
  let mut new_color = newColor(24,2,2);
  println!(" {} {} {} ", new_color.0, new_color.1, new_color.2);
  let mut person = Person::new("Mary", "Doe");
  println!("Hey my name is: {} {}", person.first_name, person.second_name);
  person.set_last_name("lopez");
  println!("{}", person.full_name());
  println!("Name in tupe{:?}", person.name_to_tuple());

}