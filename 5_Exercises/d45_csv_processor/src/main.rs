// Day 45: Project: CSV File Processor
// Build a CLI tool that reads a .csv file, filters rows based on 
// user-specified criteria, and prints results. This project teaches 
// you to work with tabular data, field access, and the powerful csv 
// crate in Rust.
// Key Concepts:
// + Use csv::Reader to parse files.
// + Access rows and columns with headers.
// + Apply filters with flexible criteria.
// With this, you're now able to build data validators, converters, 
// extractors, and ETL tools that deal with structured data.
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use csv::ReaderBuilder;

fn main() {
    let file_path = input("Enter path to .csv file: ");
    let column_name = input("Enter column to filter by: ");
    let keyword = input("Enter value to match: ");

    if let Err(e) = filter_csv(&file_path, &column_name, &keyword){
        eprintln!("Error: {}", e)
    }
}

fn filter_csv(path: &str, column: &str, value: &str) -> Result<(), Box<dyn Error>>{
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().from_reader(file);

    let headers = reader.headers()?.clone();
    // .iter() creates an iterator over the headers
    // Returns: Iterator<Item = &String>
    // .position(|h| h == column)
    // Searches the iterator for the first element matching a condition
    // |h| is a closure parameter; h is &String (each header)
    // h == column compares the header with column (a &str from user input)
    // Returns: Option<usize>
    // Some(index) if found (e.g., Some(1) if "Age" is at index 1)
    // None if not found
    let col_index = headers.iter()
        .position(|h| h == column)
        // Converts Option<usize> to Result<usize, &str>
        // If Some(index): Ok(index)
        // If None: Err("Column not found")
        // The ? operator:
        // If Ok(index): unwraps and assigns index to col_index
        // If Err(...): returns early from the function with that error
        .ok_or("Column not found")?;

    println!("Matching rows: ");
    // &s[..] converts &String to &str
    // s is &String, s[..] is a full slice of the string (type str)
    // &s[..] takes a reference, yielding &str
    // join() works on Vec<&str>, not Vec<&String>
    println!("{}", headers.iter().map(|s| &s[..]).collect::<Vec<_>>().join(" , "));

    for result in reader.records() {
        let record = result?;
        // .get(col_index), gets the value at the column index col_index
        // Returns: Option<&str>
        // Some(&str) if the index is valid, None if out of bounds
        // Example: if col_index = 1 and record = ["Alice", "25", "New York"], 
        // this returns Some("25")
        // .unwrap_or("") -> Handles the Option<&str> from .get()
        // If Some(value), returns value, if None, returns ""
        // Result: always a &str (never None)
        if record.get(col_index).unwrap_or("") == value {
            println!("{}", record.iter().collect::<Vec<_>>().join(" , "));
        }
    }


    Ok(())
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}