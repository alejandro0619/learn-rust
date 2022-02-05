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

// Earlier, w mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate's module structure, known as a module tree.

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
