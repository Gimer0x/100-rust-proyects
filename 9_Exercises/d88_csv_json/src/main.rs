// Day 88: Project: CSV to JSON Converter Tool (with Serde)
// Build a CLI tool that reads a CSV file and converts it to JSON format. 
// You’ll use serde, serde_json, and csv crates to parse and serialize 
// structured data in Rust.
// Key Concepts:
// + csv::Reader to read structured rows
// + serde::Serialize to auto-convert records to JSON
// + serde_json::to_string_pretty() for pretty formatting
// + env::args() and std::fs for file handling
// You now have a reliable converter from CSV to JSON—ideal for data transformation, 
// ETL pipelines, or API mocking.
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::env;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    name: String,
    email: String,
    age: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
 
    if args.len() != 3 {
        eprintln!("Usage: {} <input.csv> <output.json>", args[0]);
        return;
    }
 
    let input_path = &args[1];
    let output_path = &args[2];
 
    let file = File::open(input_path).expect("Failed to open CSV file");
    let mut reader = ReaderBuilder::new().from_reader(file);
 
    let mut records = Vec::new();
    for result in reader.deserialize::<Record>() {
        let record = result.expect("Failed to parse a row");
        records.push(record);
    }
 
    let json = to_string_pretty(&records).expect("Failed to serialize JSON");
    std::fs::write(output_path, json).expect("Failed to write JSON");
 
    println!("Converted {} → {}", input_path, output_path);
}
