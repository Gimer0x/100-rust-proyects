// Day 57: Project: Key-Value Store
// Build a simple persistent key-value store that supports set, get, and 
// delete commands via CLI. The data is stored in a JSON file, making it 
// a lightweight alternative to Redis or LevelDB for local storage.
// Key Concepts:
// + HashMap as in-memory DB.
// + serde_json for persistence.
// + Command pattern for extensible CLI.
// This project is your stepping stone to building CLI-based databases, 
// caches, or config managers.
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use serde::{Deserialize, Serialize};

const   STORE_FILE: &str = "key_store.json";

// #[...]
// This is an attribute: it attaches metadata to the next item (here, a struct or enum).
// The compiler and macros read attributes to change how that item is compiled or used.
// Placement: #[derive(...)] is written above the struct/enum it applies to.
#[derive(Serialize, Deserialize)]
struct Store {
    data: HashMap<String, String>,
}

// #[derive(Serialize, Deserialize)]
// derive is a built-in attribute that means: “automatically implement these traits for this type.”
// You list one or more trait names inside the parentheses.
// The compiler (with help from the listed traits’ “derive macros”) generates the trait implementation for you.
// So: “Generate implementations of Serialize and Deserialize for this type.”

impl Store {
    fn new() -> Self{
        if let Ok(file) = File::open(STORE_FILE){
            let reader = BufReader::new(file);
            if let Ok(store) = serde_json::from_reader(reader){
                return store;
            }
        }
        Store { data: HashMap::new() }
    }

    fn save(&self) {
        // OpenOptions::new() Builds a configurable file open (read/write, create, truncate, etc.).
        if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(STORE_FILE) {
            serde_json::to_writer_pretty(&mut file, self).unwrap();
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
        self.save();
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.save();
    }

    fn list(&self) {
        println!("Stored Keys:");
        for key in self.data.keys() {
            println!("- {}", key);
        }
    }
}

fn main() {
    println!("Key-Value Store (set/get/delete/list/exit)");
    let mut store = Store::new();

    loop {
        let cmd = prompt("\n> ");
        let parts: Vec<&str> = cmd.trim().splitn(3, ' ').collect();

        match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
            Some("set") if parts.len() == 3 => {
                    store.set(parts[1].to_string(), parts[2].to_string());
                    println!("Key set!");
                }
            Some("get") if parts.len() == 2 => {
                match store.get(parts[1]) {
                    Some(value) => println!("Value: {}", value),
                    None => println!("Key not found."),
                }
            }
            Some("delete") if parts.len() == 2 => {
                store.delete(parts[1]);
                println!("Key deleted (if it existed).");
            }
            Some("list") => store.list(),
            Some("exit") => {
                println!("Exiting!");
                break;
            }
            _ => println!("Invalid command!")
        }
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
