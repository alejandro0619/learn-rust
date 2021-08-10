fn main() {
    println!("Fibonacci generator");
    println!("{}", fibonacci(20));
    let string_one: String = String::from("Hola mundo");
    lenght(&string_one);
}

fn fibonacci(n: u32) -> u32{
   match n {
       0 => 1,
       1 => 1,
       _ => fibonacci(n - 1) + fibonacci(n -2),
   }

}

fn lenght(s: &String) -> usize{
    println!("{}", s);
    return s.len();
}

fn meet (){
    println!("Hello world");

}
