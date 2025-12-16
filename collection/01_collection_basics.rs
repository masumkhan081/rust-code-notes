/*
    Topic: Collections - Fundamentals
    
    Rust's standard collection types.
    
    Concepts:
    1. Vec<T>: Dynamic arrays.
    2. HashMap<K, V>: Key-value stores.
    3. HashSet<T>: Unique sets.
*/

use std::collections::{HashMap, HashSet};

pub fn main() {
    println!("--- 01 Collection Fundamentals ---");
    
    // 1. Vector
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vector: {:?}", vec);
    
    // 2. HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("HashMap: {:?}", scores);

    // 3. HashSet
    let mut books = HashSet::new();
    books.insert("The Hobbit");
    books.insert("The Hobbit"); // Duplicate ignored
    println!("HashSet: {:?}", books);
}
