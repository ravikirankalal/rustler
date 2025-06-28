// Ownership, Borrowing, and References Example
// This example demonstrates Rust's unique memory management system
//
// To run this example: cargo run --example 05_ownership_borrowing

fn main() {
    println!("=== Ownership, Borrowing, and References ===\n");
    
    // === OWNERSHIP BASICS ===
    
    println!("--- Ownership Basics ---");
    
    // Each value in Rust has an owner
    let s1 = String::from("Hello");
    println!("s1 owns the string: {}", s1);
    
    // When we assign s1 to s2, ownership moves (not copied)
    let s2 = s1;
    println!("s2 now owns the string: {}", s2);
    // println!("s1: {}", s1); // This would cause a compile error! s1 is no longer valid
    
    // For simple types (that implement Copy trait), values are copied
    let x = 5;
    let y = x;  // x is copied to y, both remain valid
    println!("x: {}, y: {}", x, y); // Both x and y are still valid
    
    // === FUNCTIONS AND OWNERSHIP ===
    
    println!("\n--- Functions and Ownership ---");
    
    let text = String::from("Hello, World!");
    take_ownership(text); // text's value moves into the function
    // println!("text: {}", text); // This would error! text is no longer valid
    
    let number = 42;
    makes_copy(number); // number is copied, original remains valid
    println!("number is still valid: {}", number);
    
    // Getting ownership back from a function
    let s3 = gives_ownership();
    println!("Received ownership: {}", s3);
    
    let s4 = String::from("World");
    let s5 = takes_and_gives_back(s4); // s4 moves in, ownership moves out to s5
    println!("Got back ownership: {}", s5);
    // println!("s4: {}", s4); // This would error! s4 is no longer valid
    
    // === REFERENCES AND BORROWING ===
    
    println!("\n--- References and Borrowing ---");
    
    let message = String::from("Hello, Rust!");
    
    // Borrowing with immutable reference
    let length = calculate_length(&message); // & creates a reference
    println!("The length of '{}' is {}", message, length); // message is still valid!
    
    // We can have multiple immutable references
    let r1 = &message;
    let r2 = &message;
    println!("r1: {}, r2: {}", r1, r2);
    
    // === MUTABLE REFERENCES ===
    
    println!("\n--- Mutable References ---");
    
    let mut mutable_string = String::from("Hello");
    println!("Before change: {}", mutable_string);
    
    // Mutable borrowing
    change_string(&mut mutable_string); // &mut creates a mutable reference
    println!("After change: {}", mutable_string);
    
    // === BORROWING RULES ===
    
    println!("\n--- Borrowing Rules ---");
    
    let mut data = String::from("Initial data");
    
    // Rule 1: You can have either one mutable reference OR any number of immutable references
    {
        let r1 = &data;        // Immutable reference
        let r2 = &data;        // Another immutable reference - OK
        println!("Immutable refs: {}, {}", r1, r2);
        // r1 and r2 go out of scope here
    }
    
    {
        let r3 = &mut data;    // Mutable reference - OK, no other references exist
        r3.push_str(" - modified");
        println!("Mutable ref: {}", r3);
        // r3 goes out of scope here
    }
    
    // Rule 2: References must always be valid (no dangling references)
    let reference_to_data = &data;
    println!("Valid reference: {}", reference_to_data);
    
    // === SLICE REFERENCES ===
    
    println!("\n--- Slice References ---");
    
    let sentence = String::from("The quick brown fox");
    
    // String slices
    let first_word = &sentence[0..3];   // "The"
    let quick = &sentence[4..9];        // "quick"
    let from_index = &sentence[4..];    // "quick brown fox"
    let to_index = &sentence[..3];      // "The"
    let whole_string = &sentence[..];   // "The quick brown fox"
    
    println!("First word: {}", first_word);
    println!("Quick: {}", quick);
    println!("From index 4: {}", from_index);
    println!("To index 3: {}", to_index);
    println!("Whole string: {}", whole_string);
    
    // Array slices
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4]; // [2, 3, 4]
    println!("Number slice: {:?}", slice);
    
    // === PRACTICAL EXAMPLES ===
    
    println!("\n--- Practical Examples ---");
    
    // Finding first word without taking ownership
    let text = String::from("Hello world from Rust");
    let first = get_first_word(&text);
    println!("First word in '{}': '{}'", text, first); // text is still valid
    
    // Modifying a vector through mutable reference
    let mut scores = vec![85, 92, 78, 96, 88];
    println!("Original scores: {:?}", scores);
    
    add_bonus_points(&mut scores, 5);
    println!("After bonus: {:?}", scores);
    
    // === COMMON PATTERNS ===
    
    println!("\n--- Common Patterns ---");
    
    // Pattern 1: Processing data without taking ownership
    let data = vec![1, 2, 3, 4, 5];
    let sum = sum_vector(&data);
    println!("Sum of {:?} is {}", data, sum); // data is still available
    
    // Pattern 2: Optional borrowing
    let optional_text = Some(String::from("Hello"));
    if let Some(ref text) = optional_text {
        println!("Found text: {}", text);
        // optional_text is still Some(String), not moved
    }
    println!("Optional text still exists: {:?}", optional_text);
    
    // Pattern 3: Returning references with lifetimes
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let longer = longest(&string1, &string2);
    println!("Longer string: {}", longer);
    
    // === LIFETIME ANNOTATIONS ===
    
    println!("\n--- Lifetime Annotations ---");
    
    // Lifetimes ensure references are valid as long as needed
    let text1 = String::from("Long string");
    let result;
    {
        let text2 = String::from("Short");
        result = longest(&text1, &text2);
        println!("Longest in inner scope: {}", result);
    }
    // result is still valid here because it refers to text1
    // which is still in scope
    
    // === ADVANCED BORROWING PATTERNS ===
    
    println!("\n--- Advanced Borrowing Patterns ---");
    
    // Method chaining with borrowing
    let mut text = String::from("hello");
    process_text(&mut text);
    println!("Processed text: {}", text);
    
    // Borrowing fields of structs
    let mut person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    update_age(&mut person.age);
    let name_ref = &person.name;
    println!("Person: {} is {} years old", name_ref, person.age);
    
    println!("\n=== Key Takeaways ===");
    println!("• Each value has exactly one owner at a time");
    println!("• When owner goes out of scope, value is dropped");
    println!("• Ownership can be moved or borrowed (referenced)");
    println!("• Borrowing rules: multiple immutable OR one mutable reference");
    println!("• References must always be valid (no dangling pointers)");
    println!("• Slices are references to contiguous sequences");
    println!("• Lifetimes ensure references live long enough");
    println!("• This system prevents memory leaks and data races!");
}

