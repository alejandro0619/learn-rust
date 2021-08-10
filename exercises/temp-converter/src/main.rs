use std::io;

fn main() {
    println!("Welcome, please enter the temperature in F");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp)
        .expect("Failed at read the line");

    let temp:f64 = temp.trim().parse().unwrap();

    println!("{} Celcius ",converter(temp));
}

fn converter(fahrenheit: f64) -> f64 {
    let result = (fahrenheit - 32.00) * 5. / 9.;
    return result
}
