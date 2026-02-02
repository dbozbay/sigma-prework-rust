use chrono::{Local, NaiveDate, Datelike};
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct DateOfBirth {
    date: NaiveDate,
}

impl FromStr for DateOfBirth {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(|date| DateOfBirth { date })
            .map_err(|_| "Invalid date format. Please use YYYY-MM-DD.")
    }
}

impl DateOfBirth {
    fn calculate_age(&self) -> u32 {
        let today = Local::now().date_naive();
        let mut age = today.year() - self.date.year();

        // Subtract 1 if birthday hasn't occurred yet this year
        if (today.month(), today.day()) < (self.date.month(), self.date.day()) {
            age -= 1;
        }

        age as u32
    }
}

/// Prompt user for a valid date of birth in YYYY-MM-DD format
fn get_valid_date_of_birth() -> DateOfBirth {
    loop {
        println!("Enter your date of birth (YYYY-MM-DD): ");

        let mut dob_input = String::new();
        io::stdin()
            .read_line(&mut dob_input)
            .expect("Failed to read input");

        match dob_input.trim().parse() {
            Ok(dob) => return dob,
            Err(err) => println!("{err}"),
        }
    }
}

fn main() {
    let dob = get_valid_date_of_birth();
    let age = dob.calculate_age();
    println!("You are {} years old.", age);
}
