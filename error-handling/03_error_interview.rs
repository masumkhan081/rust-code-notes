/*
    Topic: Error Handling - Interview Questions
    
    Questions:
    1. Implement a custom error type that wraps multiple upstream errors (e.g. IO and ParseInt).
    2. When to use `expect`, `unwrap`, vs `?` in production code?
       (Answer: Use `?` for lib/propagation. Use `expect` only if you are 100% sure or it's a fatal bug invariant.)
*/

use std::fs;
use std::io;
use std::num;

// Interview: Create a composite error type
#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

// Implementing From allows `?` to automatically convert upstream errors to our CliError
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

// A function that can fail in two ways: Read file, or Parse number
fn read_number_from_file(path: &str) -> Result<i32, CliError> {
    let content = fs::read_to_string(path)?; // map io::Error -> CliError::Io
    let num: i32 = content.trim().parse()?;  // map ParseIntError -> CliError::Parse
    Ok(num)
}

pub fn main() {
    println!("--- 03 Error Handling Interview ---");
    println!("Demonstrating composite error handling...");
    
    match read_number_from_file("number.txt") {
        Ok(n) => println!("Number: {}", n),
        Err(CliError::Io(e)) => println!("IO Failure: {}", e),
        Err(CliError::Parse(e)) => println!("Parsing Failure: {}", e),
    }
}
