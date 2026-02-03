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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maxmin() {
        assert_eq!(maxmin(&vec![3, 1, 4, 1, -5]), [4, -5]);
        assert_eq!(maxmin(&vec![10, 20, 30, 40, 50]), [50, 10]);
        assert_eq!(maxmin(&vec![-1, -2, -3, -4, -5]), [-1, -5]);
        assert_eq!(maxmin(&vec![0, 0, 0, 0]), [0, 0]);
    }

    #[test]
    fn test_parse_numbers_valid_input() {
        assert_eq!(parse_numbers("1 2 3").unwrap(), vec![1, 2, 3]);
        assert_eq!(parse_numbers("4,5,6").unwrap(), vec![4, 5, 6]);
        assert_eq!(parse_numbers("7;8;9").unwrap(), vec![7, 8, 9]);
        assert_eq!(parse_numbers("10:11:12").unwrap(), vec![10, 11, 12]);
    }

    #[test]
    fn test_parse_numbers_invalid_input() {
        assert!(parse_numbers("1, 2, three").is_err());
        assert!(parse_numbers("4; five; 6").is_err());
        assert!(parse_numbers("seven:8:9").is_err());
    }
}
