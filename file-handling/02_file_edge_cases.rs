/*
    Topic: File Handling - Edge Cases
    
    Concepts:
    1. Buffering: BufReader/BufWriter for performance on large files.
    2. Handling JSON/Serialization (serde_json) - simulated string here.
    3. Recursively walking directories.
*/

use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    println!("--- 02 File Handling Edge Cases ---");
    
    // Simulating reading a large file line by line using buffer
    // Creating a dummy file first
    use std::io::Write;
    let path = "large_dummy.txt";
    let mut f = File::create(path).unwrap();
    writeln!(f, "Line 1").unwrap();
    writeln!(f, "Line 2").unwrap();
    writeln!(f, "Line 3").unwrap();
    
    // Reading with buffer
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    
    for (idx, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => println!("{}: {}", idx + 1, content),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    let _ = std::fs::remove_file(path);
}
