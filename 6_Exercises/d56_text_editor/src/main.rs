// Day 56: Project: Build a Rust-Based Text Editor (Mini Editor)
// Build a basic text editor in the terminal using Rust. You'll open, edit, 
// and save text files line-by-line. This project introduces file I/O, string 
// manipulation, and creating a stateful interactive CLI tool.
// Key Concepts:
// + BufReader and .lines() for file reading.
// + File::create() and writeln!() for saving.
// + Stateful loop to simulate editor behavior.
// You’ve now built a basic text editor that can create, modify, and save 
// files—the foundation for more powerful CLI apps, IDE plugins, or developer tools.
use std::fs::{File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    println!("Rust Mini Text Editor");
    let file_path = prompt("Enter file to open or create: ");
    let mut lines = load_file(&file_path);

    loop {
        println!("\nCurrent file: {}", file_path);
        display_lines(&lines);

        println!("\nCommands:");
        println!("1. Add line");
        println!("2. Edit line");
        println!("3. Delete line");
        println!("4. Save");
        println!("5. Exit");

        let choice = prompt("Select an option: ");

        match choice.trim() {
            "1" => {
                let new_line = prompt("Enter new line: ");
                lines.push(new_line);
            }
            "2" => {
                let idx = prompt("Line number: ").parse::<usize>().unwrap_or(0);
                if idx == 0 || idx > lines.len(){
                    println!("Invalid line number");
                } else {
                    let updated = prompt("Enter new text: ");
                    lines[idx - 1] = updated;
                }
            }
            "3" => {
                let idx = prompt("Line number: ").parse::<usize>().unwrap_or(0);
                if idx == 0 || idx > lines.len() {
                    println!("Invalid line number.");
                } else {
                    lines.remove(idx - 1);
                }
            }
            "4" => {
                if let Err(e) = save_file(&file_path, &lines) {
                    println!("Save failed: {}", e);
                } else {
                    println!("File saved!");
                }
            }
            "5" => break,
            _ => println!("Incorrect choice!"),
        }
    }
}

fn save_file(path: &str, lines: &[String]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for line in lines {
        // macro that writes formatted text to a writer and adds a newline at the end.
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

fn display_lines(lines: &[String]) {
    println!("\n---- File Content ----");
    for (i, line) in lines.iter().enumerate() {
        println!("{:>3}: {}", i + 1, line);
    }
}

fn load_file(path: &str) -> Vec<String> {
    if Path::new(path).exists() {
        // Opens the file at path. Returns Result<File, std::io::Error>.
        // .unwrap() takes the Ok(File) value, or panics if there was an 
        // error (e.g. file not found, no permission).
        let file = File::open(path).unwrap();
        // Wraps the File in a buffered reader. Reads from the file in chunks 
        // instead of byte-by-byte, which is more efficient when reading many lines.

        // Result::ok turns a Result<T, E> into Option<T>:
        // Ok(x) → Some(x)
        // Err(_) → None
        // filter_map keeps only the Some values and throws away the Nones.
        // So: “Drop any line that was an error, keep only the successful lines as String.”
        BufReader::new(file)
            .lines() // Returns an iterator over the lines of the reader.
            .filter_map(Result::ok) // Result::ok turns a Result<T, E> into Option<T>:
            .collect() // Consumes the iterator and collects the items into a collection.
    } else {
        vec![]
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
