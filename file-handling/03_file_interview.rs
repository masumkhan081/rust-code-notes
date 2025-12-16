/*
    Topic: File Handling - Interview Questions
    
    1. Write a function to count occurrences of a word in a file efficiently.
    2. How to map a file into memory (mmap) - Conceptual.
*/

use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    println!("--- 03 File Handling Interview ---");
    
    // Setup
    let path = "search_test.txt";
    std::fs::write(path, "rust is fast\nrust is safe\nrust is cool").unwrap();
    
    let count = count_word(path, "rust");
    println!("Occurrences of 'rust': {}", count); // Expected: 3
    
    let _ = std::fs::remove_file(path);
}

fn count_word(path: &str, target: &str) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut count = 0;
    
    for line in reader.lines() {
        if let Ok(l) = line {
            count += l.matches(target).count();
        }
    }
    count
}
