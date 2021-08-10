use std::collections::HashMap;

fn input() -> Vec<String> {
    use std::io;
    let mut employees: Vec<String> = Vec::new();

    while employees.len() < 5{
        println!("Please enter name employee and its departament");

        let mut employee = String::new();
    
        io::stdin().read_line(&mut employee)
            .expect("Cannot read the line");

        let employee = employee.replace("\n", "");

        employees.push(employee);
    }
    employees.sort();
    return employees;
}

fn main() {
    let mut employee = input();
}
