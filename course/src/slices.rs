// Another data type that doesn't have ownership is the slices.
// Slices let you contiguous sequence of elements in a collecion rather than the whole collection.

//  * STRING SLICES
// Here's a small programing problem: write a function that takes a string and returns the first word it finds in the string. If the function doesn't find a space in the string, the whole string must be one word, so the entire string should be returned.

 // first_word
 fn first_word(s: &String) -> usize {
   let bytes = s.as_bytes();
   for(index, &item) in  bytes.iter().enumerate() {
     if item == b' ' {
       return index;
     }
   }
   return s.len()
 }

 fn test(){
  let mut s = String::from("Hello World, from Rust");
  let result = first_word(&s); // will return the value 5
  s.clear(); // this empties the String, making it equal to ""
}
 // This program compiles without any errors and would also do so if we used word after calling s.clear(). Because word isn't connected to the state of s at all;
 // word still contains the value of 5. We could use that value 5 with the variable s to try to extract the first word out, but this would be a bug because the contents of s have changed since we saved 5 in the word.

 // Having to worry about the index in word getting out of sync with the data in s is a tedious and error prone! Managing these indices is even more brittle is we write a second_word function like this:

 // Now we are tracking a starting and an ending index. And we have even more values that were calculated from data in a particular state but aren't tied to that state at all. We now have three unrelated variables floating around that need to be kept in sync.

// Luckily, Rust has a solution to this problem: String slices.
 fn second_word(s: &String){
  // A String Slice is a referene to part of a String, and it looks like this:
  let s = String::from("Hello World");
  let hello = &s[0..=4];
  let world = &s[6..=10];
  // This is similar to taking reference to the whole  String
  // But with the extra [0..=4] bit. Rather than a reference to the entire String, it's a reference to a portion of the String.
 }

 // Now, knowing how does str works, let's return the first word of given string:

 //           Before it accepted &String
 fn get_first_word( string: &str) -> &str {
  let bytes = string.as_bytes(); // convert the string slice into a collection of bytes
  for ( i, &item ) in bytes.iter().enumerate() { // creates a iterator and wraps an index (enumerate) that returns a tuple, so we can deconstruct it (i, &item) and loop through it
    if item == b' ' { // check if the actual item is a space
        return &string[0..i] // returns a slice that contains the word
    }
  }
  return &string[..]; // returns that word
 }

pub fn test_get_first_word(){
  // Because we changed from accepting &String to &str
  // We can use both &String and &str values
  // If we have a String we can pass a slices of the String or a reference to the String
  // The flexibility takes advantage of deref coercions.
  //This makes our API more general without losing functionality.
  // We declare a String's type variable:
  let s = String::from("Hello World");
  let word =  get_first_word(&s); // pass a reference
  let word = get_first_word(&s[..]); // Pass a Slice, wheter whole
  let word = get_first_word(&s[..=5]); // Or partial
  
  let str = "Hello world";
  let word = get_first_word(&str[0..=5]); // Works on slices of string literals wheter partials
  let word = get_first_word(&str[..]); // or whole
  let word = get_first_word(str); // Because string literals are string slices already this will work too without the slice syntax
 }
