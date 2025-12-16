/*
    Topic: Collections - Edge Cases & Performance
    
    Concepts:
    1. Pre-allocating capacity to avoid reallocation overhead.
    2. HashMap entry API for cleaner updates.
    3. Using BTreeMap/BTreeSet for sorted data.
*/

use std::collections::HashMap;

pub fn main() {
    println!("--- 02 Collection Edge Cases ---");
    
    // Efficiency: Pre-allocation
    // If we know we need 1 million items, allocate first to prevent resizing.
    let mut big_vec = Vec::with_capacity(10); // using small num for demo
    big_vec.push(1);
    println!("Vector capacity: {}", big_vec.capacity());
    
    // Entry API usage
    let mut letters = HashMap::new();
    let text = "hello world";
    
    for ch in text.chars() {
        // "or_insert" only inserts 0 if the key is missing, then returns mutable ref
        let count = letters.entry(ch).or_insert(0);
        *count += 1;
    }
    println!("Character counts: {:?}", letters);
}
