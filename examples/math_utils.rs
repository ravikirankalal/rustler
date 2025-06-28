// Math utilities module
// This file demonstrates how to organize code into separate module files

/// Adds two numbers together
/// 
/// # Examples
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Subtracts the second number from the first
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Custom error type for math operations
#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    Overflow,
}

/// Divides two floating point numbers
/// 
/// Returns an error if attempting to divide by zero
pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Calculates the power of a number
pub fn power(base: f64, exponent: u32) -> f64 {
    base.powi(exponent as i32)
}

/// Private helper function (not accessible outside this module)
fn _helper_function() -> i32 {
    42
}

// Module-level constants
pub const PI: f64 = 3.14159265359;
pub const E: f64 = 2.71828182846;

// Module-level static variable
pub static mut OPERATION_COUNT: u32 = 0;

/// Increments the operation counter (demonstrates mutable static)
/// 
/// # Safety
/// 
/// This function is unsafe because it modifies a mutable static variable
pub unsafe fn increment_operation_count() {
    OPERATION_COUNT += 1;
}

/// Gets the current operation count
pub fn get_operation_count() -> u32 {
    unsafe { OPERATION_COUNT }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }
    
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
    }
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(10.0, 0.0), Err(MathError::DivisionByZero));
    }
    
    #[test]
    fn test_power() {
        assert_eq!(power(2.0, 3), 8.0);
        assert_eq!(power(5.0, 0), 1.0);
    }
}