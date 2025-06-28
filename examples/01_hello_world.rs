// Hello World Example
// This is the traditional first program that demonstrates basic Rust syntax
// 
// To run this example: cargo run --example 01_hello_world

// This is a single-line comment. Everything after // is ignored by the compiler

/*
   This is a multi-line comment.
   You can write multiple lines here.
   Multi-line comments start with /* and end with */
*/

/// This is a documentation comment. It starts with three slashes.
/// Documentation comments are used to generate documentation for your code.
/// They support Markdown formatting and are displayed when using `cargo doc`.

/// The main function is the entry point of every Rust program.
/// When you run a Rust program, execution starts here.
fn main() {
    // println! is a macro (notice the exclamation mark)
    // Macros are like functions but they generate code at compile time
    println!("Hello, world!");
    
    // You can print multiple things at once
    println!("Welcome to Rust programming!");
    
    // println! supports formatting with placeholders
    println!("Rust was first released in {}", 2010);
    
    // You can use named parameters for clarity
    println!("My name is {name} and I'm {age} years old", name = "Rust", age = 14);
    
    // Different formatting options
    println!("Binary of 42: {:b}", 42);      // Binary: 101010
    println!("Hexadecimal of 255: {:x}", 255); // Hex: ff
    println!("Hexadecimal of 255: {:X}", 255); // Hex uppercase: FF
    
    // You can also use variables in println!
    let language = "Rust";
    let year = 2024;
    println!("Learning {} in {}", language, year);
    
    // For debugging, you can use the debug formatter {:?}
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Debug output: {:?}", numbers);
    
    // Pretty debug formatting with {:#?}
    println!("Pretty debug output: {:#?}", numbers);
    
    // Basic program structure explanation:
    // 1. Optional `use` statements to bring items into scope
    // 2. The main() function - program entry point
    // 3. Code statements inside curly braces { }
    // 4. Statements end with semicolons ;
    // 5. The last expression in a block can omit the semicolon (returns a value)
    
    println!("Program completed successfully!");
}

// Key takeaways:
// - Every Rust program needs a main() function
// - Use println! macro for printing to console
// - Comments help explain your code
// - Rust uses curly braces for code blocks
// - Most statements end with semicolons