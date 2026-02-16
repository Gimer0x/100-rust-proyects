use std::io;

fn main() {
    println!("Input number: ");

    let value = match get_input() {
        Some(num) => num,
        None => {
            println!("Invalid input value!");
            return;
        }
    };

    if value <= 1 {
        println!("Value must be greater thatn 1");
    }

    // Evaluate if value is prime number.
    println!("Is {} a prime number: {}", value, is_prime(value));
}

fn is_prime(value: u32) -> bool {
    if value <= 1 {
        return false;
    }

    if value == 2 {
        return true;
    }

    if value % 2 == 0 {
        return false;
    }

    let limit = (value as f64).sqrt() as u32 + 1;

    for i in 3..=limit {
        if value % i == 0 {
            return false;
        }
    }

    true
}

fn get_input() -> Option<u32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("No input value");

    match input.trim().parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => None
    }
}


