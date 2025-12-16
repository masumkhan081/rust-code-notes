/*
    Topic: File Handling - Fundamentals
    
    Concepts:
    1. Reading files (read_to_string).
    2. Writing files (write_all).
    3. OpenOptions for appending.
*/

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

pub fn main() -> io::Result<()> {
    println!("--- 01 File Handling Basics ---");
    
    // Write
    let path = "output.txt";
    let mut file = File::create(path)?;
    file.write_all(b"Hello, File System!")?;
    
    // Read
    let content = fs::read_to_string(path)?;
    println!("Read content: {}", content);
    
    // Cleanup
    fs::remove_file(path)?;
    Ok(())
}
