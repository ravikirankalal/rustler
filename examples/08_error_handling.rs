// Error Handling Example
// This example demonstrates Result and Option types, error handling patterns
//
// To run this example: cargo run --example 08_error_handling

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    println!("=== Error Handling in Rust ===\n");
    
    // === OPTION TYPE ===
    
    println!("--- Option Type ---");
    
    // Option represents a value that might or might not exist
    let some_number = Some(42);
    let no_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);
    
    // Pattern matching with Option
    match some_number {
        Some(value) => println!("Found value: {}", value),
        None => println!("No value found"),
    }
    
    match no_number {
        Some(value) => println!("Found value: {}", value),
        None => println!("No value found"),
    }
    
    // Using if let for concise pattern matching
    if let Some(value) = some_number {
        println!("Value using if let: {}", value);
    }
    
    // === OPTION METHODS ===
    
    println!("\n--- Option Methods ---");
    
    let maybe_number = Some(10);
    
    // unwrap() - gets the value or panics
    let value = maybe_number.unwrap();
    println!("Unwrapped value: {}", value);
    
    // unwrap_or() - gets the value or returns a default
    let no_value: Option<i32> = None;
    let default_value = no_value.unwrap_or(0);
    println!("Value with default: {}", default_value);
    
    // unwrap_or_else() - gets the value or computes a default
    let computed_default = no_value.unwrap_or_else(|| {
        println!("Computing default value");
        42
    });
    println!("Computed default: {}", computed_default);
    
    // expect() - like unwrap but with custom panic message
    let expected_value = maybe_number.expect("Expected a number but got None");
    println!("Expected value: {}", expected_value);
    
    // is_some() and is_none()
    println!("Has value? {}", maybe_number.is_some());
    println!("Is empty? {}", no_value.is_none());
    
    // === OPTION TRANSFORMATIONS ===
    
    println!("\n--- Option Transformations ---");
    
    let number = Some(5);
    
    // map() - transform the value if it exists
    let doubled = number.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);
    
    let empty: Option<i32> = None;
    let doubled_empty = empty.map(|x| x * 2);
    println!("Doubled empty: {:?}", doubled_empty);
    
    // and_then() - chain operations that return Option
    let result = number.and_then(|x| {
        if x > 0 {
            Some(x * x)
        } else {
            None
        }
    });
    println!("Conditional square: {:?}", result);
    
    // filter() - keep value only if predicate is true
    let filtered = number.filter(|&x| x > 3);
    println!("Filtered (>3): {:?}", filtered);
    
    // === RESULT TYPE ===
    
    println!("\n--- Result Type ---");
    
    // Result represents success (Ok) or failure (Err)
    let good_result: Result<i32, String> = Ok(42);
    let bad_result: Result<i32, String> = Err(String::from("Something went wrong"));
    
    println!("Good result: {:?}", good_result);
    println!("Bad result: {:?}", bad_result);
    
    // Pattern matching with Result
    match good_result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    match bad_result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    // === RESULT METHODS ===
    
    println!("\n--- Result Methods ---");
    
    let success: Result<i32, String> = Ok(100);
    let failure: Result<i32, String> = Err(String::from("Failed"));
    
    // unwrap() and expect() (same as Option)
    let value = success.clone().unwrap();
    println!("Unwrapped success: {}", value);
    
    // unwrap_or() with Result
    let default_on_error = failure.clone().unwrap_or(0);
    println!("Default on error: {}", default_on_error);
    
    // is_ok() and is_err()
    println!("Is success ok? {}", success.is_ok());
    println!("Is failure error? {}", failure.is_err());
    
    // === RESULT TRANSFORMATIONS ===
    
    println!("\n--- Result Transformations ---");
    
    let number: Result<i32, String> = Ok(5);
    
    // map() - transform the Ok value
    let doubled = number.clone().map(|x| x * 2);
    println!("Doubled result: {:?}", doubled);
    
    // map_err() - transform the Err value
    let error: Result<i32, String> = Err(String::from("error"));
    let mapped_error = error.map_err(|e| format!("Mapped: {}", e));
    println!("Mapped error: {:?}", mapped_error);
    
    // and_then() - chain operations that return Result
    let chained = number.and_then(|x| {
        if x > 0 {
            Ok(x * x)
        } else {
            Err(String::from("Negative number"))
        }
    });
    println!("Chained result: {:?}", chained);
    
    // === PRACTICAL ERROR HANDLING ===
    
    println!("\n--- Practical Error Handling ---");
    
    // Safe division function
    let result1 = safe_divide(10.0, 2.0);
    let result2 = safe_divide(10.0, 0.0);
    
    handle_division_result(result1);
    handle_division_result(result2);
    
    // Parsing numbers with error handling
    let numbers = vec!["42", "not_a_number", "100", "invalid"];
    
    for number_str in numbers {
        match parse_number(number_str) {
            Ok(num) => println!("Parsed '{}' as: {}", number_str, num),
            Err(e) => println!("Failed to parse '{}': {}", number_str, e),
        }
    }
    
    // === ERROR PROPAGATION ===
    
    println!("\n--- Error Propagation ---");
    
    // Using ? operator for error propagation
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Failed to read username: {}", e),
    }
    
    // Chaining operations with ?
    match calculate_average_from_strings(vec!["10", "20", "30"]) {
        Ok(avg) => println!("Average: {}", avg),
        Err(e) => println!("Failed to calculate average: {}", e),
    }
    
    match calculate_average_from_strings(vec!["10", "invalid", "30"]) {
        Ok(avg) => println!("Average: {}", avg),
        Err(e) => println!("Failed to calculate average: {}", e),
    }
    
    // === CUSTOM ERROR TYPES ===
    
    println!("\n--- Custom Error Types ---");
    
    let operations = vec![
        ("10", "+", "5"),
        ("20", "-", "8"),
        ("invalid", "*", "3"),
        ("12", "/", "0"),
        ("15", "%", "4"), // Unsupported operation
    ];
    
    for (a, op, b) in operations {
        match calculate(a, op, b) {
            Ok(result) => println!("{} {} {} = {}", a, op, b, result),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    // === OPTION AND RESULT COMBINATIONS ===
    
    println!("\n--- Option and Result Combinations ---");
    
    let items = vec![
        Some("42"),
        Some("not_a_number"),
        None,
        Some("100"),
    ];
    
    for (i, item) in items.iter().enumerate() {
        let result = item
            .ok_or("Missing value")
            .and_then(|s| s.parse::<i32>().map_err(|_| "Invalid number"));
        
        match result {
            Ok(num) => println!("Item {}: {}", i, num),
            Err(e) => println!("Item {}: Error - {}", i, e),
        }
    }
    
    // === ERROR HANDLING PATTERNS ===
    
    println!("\n--- Error Handling Patterns ---");
    
    // Pattern 1: Early return with ?
    match process_data("valid_data") {
        Ok(result) => println!("Processed: {}", result),
        Err(e) => println!("Processing failed: {}", e),
    }
    
    // Pattern 2: Collecting results
    let strings = vec!["1", "2", "invalid", "4"];
    let results: Result<Vec<i32>, _> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    match results {
        Ok(numbers) => println!("All parsed successfully: {:?}", numbers),
        Err(e) => println!("Failed to parse all numbers: {}", e),
    }
    
    // Pattern 3: Partition successes and failures
    let results: Vec<Result<i32, _>> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    let (successes, failures): (Vec<_>, Vec<_>) = results
        .into_iter()
        .partition(Result::is_ok);
    
    let successes: Vec<i32> = successes.into_iter().map(Result::unwrap).collect();
    let failures: Vec<_> = failures.into_iter().map(Result::unwrap_err).collect();
    
    println!("Successes: {:?}", successes);
    println!("Failures: {:?}", failures);
    
    println!("\n=== Key Takeaways ===");
    println!("• Option<T> handles presence/absence of values (Some/None)");
    println!("• Result<T, E> handles success/failure scenarios (Ok/Err)");
    println!("• Use pattern matching or methods like unwrap_or for safe handling");
    println!("• The ? operator enables concise error propagation");
    println!("• Custom error types provide better error information");
    println!("• Rust forces you to handle errors explicitly - no silent failures!");
}

// === HELPER FUNCTIONS ===

// Safe division that returns a Result
fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

// Handle division result
fn handle_division_result(result: Result<f64, String>) {
    match result {
        Ok(value) => println!("Division result: {}", value),
        Err(error) => println!("Division error: {}", error),
    }
}

// Parse a string to number with error handling
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

// Simulate reading username from file (will fail since file doesn't exist)
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    let mut file = File::open("username.txt")?; // ? propagates the error
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Calculate average from string numbers using error propagation
fn calculate_average_from_strings(numbers: Vec<&str>) -> Result<f64, ParseIntError> {
    let mut sum = 0;
    let mut count = 0;
    
    for number_str in numbers {
        let number = number_str.parse::<i32>()?; // ? propagates parse errors
        sum += number;
        count += 1;
    }
    
    Ok(sum as f64 / count as f64)
}

// === CUSTOM ERROR TYPES ===

#[derive(Debug)]
enum CalculationError {
    InvalidNumber(String),
    DivisionByZero,
    UnsupportedOperation(String),
}

impl std::fmt::Display for CalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalculationError::InvalidNumber(msg) => write!(f, "Invalid number: {}", msg),
            CalculationError::DivisionByZero => write!(f, "Division by zero"),
            CalculationError::UnsupportedOperation(op) => {
                write!(f, "Unsupported operation: {}", op)
            }
        }
    }
}

// Calculator with custom error type
fn calculate(a: &str, operation: &str, b: &str) -> Result<i32, CalculationError> {
    let num_a = a.parse::<i32>()
        .map_err(|_| CalculationError::InvalidNumber(a.to_string()))?;
    let num_b = b.parse::<i32>()
        .map_err(|_| CalculationError::InvalidNumber(b.to_string()))?;
    
    match operation {
        "+" => Ok(num_a + num_b),
        "-" => Ok(num_a - num_b),
        "*" => Ok(num_a * num_b),
        "/" => {
            if num_b == 0 {
                Err(CalculationError::DivisionByZero)
            } else {
                Ok(num_a / num_b)
            }
        },
        _ => Err(CalculationError::UnsupportedOperation(operation.to_string())),
    }
}

// Data processing with early return
fn process_data(data: &str) -> Result<String, String> {
    if data.is_empty() {
        return Err("Data is empty".to_string());
    }
    
    if data == "invalid_data" {
        return Err("Invalid data format".to_string());
    }
    
    // Simulate processing
    Ok(format!("Processed: {}", data.to_uppercase()))
}