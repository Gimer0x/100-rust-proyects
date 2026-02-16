// Day 41: Project: Directory Scanner
// Build a CLI tool that scans a directory and prints all file and folder names recursively. 
// You’ll learn to work with file metadata, path traversal, and recursive logic using std::fs 
// and std::path.

// Key Concepts:
// fs::read_dir() – Read contents of a folder.
// file_type.is_file() and .is_dir() – Detect types.
// Recursive traversal with depth-based indentation.
// This project gives you the tools to build file explorers, backup tools, 
// or search utilities—and lays the groundwork for working with the OS!
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    println!("Directory Scanner!");

    let dir = input("Enter a directory path: ");
    let path = Path::new(&dir);

    if path.exists() && path.is_dir() {
        println!("Scanning '{}", dir);
        scan_dir(path, 0);
    } else {
        println!("Invalid directory!");
    }
}

fn scan_dir(path: &Path, depth: usize) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();
                let name = entry.file_name().into_string().unwrap_or_default();
                let indent = " ".repeat(depth);

                if file_type.is_dir() {
                    println!("{} {}.{}", indent, depth + 1, name);
                    scan_dir(&entry.path(), depth + 1);
                } else if file_type.is_file() {
                    println!("{} {}", indent, name);
                }
            }
        }
    } else {
        // {:?} is used to print types that only support Debug, like Path.
        println!("Failed to read contents of {:?}", path);
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
