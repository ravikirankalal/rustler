// Control Flow Example
// This example demonstrates conditional statements and loops in Rust
//
// To run this example: cargo run --example 03_control_flow

fn main() {
    println!("=== Control Flow in Rust ===\n");
    
    // === IF EXPRESSIONS ===
    
    println!("--- If Expressions ---");
    
    let number = 42;
    
    // Basic if statement
    if number > 0 {
        println!("{} is positive", number);
    }
    
    // if-else
    let is_even = if number % 2 == 0 {
        true  // Note: no semicolon here - this is the return value
    } else {
        false
    };
    println!("Is {} even? {}", number, is_even);
    
    // Multiple conditions with else if
    let grade = 85;
    let letter_grade = if grade >= 90 {
        "A"
    } else if grade >= 80 {
        "B"
    } else if grade >= 70 {
        "C"
    } else if grade >= 60 {
        "D"
    } else {
        "F"
    };
    println!("Grade {} receives letter grade: {}", grade, letter_grade);
    
    // Logical operators
    let age = 25;
    let has_license = true;
    
    if age >= 18 && has_license {
        println!("Can drive!");
    }
    
    let is_weekend = false;
    let is_holiday = true;
    
    if is_weekend || is_holiday {
        println!("No work today!");
    }
    
    let is_raining = false;
    if !is_raining {
        println!("Great weather for a walk!");
    }
    
    // === MATCH EXPRESSIONS ===
    
    println!("\n--- Match Expressions ---");
    
    // Match is like switch in other languages, but more powerful
    let day_number = 3;
    let day_name = match day_number {
        1 => "Monday",
        2 => "Tuesday", 
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day", // The underscore is a catch-all pattern
    };
    println!("Day {} is {}", day_number, day_name);
    
    // Match with ranges
    let score = 85;
    let performance = match score {
        90..=100 => "Excellent",     // Inclusive range
        80..=89 => "Good",
        70..=79 => "Average",
        60..=69 => "Below Average",
        0..=59 => "Poor",
        _ => "Invalid score",
    };
    println!("Score {} is {}", score, performance);
    
    // Match with guards (additional conditions)
    let temperature = 25;
    let weather_description = match temperature {
        temp if temp > 30 => "Hot",
        temp if temp > 20 => "Warm",
        temp if temp > 10 => "Cool",
        temp if temp > 0 => "Cold",
        _ => "Freezing",
    };
    println!("Temperature {}°C is {}", temperature, weather_description);
    
    // Match with tuples
    let point = (3, 0);
    match point {
        (0, 0) => println!("Point is at origin"),
        (x, 0) => println!("Point is on x-axis at x = {}", x),
        (0, y) => println!("Point is on y-axis at y = {}", y),
        (x, y) => println!("Point is at ({}, {})", x, y),
    }
    
    // === LOOP EXPRESSIONS ===
    
    println!("\n--- Loop Expressions ---");
    
    // Infinite loop (use with break)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2; // break with a value
        }
        println!("Loop iteration: {}", counter);
    };
    println!("Loop returned: {}", result);
    
    // === WHILE LOOPS ===
    
    println!("\n--- While Loops ---");
    
    let mut countdown = 5;
    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }
    println!("Blast off! 🚀");
    
    // === FOR LOOPS ===
    
    println!("\n--- For Loops ---");
    
    // Iterating over a range
    println!("Counting from 1 to 5:");
    for number in 1..=5 {  // Inclusive range
        println!("  {}", number);
    }
    
    // Iterating over an array
    let fruits = ["apple", "banana", "cherry", "date"];
    println!("Fruits in the basket:");
    for fruit in fruits.iter() {
        println!("  {}", fruit);
    }
    
    // Iterating with index using enumerate
    println!("Fruits with index:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", index, fruit);
    }
    
    // Iterating over a collection in reverse
    println!("Reverse countdown:");
    for number in (1..=5).rev() {
        println!("  {}", number);
    }
    
    // === NESTED LOOPS AND LOOP LABELS ===
    
    println!("\n--- Nested Loops with Labels ---");
    
    let mut count = 0;
    'outer: loop {
        'inner: loop {
            count += 1;
            println!("Count: {}", count);
            
            if count == 3 {
                break 'inner; // Break inner loop
            }
            if count == 6 {
                break 'outer; // Break outer loop
            }
        }
        
        if count >= 6 {
            break;
        }
    }
    
    // === PRACTICAL EXAMPLES ===
    
    println!("\n--- Practical Examples ---");
    
    // Finding the first even number in a list
    let numbers = [1, 3, 5, 8, 9, 12];
    let mut first_even = None;
    
    for &num in numbers.iter() {
        if num % 2 == 0 {
            first_even = Some(num);
            break;
        }
    }
    
    match first_even {
        Some(num) => println!("First even number: {}", num),
        None => println!("No even numbers found"),
    }
    
    // FizzBuzz example
    println!("\nFizzBuzz (1-15):");
    for i in 1..=15 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
    
    // Simple calculator using match
    println!("\nSimple Calculator:");
    let operations = [(10, '+', 5), (10, '-', 3), (6, '*', 7), (15, '/', 3)];
    
    for (a, op, b) in operations.iter() {
        let result = match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => {
                println!("Unknown operator: {}", op);
                continue;
            }
        };
        println!("{} {} {} = {}", a, op, b, result);
    }
    
    println!("\n=== Key Takeaways ===");
    println!("• if expressions can return values (no semicolon on last expression)");
    println!("• match is exhaustive - you must handle all possible cases");
    println!("• loop creates infinite loops, use break to exit");
    println!("• while loops continue while condition is true");
    println!("• for loops are great for iterating over collections and ranges");
    println!("• Use loop labels to break/continue specific nested loops");
    println!("• Combine control flow for complex logic patterns");
}