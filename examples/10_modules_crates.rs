// Modules and Crates Example
// This example demonstrates organizing code, using Cargo, and basic external crate usage
//
// To run this example: cargo run --example 10_modules_crates

// For this example, we'll include the math module inline rather than as a separate file
// In a real project, you would organize modules in separate files
mod math_utils {
    /// Adds two numbers together
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Multiplies two numbers
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    /// Custom error type for math operations
    #[derive(Debug, PartialEq)]
    pub enum MathError {
        DivisionByZero,
    }

    /// Divides two floating point numbers
    pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
}
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }
    
    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }
        
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
        
        pub fn circumference(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }
    
    pub mod rectangle {
        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }
        
        impl Rectangle {
            pub fn new(width: f64, height: f64) -> Rectangle {
                Rectangle { width, height }
            }
            
            pub fn area(&self) -> f64 {
                self.width * self.height
            }
            
            pub fn perimeter(&self) -> f64 {
                2.0 * (self.width + self.height)
            }
        }
    }
}

// Using external dependencies (these would be in Cargo.toml)
use std::collections::HashMap;
use std::fs;
use std::env;

// Using items from our modules
use shapes::Circle;
use shapes::rectangle::Rectangle;
use math_utils::{add, multiply, divide};

fn main() {
    println!("=== Modules and Crates in Rust ===\n");
    
    // === MODULE BASICS ===
    
    println!("--- Module Basics ---");
    
    // Using functions from our math_utils module
    let sum = add(5, 3);
    let product = multiply(4, 7);
    
    println!("5 + 3 = {}", sum);
    println!("4 * 7 = {}", product);
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // === NESTED MODULES ===
    
    println!("\n--- Nested Modules ---");
    
    // Using types from nested modules
    let circle = Circle::new(5.0);
    println!("Circle with radius {}: area = {:.2}, circumference = {:.2}", 
             circle.radius, circle.area(), circle.circumference());
    
    let rectangle = Rectangle::new(4.0, 6.0);
    println!("Rectangle {}x{}: area = {:.2}, perimeter = {:.2}",
             rectangle.width, rectangle.height, rectangle.area(), rectangle.perimeter());
    
    // === USE STATEMENTS ===
    
    println!("\n--- Use Statements ---");
    
    // We can bring specific items into scope
    // Already done at the top with: use shapes::Circle;
    
    // We can also use glob imports (generally not recommended)
    // use shapes::*;
    
    // We can create aliases
    type Point = (f64, f64);
    let origin: Point = (0.0, 0.0);
    let point: Point = (3.0, 4.0);
    
    println!("Origin: {:?}", origin);
    println!("Point: {:?}", point);
    
    // === STANDARD LIBRARY MODULES ===
    
    println!("\n--- Standard Library Modules ---");
    
    // Collections module
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    
    println!("Scores: {:?}", scores);
    
    // Environment module
    println!("Current directory: {:?}", env::current_dir().unwrap_or_default());
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    println!("Command line arguments: {:?}", args);
    
    // Environment variables
    match env::var("HOME") {
        Ok(home) => println!("Home directory: {}", home),
        Err(_) => println!("HOME environment variable not set"),
    }
    
    // === FILE SYSTEM OPERATIONS ===
    
    println!("\n--- File System Operations ---");
    
    // Create a temporary file for demonstration
    let content = "Hello from Rust!\nThis is a test file.\nModules are awesome!";
    
    match fs::write("/tmp/test_file.txt", content) {
        Ok(()) => {
            println!("File written successfully");
            
            // Read the file back
            match fs::read_to_string("/tmp/test_file.txt") {
                Ok(file_content) => {
                    println!("File contents:");
                    for line in file_content.lines() {
                        println!("  {}", line);
                    }
                },
                Err(e) => println!("Error reading file: {}", e),
            }
            
            // Get file metadata
            match fs::metadata("/tmp/test_file.txt") {
                Ok(metadata) => {
                    println!("File size: {} bytes", metadata.len());
                    println!("Is file: {}", metadata.is_file());
                    println!("Is directory: {}", metadata.is_dir());
                },
                Err(e) => println!("Error getting metadata: {}", e),
            }
        },
        Err(e) => println!("Error writing file: {}", e),
    }
    
    // === PRIVACY AND VISIBILITY ===
    
    println!("\n--- Privacy and Visibility ---");
    
    // Demonstrate visibility rules
    let library = library::Library::new();
    println!("Created library with {} books", library.book_count());
    
    // This would not compile - private function
    // library::private_function();
    
    // === PATH SYNTAX ===
    
    println!("\n--- Path Syntax ---");
    
    // Absolute paths start from the crate root
    let circle2 = crate::shapes::Circle::new(3.0);
    println!("Circle2 area: {:.2}", circle2.area());
    
    // Relative paths start from the current module
    // Since we're in main, these are the same as absolute paths in this case
    
    // Self keyword refers to the current module
    demonstrate_self_usage();
    
    // Super keyword refers to the parent module
    // (demonstrated in nested modules)
    
    // === CARGO WORKSPACE CONCEPTS ===
    
    println!("\n--- Cargo and Project Structure ---");
    
    println!("Project structure explanation:");
    println!("  src/");
    println!("    main.rs           - Binary crate root");
    println!("    lib.rs            - Library crate root (if exists)");
    println!("    bin/              - Additional binaries");
    println!("    examples/         - Example programs (like this one!)");
    println!("  tests/              - Integration tests");
    println!("  benches/            - Benchmarks");
    println!("  Cargo.toml          - Package manifest");
    println!("  Cargo.lock          - Dependency lock file");
    
    // === MODULE PATTERNS ===
    
    println!("\n--- Common Module Patterns ---");
    
    // 1. Single file modules
    println!("1. Single file modules: math_utils.rs");
    
    // 2. Directory modules
    println!("2. Directory modules: shapes/ with mod.rs");
    
    // 3. Inline modules
    println!("3. Inline modules: defined directly in this file");
    
    // 4. External crates
    println!("4. External crates: added via Cargo.toml dependencies");
    
    // === TESTING MODULES ===
    
    println!("\n--- Testing Modules ---");
    
    println!("Run tests with: cargo test");
    println!("Run specific test: cargo test test_name");
    println!("Run tests in specific module: cargo test module_name");
    
    // === DOCUMENTATION ===
    
    println!("\n--- Documentation ---");
    
    println!("Generate docs with: cargo doc");
    println!("Open docs with: cargo doc --open");
    println!("Documentation comments start with ///");
    
    println!("\n=== Key Takeaways ===");
    println!("• Modules organize code into logical units");
    println!("• Use 'pub' keyword to make items public");
    println!("• File system mirrors module structure");
    println!("• 'use' statements bring items into scope");
    println!("• Cargo manages dependencies and project structure");
    println!("• Standard library provides many useful modules");
    println!("• Privacy rules enforce proper encapsulation");
    println!("• External crates extend functionality");
}

