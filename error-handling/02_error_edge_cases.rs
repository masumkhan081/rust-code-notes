/*
    Topic: Error Handling - Edge Cases & Custom Types
    
    Concepts:
    1. The `Anyhow` and `ThisError` crate usage pattern (simulated here with std).
    2. Boxing errors vs Custom Enums.
    3. Unwrap/Expect safety.
*/

use std::fmt;
use std::error::Error;

// Custom Error Type
#[derive(Debug)]
enum MyCustomError {
    Validation(String),
    Network(u32),
}

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::Validation(msg) => write!(f, "Validation Error: {}", msg),
            MyCustomError::Network(code) => write!(f, "Network Error Code: {}", code),
        }
    }
}

impl Error for MyCustomError {}

// Edge Case: Handling multiple error types in one function
// "Box<dyn Error>" is a trait object that can hold ANY error.
fn mixed_error_function(flag: bool) -> Result<(), Box<dyn Error>> {
    if flag {
        // Returning a standard IO error
        let _f = std::fs::File::open("non_existent_file")?;
        Ok(())
    } else {
        // Returning a custom error
        Err(Box::new(MyCustomError::Validation("Invalid input".into())))
    }
}

pub fn main() {
    println!("--- 02 Error Handling Edge Cases ---");

    if let Err(e) = mixed_error_function(false) {
        println!("Caught dynamic error: {}", e);
    }
}
