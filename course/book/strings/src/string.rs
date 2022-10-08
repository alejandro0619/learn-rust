pub fn strings(){
    // We talked about Strins already, but we'll look at them in a more depth now.
    // New Rustaceans commonly get stuck on strings for a combination of three reasons:
    // Rust's propensity for exposing possible errors
    // strings being a more complicated data structure than many others
    // and UTF-8.
    // These factors combine in a way that can seem difficult when
    // you're coming from other programming languages.
    //
    // We discuss strings in the context of collections because strings are
    // implemented as collection of bytes, plus som methods to provide a useful
    // functionality when those bytes are interpreted as texts.
    //
    // In this section, we'll talk the operations on String that every 
    // collection type has, such as creating, updating, and reading.
    // We'll also discuss the ways in which String is different from other
    // collections, namely how to indexing into a string is complicated
    // by the differences between how people and computers interpret 
    // String data.
    
    let mut s = String::new(); // New empty string, mutable, so we can modify it.
    s.push_str("Hello");  //  Adding str = "Hello
    s.push_str("World!"); // Adding str = "World!"

    println!("We can convert a {}", "str to a String".to_string()); // Converting str to String with the method to_string()

    // We can't index on a string, since all strings are UTF-8 encoded, indexing without caution might return a invalidad character:
    // This &"hola"[0] should return "h" since the lenght of the string is 4, that means, each character takes up to 1 bytes
    // but &"你好"[0] will not return '你', because the string is 6 elements long, that means, a character takes up more bytes to be represented
    // So as accessing to an index might return a invalidad UTF-8 encoded character.

    // We can break a String into slices, let's create a slice for hello and other for world!
    // But again, this could lead to errors.
    let hello = &s[..=4];
    let world = &s[5..];
    println!("Hello string slice: {}, world string slice: {}",hello,  world);

}

pub fn iter_over_strings(){

    let str_test = String::from("Hello world, 你好 in chinese");

    // We can convert it into a iterable collection of chars:

    for c in str_test.chars(){
            println!("{}", c);
    }
    // Or even in collection of bytes, remember valid UTF-8 encoded character may be made up of more than 1 byte.
    for b in str_test.bytes(){
        println!("{}", b);
    }
}
