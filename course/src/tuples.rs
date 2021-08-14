// tuples group together values of different types:
// MAx 12 el:

pub fn run() {
  let person: (&str, &str, i8) = ("Alejandro", "Vzla", 17);
  println!("{} Is from , {}, and is {}", person.0, person.1, person.2);
}