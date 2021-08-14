pub fn main() {
  // Variables can be type annotated.
  let _logical: bool = true;

  let _a_float: f64 = 1.0;  // Regular annotation
  let _an_integer   = 5i32; // Suffix annotation

  // Or a default will be used.
  let _default_float   = 3.0; // `f64`
  let _default_integer = 7;   // `i32`
  
  // A type can also be inferred from context 
  let mut _inferred_type = 12; // Type i64 is inferred from another line
  _inferred_type = 4294967296i64;
  
  // A mutable variable's value can be changed.
  let mut _mutable = 12; // Mutable `i32`
  _mutable = 21;
  
  // Error! The type of a variable can't be changed.
  // mutable = true;
  
  // Variables can be overwritten with shadowing.
  let _mutable = true;
}
