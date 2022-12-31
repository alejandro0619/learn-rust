// Smart pointers.
//
// Box<T>
// Rc<T>

mod example_box;
mod example_rc;
use example_box::List::{Cons, Nil};
use example_rc::List;
use std::rc::Rc;

fn main() {
    // Box example
    println!("{}", Cons(2, Box::new(Cons(4, Box::new(Nil)))));
    // Rc example
    let a = Rc::new(List::Cons(2, Rc::new(List::Cons(3, Rc::new(List::Nil)))));
    let b = List::Cons(5, Rc::clone(&a));
    let c = List::Cons(37, Rc::clone(&a));

    println!("{}\n{}\n{}\n", a, b, c);
    
}
