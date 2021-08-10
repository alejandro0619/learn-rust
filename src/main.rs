// new hashMap:
use std::collections::HashMap;
fn puntuation_team(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yelow"), 50);

    // zipping: tuples vec into a hashmap
    let teams = vec![String::from("Blue"), String::from("yellow")];
    let initial_score = vec![10, 50];

    // zipping method:
    let score: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();

}

fn stuff_fnc(){
    let field_name: String = String::from("A name");
    let field_value: String = String::from("A value");
    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    map.insert(String::from("Hola"), String::from("Mundo"));
}

fn get_value(){
    let mut map = HashMap::new();
    let x = 9;
    let y = 2;
    map.insert(x, y);
    let get_method = map.get(&x);
    if let Some(2) = get_method {
        println!("the value is 2");
    } else {
        println!("No match with value 2");
    }

}

fn iter_over_hashmap(){
    let mut score = HashMap::new();
    score.insert(String::from("Blue"), String::from("20"));
    score.insert(String::from("yellow"), String::from("240"));
    for (k, v) in &score {
        println!("keys:{} scores:{}", k, v);
    }
}
fn mod_hashmap(){
    //rewritting:
    let mut score = HashMap::new();
    score.insert(String::from("key 1"), String::from("Value 1"));
    score.insert(String::from("key 1"), String::from("Value 2"));
    //insert value if keyword doesnt have value already
    score.insert(String::from("key 3"), String::from("Value 3"));
    score.entry(String::from("key 3")).or_insert(String::from("Value 4"));
    //update based in before value
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    println!("{:?}", text.split_whitespace());
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
fn main() {
    puntuation_team();
    stuff_fnc();
    get_value();
    iter_over_hashmap();
    mod_hashmap();

}
