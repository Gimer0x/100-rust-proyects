use std::io;

fn main() {
    println!("BMI Calculator");

    // Get Weight
    println!("Get weight: ");
    let weight = match get_input_value() {
        Some(value) => value,
        None => {
            println!("Invalid input for weight");
            return;
        }
    };
    // Get Height
    println!("Get height: ");
    let height = match get_input_value() {
        Some(value) => value,
        None => {
            println!("Incorrect input for height");
            return;
        }
    };

    // Compute the BMI and classify the results
    let bmi = get_bmi(weight, height);

    println!("Your bmi is {:.1}", bmi);

    let category = classify_bmi(bmi);
    println!("BMI category: {}", category);
}

fn classify_bmi(bmi:f64) -> &'static str{
    if bmi < 18.5 {
        "Underweight"
    } else if bmi >= 18.5 && bmi < 24.9 {
        "Normal"
    } else if bmi >= 24.9 && bmi < 29.9{
        "Overweight"
    } else {
        "Obesity"
    }
}

fn get_bmi(weight:f64, height:f64) -> f64 {
    weight / (height * height)
}

fn get_input_value() -> Option<f64>{
    // Declar variable as string
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read!");

    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}