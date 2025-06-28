// Testing Example
// This example demonstrates writing and running tests in Rust
//
// To run this example: cargo run --example 12_testing
// To run the tests: cargo test test_in_testing_example

fn main() {
    println!("=== Testing in Rust ===\n");
    
    println!("--- Overview ---");
    println!("Rust has built-in support for testing with the #[test] attribute");
    println!("Tests are typically written in the same file as the code they test");
    println!("Integration tests go in the tests/ directory");
    println!("Use 'cargo test' to run all tests");
    
    // === DEMONSTRATING THE CODE TO BE TESTED ===
    
    println!("\n--- Code Under Test ---");
    
    // Demonstrate the functions we'll test
    let calc = Calculator::new();
    println!("Calculator operations:");
    println!("  5 + 3 = {}", calc.add(5, 3));
    println!("  10 - 4 = {}", calc.subtract(10, 4));
    println!("  6 * 7 = {}", calc.multiply(6, 7));
    
    match calc.divide(15, 3) {
        Ok(result) => println!("  15 / 3 = {}", result),
        Err(e) => println!("  Division error: {:?}", e),
    }
    
    match calc.divide(10, 0) {
        Ok(result) => println!("  10 / 0 = {}", result),
        Err(e) => println!("  Division error: {:?}", e),
    }
    
    // Demonstrate string operations
    let text_processor = TextProcessor::new();
    let text = "Hello, World!";
    println!("\nText processing:");
    println!("  Original: '{}'", text);
    println!("  Word count: {}", text_processor.count_words(text));
    println!("  Is palindrome: {}", text_processor.is_palindrome("racecar"));
    println!("  Is palindrome: {}", text_processor.is_palindrome("hello"));
    println!("  Reverse: '{}'", text_processor.reverse(text));
    
    // Demonstrate geometric calculations
    let rect = Rectangle::new(5.0, 3.0);
    println!("\nRectangle operations:");
    println!("  Rectangle: {}x{}", rect.width, rect.height);
    println!("  Area: {}", rect.area());
    println!("  Perimeter: {}", rect.perimeter());
    println!("  Is square: {}", rect.is_square());
    
    let square = Rectangle::new(4.0, 4.0);
    println!("  Square: {}x{}", square.width, square.height);
    println!("  Is square: {}", square.is_square());
    
    println!("\n--- Test Types ---");
    println!("1. Unit tests: Test individual functions or methods");
    println!("2. Integration tests: Test multiple components together");
    println!("3. Documentation tests: Test code examples in documentation");
    
    println!("\n--- Running Tests ---");
    println!("• cargo test                     - Run all tests");
    println!("• cargo test test_name           - Run specific test");
    println!("• cargo test -- --nocapture      - Show println! output");
    println!("• cargo test -- --test-threads=1 - Run tests sequentially");
    
    println!("\n--- Test Attributes ---");
    println!("• #[test]                - Mark function as a test");
    println!("• #[should_panic]        - Test should panic");
    println!("• #[ignore]              - Skip test unless explicitly run");
    println!("• #[cfg(test)]           - Only compile when testing");
    
    println!("\n=== To see the actual tests, look at the code below main() ===");
    println!("Run 'cargo test test_in_testing_example' to execute the tests!");
}

// === CODE TO BE TESTED ===

/// A simple calculator struct
#[derive(Debug)]
pub struct Calculator;

