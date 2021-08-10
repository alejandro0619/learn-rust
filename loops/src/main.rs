fn loops(){
    let mut counter: u32 = 0; //declare a mut var
    let result = loop {
        counter += 1; // add 1 to counter
        println!("{}", counter); //print

        if counter == 10 { // if count = 10 break
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

fn _while_loop(){
    let mut number: u32 = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

}

fn _for_array(){
    let array: [i32; 5]= [10, 20, 30, 40, 50];
    let mut index_array: usize = 0;
    while index_array < 5 {
        println!("The value is {}", array[index_array]);
        index_array = index_array + 1;
    }
    // Puede causar error si la longitud del array es erróneo, además agrega más tiempo de compilación.
}
fn better_for_array(){
    let a: [i32; 5] = [10 , 20, 30, 40, 50];
    for e in a.iter() {
        println!("The value is: {}", e)
    }
}

fn better_while_array(){
    for number in (1..4).rev(){
        println!("{}", number);
    }
}
fn main() {
    // loops function:println!
    loops();
    println!("---------------\n");
    // while:
    better_while_array();
    println!("---------------\n");
    // for by an array:
     better_for_array();
     println!("---------------\n");
}
