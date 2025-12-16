/*
    Topic: Async Programming - Interview Questions
    
    Common interview challenges related to Async Rust.
    
    Questions:
    1. How does `Tokio` basic scheduling work? (Concept)
    2. Implement a simple timeout wrapper around a future.
    3. Explain `Pin<Box<T>>` and why `Future` needs to be pinned.
*/

use std::future::Future;
use std::pin::Pin;
use tokio::time::{sleep, Duration};

// Interview Question 1: Implement a simple Timeout
// Write a function that races a future against a timer.
async fn with_timeout<F: Future>(future: F, duration: Duration) -> Result<F::Output, &'static str> {
    tokio::select! {
        res = future => Ok(res),
        _ = sleep(duration) => Err("Timed out"),
    }
}

// Interview Question 2: Why Pinning?
// Explanation: Futures are state machines. If they contain self-referential structs 
// (e.g., a reference to a local variable), moving them in memory would invalidate pointers.
// `Pin` guarantees the object won't move, making it safe to poll.

#[tokio::main]
async fn main() {
    println!("--- 03 Async Interview Prep ---");

    // Testing Timeout
    let slow_task = async {
        sleep(Duration::from_millis(500)).await;
        "Success"
    };

    match with_timeout(slow_task, Duration::from_millis(200)).await {
        Ok(v) => println!("Task result: {}", v),
        Err(e) => println!("Task failed: {}", e), // Expected: Timed out
    }
}
