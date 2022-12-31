use std::rc::Rc;
use std::fmt::Display;

// When we want to keep track of all the references to an value
// And we don't want to deallocate the value until all references are gone
// We can implement Rc, we can copy the values, and we can know how many references the value has.
// It will be gone when strong_count: 0.
// Another thing is that we don't deep copy elements but we do increase the reference count.
// So it shouldn't have a bad impact of performance and memory usage.
pub enum List<T> {
  Cons(T, Rc<List<T>>),
  Nil
}

impl<T: Display> Display for List<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      List::Cons(ref num, cons) => {
        write!(f, "Count: {} | Item: {} | List: {}", Rc::strong_count(cons), num, cons )
      },
      List::Nil => write!(f, "\n")
    }
  }
}