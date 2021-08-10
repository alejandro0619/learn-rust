use std::collections::HashMap;

fn calculate_media(vector: &Vec<i32>) -> i32 {
  let mut  acc = 0;
  let mut media = 0;
  
   for x in 0..vector.len() {
      acc += vector[x];

      media = acc / vector.len() as i32;
    };
    return media;
 }
 
fn medium(vector: &Vec<i32>) -> i32{
    let medium = vector.len() as i32 / 2;

    return medium;
}

fn duplicated(vector: &Vec<i32>) -> HashMap<&i32, i32> {

   let mut map = HashMap::new();

    for x in vector.iter() {
        let count = map.entry(x).or_insert(0);
            *count += 1;
    }
    return map;
}

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5, 5];
    
    
    
    println!("{} -- {}-- {:?}",calculate_media(&vec1),
    medium(&vec1), duplicated(&vec1));
}
