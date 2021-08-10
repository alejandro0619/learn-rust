#[derive(Debug)]
enum UsState {
    Alabama, 
    Alaska,
    // -- snippets --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coins: Coin) -> u32 {
    match coins {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => {
            println!("Lucky Penny");
            1
        },
        Coin::Dime => {
            println!("Lucky Penny");
            10
        },
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}
fn main(){
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    other_u8_value();
    some_u8_value();
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn some_u8_value(){
    let an_u8 = Some(0u8);
    match an_u8 {
        Some(3) => println!("Ready"),
        _ => (),
    }
}
fn other_u8_value(){
    let an_u8 = Some(0u8);
    if let Some(3) = an_u8 {
        println!("Caught!!!!");
    }
}
