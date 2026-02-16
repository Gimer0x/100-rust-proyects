use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    
    let numbers = vec![1,3,5,7,9,11,13];
    let words = vec!["apple", "banana", "cherry", "date", "fig", "grape"];

    println!("\n1. Search Numbers");
    println!("2. Search Words");
    
    let choice = input("Select option: ");

    match choice.as_str() {
        "1" => {
            let query = input("Enter value to search: ");

            if let Ok(q) = query.parse::<i32>() {
                match binary_search(&numbers, &q){
                    Some(index) => println!("Found at index: {} ", index),
                    None => println!("Not found!")
                }
            } else {
                println!("Invalid number!");
            }
        }
        "2" => {
            let query = input("Enter value to search: ");
            match binary_search(&words, &(query.as_str())) {
                Some(index) => println!("Found at index: {} ", index),
                None => println!("Not found!")
            }
        }
        _ => println!("Incorrect choice!")

    }
}

fn binary_search<T: PartialOrd>(list: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let mid = (low + high) / 2;

        match list[mid].partial_cmp(target).unwrap() {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }

    None
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
