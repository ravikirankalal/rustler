// Traits and Generics Example
// This example demonstrates implementing traits, generic functions and structs
//
// To run this example: cargo run --example 09_traits_generics

use std::fmt::Display;

fn main() {
    println!("=== Traits and Generics in Rust ===\n");
    
    // === BASIC TRAITS ===
    
    println!("--- Basic Traits ---");
    
    let dog = Dog {
        name: String::from("Buddy"),
        breed: String::from("Golden Retriever"),
    };
    
    let cat = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };
    
    // Using trait methods
    dog.speak();
    cat.speak();
    
    println!("{}", dog.info());
    println!("{}", cat.info());
    
    // === TRAIT OBJECTS ===
    
    println!("\n--- Trait Objects ---");
    
    // Store different types that implement the same trait
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(dog),
        Box::new(cat),
        Box::new(Bird { species: String::from("Parrot") }),
    ];
    
    println!("All animals speak:");
    for animal in &animals {
        animal.speak();
    }
    
    // === DERIVED TRAITS ===
    
    println!("\n--- Derived Traits ---");
    
    let point1 = Point { x: 3, y: 4 };
    let point2 = Point { x: 3, y: 4 };
    let point3 = Point { x: 1, y: 2 };
    
    // Debug trait
    println!("Point1: {:?}", point1);
    
    // PartialEq trait
    println!("Point1 == Point2: {}", point1 == point2);
    println!("Point1 == Point3: {}", point1 == point3);
    
    // Clone trait
    let point4 = point1.clone();
    println!("Cloned point: {:?}", point4);
    
    // === BASIC GENERICS ===
    
    println!("\n--- Basic Generics ---");
    
    // Generic functions
    let int_pair = create_pair(5, 10);
    let string_pair = create_pair("hello".to_string(), "world".to_string());
    let float_pair = create_pair(3.14, 2.71);
    
    println!("Int pair: {:?}", int_pair);
    println!("String pair: {:?}", string_pair);
    println!("Float pair: {:?}", float_pair);
    
    // Generic struct
    let int_container = Container::new(42);
    let string_container = Container::new("Hello, Rust!");
    
    println!("Int container: {:?}", int_container);
    println!("String container: {:?}", string_container);
    
    println!("Container values: {} and {}", 
             int_container.get(), string_container.get());
    
    // === TRAIT BOUNDS ===
    
    println!("\n--- Trait Bounds ---");
    
    // Functions with trait bounds
    let numbers = vec![1, 5, 3, 9, 2];
    let largest_num = find_largest(&numbers);
    println!("Largest number: {}", largest_num);
    
    let words = vec!["apple", "zebra", "banana", "cherry"];
    let largest_word = find_largest(&words);
    println!("Largest word: {}", largest_word);
    
    // Display trait bound
    display_item(42);
    display_item("Hello, World!");
    display_item(3.14159);
    
    // Multiple trait bounds
    let summary1 = summarize_and_display(vec![1, 2, 3, 4, 5]);
    let summary2 = summarize_and_display(vec!["a", "b", "c"]);
    println!("{}", summary1);
    println!("{}", summary2);
    
    // === ADVANCED TRAIT IMPLEMENTATIONS ===
    
    println!("\n--- Advanced Trait Implementations ---");
    
    let mut shopping_list = ShoppingList::new();
    shopping_list.add_item("Milk".to_string());
    shopping_list.add_item("Bread".to_string());
    shopping_list.add_item("Eggs".to_string());
    
    println!("Shopping list: {}", shopping_list);
    println!("Number of items: {}", shopping_list.len());
    
    // Summary trait
    println!("Summary: {}", shopping_list.summarize());
    
    // === WHERE CLAUSES ===
    
    println!("\n--- Where Clauses ---");
    
    let result1 = complex_function(10, 20);
    let result2 = complex_function("hello", "world");
    
    println!("Complex result 1: {}", result1);
    println!("Complex result 2: {}", result2);
    
    // === ASSOCIATED TYPES ===
    
    println!("\n--- Associated Types ---");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.into_iter().collect();
    println!("Doubled numbers: {:?}", doubled);
    
    // Custom iterator
    let counter = Counter::new();
    let first_five: Vec<usize> = counter.take(5).collect();
    println!("First five from counter: {:?}", first_five);
    
    // === GENERIC IMPLEMENTATIONS ===
    
    println!("\n--- Generic Implementations ---");
    
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("Stack contents:");
    while let Some(item) = stack.pop() {
        println!("  Popped: {}", item);
    }
    
    // String stack
    let mut string_stack = Stack::new();
    string_stack.push("first");
    string_stack.push("second");
    string_stack.push("third");
    
    println!("String stack size: {}", string_stack.size());
    
    // === OPERATOR OVERLOADING ===
    
    println!("\n--- Operator Overloading ---");
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // Using Add trait
    
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p1 + p2 = {:?}", p3);
    
    // === DEFAULT IMPLEMENTATIONS ===
    
    println!("\n--- Default Implementations ---");
    
    let article = Article {
        title: "Understanding Rust Traits".to_string(),
        author: "Rust Developer".to_string(),
        content: "Traits are a powerful feature...".to_string(),
    };
    
    let tweet = Tweet {
        username: "@rustlang".to_string(),
        content: "Rust 1.70 is now available!".to_string(),
        reply: false,
        retweet: false,
    };
    
    // Default implementation
    println!("Article summary: {}", article.summarize());
    
    // Custom implementation
    println!("Tweet summary: {}", tweet.summarize());
    
    // === CONDITIONAL IMPLEMENTATIONS ===
    
    println!("\n--- Conditional Implementations ---");
    
    let wrapper1 = Wrapper(42); // Use a simple integer instead of Vec
    let wrapper2 = Wrapper("Hello, World!");
    
    println!("Wrapper1: {}", wrapper1);
    println!("Wrapper2: {}", wrapper2);
    
    // This will only work for types that implement Display
    // The blanket implementation provides Display for Wrapper<T> where T: Display
    
    println!("\n=== Key Takeaways ===");
    println!("• Traits define shared behavior across different types");
    println!("• Generics enable code reuse with type safety");
    println!("• Trait bounds restrict generic types to those implementing specific traits");
    println!("• Associated types allow traits to define placeholder types");
    println!("• Default trait implementations reduce code duplication");
    println!("• Operator overloading is done through trait implementations");
    println!("• Where clauses make complex trait bounds more readable");
}

