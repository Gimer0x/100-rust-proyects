use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    // Generate a Rnd number
    let random_number = rand::rng().random_range(1..=100);

    loop {
        println!("Input a number: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let guess:u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small.."),
            Ordering::Greater => println!("Too big.."),
            Ordering::Equal => {
                println!("Congratulations!");
                break;
            }
        }
    }
}
