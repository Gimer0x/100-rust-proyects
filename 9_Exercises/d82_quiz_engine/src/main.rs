// Day 82: Project: Interactive Quiz Engine (CSV-Based Quiz Game)
// Build a CLI-based quiz app that reads questions and answers from a CSV file 
// and quizzes the user. You'll use csv crate for parsing, and practice control 
// flow, scoring, and clean CLI design.
// Key Concepts:
// + csv::Reader for parsing tabular text
// + serde::Deserialize for mapping rows to structs
// + Clean CLI with flush, read_line, and user feedback
// You now have a CLI quiz game that's easy to scale, localize, or plug into a web UI!
use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct Question {
    question: String,
    answer: String,
}
 

fn main() {
    let file = File::open("questions.csv").expect("Failed to open CSV file");

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut score = 0;
    let mut total = 0;

    println!("Welcome to the Rust Quiz!");
    println!("-------------------------\n");

    for result in reader.deserialize::<Question>() {
        let q: Question = result.expect("Failed to parse rows!");
        total += 1;

        let user_answer = print_question(&q.question);

        if user_answer.eq_ignore_ascii_case(&q.answer) {
            println!("✅ Correct!\n");
            score += 1;
        } else {
            println!("❌ Incorrect! Correct answer: {}\n", q.answer);
        }
    }

    println!("You scored {}/{}!", score, total);
}

fn print_question(question: &str) -> String{
    println!("❓ {}", question);
    print!("Your answer: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let user_answer = input.trim();

    user_answer.to_string()
}
