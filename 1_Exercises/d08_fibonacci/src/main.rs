use std::io;

fn main() {
    // Get input value
    println!("Enter the number of terms: ");

    let terms = match validate_input() {
        Some(num) => num,
        None => {
            println!("Invalid input!");
            return;
        }
    };

    // Compute the fibonacci series and print the fibonacci values

    println!("Fibonacci serie: {:?}", generate_fibonacci(terms));
}

fn generate_fibonacci(num: u32) -> Vec<u64>{
    let mut sequence = vec!();
    if num >= 1 { sequence.push(0) };
    if num >= 2 { sequence.push(1) };
    
    for i in 2..=num {
        let next = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next);
    }

    sequence
}

fn validate_input() -> Option<u32>{

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input!");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None
    }

}
