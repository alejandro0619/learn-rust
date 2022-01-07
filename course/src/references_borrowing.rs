// We can share a variable to a chunk of code (function) without that function taking ownership of the value you just passed.
// Let's use something called references.

fn test_one(){
  let s1 = String::from("Hello");
  let length = calculate_length(&s1); // & means "address of"
  println!("{}", length);
  let mut s2 = String::from("Hello!!")l
  modify_string(&s2);

}
// The ampersant preceding the String type tell the function that it will receive the address where a String value is stored
// But it is not the owner of that value.
fn calculate_length(s: &String) -> usize {
  s.len()
}

// To modify the value we are borrowing we need to:
// &mut means we are borrowing the value of the variable and we are going to modify it.
fn modify_string(s: &mut String) {
  s.push_str(", World");
}