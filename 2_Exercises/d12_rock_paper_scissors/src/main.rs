use std::io;
use rand::Rng;

enum GameResult {
    Win,
    Lose,
    Draw
}

fn main() {
    println!("Enter 'rock', 'paer', or 'scissors'. Type 'quit' to exit!");

    loop {
        println!("Make your choice: ");

        let user_choice = get_user_choice();

        if user_choice == "quit" {
            println!("Bye!");
            break;
        }

        let computer_choice = get_computer_choice();

        println!("Computer choice: {}", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GameResult::Win => println!("You win!"),
            GameResult::Lose => println!("You lose!"),
            GameResult::Draw => println!("It's a draw!"),
        }
    }
}

fn determine_winner(user: &str, computer:&str) -> GameResult {
    match (user, computer) {
        ("rock", "scissors") => GameResult::Win,
        ("scissors", "paper") => GameResult::Win,
        ("paper", "rock") => GameResult::Win,
        (a,b) if a == b => GameResult::Draw,
        _ => GameResult::Lose
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let index = rand::rng().random_range(0..choices.len());
    choices[index].to_string()
}

fn get_user_choice() -> String {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Invalid input!");

    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "rock" | "paper" | "scissors" | "quit" => choice,
        _ => {
            println!("Invalid choice! Choice again:");
            get_user_choice()
        }
    }
}

