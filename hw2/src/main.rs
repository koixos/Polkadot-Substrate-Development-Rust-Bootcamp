use std::{self, io::{self, Write}, num::ParseFloatError};

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut operation = String::new();
 
    //taking first number
    print!("Enter the first number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut a).expect("failed to readline");

    //taking operation sign
    print!("Enter the operation: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut operation).expect("failed to readline");

    //taking second number
    print!("Enter the second number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut b).expect("failed to readline");

    //parsing numbers from String to f64
    let parsed_a = parse(a);
    let parsed_b = parse(b);

    //checking if given values are valid
    match (parsed_a, parsed_b) {
        (Ok(a), Ok(b)) => {
            let mut result_value: Result<f64, String>; 
            //matching given operation with enum value
            match operation {
                _ => result_value = calculate(Operation::Add(a, b)),
                /*String::from("-") => calculate(Operation::Subtract(a, b)),
                String::from("*") => calculate(Operation::Multiply(a, b)),
                String::from("/") => calculate(Operation::Divide(a, b)),
                _ => println!("Please enter a valid operation!"),
            */}
            match result_value {
                Ok(r) => println!("{r}"),
                Err(msg) => println!("Error: {msg}"),
            }
            
        },
        _ => println!("Please enter valid inputs!"),
    }
}

fn parse(value: String) -> Result<f64, ParseFloatError> {
    let parsed_value = value.parse::<f64>();
    parsed_value
}

fn calculate(operation: Operation) -> Result<f64, String> {
    match operation {
        Operation::Add(a, b) => {
            Ok(a + b)
        },
        Operation::Subtract(a, b) => {
            Ok(a - b)
        },
        Operation::Multiply(a, b) => {
            Ok(a * b)
        },
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err("Divide by zero error".to_string())
            } else {
                Ok(a / b)
            }
        },
    }
}