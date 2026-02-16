// Import crates for oppening and reading a file
use std::env;
use std::fs::File;
use std::io::{Read};

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Type: cargo run <file_path>");
    }

    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    //Read the file contents
    let mut file = match File::open(file_path) {
        Ok(num) => num,
        Err(err) => {
            println!("Error opening file {}", err);
            return;
        }
    };

    let mut contents = String::new();

    // Copy the content of the file to contents variable after opening the file
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", err);
        return;
    }

    // Count words
    let word_count = count_words(&contents);
    println!("Word Count: {}", word_count);
}

fn count_words(text: &str) -> usize{
    text.split_whitespace().count()
}
