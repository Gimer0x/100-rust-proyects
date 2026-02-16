// Day 100: Project: AI Prompt Pack CLI Manager
// For your grand finale, build a CLI app that lets users create, edit, tag, and search 
// AI prompts — a personal “prompt pack” manager! You'll practice file-based storage, 
// search and filter logic, and clean CLI design.
// Features:
// + Add a prompt with title, content, and tags
// + View all saved prompts
// + Search prompts by tag (e.g. summarize, email, creative)
// 
// Key Concepts:
// + JSON file as a lightweight DB
// + UUID for unique prompt IDs
// + Rust CLI patterns: stdin, flush, serde I/O
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use uuid::Uuid;
 
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Prompt {
    id: String,
    title: String,
    tags: Vec<String>,
    content: String,
}
 
fn load_prompts() -> Vec<Prompt> {
    fs::read_to_string("prompts.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}
 
fn save_prompts(prompts: &[Prompt]) {
    let json = serde_json::to_string_pretty(prompts).unwrap();
    fs::write("prompts.json", json).expect("Failed to save prompts");
}
 
fn add_prompt() {
    let mut title = String::new();
    let mut content = String::new();
    let mut tags = String::new();
 
    print!("Title: "); io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();
 
    print!("Prompt content: "); io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content).unwrap();
 
    print!("Tags (comma-separated): "); io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tags).unwrap();
 
    let prompt = Prompt {
        id: Uuid::new_v4().to_string(),
        title: title.trim().to_string(),
        content: content.trim().to_string(),
        tags: tags.split(',').map(|s| s.trim().to_string()).collect(),
    };
 
    let mut prompts = load_prompts();
    prompts.push(prompt);
    save_prompts(&prompts);
    println!("Prompt added!");
}
 
fn list_prompts() {
    let prompts = load_prompts();
    for (i, prompt) in prompts.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, prompt.title, prompt.tags.join(", "));
    }
}
 
fn search_by_tag(tag: &str) {
    let prompts = load_prompts();
    let results: Vec<_> = prompts
        .into_iter()
        .filter(|p| p.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
        .collect();
 
    if results.is_empty() {
        println!("No prompts found with tag '{}'", tag);
    } else {
        for prompt in results {
            println!("{}: {}\n{}\n", prompt.title, prompt.tags.join(", "), prompt.content);
        }
    }
}
 
fn main() {
    println!("Prompt Pack CLI");
    println!("1) Add Prompt");
    println!("2) List Prompts");
    println!("3) Search by Tag");
    print!("Choose option: ");
    io::stdout().flush().unwrap();
 
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
 
    match choice.trim() {
        "1" => add_prompt(),
        "2" => list_prompts(),
        "3" => {
            print!("Enter tag: "); 
            io::stdout().flush().unwrap();
            let mut tag = String::new();
            io::stdin().read_line(&mut tag).unwrap();
            search_by_tag(tag.trim());
        }
        _ => println!("Invalid option"),
    }
}