// Exercise:
// Given a String, find how many asterisks it has and return the number:

fn count_asterisks(string: &str) -> i32 {
    let mut count_asterisk: i32 = 0;
    let bytes = string.as_bytes(); // convert string to bytes
  
    for chars in bytes.iter() {
      if chars == &b'*' {
        count_asterisk = count_asterisk + 1;
      }
    }
    return  count_asterisk;
  }
  fn main() {
    let test_word = "Hell*o W*rld";
    let asterisks = count_asterisks(&test_word);
    println!("I found {} asterisks", asterisks);
  }
  