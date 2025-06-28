// Functions Example
// This example demonstrates function declaration, parameters, return values, and closures
//
// To run this example: cargo run --example 04_functions

fn main() {
    println!("=== Functions in Rust ===\n");
    
    // === BASIC FUNCTION CALLS ===
    
    println!("--- Basic Function Calls ---");
    
    // Calling a simple function
    greet();
    
    // Function with parameters
    greet_person("Alice");
    
    // Function with return value
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // Function with multiple parameters and return
    let result = calculate_area(4.0, 6.0);
    println!("Area of rectangle (4.0 x 6.0): {:.2}", result);
    
    // === EXPRESSION VS STATEMENT ===
    
    println!("\n--- Expression vs Statement ---");
    
    let number = 42;
    let doubled = double(number);
    println!("Double of {} is {}", number, doubled);
    
    // Demonstrating early return
    let grade = check_grade(85);
    println!("Grade for score 85: {}", grade);
    
    // === FUNCTION WITH MULTIPLE RETURN TYPES ===
    
    println!("\n--- Functions with Different Return Scenarios ---");
    
    let divide_result1 = safe_divide(10.0, 2.0);
    let divide_result2 = safe_divide(10.0, 0.0);
    
    match divide_result1 {
        Some(result) => println!("10.0 / 2.0 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    match divide_result2 {
        Some(result) => println!("10.0 / 0.0 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    // === HIGHER-ORDER FUNCTIONS ===
    
    println!("\n--- Higher-Order Functions ---");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Original: {:?}", numbers);
    println!("Squared: {:?}", squared);
    
    let sum_of_squares: i32 = numbers.iter().map(|&x| x * x).sum();
    println!("Sum of squares: {}", sum_of_squares);
    
    // Using function as parameter
    let operation_result = apply_operation(10, 3, add);
    println!("Applying add function: {}", operation_result);
    
    // === CLOSURES ===
    
    println!("\n--- Closures ---");
    
    // Simple closure
    let multiply_by_two = |x| x * 2;
    println!("Multiply 5 by 2: {}", multiply_by_two(5));
    
    // Closure with type annotations
    let divide_by: fn(i32, i32) -> i32 = |a, b| a / b;
    println!("12 divided by 3: {}", divide_by(12, 3));
    
    // Closure capturing environment
    let multiplier = 3;
    let multiply_by_captured = |x| x * multiplier;
    println!("Multiply 7 by captured value {}: {}", multiplier, multiply_by_captured(7));
    
    // Mutable closure
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    
    println!("Counter: {}", increment());
    println!("Counter: {}", increment());
    println!("Counter: {}", increment());
    
    // === CLOSURE EXAMPLES WITH COLLECTIONS ===
    
    println!("\n--- Closures with Collections ---");
    
    let words = vec!["hello", "world", "rust", "programming"];
    
    // Filter words with more than 4 characters
    let long_words: Vec<&str> = words.iter()
        .filter(|&&word| word.len() > 4)
        .cloned()
        .collect();
    println!("Words longer than 4 chars: {:?}", long_words);
    
    // Find first word starting with 'r'
    let r_word = words.iter()
        .find(|&&word| word.starts_with('r'));
    match r_word {
        Some(&word) => println!("First word starting with 'r': {}", word),
        None => println!("No word starting with 'r' found"),
    }
    
    // Transform and collect
    let capitalized: Vec<String> = words.iter()
        .map(|&word| word.to_uppercase())
        .collect();
    println!("Capitalized: {:?}", capitalized);
    
    // === FUNCTION POINTERS ===
    
    println!("\n--- Function Pointers ---");
    
    let math_ops = [add, subtract, multiply];
    for (i, op) in math_ops.iter().enumerate() {
        let result = op(12, 4);
        let op_name = match i {
            0 => "add",
            1 => "subtract", 
            2 => "multiply",
            _ => "unknown",
        };
        println!("12 {} 4 = {}", op_name, result);
    }
    
    // === RECURSIVE FUNCTIONS ===
    
    println!("\n--- Recursive Functions ---");
    
    let factorial_5 = factorial(5);
    println!("Factorial of 5: {}", factorial_5);
    
    let fib_10 = fibonacci(10);
    println!("10th Fibonacci number: {}", fib_10);
    
    // === METHODS VS FUNCTIONS ===
    
    println!("\n--- Methods vs Functions ---");
    
    let rect = Rectangle::new(5.0, 3.0);
    println!("Rectangle: {}x{}", rect.width, rect.height);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    
    let rect2 = Rectangle::new(4.0, 4.0);
    println!("Can rectangle fit in rect2? {}", rect.can_fit(&rect2));
    
    println!("\n=== Key Takeaways ===");
    println!("• Functions are declared with 'fn' keyword");
    println!("• Parameters must have type annotations");
    println!("• Return type comes after '->'");
    println!("• Last expression is returned (no semicolon)");
    println!("• Closures capture their environment");
    println!("• Closures are anonymous functions with |param| syntax");
    println!("• Functions can be passed as parameters");
    println!("• Methods are functions associated with types");
}

// === FUNCTION DEFINITIONS ===

// Simple function with no parameters or return value
fn greet() {
    println!("Hello from a function!");
}

// Function with parameter
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Function with parameters and return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon - this is returned
}

// Function with early return
fn check_grade(score: i32) -> &'static str {
    if score >= 90 {
        return "A";  // Early return with 'return' keyword
    }
    if score >= 80 {
        return "B";
    }
    if score >= 70 {
        return "C";
    }
    if score >= 60 {
        return "D";
    }
    "F"  // Default return (no semicolon)
}

// Function calculating area
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

// Function demonstrating expression-based return
fn double(x: i32) -> i32 {
    x * 2  // Expression returned
}

// Function returning Option for error handling
fn safe_divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Higher-order function that takes another function as parameter
fn apply_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(a, b)
}

// More arithmetic functions for function pointer examples
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Recursive function for factorial
fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// Recursive function for Fibonacci
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// === STRUCT WITH METHODS ===

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function (like static method)
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Method that borrows self immutably
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    // Another method
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    // Method that takes another Rectangle as parameter
    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}