// Structs and Enums Example
// This example demonstrates struct and enum definitions, usage, and pattern matching
//
// To run this example: cargo run --example 06_structs_enums

fn main() {
    println!("=== Structs and Enums in Rust ===\n");
    
    // === STRUCT BASICS ===
    
    println!("--- Struct Basics ---");
    
    // Creating struct instances
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
        active: true,
    };
    
    println!("Person: {} ({}), Email: {}, Active: {}", 
             person1.name, person1.age, person1.email, person1.active);
    
    // Accessing individual fields
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    
    // Mutable structs
    let mut person2 = Person {
        name: String::from("Bob"),
        age: 25,
        email: String::from("bob@example.com"),
        active: false,
    };
    
    person2.age = 26; // Can modify fields if struct is mutable
    person2.active = true;
    println!("Updated person: {} is now {} and active: {}", 
             person2.name, person2.age, person2.active);
    
    // === STRUCT CONSTRUCTION SHORTCUTS ===
    
    println!("\n--- Struct Construction Shortcuts ---");
    
    // Field init shorthand
    let name = String::from("Charlie");
    let email = String::from("charlie@example.com");
    let person3 = Person {
        name, // Same as name: name,
        email, // Same as email: email,
        age: 35,
        active: true,
    };
    println!("Created with shorthand: {}", person3.name);
    
    // Struct update syntax
    let person4 = Person {
        name: String::from("Diana"),
        email: String::from("diana@example.com"),
        ..person3 // Take remaining fields from person3
    };
    println!("Created with update syntax: {} (age: {})", person4.name, person4.age);
    
    // === TUPLE STRUCTS ===
    
    println!("\n--- Tuple Structs ---");
    
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let red = Color(255, 0, 0);
    
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("White: ({}, {}, {})", white.0, white.1, white.2);
    println!("Red: ({}, {}, {})", red.0, red.1, red.2);
    
    let origin = Point(0, 0);
    let point = Point(3, 4);
    println!("Origin: ({}, {})", origin.0, origin.1);
    println!("Point: ({}, {})", point.0, point.1);
    
    // === UNIT STRUCTS ===
    
    println!("\n--- Unit Structs ---");
    
    let unit = UnitStruct;
    println!("Unit struct created: {:?}", unit);
    
    // === METHODS AND ASSOCIATED FUNCTIONS ===
    
    println!("\n--- Methods and Associated Functions ---");
    
    // Using associated function (constructor)
    let rect1 = Rectangle::new(5.0, 3.0);
    let rect2 = Rectangle::square(4.0);
    
    println!("Rectangle 1: {}x{}", rect1.width, rect1.height);
    println!("Rectangle 2 (square): {}x{}", rect2.width, rect2.height);
    
    // Using methods
    println!("Area of rectangle 1: {}", rect1.area());
    println!("Perimeter of rectangle 1: {}", rect1.perimeter());
    println!("Is rectangle 1 square? {}", rect1.is_square());
    println!("Is rectangle 2 square? {}", rect2.is_square());
    
    // Method that takes ownership
    let dimensions = rect1.dimensions();
    println!("Dimensions: {:?}", dimensions);
    
    // Can rectangle 1 hold rectangle 2?
    let small_rect = Rectangle::new(2.0, 1.0);
    println!("Can rectangle 2 hold small rectangle? {}", 
             rect2.can_hold(&small_rect));
    
    // === ENUM BASICS ===
    
    println!("\n--- Enum Basics ---");
    
    let sunny = Weather::Sunny;
    let rainy = Weather::Rainy;
    let snowy = Weather::Snowy { inches: 3 };
    let cloudy = Weather::Cloudy { coverage: 75 };
    
    describe_weather(sunny);
    describe_weather(rainy);
    describe_weather(snowy);
    describe_weather(cloudy);
    
    // === ENUMS WITH DATA ===
    
    println!("\n--- Enums with Data ---");
    
    let msg1 = Message::Text(String::from("Hello, World!"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::ChangeColor(255, 0, 0);
    let msg4 = Message::Quit;
    
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
    
    // === IP ADDRESS EXAMPLE ===
    
    println!("\n--- IP Address Example ---");
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    print_ip_address(home);
    print_ip_address(loopback);
    
    // === OPTION ENUM ===
    
    println!("\n--- Option Enum ---");
    
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);
    
    // Pattern matching with Option
    match some_number {
        Some(value) => println!("Found a value: {}", value),
        None => println!("No value found"),
    }
    
    match absent_number {
        Some(value) => println!("Found a value: {}", value),
        None => println!("No value found"),
    }
    
    // Using if let with Option
    if let Some(value) = some_string {
        println!("String value: {}", value);
    }
    
    // === RESULT ENUM ===
    
    println!("\n--- Result Enum ---");
    
    let good_result: Result<i32, String> = Ok(42);
    let bad_result: Result<i32, String> = Err(String::from("Something went wrong"));
    
    match good_result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    match bad_result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    // === PATTERN MATCHING ADVANCED ===
    
    println!("\n--- Advanced Pattern Matching ---");
    
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers {
        match number {
            1 => println!("One!"),
            2 | 3 => println!("Two or three!"),
            4..=6 => println!("Four through six!"),
            _ => println!("Something else!"),
        }
    }
    
    // Destructuring structs in match
    let person = Person {
        name: String::from("John"),
        age: 25,
        email: String::from("john@example.com"),
        active: true,
    };
    
    match person {
        Person { name, age: 25, .. } => {
            println!("Found a 25-year-old named {}", name);
        },
        Person { name, age, active: true, .. } => {
            println!("Found an active person named {} aged {}", name, age);
        },
        _ => println!("Some other person"),
    }
    
    // === PRACTICAL EXAMPLES ===
    
    println!("\n--- Practical Examples ---");
    
    // Creating a simple calculator using enums
    let operations = vec![
        Operation::Add(10, 5),
        Operation::Subtract(10, 3),
        Operation::Multiply(4, 7),
        Operation::Divide(15, 3),
        Operation::Divide(10, 0), // Division by zero
    ];
    
    for op in operations {
        let result = calculate(op);
        match result {
            Ok(value) => println!("Result: {}", value),
            Err(error) => println!("Error: {}", error),
        }
    }
    
    // State machine example
    let mut game = Game::new();
    game.start();
    game.pause();
    game.resume();
    game.end();
    
    println!("\n=== Key Takeaways ===");
    println!("• Structs group related data together");
    println!("• Use impl blocks to define methods and associated functions");
    println!("• Enums define types with multiple possible variants");
    println!("• Enums can store data in their variants");
    println!("• Pattern matching with match is exhaustive");
    println!("• Option and Result are powerful enums for handling null/error cases");
    println!("• Destructuring allows extracting data from structs and enums");
}

