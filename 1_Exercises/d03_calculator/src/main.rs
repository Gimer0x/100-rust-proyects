use std::io;

fn add(a: f64, b:f64) -> f64{
    a + b
}

fn substract(a: f64, b:f64) -> f64{
    a - b
}

fn multiplication(a: f64, b:f64) -> f64{
    a * b
}

fn division(a: f64, b:f64) -> f64{
    if b == 0.0 {
        println!("Division by zero not valid!");
        std::process::exit(1);
    }
    a / b
}

fn main() {
    println!("Calculator: ");
    println!("Emter your expression: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    let exp: Vec<&str> = input.trim().split_whitespace().collect();

    if exp.len() != 3 {
        println!("Expression invalid!");
        return;
    }

    let a:f64 = match exp[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number a");
            return;
        }
    };

    let operator = exp[1];

    let b:f64 = match exp[2].parse() {
        Ok(num) => num,
        Err(_) => {
                println!("Invalid number b");
                return;
            }
    };

    let result: f64 = match operator {
        "+" => add(a,b),
        "-" => substract(a,b),
        "*" => multiplication(a,b),
        "/" => division(a,b),
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Result: {:.2}", result);
}
