use std::fmt::Display;

// Box allows us to store data on the heap while keeping its pointer in the stack (duh)
// let's imlement a Cons List.
// Since rust needs to know how much space to allocate for the enum bellow, we cannot implement this data structure using the traditional ways.
//let's allocate the enum on the heap.

pub enum List<T> {
  Cons(T, Box<List<T>>),
  Nil
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Cons(ref num, ref cons) => write!(f, "{} {}", num, cons),
            List::Nil => write!(f, "\n")
        }
    }
}
