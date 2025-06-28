// Standard Library Features Example
// This example demonstrates File I/O, formatting, command line arguments, and external crates
//
// To run this example: cargo run --example 11_stdlib_features
// To run with arguments: cargo run --example 11_stdlib_features -- arg1 arg2 arg3

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH, Duration, Instant};
use std::thread;

// External crates
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

fn main() {
    println!("=== Standard Library Features ===\n");
    
    // === COMMAND LINE ARGUMENTS ===
    
    println!("--- Command Line Arguments ---");
    
    let args: Vec<String> = env::args().collect();
    println!("Program name: {}", args[0]);
    
    if args.len() > 1 {
        println!("Arguments provided:");
        for (i, arg) in args.iter().skip(1).enumerate() {
            println!("  Arg {}: {}", i + 1, arg);
        }
    } else {
        println!("No arguments provided");
        println!("Try running: cargo run --example 11_stdlib_features -- hello world rust");
    }
    
    // === ENVIRONMENT VARIABLES ===
    
    println!("\n--- Environment Variables ---");
    
    // Get environment variables
    match env::var("HOME") {
        Ok(home) => println!("Home directory: {}", home),
        Err(_) => println!("HOME not set"),
    }
    
    match env::var("PATH") {
        Ok(path) => {
            let paths: Vec<&str> = path.split(':').take(3).collect();
            println!("First 3 PATH entries: {:?}", paths);
        },
        Err(_) => println!("PATH not set"),
    }
    
    // Set environment variable for this process
    env::set_var("RUST_EXAMPLE", "Hello from Rust!");
    match env::var("RUST_EXAMPLE") {
        Ok(value) => println!("RUST_EXAMPLE: {}", value),
        Err(_) => println!("Failed to set RUST_EXAMPLE"),
    }
    
    // List some environment variables
    println!("Some environment variables:");
    for (key, value) in env::vars().take(5) {
        println!("  {}: {}", key, value);
    }
    
    // === FILE I/O BASICS ===
    
    println!("\n--- File I/O Basics ---");
    
    let filename = "/tmp/rust_example.txt";
    let content = "Hello, File I/O!\nThis is line 2.\nThis is line 3.\n";
    
    // Write to file
    match fs::write(filename, content) {
        Ok(()) => println!("File written successfully"),
        Err(e) => println!("Error writing file: {}", e),
    }
    
    // Read entire file
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("File contents:");
            for line in contents.lines() {
                println!("  {}", line);
            }
        },
        Err(e) => println!("Error reading file: {}", e),
    }
    
    // Read file as bytes
    match fs::read(filename) {
        Ok(bytes) => println!("File size: {} bytes", bytes.len()),
        Err(e) => println!("Error reading file as bytes: {}", e),
    }
    
    // === ADVANCED FILE I/O ===
    
    println!("\n--- Advanced File I/O ---");
    
    // Using File struct with more control
    let advanced_filename = "/tmp/rust_advanced.txt";
    
    // Write using File and BufWriter
    match File::create(advanced_filename) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            writeln!(writer, "Line 1 from BufWriter").unwrap();
            writeln!(writer, "Line 2 from BufWriter").unwrap();
            writeln!(writer, "Numbers: {} {} {}", 1, 2, 3).unwrap();
            // BufWriter is automatically flushed when dropped
            println!("Advanced file written with BufWriter");
        },
        Err(e) => println!("Error creating file: {}", e),
    }
    
    // Read using BufReader
    match File::open(advanced_filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            println!("Reading with BufReader:");
            for (i, line) in reader.lines().enumerate() {
                match line {
                    Ok(content) => println!("  Line {}: {}", i + 1, content),
                    Err(e) => println!("  Error reading line: {}", e),
                }
            }
        },
        Err(e) => println!("Error opening file: {}", e),
    }
    
    // Append to file
    match OpenOptions::new().create(true).append(true).open(advanced_filename) {
        Ok(mut file) => {
            writeln!(file, "Appended line").unwrap();
            println!("Line appended to file");
        },
        Err(e) => println!("Error appending to file: {}", e),
    }
    
    // === FILE METADATA ===
    
    println!("\n--- File Metadata ---");
    
    match fs::metadata(filename) {
        Ok(metadata) => {
            println!("File metadata:");
            println!("  Size: {} bytes", metadata.len());
            println!("  Is file: {}", metadata.is_file());
            println!("  Is directory: {}", metadata.is_dir());
            println!("  Read-only: {}", metadata.permissions().readonly());
            
            if let Ok(modified) = metadata.modified() {
                println!("  Modified: {:?}", modified);
            }
        },
        Err(e) => println!("Error getting metadata: {}", e),
    }
    
    // === DIRECTORY OPERATIONS ===
    
    println!("\n--- Directory Operations ---");
    
    let dir_path = "/tmp/rust_example_dir";
    
    // Create directory
    match fs::create_dir_all(dir_path) {
        Ok(()) => println!("Directory created: {}", dir_path),
        Err(e) => println!("Error creating directory: {}", e),
    }
    
    // Create files in directory
    for i in 1..=3 {
        let file_path = format!("{}/file{}.txt", dir_path, i);
        let file_content = format!("This is file number {}\n", i);
        fs::write(&file_path, file_content).unwrap();
    }
    
    // List directory contents
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            println!("Directory contents:");
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let name = path.file_name().unwrap().to_str().unwrap();
                        println!("  {}", name);
                    },
                    Err(e) => println!("  Error reading entry: {}", e),
                }
            }
        },
        Err(e) => println!("Error reading directory: {}", e),
    }
    
    // === STRING FORMATTING ===
    
    println!("\n--- String Formatting ---");
    
    let name = "Alice";
    let age = 30;
    let height = 5.6;
    
    // Basic formatting
    println!("Name: {}, Age: {}, Height: {:.1}ft", name, age, height);
    
    // Positional arguments
    println!("{0} is {1} years old. {0} is {2:.1}ft tall.", name, age, height);
    
    // Named arguments
    println!("{name} is {age} years old and {height:.2}ft tall", 
             name=name, age=age, height=height);
    
    // Formatting options
    println!("Binary: {:b}", 42);
    println!("Hexadecimal: {:x}", 255);
    println!("Octal: {:o}", 64);
    println!("Scientific: {:e}", 1234.5);
    println!("Percentage: {:.1}%", 0.75 * 100.0); // Convert to percentage manually
    
    // Padding and alignment
    println!("Left aligned:  '{:<10}'", "hello");
    println!("Right aligned: '{:>10}'", "hello");
    println!("Center aligned:'{:^10}'", "hello");
    println!("Zero padded:   '{:05}'", 42);
    
    // === TIME AND DATES ===
    
    println!("\n--- Time and Dates ---");
    
    // System time
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            println!("Seconds since Unix epoch: {}", duration.as_secs());
        },
        Err(e) => println!("SystemTime before Unix epoch: {}", e),
    }
    
    // Measuring elapsed time
    let start = Instant::now();
    thread::sleep(Duration::from_millis(100)); // Simulate work
    let elapsed = start.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
    
    // Using chrono crate for better date/time handling
    let local_time: DateTime<Local> = Local::now();
    let utc_time: DateTime<Utc> = Utc::now();
    
    println!("Local time: {}", local_time.format("%Y-%m-%d %H:%M:%S"));
    println!("UTC time: {}", utc_time.format("%Y-%m-%d %H:%M:%S UTC"));
    
    // === PROCESS EXECUTION ===
    
    println!("\n--- Process Execution ---");
    
    // Execute a simple command
    match Command::new("echo").arg("Hello from subprocess!").output() {
        Ok(output) => {
            println!("Command output:");
            println!("  stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("  stderr: {}", String::from_utf8_lossy(&output.stderr));
            println!("  status: {}", output.status);
        },
        Err(e) => println!("Error executing command: {}", e),
    }
    
    // List files using ls command
    match Command::new("ls").arg("-la").arg("/tmp").output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Directory listing (first 5 lines):");
                for (i, line) in stdout.lines().take(5).enumerate() {
                    println!("  {}: {}", i + 1, line);
                }
            }
        },
        Err(e) => println!("Error listing directory: {}", e),
    }
    
    // === SERIALIZATION WITH EXTERNAL CRATES ===
    
    println!("\n--- Serialization (External Crates) ---");
    
    // Define a struct for serialization
    #[derive(Serialize, Deserialize, Debug)]
    struct Person {
        name: String,
        age: u32,
        city: String,
        hobbies: Vec<String>,
    }
    
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        city: "New York".to_string(),
        hobbies: vec!["reading".to_string(), "hiking".to_string(), "coding".to_string()],
    };
    
    // Serialize to JSON
    match serde_json::to_string_pretty(&person) {
        Ok(json) => {
            println!("Serialized to JSON:");
            println!("{}", json);
            
            // Write JSON to file
            let json_file = "/tmp/person.json";
            fs::write(json_file, &json).unwrap();
            println!("JSON written to {}", json_file);
            
            // Read and deserialize JSON
            match fs::read_to_string(json_file) {
                Ok(json_content) => {
                    match serde_json::from_str::<Person>(&json_content) {
                        Ok(deserialized_person) => {
                            println!("Deserialized from JSON: {:?}", deserialized_person);
                        },
                        Err(e) => println!("Error deserializing JSON: {}", e),
                    }
                },
                Err(e) => println!("Error reading JSON file: {}", e),
            }
        },
        Err(e) => println!("Error serializing to JSON: {}", e),
    }
    
    // === USER INPUT ===
    
    println!("\n--- User Input ---");
    
    println!("Enter your name (or press Enter to skip): ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            if !input.is_empty() {
                println!("Hello, {}!", input);
            } else {
                println!("No input provided");
            }
        },
        Err(e) => println!("Error reading input: {}", e),
    }
    
    // === RANDOM NUMBERS (using standard library) ===
    
    println!("\n--- Random Numbers ---");
    
    // Note: For production code, you'd typically use the `rand` crate
    // Here we'll demonstrate a simple pseudo-random approach
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let time_seed = now.duration_since(UNIX_EPOCH).unwrap().as_nanos();
    
    fn simple_random(seed: u64, time_seed: u128) -> u64 {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        time_seed.hash(&mut hasher);
        hasher.finish()
    }
    
    for i in 0..5 {
        let random_num = simple_random(i, time_seed) % 100;
        println!("Pseudo-random number {}: {}", i + 1, random_num);
    }
    
    // === CLEANUP ===
    
    println!("\n--- Cleanup ---");
    
    // Remove files and directories we created
    let _ = fs::remove_file(filename);
    let _ = fs::remove_file(advanced_filename);
    let _ = fs::remove_file("/tmp/person.json");
    let _ = fs::remove_dir_all(dir_path);
    println!("Cleaned up temporary files and directories");
    
    println!("\n=== Key Takeaways ===");
    println!("• std::env provides access to environment variables and arguments");
    println!("• std::fs offers comprehensive file and directory operations");
    println!("• std::io provides traits and utilities for input/output operations");
    println!("• BufReader/BufWriter improve performance for many small reads/writes");
    println!("• std::process allows executing external commands");
    println!("• External crates like serde enable powerful serialization");
    println!("• std::time provides system time and duration measurements");
    println!("• Rust's standard library is comprehensive and well-designed");
}