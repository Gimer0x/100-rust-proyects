use std::io;

fn main() {
    println!("Get input: ");
    // Get the input string
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input!");

    // Clean the string
    let cleaned_string = clean_string(&input);
    // Return if palindrome

    if cleaned_string.is_empty() {
        println!("Input a valid string!");
        return;
    }

    if is_palindrome(&cleaned_string) {
        println!("{}: is palindrome!", input.trim());
    } else{
        println!("{}: is not a palindrome!", input.trim());
    }
}
fn is_palindrome(input: &str) -> bool {
    input == input.chars().rev().collect::<String>()
}

fn clean_string(input:&str) -> String {
    input
        .chars() // Iterate over each character
        .filter(|c| c.is_alphanumeric()) // Keep only letters and numbers
        .map(|c| c.to_lowercase().to_string()) // Convert to lowercase
        .collect::<String>() // Collect into a new String
}
