// Day 85: Project: Git Commit Message Helper
// Create a CLI tool that reads staged file names from git status, analyzes them, and suggests 
// contextual commit message templates (e.g., feat, fix, refactor). You’ll use std::process::Command 
// to run shell commands and parse Git output.
// Key Concepts:
// + Run shell commands with std::process::Command
// + Parse staged file list from git diff --cached --name-only
// + Heuristics to suggest commit prefixes: feat, fix, docs, test, chore
// You now have a smart commit helper that improves consistency and productivity in teams 
// and solo workflows.
use std::process::Command;

fn main() {
    println!("Git Commit Message Helper\n");
 
    // Run `git diff --cached --name-only`
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .expect("Failed to run git");
 
    if !output.status.success() {
        eprintln!("Not a git repository or no staged files.");
        return;
    }
 
    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<&str> = stdout.lines().collect();
 
    if files.is_empty() {
        println!("No staged files found.");
        return;
    }
 
    println!("Staged files:");
    for file in &files {
        println!("• {}", file);
    }
 
    println!("\nSuggested commit types:");
    for file in &files {
        if file.contains("test") {
            println!("- test: add test related changes");
        } else if file.contains("README") || file.ends_with(".md") {
            println!("- docs: update documentation");
        } else if file.ends_with(".rs") {
            println!("- feat: add feature in {}", file);
        } else if file.contains("fix") || file.contains("bug") {
            println!("- fix: fix bug in {}", file);
        } else {
            println!("- chore: update {}", file);
        }
    }
 
    println!("\nExample:");
    println!("git commit -m \"feat: add user profile API\"");
}