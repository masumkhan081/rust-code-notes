/*
    Topic: Structs and Enums - Fundamentals
    
    Concepts:
    1. Struct definition and instantiation.
    2. Tuple structs.
    3. Enums and Pattern Matching.
*/

struct User {
    username: String,
    active: bool,
}

enum Status {
    Active,
    Inactive,
    Suspended(String), // Enum with data
}

pub fn main() {
    println!("--- 01 Struct Enum Fundamentals ---");
    
    let user = User {
        username: String::from("alice"),
        active: true,
    };
    
    let status = Status::Suspended(String::from("Spamming"));
    
    match status {
        Status::Active => println!("Keep going"),
        Status::Inactive => println!("Wake up"),
        Status::Suspended(reason) => println!("Banned due to: {}", reason),
    }
}