// === INLINE MODULES ===

mod library {
    pub struct Library {
        books: Vec<String>,
    }
    
    impl Library {
        pub fn new() -> Library {
            Library {
                books: vec![
                    "The Rust Programming Language".to_string(),
                    "Programming Rust".to_string(),
                    "Rust in Action".to_string(),
                ],
            }
        }
        
        pub fn book_count(&self) -> usize {
            self.books.len()
        }
        
        // This is private - cannot be called from outside this module
        #[allow(dead_code)]
        fn add_book(&mut self, title: String) {
            self.books.push(title);
        }
    }
    
    // Private function
    #[allow(dead_code)]
    fn private_function() {
        println!("This is a private function");
    }
}

fn demonstrate_self_usage() {
    // Self refers to the current module (main in this case)
    // Since we're in the root module, self and crate are equivalent here
    let _circle = self::shapes::Circle::new(1.0);
    println!("Self keyword demonstration completed");
}

// === CONDITIONAL COMPILATION ===

#[cfg(target_os = "linux")]
fn platform_specific() {
    println!("Running on Linux");
}

#[cfg(target_os = "windows")]
fn platform_specific() {
    println!("Running on Windows");
}

#[cfg(target_os = "macos")]
fn platform_specific() {
    println!("Running on macOS");
}

// This attribute includes code only when testing
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_math_operations() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(multiply(4, 5), 20);
    }
    
    #[test]
    fn test_shapes() {
        let circle = Circle::new(1.0);
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.001);
    }
}