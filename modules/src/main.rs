// mods:
mod sound;

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32
    }
    impl Vegetable {
        pub fn new(name: &str) -> Vegetable{
            Vegetable{
            name: String::from(name),
            id: 1,
            }
        }
    }
}

mod menu {
    #[derive(Debug)]
    pub enum Appetizer {
        soup,
        salad,
    }
}
// use asbolute path:
use crate::sound::instrument;
// use relative path:
// use self::sound::instrument;

//main:
fn main() {
    //using use keyword:
    instrument::clarinet();
    // absolute:
    crate::sound::instrument::clarinet();
    // relative:
    sound::instrument::clarinet();

    let mut v = plant::Vegetable::new("squash");
    println!("{}", v.name);
    println!("{:?}", menu::Appetizer::salad);
}

