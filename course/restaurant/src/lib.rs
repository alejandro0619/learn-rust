// This code explain how modules works in Rust

// Modules let's us orgnaize code within a crate into groups for readability and easy reuse. 
// modules also control the privacy of items.
// Which is whether an item can be used by outside code, or is an internal implementation detail and not available for outside use.

// Let's write a library crate that provides the functionality of a restaurant. We'll define the signatures of functions but leave their bodies empty to concentrate on the organization of code rather than actually implementing a restaurant in code.

// In the restaurant industry, some parts of a restaurant are referred to as front of house an others as back of house.
// Front of house are where costumers are, servers takes orders, and payment, and the bartenders make drinks.

// Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

// To structure our crate in the same way that a real resturant works, we can organize the functions into nested modules.

//----
// We define a module by starting with the mod keyword and then specify the name of the module (in this case front_of_house) and place curly brackets around the body of the module.
//Inside modues we can have other modules, as in this case, with the modles hosting and serving. 
//Modules can also hold definitions for other items, such as structs, enums, constants, traits or functions.

// By using modules we can group related definitions together and name why they're related. 
// Programmers using this code would have an easier time finding the definitions they wnted to use because they coukd navigate the code based on the groups rather than
// Having to read though all the definitions. Programmers adding new functionality to this code would know where to place the code to keep the program organized.

// Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate's module structure, known as a module tree.
 /*
 crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

 */
mod front_of_house {
    pub mod hosting { // this submodule is public
        pub fn add_to_waitlist() {} // this function id public

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


// Path for referring to an item in the module tree.

// to show where to find an item in a module tree, we use path in the same way we use a path when navigating a filesystem, 
//if we want to call a function, we need to know its path.
// A path can take two forms:
// An absolute path starts from the crate root by using a crate name or a literal crate.
// A relative path starts from the current module an uses self, super or an identifier in the current module.

// Both absolute an relative paths are followed by one or more identifiers seprated by double colons

// To access to add_to_waitlist function from a absolute path:
// * crate::front_of_house::hosting::add_to_waitlist
// To access to add_to_waitlist from relative path:
// * front_of_house::hosting::add_to_waitlist

// front of house isn't public but because 
// eat_at_restaurant is its sibling, it can access to each other
pub fn eat_at_restaurant(){
    // ...code
}

// ~ Starting relative path using super:
// we can also construct relative path that begin in the parent module by using super at the start of the path.
// This is like starting a filesystem path with the .. syntax. 
// Why would we want to do this?

// Consider the following code, that models the situation in which a chef fixes an incorrect order
// and personally brings it out to the customer.
// The funciton fix_incorrect_order calls the function serve_order by specifying the path to the serve_order starting with super:

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
    fn fix_incorrect_order(){
        cook_order(); // doesn't need to use super because cook_order is a sibling function
        super::serve_order(); // needs to use super because serve_order is in the parent scope.
    }
    fn cook_order(){}
}

// ~ Making structs and enums public
// we can also use pub keyword to designate structs and enums as public, but there are few extra details. 
// If we use pub before a struct definition, we make the struct public,
// but the strutc's fields we still be private. 
// We can make each field public or not on a case-by-case basis.
// Let's se how we can define back_of_house::Breakfast struct with a public toast field but a private seasonal_fruit field.
// This models the case in a restaurant where the customer can pick a typ of bread that comes with a meal.
// But the chef decides which fruit accompanies the meal based on what's in season and in stock.
// The available fuit changes quickly, so customers can't choose the fruit or even see which fruit they'll get.

fn customers_eat(){
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");
}