use std::{error::Error, io};

use chrono::{Datelike, Local, NaiveDate};

/// Prompt user for a date of birth in YYYY-MM-DD format
fn get_user_dob() -> io::Result<String> {
    println!("Enter your date of birth (YYYY-MM-DD): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// Parse the input date of birth into a NaiveDate
fn parse_date(input: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .map_err(|_| "Invalid date format. Please use YYYY-MM-DD.".to_string())
}

/// Calculate age based on date of birth
fn calculate_age(dob: NaiveDate) -> Result<u32, String> {
    let today = Local::now().date_naive();

    if dob > today {
        return Err("Invalid date of birth. Cannot be in the future.".to_string());
    }

    let mut age = today.year() - dob.year();

    // Subtract 1 if birthday hasn't occurred yet this year
    if (today.month(), today.day()) < (dob.month(), dob.day()) {
        age -= 1;
    }

    Ok(age as u32)
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let input = get_user_dob()?;

        match parse_date(&input) {
            Ok(dob) => match calculate_age(dob) {
                Ok(age) => {
                    println!("You are {age} years old.");
                    break;
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }

    Ok(())
}
