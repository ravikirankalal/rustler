// Concurrency Example
// This example demonstrates threads, channels, and basic concurrent programming
//
// To run this example: cargo run --example 13_concurrency

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Concurrency in Rust ===\n");
    
    // === BASIC THREADS ===
    
    println!("--- Basic Threads ---");
    
    // Spawn a simple thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: Count {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Do work in main thread
    for i in 1..=3 {
        println!("Main: Count {}", i);
        thread::sleep(Duration::from_millis(150));
    }
    
    // Wait for the spawned thread to finish
    handle.join().unwrap();
    println!("Thread completed");
    
    // === MOVING DATA INTO THREADS ===
    
    println!("\n--- Moving Data into Threads ---");
    
    let data = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        println!("Thread received data: {:?}", data);
        let sum: i32 = data.iter().sum();
        println!("Sum calculated in thread: {}", sum);
        sum // Return value from thread
    });
    
    // data is no longer accessible in main thread due to move
    // println!("Data in main: {:?}", data); // This would not compile
    
    let result = handle.join().unwrap();
    println!("Result from thread: {}", result);
    
    // === MULTIPLE THREADS ===
    
    println!("\n--- Multiple Threads ---");
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("Worker thread {} starting", i);
            thread::sleep(Duration::from_millis(100 * (i + 1) as u64));
            println!("Worker thread {} finished", i);
            format!("Result from thread {}", i)
        });
        handles.push(handle);
    }
    
    // Collect results from all threads
    let mut results = vec![];
    for handle in handles {
        let result = handle.join().unwrap();
        results.push(result);
    }
    
    println!("All thread results: {:?}", results);
    
    // === CHANNELS FOR COMMUNICATION ===
    
    println!("\n--- Channels for Communication ---");
    
    // Create a channel
    let (tx, rx) = mpsc::channel();
    
    // Spawn thread that sends data
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    // Receive data in main thread
    for received in rx {
        println!("Received: {}", received);
    }
    
    // === MULTIPLE PRODUCERS ===
    
    println!("\n--- Multiple Producers ---");
    
    let (tx, rx) = mpsc::channel();
    
    // Clone the transmitter for multiple producers
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for j in 0..3 {
                let msg = format!("Producer {} message {}", i, j);
                tx_clone.send(msg).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
    
    // Drop the original transmitter
    drop(tx);
    
    // Receive all messages
    let mut all_messages = vec![];
    for received in rx {
        all_messages.push(received);
    }
    
    println!("Received {} messages:", all_messages.len());
    for msg in all_messages {
        println!("  {}", msg);
    }
    
    // === SHARED STATE WITH MUTEX ===
    
    println!("\n--- Shared State with Mutex ---");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            println!("Thread {} finished incrementing", i);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
    
    // === WORKER POOL PATTERN ===
    
    println!("\n--- Worker Pool Pattern ---");
    
    let (job_tx, job_rx) = mpsc::channel();
    let job_rx = Arc::new(Mutex::new(job_rx));
    
    // Create worker threads
    let mut workers = vec![];
    for id in 0..3 {
        let rx = Arc::clone(&job_rx);
        let worker = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(job_id) => {
                        println!("Worker {} processing job {}", id, job_id);
                        thread::sleep(Duration::from_millis(500)); // Simulate work
                        println!("Worker {} completed job {}", id, job_id);
                    },
                    Err(_) => {
                        println!("Worker {} shutting down", id);
                        break;
                    }
                }
            }
        });
        workers.push(worker);
    }
    
    // Send jobs to workers
    for job_id in 1..=6 {
        job_tx.send(job_id).unwrap();
    }
    
    // Close the channel to signal workers to shut down
    drop(job_tx);
    
    // Wait for all workers to finish
    for worker in workers {
        worker.join().unwrap();
    }
    
    // === CONCURRENT DATA PROCESSING ===
    
    println!("\n--- Concurrent Data Processing ---");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let data = Arc::new(data);
    let results = Arc::new(Mutex::new(vec![]));
    
    let mut handles = vec![];
    let chunk_size = 3;
    
    for start in (0..data.len()).step_by(chunk_size) {
        let data = Arc::clone(&data);
        let results = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            let end = std::cmp::min(start + chunk_size, data.len());
            let chunk_sum: i32 = data[start..end].iter().sum();
            
            println!("Thread processing chunk [{}, {}): sum = {}", start, end, chunk_sum);
            
            // Add result to shared results vector
            let mut results = results.lock().unwrap();
            results.push(chunk_sum);
        });
        
        handles.push(handle);
    }
    
    // Wait for all processing to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_results = results.lock().unwrap();
    let total_sum: i32 = final_results.iter().sum();
    println!("Chunk sums: {:?}", *final_results);
    println!("Total sum: {}", total_sum);
    
    // === ERROR HANDLING IN THREADS ===
    
    println!("\n--- Error Handling in Threads ---");
    
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            if i == 1 {
                panic!("Thread {} panicked!", i);
            }
            format!("Thread {} completed successfully", i)
        })
    }).collect();
    
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(result) => println!("Thread {}: {}", i, result),
            Err(_) => println!("Thread {} panicked", i),
        }
    }
    
    // === THREAD SYNCHRONIZATION EXAMPLE ===
    
    println!("\n--- Thread Synchronization ---");
    
    let barrier = Arc::new(std::sync::Barrier::new(3));
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} doing initial work", i);
            thread::sleep(Duration::from_millis(100 * (i + 1) as u64));
            
            println!("Thread {} waiting at barrier", i);
            barrier.wait();
            
            println!("Thread {} passed barrier, doing final work", i);
            thread::sleep(Duration::from_millis(50));
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // === PRODUCER-CONSUMER PATTERN ===
    
    println!("\n--- Producer-Consumer Pattern ---");
    
    let (tx, rx) = mpsc::channel();
    let buffer_size = 5;
    
    // Producer thread
    let producer = thread::spawn(move || {
        for i in 1..=10 {
            let item = format!("Item {}", i);
            println!("Producing: {}", item);
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        println!("Producer finished");
    });
    
    // Consumer thread
    let consumer = thread::spawn(move || {
        for received in rx {
            println!("Consuming: {}", received);
            thread::sleep(Duration::from_millis(150)); // Consumer is slower
        }
        println!("Consumer finished");
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    // === CONCURRENT CALCULATIONS ===
    
    println!("\n--- Concurrent Calculations ---");
    
    fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }
    
    let numbers = vec![35, 36, 37, 38];
    let mut handles = vec![];
    
    for num in numbers {
        let handle = thread::spawn(move || {
            let start = std::time::Instant::now();
            let result = fibonacci(num);
            let duration = start.elapsed();
            (num, result, duration)
        });
        handles.push(handle);
    }
    
    println!("Calculating Fibonacci numbers concurrently:");
    for handle in handles {
        let (num, result, duration) = handle.join().unwrap();
        println!("  fib({}) = {} (took {:?})", num, result, duration);
    }
    
    println!("\n=== Key Takeaways ===");
    println!("• Use thread::spawn() to create new threads");
    println!("• move closures transfer ownership to threads");
    println!("• join() waits for thread completion and gets the result");
    println!("• Channels (mpsc) enable safe communication between threads");
    println!("• Arc<Mutex<T>> allows sharing mutable data between threads");
    println!("• Rust's ownership system prevents data races at compile time");
    println!("• Use worker pools for managing concurrent tasks");
    println!("• Barriers synchronize threads at specific points");
    println!("• Producer-consumer pattern handles different processing speeds");
}