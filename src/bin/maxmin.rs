use std::{io, num::ParseIntError};

/// Prompts the user for input and returns the entered string.
fn get_user_input() -> String {
    println!("Enter a list of integers: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().to_string()
}

/// Parses a string of integers separated by spaces, commas, semicolons, or colons.
fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut result = input.to_string();

    for sep in [",", ";", ":"] {
        result = result.replace(sep, " ");
    }

    result
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()
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
    loop {
        let input = get_user_input();

        match parse_numbers(&input) {
            Ok(numbers) => {
                if numbers.len() < 2 {
                    println!("Please enter at least two integers.");
                    continue;
                }
                println!("The maximum and minimum values are: {:?}", maxmin(&numbers));
                break;
            }
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        }
    }
}
