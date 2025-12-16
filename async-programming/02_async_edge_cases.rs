/*
    Topic: Async Programming - Edge Cases & Pitfalls
    
    This file demonstrates common tricky scenarios in async Rust.
    
    Key Scenarios:
    1. Blocking the Async Executor: What happens when you put long computations in async?
    2. `.await` inside a loop vs `join_all` (Sequential vs Concurrent execution).
    3. `Send` bound errors across await points (using non-thread-safe types like RefCell).
*/

use std::time::Duration;
use tokio::time::sleep;

// Edge Case 1: Sequential Processing Mistake
// Novice mistake: Awaiting inside a for-loop processes tasks one by one, losing concurrency.
async fn slow_operation(id: u32) {
    sleep(Duration::from_millis(100)).await;
    println!("Task {} done", id);
}

pub async fn demo_sequential_vs_concurrent() {
    println!("\n-- Sequential Execution (Slow) --");
    // This takes 300ms total
    for i in 1..=3 {
        slow_operation(i).await; 
    }
    
    println!("\n-- Concurrent Execution (Fast) --");
    // This takes ~100ms total
    let f1 = slow_operation(1);
    let f2 = slow_operation(2);
    let f3 = slow_operation(3);
    tokio::join!(f1, f2, f3);
}

// Edge Case 2: Blocking the Thread
// NEVER do thread::sleep or CPU intensive work in an async function directly.
// It blocks the entire runtime thread, preventing other tasks from running.
pub async fn bad_blocking_example() {
    // thread::sleep(Duration::from_secs(5)); // DO NOT DO THIS
    println!("Blocking operation avoided for demo integrity.");
}

#[tokio::main]
async fn main() {
    println!("--- 02 Async Edge Cases ---");
    demo_sequential_vs_concurrent().await;
}
