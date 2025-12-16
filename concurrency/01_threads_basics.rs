/*
    Topic: Concurrency - Basic Threads
    
    Fundametals of spawning threads and waiting for them.
    
    Concepts:
    1. thread::spawn
    2. JoinHandle (.join())
    3. Moving ownership into threads (move keywords)
*/

use std::thread;
use std::time::Duration;

pub fn main() {
    println!("--- 01 Threads Basics ---");

    // 1. Spawning a thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Spawned thread: {} (executing concurrently)", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread work
    for i in 1..3 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 2. Waiting for thread to finish
    handle.join().unwrap();
    
    // 3. Using `move` closures
    let v = vec![1, 2, 3];
    let handle_move = thread::spawn(move || {
        println!("Vector captured by value: {:?}", v);
    });
    handle_move.join().unwrap();
}
