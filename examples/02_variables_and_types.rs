// Variables and Data Types Example
// This example demonstrates Rust's variable system and built-in data types
//
// To run this example: cargo run --example 02_variables_and_types

fn main() {
    println!("=== Variables and Data Types in Rust ===\n");
    
    // === VARIABLES ===
    
    // Variables are immutable by default in Rust
    let x = 5;
    println!("Immutable variable x: {}", x);
    
    // This would cause a compile error:
    // x = 6; // Cannot assign twice to immutable variable
    
    // To make a variable mutable, use the 'mut' keyword
    let mut y = 10;
    println!("Mutable variable y: {}", y);
    y = 15; // This is allowed because y is mutable
    println!("Modified y: {}", y);
    
    // === VARIABLE SHADOWING ===
    
    // You can declare a new variable with the same name (shadowing)
    let z = 20;
    println!("First z: {}", z);
    
    let z = z + 5; // This creates a new variable, doesn't modify the old one
    println!("Shadowed z: {}", z);
    
    let z = "Now I'm a string!"; // You can even change the type with shadowing
    println!("Shadowed z again: {}", z);
    
    // === SCALAR DATA TYPES ===
    
    println!("\n--- Scalar Data Types ---");
    
    // Integers (signed and unsigned, various sizes)
    let signed_int: i32 = -42;           // 32-bit signed integer
    let unsigned_int: u32 = 42;          // 32-bit unsigned integer
    let big_int: i64 = 9_223_372_036_854_775_807; // 64-bit signed (note underscores for readability)
    let byte_val: u8 = 255;              // 8-bit unsigned (0 to 255)
    
    println!("Signed integer (i32): {}", signed_int);
    println!("Unsigned integer (u32): {}", unsigned_int);
    println!("Big integer (i64): {}", big_int);
    println!("Byte value (u8): {}", byte_val);
    
    // Floating-point numbers
    let float32: f32 = 3.14159;          // 32-bit floating point
    let float64: f64 = 2.718281828459045; // 64-bit floating point (default)
    
    println!("32-bit float: {}", float32);
    println!("64-bit float: {}", float64);
    
    // Boolean type
    let is_rust_awesome: bool = true;
    let is_learning_fun = false; // Type inference works here
    
    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Is learning fun? {}", is_learning_fun);
    
    // Character type (Unicode scalar values)
    let letter: char = 'R';
    let emoji: char = '🦀'; // Rust's mascot!
    let chinese_char: char = '中';
    
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);
    println!("Chinese character: {}", chinese_char);
    
    // === COMPOUND DATA TYPES ===
    
    println!("\n--- Compound Data Types ---");
    
    // Tuples - group multiple values of different types
    let person: (String, i32, bool) = (String::from("Alice"), 30, true);
    println!("Person tuple: {:?}", person);
    
    // Accessing tuple elements by index
    let (name, age, is_employed) = person; // Destructuring
    println!("Name: {}, Age: {}, Employed: {}", name, age, is_employed);
    
    let coordinates = (3.0, 4.0);
    println!("X coordinate: {}", coordinates.0); // Access by index
    println!("Y coordinate: {}", coordinates.1);
    
    // Arrays - multiple values of the same type, fixed length
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Type and length specified
    let repeated = [3; 4]; // Creates [3, 3, 3, 3]
    
    println!("Numbers array: {:?}", numbers);
    println!("Repeated array: {:?}", repeated);
    println!("First number: {}", numbers[0]);
    println!("Array length: {}", numbers.len());
    
    // === TYPE INFERENCE ===
    
    println!("\n--- Type Inference ---");
    
    // Rust can often infer types automatically
    let inferred_int = 42;        // Rust infers i32
    let inferred_float = 3.14;    // Rust infers f64
    let inferred_string = "Hello"; // Rust infers &str (string slice)
    
    println!("Inferred integer: {}", inferred_int);
    println!("Inferred float: {}", inferred_float);
    println!("Inferred string: {}", inferred_string);
    
    // === CONSTANTS ===
    
    // Constants are always immutable and must have a type annotation
    const MAX_SCORE: u32 = 100_000;
    println!("Maximum score: {}", MAX_SCORE);
    
    // === STRING TYPES ===
    
    println!("\n--- String Types ---");
    
    // String slice (&str) - immutable reference to string data
    let string_slice: &str = "This is a string slice";
    println!("String slice: {}", string_slice);
    
    // String - heap-allocated, mutable string
    let mut owned_string = String::from("Hello");
    owned_string.push_str(", World!");
    println!("Owned string: {}", owned_string);
    
    // === TYPE ANNOTATIONS ===
    
    // Sometimes you need to be explicit about types
    let parsed_number: i32 = "42".parse().expect("Not a number!");
    println!("Parsed number: {}", parsed_number);
    
    // Using turbo fish syntax for type specification
    let parsed_float = "3.14".parse::<f64>().expect("Not a float!");
    println!("Parsed float: {}", parsed_float);
    
    println!("\n=== Key Takeaways ===");
    println!("• Variables are immutable by default, use 'mut' for mutability");
    println!("• Shadowing allows reusing variable names with new types");
    println!("• Rust has rich scalar types: integers, floats, booleans, chars");
    println!("• Compound types include tuples and arrays");
    println!("• Type inference helps reduce boilerplate");
    println!("• Constants are always immutable and require type annotations");
}