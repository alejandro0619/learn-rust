mod front_of_house; 
// tells Rust to load the contents of the module from another file with the same name as the module.

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}