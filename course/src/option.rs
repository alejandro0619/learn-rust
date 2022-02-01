// Is another enum defined by the library, is used in many
// places because it encondes the very common scenario in which
// a value could be something or it could be nothing.

// Expressing this concept in terms of the type system means
// the compiler can check whether you've handled all the cases you should be handling;

// This functionality can prevent bugs that are extremely common in other programming languages

// The problem with null values is that if your try to use them as a not-null value, you'll get an error of some kind. Because this null or not-null property is pervasive, it's extremely  easy to make this kind of error.

// However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent.

let some_number  = Some(4);
let some_string = Some("Some string");

let absent_number: Option<i32> = None;
