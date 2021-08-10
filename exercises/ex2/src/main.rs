fn pigify(string: &mut str) -> String {
    let  c = string.chars().next().unwrap();
    match c {
        'a' | 'e' | 'i' | 'o'| 'u' =>{
            let hay = String::from("hay");
            let result = format!("{}{}", string, hay);
            result
        },
        _ =>{

            let first_char = string.chars().next().unwrap();
            let ay = String::from("ay");
            let this_string = string.to_string();
            let this_string = this_string.replace(first_char, "");
            
            let result = format!("{}{}{}", this_string, first_char, ay );
            result
        
       
        }
    }
}
fn main(){
    let mut s = String::from("World");
    println!("{}", pigify(&mut s));

}