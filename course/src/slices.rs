// Anotehr data type that doesn't have ownership is the slices.
// Slices let you contiguous sequence of elements in a collecion rather than the whole collection.

// Here's a small programing problem: write a function that takes a string and returns the first word it finds in the string. If the function doesn't find a space in the string, the whole string must be one word, so the entire string should be returned.

 // first_word
 fn first_word(s: &String) {
   let bytes = s.as_bytes();
   for(index, &item) in  bytes.iter().enumerate() {
     if item == b' ' {
       return i;
     }
   }
   s.len()
 }

 fn test(){
  let s = String::from("Hello World, from Rust");
  let result = first_word(&s); // will return the value 5
  s.clear(); // this empties the String, making it equal to ""
}
 // This program compiles without any errors and would also do so if we used word after calling s.clear(). Because word isn't connected to the state of s at all;
 // word still contains the value of 5. We could use thta value 5 with the variable s to try to extract the first word out, but this would be a bug because the contents of s have changed since we saved 5 in the word.

 // Having to worry about the index in word getting out of sync with the data in s is a tedious and error prone! Managing these indices is even more brttile is we write a second_word function.
 // Managing these indices is even more brittle if we write a second_word function like this:

 fn second_word() -> (usize, usize){

 }

// Now we are tracking a starting and an ending index. And we have even more values that were cslculated from data in a particular state but aren't tied to that state at all. We now have three unrelated variables floating around that need to be kept in sync.

// Luckily, Rust has a solution to this problem: String slices.