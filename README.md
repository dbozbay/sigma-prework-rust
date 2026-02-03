# Sigma Labs Pre-Course

A collection of programming exercises completed as part of the Sigma Labs pre-course program, implemented in Rust.

## Requirements

- Rust 1.70 or higher
- Cargo (comes with Rust)

## Installation

1. Install Rust using `rustup`:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Clone this repository:

    ```bash
    git clone https://github.com/dbozbay/sigma-prework-rust.git
    cd sigma-prework-rust
    ```

3. The project is ready to use! Cargo will automatically handle dependencies when you build or run the project.

## Exercises

### 1. Maximum and Minimum Finder

Prompts the user to enter a list of integers and returns the highest and lowest numbers without using built-in max/min functions.

**Usage:**
```bash
# Build and run the project
cargo run --bin maxmin

# Or build first, then run
cargo build --release
./target/release/maxmin
```

**Example:**
```
Enter a list of integers: 2, 4, 1, 0, 2, -1
The maximum and minimum values are: [4, -1]
```

### 2. Age Calculator

Asks the user for their date of birth and calculates their current age.

**Usage:**
```bash
# Build and run the project
cargo run --bin age

# Or build first, then run
cargo build --release
./target/release/age
```

**Example:**
```
Enter your date of birth (YYYY-MM-DD): 1995-06-15
You are 29 years old.
```

## Testing

Run the test suite to verify all exercises work correctly:

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test maxmin
cargo test age
```

## License

This project is for educational purposes.