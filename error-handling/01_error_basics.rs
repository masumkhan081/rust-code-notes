/*
    Topic: Error Handling - Fundamentals
    
    Rust treats error handling as a first-class citizen using `Result` and `Option`.
    
    Concepts:
    1. Unrecoverable vs Recoverable errors (panic! vs Result).
    2. The `?` operator for propagation.
    3. Custom Error types basics.
*/

use std::fs::File;
use std::io::{self, Read};

// 1. Basic Result Handling
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 2. The `?` Operator (Concise version of above)
fn read_username_concise() -> Result<String, io::Error> {
    // The ? operator automatically returns Err if the result is Err.
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn main() {
    println!("--- 01 Error Handling Basics ---");
    
    // Simulating call
    match read_username_concise() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Failed to read file: {}", e), // Expected failure if file doesn't exist
    }
}