// === TRAIT DEFINITIONS ===

// Basic trait
trait Animal {
    fn speak(&self);
    fn info(&self) -> String;
}

// Trait with default implementation
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// === STRUCT DEFINITIONS ===

struct Dog {
    name: String,
    breed: String,
}

struct Cat {
    name: String,
    age: u32,
}

struct Bird {
    species: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Container<T> {
    value: T,
}

struct ShoppingList {
    items: Vec<String>,
}

struct Article {
    title: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

struct Wrapper<T>(T);

struct Counter {
    current: usize,
}

#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

// === TRAIT IMPLEMENTATIONS ===

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says: Woof! Woof!", self.name);
    }
    
    fn info(&self) -> String {
        format!("{} is a {}", self.name, self.breed)
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says: Meow!", self.name);
    }
    
    fn info(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

impl Animal for Bird {
    fn speak(&self) {
        println!("The {} squawks!", self.species);
    }
    
    fn info(&self) -> String {
        format!("This is a {}", self.species)
    }
}

// Operator overloading using Add trait
impl std::ops::Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Generic implementation
impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
}

impl ShoppingList {
    fn new() -> ShoppingList {
        ShoppingList {
            items: Vec::new(),
        }
    }
    
    fn add_item(&mut self, item: String) {
        self.items.push(item);
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

impl Display for ShoppingList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.items.join(", "))
    }
}

impl Summary for ShoppingList {
    fn summarize(&self) -> String {
        format!("Shopping list with {} items", self.items.len())
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {} ({} chars)", self.title, self.author, self.content.len())
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let status = if self.reply { "reply" } else if self.retweet { "retweet" } else { "original" };
        format!("{}: {} [{}]", self.username, self.content, status)
    }
}

// Conditional implementation - only implement Display for Wrapper<T> where T implements Display
impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Wrapper({})", self.0)
    }
}

impl Counter {
    fn new() -> Counter {
        Counter { current: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 10 {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            items: Vec::new(),
        }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

// === GENERIC FUNCTIONS ===

fn create_pair<T>(first: T, second: T) -> (T, T) {
    (first, second)
}

// Function with trait bound
fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn display_item<T: Display>(item: T) {
    println!("Displaying: {}", item);
}

// Multiple trait bounds
fn summarize_and_display<T>(items: Vec<T>) -> String 
where
    T: Display + Clone,
{
    let length = items.len();
    if length > 0 {
        format!("List with {} items, first: {}", length, items[0])
    } else {
        "Empty list".to_string()
    }
}

// Complex function with where clause
fn complex_function<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Display + Clone,
{
    format!("t: {}, u: {}", t, u)
}