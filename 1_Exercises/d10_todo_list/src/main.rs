// Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\nTo-Do List Menu:");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Delete Task");
        println!("5. Exit");

        let choice = get_input("Enter your choice");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("Tasks saved!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks!");
    let mut file = File::create("tasks.json").expect("Failed to create file!");
    file.write_all(json.as_bytes()).expect("Failed to write tasks to file!");
}

fn mark_task_completed(tasks: &mut Vec<Task>) {
    let id = get_input("Tasks to complete: ");
    if let Ok(id)  = id.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id){
            task.completed = true;
            println!("Tasks completed!");
        } else{
            println!("No task available!");
        }
    } else {
        println!("Invalid tasks!");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let id = get_input("Tasks to delete: ");

    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(index) = tasks.iter().position(|t| t.id == id){
            tasks.remove(index);
            println!("Tasks deleted!");
        }
    } else {
        println!("Invalid task!");
    }
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks!");
    } else {
        for task in tasks {
            let status = if task.completed {"✅"} else {"❌"};
            println!("{} - {} : {}", task.id, status, task.description)
        }
    }

}

fn add_task(tasks: &mut Vec<Task>){
    let description = get_input("Enter task description:");
    let id = tasks.len() + 1;

    tasks.push(Task {
        id,
        description: description.trim().to_string(),
        completed: false
    });
    println!("Tasks added!");
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Invalid input!");

    choice
}

fn load_tasks() -> Vec<Task>{
    match fs::read_to_string("tasks.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}
