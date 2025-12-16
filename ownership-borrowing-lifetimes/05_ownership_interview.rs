/*
    Topic: Ownership & Lifetimes - Interview Questions
    
    1. What is the "Double Free" error and how does Rust prevent it?
    2. Create a struct that holds a reference. What is the lifetime syntax?
    3. Explain `NLL` (Non-Lexical Lifetimes).
*/

// Interview Q2: Struct with reference
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn main() {
    println!("--- 05 Ownership Interview ---");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {}", i.part);
    
    // NLL Demo:
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    // In old Rust, y's scope would last until end of block, blocking 'x' usage.
    // In NLL, y dies here (last usage).
    
    println!("x: {}", x);
}