// === HELPER FUNCTIONS ===

// Function that takes ownership
fn take_ownership(some_string: String) {
    println!("Function took ownership: {}", some_string);
} // some_string goes out of scope and is dropped

// Function that copies
fn makes_copy(some_integer: i32) {
    println!("Function received copy: {}", some_integer);
} // some_integer goes out of scope, but it's just a copy

// Function that gives ownership
fn gives_ownership() -> String {
    let some_string = String::from("Hello from function");
    some_string // Return moves ownership to calling function
}

// Function that takes and gives back ownership
fn takes_and_gives_back(a_string: String) -> String {
    a_string // Return moves ownership back
}

// Function that borrows (doesn't take ownership)
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it's just a reference, so nothing is dropped

// Function that borrows mutably
fn change_string(some_string: &mut String) {
    some_string.push_str(", World!");
}

// Function returning a string slice
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Function that modifies a vector through mutable reference
fn add_bonus_points(scores: &mut Vec<i32>, bonus: i32) {
    for score in scores.iter_mut() {
        *score += bonus;
    }
}

// Function that processes data without taking ownership
fn sum_vector(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

// Function with lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Function demonstrating method chaining with mutable borrowing
fn process_text(text: &mut String) {
    text.push_str(" world");
    text.make_ascii_uppercase();
}

// Struct for demonstrating field borrowing
struct Person {
    name: String,
    age: u32,
}

fn update_age(age: &mut u32) {
    *age += 1;
}