/*
    Topic: Concurrency - Shared State (Mutex & Arc)
    
    How to safely share data between threads.
    
    Concepts:
    1. Arc (Atomic Reference Counting) - for shared ownership across threads.
    2. Mutex (Mutual Exclusion) - for allowing one thread to access data at a time.
*/

use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    println!("--- 02 Shared State ---");

    // We want to share this counter across multiple threads
    // Arc allows multiple owners
    // Mutex allows interior mutability safely
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the mutex to get access to data
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
