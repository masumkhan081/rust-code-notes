/*
    Topic: Structs and Enums - Interview Questions
    
    1. What is the size of an Enum? (Discriminant + Largest variant).
    2. Implement the "New Type" pattern.
*/

// New Type Pattern: 
// Wrapping a type to enforce safety or add behavior without overhead.
struct UserId(u64);

pub fn main() {
    println!("--- 03 Struct Enum Interview ---");
    
    let id = UserId(100200);
    println!("Created secure UserID");
    
    // Size demonstration
    println!("Size of Option<i32>: {}", std::mem::size_of::<Option<i32>>()); // 8 bytes (4 for i32, 4 for alignment+tag)
}
