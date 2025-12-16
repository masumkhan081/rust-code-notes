/*
    Topic: Functions - Fundamentals
    
    Concepts:
    1. Function syntax, arguments, return values.
    2. Statements vs Expressions.
    3. Closures (intro).
*/

fn add_one(x: i32) -> i32 {
    x + 1 // Expression (no semicolon) returns value
}

pub fn main() {
    println!("--- 01 Function Fundamentals ---");
    
    let y = add_one(5);
    println!("5 + 1 = {}", y);
    
    // Diverging function (never returns) - Example
    // fn panic_function() -> ! { panic!("Crash"); }
}
