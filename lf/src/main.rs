fn main() {
    {
        let x = 5;
        
        let r = &x;
        
        println!("{}", r);
    }
    // works
    let str1 = String::from("Hola");
    {
        let string2 = String::from("xyz");
        let result = longest(str1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // lifetime static:
    let s: &'static str = "I have a static lifetime";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x 
    } else {
        y
    }
}

//lifetime in structs:
struct ImportanExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportanExcerpt<'a>{
    fn level(&self) -> i32{
        4
    }
}
// apply third elision rules:
impl<'a> ImportanExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn stuff(){
    let novel = String::from("what is this");
    let first_sentence = novel.split('.')
    .next()
    .expect("Could not finda '.' ");
    let i = ImportanExcerpt {
         part: first_sentence
        };
}

// reglas de Elision:
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }   
    &s[..]
}
