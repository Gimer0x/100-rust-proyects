// This brinds a module, use std::io::Read brinds the Read trait into scope.
use std::io;

fn get_temperature() -> f64{
    let mut input_value = String::new();
    io::stdin().read_line(&mut input_value).expect("Failed to read input");

    let temp:f64 = match input_value.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Incorrect input");
            return 0.0;
        }
    };

    temp
}

fn celcius_to_farenheit(){
    let temp = get_temperature();

    if temp != 0.0 {
        let farenheit = (temp * 9.0 / 5.0) + 32.0;
        println!("{:.2} C is {:.2} F", temp, farenheit);
    }
}

fn farenheit_to_celcius(){
    let temp = get_temperature();

    if temp != 0.0 {
        let celsius = (temp - 32.0) * 5.0 / 9.0;
        println!("{:.2} F is {:.2} C", temp, celsius);
    }
}

fn main() {
    println!("Temperature Converter");
    println!("1. Celsius to Farenheit");
    println!("2. Farenheit to Celcius");
    println!("Select one option: ");

    // Creation of a string variable
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    // Result<T, E> is an enum with two variants Ok(T) and Err(E)
    let new_choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice");
            return;
        }
    };

    if new_choice == 1{
        celcius_to_farenheit();
    } else if new_choice == 2 {
        farenheit_to_celcius();
    } else {
        println!("Invalid choice!");
    }
}