// === STRUCT DEFINITIONS ===

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

// Tuple structs
struct Color(u8, u8, u8);
struct Point(i32, i32);

// Unit struct
#[derive(Debug)]
struct UnitStruct;

// Rectangle with methods
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Another constructor
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Method that borrows self
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // Method that takes ownership
    fn dimensions(self) -> (f64, f64) {
        (self.width, self.height)
    }
    
    // Method that takes another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

// === ENUM DEFINITIONS ===

#[derive(Debug)]
enum Weather {
    Sunny,
    Rainy,
    Snowy { inches: u32 },
    Cloudy { coverage: u8 },
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
    ChangeColor(u8, u8, u8),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

// State machine example
#[derive(Debug)]
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

struct Game {
    state: GameState,
}

impl Game {
    fn new() -> Game {
        Game {
            state: GameState::Menu,
        }
    }
    
    fn start(&mut self) {
        self.state = GameState::Playing;
        println!("Game started! State: {:?}", self.state);
    }
    
    fn pause(&mut self) {
        match self.state {
            GameState::Playing => {
                self.state = GameState::Paused;
                println!("Game paused! State: {:?}", self.state);
            },
            _ => println!("Cannot pause from current state: {:?}", self.state),
        }
    }
    
    fn resume(&mut self) {
        match self.state {
            GameState::Paused => {
                self.state = GameState::Playing;
                println!("Game resumed! State: {:?}", self.state);
            },
            _ => println!("Cannot resume from current state: {:?}", self.state),
        }
    }
    
    fn end(&mut self) {
        self.state = GameState::GameOver;
        println!("Game ended! State: {:?}", self.state);
    }
}

// === HELPER FUNCTIONS ===

fn describe_weather(weather: Weather) {
    match weather {
        Weather::Sunny => println!("It's a beautiful sunny day!"),
        Weather::Rainy => println!("Don't forget your umbrella!"),
        Weather::Snowy { inches } => {
            println!("Snow warning: {} inches expected!", inches);
        },
        Weather::Cloudy { coverage } => {
            println!("Cloudy with {}% cloud coverage", coverage);
        },
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received"),
        Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
        Message::Text(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        },
    }
}

fn print_ip_address(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 address: {}.{}.{}.{}", a, b, c, d);
        },
        IpAddr::V6(addr) => {
            println!("IPv6 address: {}", addr);
        },
    }
}

fn calculate(operation: Operation) -> Result<i32, String> {
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0 {
                Err(String::from("Cannot divide by zero"))
            } else {
                Ok(a / b)
            }
        },
    }
}