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

    pub enum Appetizer {
        Soup, 
        Salad
    }
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

// ~ Making structs public
// we can also use pub keyword to designate structs and enums as public, but there are few extra details. 
// If we use pub before a struct definition, we make the struct public,
// but the strutc's fields we still be private. 
// We can make each field public or not on a case-by-case basis.
// Let's se how we can define back_of_house::Breakfast struct with a public toast field but a private seasonal_fruit field.
// This models the case in a restaurant where the customer can pick a typ of bread that comes with a meal.
// But the chef decides which fruit accompanies the meal based on what's in season and in stock.
// The available fuit changes quickly, so customers can't choose the fruit or even see which fruit they'll get.

fn costomers_eat(){
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");
}

// Because toast field is public in back_of_house::Breakfast, we can read and rewrite it's value using dot notation.
// Also, note that because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast.
// We've named it summer. If Breakfast didn't have such a function, we could'nt create a instance of Breakfast in costumers_eat() because we couldn't set the value of seasonal_fruit field in costumers_eat() function

// ~  Making enums public
// Because we made the Appetizer enum public, we can use the Soup and Salad variants in costumers_eat. Enums aren't very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enums variants is to be public.

// ~ Bringing paths into scope with the use keyword.
// It might seem like the path we've written to call functions so far are inconveniently  long and repetitive. 

// Fortunately, there's a way to simplfy this process. We can bring a path into a scope once and then call them in that path as if they're local items with the use keyword.

// We bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant function so we only have to specify hosting::add_to_waitlist to call the function in the following function:

//We "import" the module :
use crate::front_of_house::hosting;

fn test1(){
    hosting::add_to_waitlist(); // we use it here
}
// We can bring a item using use and a relative path, too:

// use self::front_of_house::hosting;

fn test2(){
    // hosting::add_to_waitlist();
}

// ~ creating an idiomatic use paths:
// If we wonder why do we use: crate::front_of_house::hosting
// rather than specifying all the way out to the add_to_waitlist function
// to archieve the same result.

// Bringing the function's parent module into scope with use means we have to specify the parent moduke when calling the function. 
// Specifying the parent module when calling the function
// makes it clear that the function isn't locally defined while still minimizing repetition of the full path .

// While importing Structs and enums it's idiomatic to specify the full path.
// A exception to it if we import two modules with the same struct's name, we should import it by using it's parent

// ~ Providing new names with the as keyword.
// There's another solution to the problem of bringin two types of the same name into the same scope with use : after the path we can specify as  and a new local name alias, for the type.

// ~ Re exporting using pub use

// When we bring a name into scppe with the use keyword, the name available in the new scope is private. 
// To enable the code that calls our code to refer to that name as if it had been defined in that code's scope, we can combine pub and use keywords.
// This technique is called re-exporting because we're bringin an item into scope but also making that item available for others to bring into their scope

// ~ using nested paths to clean up large use lists
// If we're using multiple item defined in the the same module, listing each item on its own line can take up a lot of vertical space in our files.
// This way:
// use std::cmp::Ordering;
// use std::io;

// Instead, we can do:
// use std::{cmp::Ordering, io};

// We can use nested path at any level in a path, which is useful when combining two use statements that share a subpath:
// use std::io;
// use std::io::Write;

// Instead, we can do:
// use std::io{Self, Write}; // Self refers to std::io itself

//~ Glob operator;
// If we want to bring all public items defined in a path into scope, we can specify that path followed by *, the glob operator.

// use std::collections::*

// Be careful when using the glob operator, Glob can make it harder to tell what names ar in scope and where a name used in your was defined

// ~ Separating modules into different files:
// So far, all the eamples in this chaptes defined multiple modules in one file.
// But as the module get bigger, you migth want to move their definitions to a separate file to make the code easier to navigate.

// Let's  go to the new_restaurant folder