#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }
    
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
    
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
    
    pub fn divide(&self, a: i32, b: i32) -> Result<i32, CalculatorError> {
        if b == 0 {
            Err(CalculatorError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
}

/// Text processing utilities
pub struct TextProcessor;

impl TextProcessor {
    pub fn new() -> Self {
        TextProcessor
    }
    
    pub fn count_words(&self, text: &str) -> usize {
        text.split_whitespace().count()
    }
    
    pub fn is_palindrome(&self, text: &str) -> bool {
        let cleaned: String = text.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().to_string())
            .collect();
        cleaned == cleaned.chars().rev().collect::<String>()
    }
    
    pub fn reverse(&self, text: &str) -> String {
        text.chars().rev().collect()
    }
    
    pub fn capitalize_words(&self, text: &str) -> String {
        text.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.collect::<String>().to_lowercase(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Rectangle for geometric calculations
#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
    
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

// === TESTS ===

#[cfg(test)]
mod test_in_testing_example {
    use super::*;
    
    // === BASIC UNIT TESTS ===
    
    #[test]
    fn test_calculator_addition() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.add(-1, 1), 0);
        assert_eq!(calc.add(0, 0), 0);
    }
    
    #[test]
    fn test_calculator_subtraction() {
        let calc = Calculator::new();
        assert_eq!(calc.subtract(5, 3), 2);
        assert_eq!(calc.subtract(0, 5), -5);
        assert_eq!(calc.subtract(10, 10), 0);
    }
    
    #[test]
    fn test_calculator_multiplication() {
        let calc = Calculator::new();
        assert_eq!(calc.multiply(3, 4), 12);
        assert_eq!(calc.multiply(-2, 5), -10);
        assert_eq!(calc.multiply(0, 100), 0);
    }
    
    #[test]
    fn test_calculator_division_success() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10, 2), Ok(5));
        assert_eq!(calc.divide(7, 3), Ok(2)); // Integer division
        assert_eq!(calc.divide(0, 5), Ok(0));
    }
    
    #[test]
    fn test_calculator_division_by_zero() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10, 0), Err(CalculatorError::DivisionByZero));
        assert_eq!(calc.divide(-5, 0), Err(CalculatorError::DivisionByZero));
    }
    
    // === TEXT PROCESSING TESTS ===
    
    #[test]
    fn test_word_count() {
        let processor = TextProcessor::new();
        assert_eq!(processor.count_words("hello world"), 2);
        assert_eq!(processor.count_words(""), 0);
        assert_eq!(processor.count_words("   one   two   three   "), 3);
        assert_eq!(processor.count_words("single"), 1);
    }
    
    #[test]
    fn test_palindrome_detection() {
        let processor = TextProcessor::new();
        assert!(processor.is_palindrome("racecar"));
        assert!(processor.is_palindrome("A man a plan a canal Panama"));
        assert!(processor.is_palindrome(""));
        assert!(processor.is_palindrome("a"));
        assert!(!processor.is_palindrome("hello"));
        assert!(!processor.is_palindrome("rust"));
    }
    
    #[test]
    fn test_string_reversal() {
        let processor = TextProcessor::new();
        assert_eq!(processor.reverse("hello"), "olleh");
        assert_eq!(processor.reverse(""), "");
        assert_eq!(processor.reverse("a"), "a");
        assert_eq!(processor.reverse("rust"), "tsur");
    }
    
    #[test]
    fn test_capitalize_words() {
        let processor = TextProcessor::new();
        assert_eq!(processor.capitalize_words("hello world"), "Hello World");
        assert_eq!(processor.capitalize_words("rust programming"), "Rust Programming");
        assert_eq!(processor.capitalize_words(""), "");
        assert_eq!(processor.capitalize_words("a"), "A");
    }
    
    // === GEOMETRIC TESTS ===
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(5.0, 3.0);
        assert_eq!(rect.area(), 15.0);
        
        let square = Rectangle::new(4.0, 4.0);
        assert_eq!(square.area(), 16.0);
    }
    
    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle::new(5.0, 3.0);
        assert_eq!(rect.perimeter(), 16.0);
        
        let square = Rectangle::new(4.0, 4.0);
        assert_eq!(square.perimeter(), 16.0);
    }
    
    #[test]
    fn test_rectangle_is_square() {
        let rect = Rectangle::new(5.0, 3.0);
        assert!(!rect.is_square());
        
        let square = Rectangle::new(4.0, 4.0);
        assert!(square.is_square());
        
        // Test with floating point precision
        let almost_square = Rectangle::new(4.0, 4.000000001);
        assert!(!almost_square.is_square());
    }
    
    // === TESTING PATTERNS ===
    
    #[test]
    fn test_multiple_assertions() {
        let calc = Calculator::new();
        
        // Test multiple related operations
        let results = vec![
            calc.add(1, 1),
            calc.add(2, 2),
            calc.add(3, 3),
        ];
        
        assert_eq!(results, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_with_setup() {
        // Setup test data
        let test_strings = vec![
            "hello",
            "world",
            "rust",
            "programming",
        ];
        
        let processor = TextProcessor::new();
        
        // Test all strings
        for s in test_strings {
            let reversed = processor.reverse(s);
            let double_reversed = processor.reverse(&reversed);
            assert_eq!(s, double_reversed);
        }
    }
    
    // === ERROR TESTING ===
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_panicking_function() {
        // This function panics when dividing by zero
        fn divide_and_panic(a: i32, b: i32) -> i32 {
            if b == 0 {
                panic!("Division by zero");
            }
            a / b
        }
        
        divide_and_panic(10, 0); // This should panic
    }
    
    // === CONDITIONAL TESTS ===
    
    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_specific() {
        // This test only runs on Linux
        assert!(true);
    }
    
    #[test]
    #[ignore]
    fn test_expensive_operation() {
        // This test is ignored by default
        // Run with: cargo test -- --ignored
        println!("This is an expensive test that's normally skipped");
        assert!(true);
    }
    
    // === HELPER FUNCTIONS FOR TESTS ===
    
    fn setup_test_rectangle() -> Rectangle {
        Rectangle::new(10.0, 5.0)
    }
    
    #[test]
    fn test_with_helper() {
        let rect = setup_test_rectangle();
        assert_eq!(rect.area(), 50.0);
        assert_eq!(rect.perimeter(), 30.0);
        assert!(!rect.is_square());
    }
    
    // === FLOATING POINT COMPARISONS ===
    
    #[test]
    fn test_floating_point_equality() {
        let a = 0.1 + 0.2;
        let b = 0.3;
        
        // Don't do this - floating point precision issues
        // assert_eq!(a, b);
        
        // Do this instead - compare with epsilon
        let epsilon = 1e-10;
        assert!((a - b).abs() < epsilon);
    }
    
    // === TESTING COLLECTIONS ===
    
    #[test]
    fn test_vector_operations() {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 1);
        assert_eq!(v[2], 3);
        
        let popped = v.pop();
        assert_eq!(popped, Some(3));
        assert_eq!(v.len(), 2);
    }
    
    // === CUSTOM ASSERTION MACROS ===
    
    macro_rules! assert_near {
        ($a:expr, $b:expr, $epsilon:expr) => {
            assert!(
                ($a - $b).abs() < $epsilon,
                "assertion failed: {} is not near {} (epsilon: {})",
                $a, $b, $epsilon
            );
        };
    }
    
    #[test]
    fn test_with_custom_assertion() {
        let rect = Rectangle::new(3.0, 4.0);
        let expected_area = 12.0;
        
        assert_near!(rect.area(), expected_area, 0.001);
    }
}