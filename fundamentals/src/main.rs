// add function
fn add_func(x: u8, y:u8){
    let result = x + y;
    println!("The result is {}", result)
}
// conditional function:
fn conditionals(x: i8) -> bool{
     let true_var = if x !=0{
        true
    } else{
        false
    };
    return true_var;
}

//Comienza el programa aqui:

fn main() {
    // variables:
    // inmutables:
    let _x = 6;
    
    // mutables:
    let mut _y = 9;
    _y = 10;
    

    // Constantes:
    const _MAX_POINT: u32 = 100_000;
    

    // Shadowing:

    
    let _variable_one = 5;
    let _variable_one = _variable_one + 20;
    let _variable_one = _variable_one * 2;


    // Exercise of shadowing:
    let _user_input = "     ";
    let _user_input = _user_input.len();
    

    // Tipos de datos escalares:


    // Int:
    let _my_num = 6;

    //Floating:

    let _a = 2.001840512411505054121; //f64
    let _b: f32 = 3.01805218550512581505151510550; //f32


     //Operaciones aritmeticas:


    //suma
    let mut _num_one = 5;
    let mut _num_two= 5;
    let _add = _num_one + _num_two;

    //resta:
    _num_one = 9;
    _num_two = 4;
    let _minus= _num_one - _num_two;
    
    // Multiplicación:
    _num_one = 4;
    _num_two = 3;
    let _multiply = _num_one * _num_two;

    //división:
    _num_one = 1;
    _num_two = 2;
    let _divide = _num_one / _num_two;


    //Boolean: 
    let _t = true; // notacion implicita
    let _f: bool = false; // notacion explicita

    // Un solo carácter:

    let _c1 = 'a';
    let _c2: char = 'b'; // declaración explícita

    // Tipos compuestos:

    // Tupla:

    let _my_tup: (i32, f64, u8) = (50, 5.1, 2);

    //Desestructuración de tuplas por concordancia de patrones:
    let (_c, _v, _b) = _my_tup;
    
    //Arrays:

    let _array_one = [1, 2, 3, 4, 5];

    // Las matrices tienen un aspecto interesante; se trata de la definición: [tipo; número]. Por ejemplo:
    let _array_two : [i32; 4] = [1, 2, 3, 4]; //explícito

    //acceder a elementos:
    _array_two[0]; // 1

    add_func(8, 9);
    let result_one = conditionals(0);
    println!("{}", result_one);


} 
