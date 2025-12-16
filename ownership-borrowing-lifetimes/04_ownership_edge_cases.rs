/*
    Topic: Ownership & Lifetimes - Edge Cases
    
    Concepts:
    1. Lifetime Elision Rules (where you don't need to write 'a).
    2. Multiple references in function arguments.
    3. 'static lifetime (str literals vs static).
*/

// Elision example: Rust infers lifetimes here
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn main() {
    println!("--- 04 Ownership Edge Cases ---");
    println!("First word of 'Hello World': {}", first_word("Hello World"));
    
    // Static lifetime
    let s: &'static str = "I live forever in the binary";
    println!("{}", s);
}
