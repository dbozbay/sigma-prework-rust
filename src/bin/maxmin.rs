use std::io;

/// Prompts the user for a list of integers and validates the input.
fn get_valid_numbers() -> Vec<i32> {
    loop {
        println!("Enter a list of integers separated by commas: ");

        let mut numbers_input = String::new();

        io::stdin()
            .read_line(&mut numbers_input)
            .expect("Failed to read input.");

        let result: Result<Vec<i32>, _> = numbers_input
            .split(",")
            .map(|s| s.trim().parse::<i32>())
            .collect();

        match result {
            Ok(numbers) if numbers.len() >= 2 => return numbers,
            Ok(_) => println!("Please enter at least two integers."),
            Err(_) => println!("Invalid input. Please enter integers only."),
        }
    }
}

/// Returns the maximum and minimum values from a slice of integers.
fn maxmin(numbers: &[i32]) -> [i32; 2] {
    let mut lowest = numbers[0];
    let mut highest = numbers[0];

    for &num in &numbers[1..] {
        if num < lowest {
            lowest = num;
        }

        if num > highest {
            highest = num;
        }
    }

    [highest, lowest]
}

fn main() {
    let numbers = get_valid_numbers();
    println!("The maximum and minimum values are: {:?}", maxmin(&numbers));
}
