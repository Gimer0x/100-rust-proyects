// The "self" part imports the module name io into the current scope.
// So “you can write io::...” instead of the full path std::io::...
// Write is a trait inside std::io
// .flush() is a method from the Write trait
use std::io::{self, Write};

fn main() {
    println!("Sorting Demo!");

    loop {
        println!("\nChoose type to sort:");
        println!("1. Integers");
        println!("2. Words");
        println!("3. Exit");

        match get_input("Select and option: ").as_str() {
            "1" => {
                let raw = get_input("Enter comma-separated integers:");

                let mut nums: Vec<i32> = raw
                    .split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();

                bubble_sort(&mut nums);
                println!("Sorted: {:?}", nums);
            }
            "2" => {
                let raw = get_input("Enter comma-separated integers:");

                let mut words: Vec<String> = raw
                        .split(',')
                        .filter_map(|s| Some(s.trim().to_string()))
                        .collect();
                
                insertion_sort(&mut words);

                println!("Sorted: {:?}", words);
            }
            "3" => break,
            _ => println!("Incorrect option!")
        }
    }
}

fn insertion_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        // .clone() creates a new owned value so key can own it without borrowing from arr.
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
}

// Generic bubble sort
// fn gen_bubble_sort<T: PartialOrd>(arr: &mut [T]) {
//     let len = arr.len();
//     for i in 0..len {
//         for j in 0..len - i - 1 {
//             if arr[j] > arr[j + 1] {
//                 arr.swap(j, j + 1);
//             }
//         }
//     }
// }

fn bubble_sort(nums: &mut Vec<i32>) {
    let len = nums.len();

    for i in 0..len {
        for j in 0..len - i - 1{
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1);
            }
        }
    }
}
 

fn get_input(prompt: &str) -> String{
    println!("{}", prompt);

    let mut input = String::new();
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Incorrect input");

    input.trim().to_string()
}
