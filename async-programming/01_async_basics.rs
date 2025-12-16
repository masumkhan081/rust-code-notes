/*
    Topic: Async Programming - Fundamentals
    
    This file covers the basics of writing asynchronous code in Rust.
    
    Key Concepts:
    1. `async fn` and `.await` syntax.
    2. The `Future` trait (conceptual understanding).
    3. Using a runtime (like `tokio`) to execute async code.
    
    Note: To run these examples, you usually need an async runtime in your Cargo.toml.
    Example:
    [dependencies]
    tokio = { version = "1", features = ["full"] }
*/

// Example 1: Basic Async Function
// Async functions return a Future, which does nothing until `.await`ed or polled.
async fn hello_world() {
    println!("Hello, Async World!");
}

// Example 2: Awaiting a Future
// Inside an async function or block, we use `.await` to suspend execution until the future completes.
async fn do_async_work() {
    let future = hello_world();
    // At this point, "Hello, Async World!" has NOT been printed.
    
    future.await; // Now the future is driven to completion.
}

// Example 3: Returning Values from Async Functions
async fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::main]
pub async fn main() {
    println!("--- 01 Async Fundamentals ---");
    
    do_async_work().await;
    
    let sum = add(10, 20).await;
    println!("Async Sum: {}", sum);
}
