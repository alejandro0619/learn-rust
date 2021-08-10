fn main() {
    // creating a string:
    let mut s = String::new();
    let data = "Data";
    let s1 = data.to_string();
    let s2 = "data 2".to_string();
    let s3 = String::from("is the same as to.string");

    // updating a string:
    s.push_str("bar");
    let paragraph = s1 + &s2;
    println!("{}", paragraph);

    // format:
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // works like println! but it returns a String, doesn't return a display:
    let p = format!("{}-{}-{}", tic, tac, toe);

    // indexing string:
    let s4 = "hello";
    let slice = &s4[..2];
    for c in "Hello guys".chars() {
        println!("{}", c);
    }
    
}
