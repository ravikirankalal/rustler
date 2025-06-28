# rustler

A comprehensive collection of beginner-friendly Rust examples to help new developers learn the Rust programming language.

## Examples

This repository contains 13 comprehensive examples covering core Rust concepts:

### 1. Hello World (`01_hello_world.rs`)
- Basic printing with `println!` macro
- Program structure and entry point
- Different types of comments
- String formatting and placeholders
- **Run:** `cargo run --example 01_hello_world`

### 2. Variables and Data Types (`02_variables_and_types.rs`)
- Immutable and mutable variables
- Variable shadowing
- Scalar types: integers, floats, booleans, characters
- Compound types: tuples and arrays
- Type inference and annotations
- **Run:** `cargo run --example 02_variables_and_types`

### 3. Control Flow (`03_control_flow.rs`)
- `if` expressions and conditions
- Pattern matching with `match`
- Loops: `for`, `while`, and `loop`
- Loop labels and control
- Practical examples (FizzBuzz, calculator)
- **Run:** `cargo run --example 03_control_flow`

### 4. Functions (`04_functions.rs`)
- Function declaration and parameters
- Return values and expressions
- Closures and capturing environment
- Higher-order functions
- Methods vs functions
- **Run:** `cargo run --example 04_functions`

### 5. Ownership, Borrowing, and References (`05_ownership_borrowing.rs`)
- Ownership rules and memory management
- Borrowing with references (`&` and `&mut`)
- Lifetimes and scope
- String slices and array slices
- Common ownership patterns
- **Run:** `cargo run --example 05_ownership_borrowing`

### 6. Structs and Enums (`06_structs_enums.rs`)
- Struct definitions and instantiation
- Methods and associated functions
- Tuple structs and unit structs
- Enum variants with data
- Pattern matching and destructuring
- **Run:** `cargo run --example 06_structs_enums`

### 7. Collections (`07_collections.rs`)
- Vectors: creation, modification, iteration
- Strings: manipulation and methods
- HashMaps: key-value storage and operations
- Iterator methods: `map`, `filter`, `collect`
- Practical examples and patterns
- **Run:** `cargo run --example 07_collections`

### 8. Error Handling (`08_error_handling.rs`)
- `Option<T>` for handling absence of values
- `Result<T, E>` for recoverable errors
- Error propagation with `?` operator
- Custom error types
- Error handling patterns and best practices
- **Run:** `cargo run --example 08_error_handling`

### 9. Traits and Generics (`09_traits_generics.rs`)
- Trait definitions and implementations
- Generic functions and structs
- Trait bounds and where clauses
- Associated types and default implementations
- Operator overloading through traits
- **Run:** `cargo run --example 09_traits_generics`

### 10. Modules and Crates (`10_modules_crates.rs`)
- Module organization and visibility
- File system and module structure
- `use` statements and imports
- Cargo project structure
- External crate usage
- **Run:** `cargo run --example 10_modules_crates`

### 11. Standard Library Features (`11_stdlib_features.rs`)
- Command line arguments and environment variables
- File I/O operations with `std::fs`
- Process execution and system interaction
- Time and date handling
- String formatting and manipulation
- External crate integration (serde, chrono)
- **Run:** `cargo run --example 11_stdlib_features`

### 12. Testing (`12_testing.rs`)
- Unit tests with `#[test]` attribute
- Test organization and structure
- Assertion macros and custom assertions
- Error testing and `#[should_panic]`
- Test configuration and attributes
- **Run:** `cargo run --example 12_testing`
- **Run tests:** `cargo test test_in_testing_example`

### 13. Basic Concurrency (`13_concurrency.rs`)
- Thread creation and management
- Data sharing between threads
- Channels for thread communication
- Mutex and Arc for shared state
- Worker pools and synchronization patterns
- **Run:** `cargo run --example 13_concurrency`

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ravikirankalal/rustler.git
   cd rustler
   ```

2. **Run any example:**
   ```bash
   cargo run --example 01_hello_world
   ```

3. **Run all tests:**
   ```bash
   cargo test
   ```

4. **Build the project:**
   ```bash
   cargo build
   ```

## Features

- **Beginner-friendly:** Each example is self-contained with clear explanations
- **Comprehensive comments:** Extensive documentation within each example
- **Practical examples:** Real-world usage patterns and best practices
- **Progressive learning:** Examples build upon previous concepts
- **Runnable code:** All examples compile and run successfully

## Requirements

- Rust 2021 edition or later
- Cargo (comes with Rust installation)

## External Dependencies

The examples use a minimal set of external crates:
- `serde` and `serde_json` for serialization examples
- `chrono` for advanced time/date handling

## Learning Path

For beginners, it's recommended to go through the examples in order:

1. Start with **Hello World** to understand basic syntax
2. Learn about **Variables and Data Types** for Rust's type system
3. Master **Control Flow** for program logic
4. Understand **Functions** for code organization
5. Grasp **Ownership and Borrowing** (Rust's unique feature)
6. Learn **Structs and Enums** for data modeling
7. Explore **Collections** for data manipulation
8. Master **Error Handling** for robust programs
9. Understand **Traits and Generics** for code reuse
10. Learn **Modules** for large project organization
11. Explore **Standard Library** features
12. Practice **Testing** for code quality
13. Learn **Concurrency** for parallel programming

## Contributing

Feel free to submit issues or pull requests to improve these examples!

## License

This project is open source and available under the GPL-3.0 License.