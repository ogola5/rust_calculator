enum Operation {
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}
fn calculate (operation:Operation) -> f64 {
    use Operation::*;

    match operation {
        Add(a,b) => a +b,
        Subtract(a,b ) => a -b,
        Multiply(a,b ) => a *b,
        Divide(a,b ) => {
            if b != 0.0{
                a/b
            }else{
                println!("Cannot divide by zero");
                0.0
            }
        }
    }
}

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut operator = String::new();
    let mut input2 = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let input1: f64 = input1.trim().parse().expect("Please type a number!");

    println!("Enter the operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("Failed to read line");

    println!("Enter the second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let input2: f64 = input2.trim().parse().expect("Please type a number!");

    let operation = match operator.trim() {
        "+" => Operation::Add(input1, input2),
        "-" => Operation::Subtract(input1, input2),
        "*" => Operation::Multiply(input1, input2),
        "/" => Operation::Divide(input1, input2),
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    let result = calculate(operation);
    println!("The result is: {}", result);
}
