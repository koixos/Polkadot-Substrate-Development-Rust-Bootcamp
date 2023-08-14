fn main() {
    // OPTION & RESULT

    // OPTION
    // generally used to see if we have a value since there is not NULL in Rust
    
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("Square root of {number} is {value}."),
        None => println!("Square root of {number} is not a real number."),
    }

    // RESULT

    // enum Result<T, E> {
    //    Ok(T),
    //    Err(E),
    //}

    let a = 10.0;
    let b = 0.0;
    let c = 5.2;

    let division1 = divide(a, b);

    match division1 {
        Ok(value) => println!("{a} divided by {b} is {value}."),
        Err(msg) => println!("{msg}"),
    }

    let division2 = divide(a, c);

    match division2 {
        Ok(value) => println!("{a} divided by {b} is {value}."),
        Err(msg) => println!("Error: {msg}"),
    }

    // EXAMPLE
    let base = get_from_database("base");
    let height = get_from_database("height");
    
    let area = calculate_triangle_area(base, height);

    match area {
        Ok(a) => println!("Calculated area of given triangle is {a}"),
        Err(msg) => println!("Error: {msg}"),
    }
}

fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a / b)
    }
}

fn get_from_database(key: &str) -> Option<f64> {
    let database: [(&str, Option<f64>); 2] = [("base", Some(4.0)), ("height", None)];

    for (k, v) in database {
        if k == key {
            return v;
        }
    }

    None
}

fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height must be positive numbers".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        },
        (None, _) => Err("The base is missing".to_string()), 
        (_, None) => Err("The height is missing".to_string()),
    }
}