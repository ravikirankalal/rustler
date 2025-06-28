// Collections Example
// This example demonstrates Vectors, Strings, HashMaps, and iteration patterns
//
// To run this example: cargo run --example 07_collections

use std::collections::HashMap;

fn main() {
    println!("=== Collections in Rust ===\n");
    
    // === VECTORS ===
    
    println!("--- Vectors ---");
    
    // Creating vectors
    let mut numbers = Vec::new(); // Empty vector
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Numbers vector: {:?}", numbers);
    
    // Vector using vec! macro
    let fruits = vec!["apple", "banana", "cherry"];
    println!("Fruits vector: {:?}", fruits);
    
    // Vector with explicit type
    let mut scores: Vec<i32> = vec![85, 92, 78];
    println!("Scores: {:?}", scores);
    
    // Adding elements
    scores.push(96);
    scores.push(88);
    println!("Updated scores: {:?}", scores);
    
    // Removing elements
    let last_score = scores.pop(); // Returns Option<T>
    println!("Removed score: {:?}", last_score);
    println!("Scores after pop: {:?}", scores);
    
    // Accessing elements
    let first = &scores[0]; // Panics if index doesn't exist
    println!("First score: {}", first);
    
    let second = scores.get(1); // Returns Option<&T>
    match second {
        Some(score) => println!("Second score: {}", score),
        None => println!("No second score"),
    }
    
    // Safe access with bounds checking
    let maybe_tenth = scores.get(10);
    println!("Tenth score: {:?}", maybe_tenth);
    
    // Vector capacity and length
    println!("Length: {}, Capacity: {}", scores.len(), scores.capacity());
    
    // === ITERATING OVER VECTORS ===
    
    println!("\n--- Iterating over Vectors ---");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Immutable references
    println!("Iterating with immutable references:");
    for number in &numbers {
        println!("  {}", number);
    }
    
    // Mutable references
    let mut mutable_numbers = vec![1, 2, 3, 4, 5];
    println!("Doubling each number:");
    for number in &mut mutable_numbers {
        *number *= 2;
    }
    println!("  Doubled: {:?}", mutable_numbers);
    
    // Taking ownership
    println!("Taking ownership (consuming the vector):");
    for number in numbers {
        println!("  Owned: {}", number);
    }
    // numbers is no longer accessible here
    
    // === VECTOR METHODS ===
    
    println!("\n--- Vector Methods ---");
    
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Original data: {:?}", data);
    
    // Sorting
    data.sort();
    println!("Sorted: {:?}", data);
    
    // Deduplication (requires sorted data)
    data.dedup();
    println!("Deduplicated: {:?}", data);
    
    // Other useful methods
    let contains_five = data.contains(&5);
    println!("Contains 5? {}", contains_five);
    
    let position_of_four = data.iter().position(|&x| x == 4);
    println!("Position of 4: {:?}", position_of_four);
    
    let sum: i32 = data.iter().sum();
    println!("Sum: {}", sum);
    
    // === STRINGS ===
    
    println!("\n--- Strings ---");
    
    // String literal (string slice)
    let string_literal = "Hello, World!";
    println!("String literal: {}", string_literal);
    
    // Creating String objects
    let mut owned_string = String::new();
    owned_string.push_str("Hello");
    owned_string.push(' ');
    owned_string.push_str("Rust");
    println!("Built string: {}", owned_string);
    
    // String from literal
    let greeting = String::from("Hello, World!");
    println!("String from literal: {}", greeting);
    
    // String concatenation
    let hello = String::from("Hello");
    let world = String::from("World");
    let combined = hello + ", " + &world; // hello is moved here
    println!("Combined string: {}", combined);
    
    // format! macro for concatenation (doesn't take ownership)
    let name = "Alice";
    let age = 30;
    let formatted = format!("My name is {} and I'm {} years old", name, age);
    println!("Formatted string: {}", formatted);
    
    // === STRING METHODS ===
    
    println!("\n--- String Methods ---");
    
    let text = String::from("  Hello, Rust Programming!  ");
    println!("Original: '{}'", text);
    
    // Length and capacity
    println!("Length: {}, Capacity: {}", text.len(), text.capacity());
    
    // Trimming whitespace
    let trimmed = text.trim();
    println!("Trimmed: '{}'", trimmed);
    
    // Case conversion
    println!("Uppercase: {}", text.to_uppercase());
    println!("Lowercase: {}", text.to_lowercase());
    
    // String splitting
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("Words: {:?}", words);
    
    let parts: Vec<&str> = text.split(',').collect();
    println!("Split by comma: {:?}", parts);
    
    // String searching
    let contains_rust = text.contains("Rust");
    println!("Contains 'Rust': {}", contains_rust);
    
    let starts_with = text.trim().starts_with("Hello");
    println!("Starts with 'Hello': {}", starts_with);
    
    // String replacement
    let replaced = text.replace("Rust", "Python");
    println!("Replaced: {}", replaced);
    
    // === ITERATING OVER STRINGS ===
    
    println!("\n--- Iterating over Strings ---");
    
    let text = "Hello, 🦀!";
    
    // Iterating over characters
    println!("Characters:");
    for ch in text.chars() {
        println!("  {}", ch);
    }
    
    // Iterating over bytes
    println!("Bytes:");
    for byte in text.bytes() {
        println!("  {}", byte);
    }
    
    // === HASHMAPS ===
    
    println!("\n--- HashMaps ---");
    
    // Creating a HashMap
    let mut student_grades = HashMap::new();
    student_grades.insert(String::from("Alice"), 95);
    student_grades.insert(String::from("Bob"), 87);
    student_grades.insert(String::from("Charlie"), 92);
    
    println!("Student grades: {:?}", student_grades);
    
    // Accessing values
    let alice_grade = student_grades.get("Alice");
    match alice_grade {
        Some(grade) => println!("Alice's grade: {}", grade),
        None => println!("Alice not found"),
    }
    
    // Using unwrap_or for default values
    let diana_grade = student_grades.get("Diana").unwrap_or(&0);
    println!("Diana's grade (default 0): {}", diana_grade);
    
    // Inserting with different strategies
    student_grades.insert(String::from("Alice"), 98); // Overwrites existing
    println!("Alice's updated grade: {:?}", student_grades.get("Alice"));
    
    // Insert only if key doesn't exist
    student_grades.entry(String::from("Eve")).or_insert(85);
    student_grades.entry(String::from("Alice")).or_insert(90); // Won't overwrite
    println!("After conditional inserts: {:?}", student_grades);
    
    // Updating based on old value
    let eve_entry = student_grades.entry(String::from("Eve")).or_insert(0);
    *eve_entry += 5; // Bonus points
    println!("Eve's grade after bonus: {:?}", student_grades.get("Eve"));
    
    // === ITERATING OVER HASHMAPS ===
    
    println!("\n--- Iterating over HashMaps ---");
    
    println!("All student grades:");
    for (name, grade) in &student_grades {
        println!("  {}: {}", name, grade);
    }
    
    // Iterating over keys only
    println!("Student names:");
    for name in student_grades.keys() {
        println!("  {}", name);
    }
    
    // Iterating over values only
    let total_points: i32 = student_grades.values().sum();
    println!("Total points: {}", total_points);
    
    // === PRACTICAL EXAMPLES ===
    
    println!("\n--- Practical Examples ---");
    
    // Word frequency counter
    let text = "the quick brown fox jumps over the lazy dog the fox is quick";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word frequencies:");
    for (word, count) in &word_count {
        println!("  {}: {}", word, count);
    }
    
    // Finding most common word
    let most_common = word_count
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(word, count)| (word, count));
    
    if let Some((word, count)) = most_common {
        println!("Most common word: '{}' appears {} times", word, count);
    }
    
    // Group students by grade ranges
    let all_grades = vec![
        ("Alice", 95), ("Bob", 87), ("Charlie", 92),
        ("Diana", 78), ("Eve", 90), ("Frank", 65),
    ];
    
    let mut grade_groups = HashMap::new();
    
    for (name, grade) in all_grades {
        let grade_range = match grade {
            90..=100 => "A",
            80..=89 => "B", 
            70..=79 => "C",
            60..=69 => "D",
            _ => "F",
        };
        
        grade_groups.entry(grade_range).or_insert(Vec::new()).push(name);
    }
    
    println!("Students by grade range:");
    for (range, students) in &grade_groups {
        println!("  {}: {:?}", range, students);
    }
    
    // === ADVANCED COLLECTION OPERATIONS ===
    
    println!("\n--- Advanced Collection Operations ---");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Filtering
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Even numbers: {:?}", evens);
    
    // Mapping
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squares: {:?}", squares);
    
    // Chaining operations
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("Even squares: {:?}", even_squares);
    
    // Fold/reduce operations
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("Sum: {}, Product: {}", sum, product);
    
    // Custom fold
    let sum_of_squares: i32 = numbers.iter().fold(0, |acc, &x| acc + x * x);
    println!("Sum of squares: {}", sum_of_squares);
    
    // Finding elements
    let first_greater_than_five = numbers.iter().find(|&&x| x > 5);
    println!("First number > 5: {:?}", first_greater_than_five);
    
    // All/any predicates
    let all_positive = numbers.iter().all(|&x| x > 0);
    let any_greater_than_five = numbers.iter().any(|&x| x > 5);
    println!("All positive: {}, Any > 5: {}", all_positive, any_greater_than_five);
    
    println!("\n=== Key Takeaways ===");
    println!("• Vectors store multiple values of the same type");
    println!("• Use Vec::new() or vec! macro to create vectors");
    println!("• Strings are UTF-8 encoded and growable");
    println!("• String vs &str: owned vs borrowed string data");
    println!("• HashMaps store key-value pairs with O(1) average access");
    println!("• Iterator methods like map, filter, fold enable functional programming");
    println!("• Collections integrate seamlessly with Rust's ownership system");
}