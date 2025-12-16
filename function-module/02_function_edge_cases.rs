/*
    Topic: Functions - Edge Cases
    
    Concepts:
    1. Higher Order Functions (functions that take functions).
    2. Returning Closures.
    3. Use of `impl Fn`, `impl FnMut`, `impl FnOnce`.
*/

// Example: Function taking another function as argument
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

pub fn main() {
    println!("--- 02 Function Edge Cases ---");
    
    let result = apply_twice(|x| x * 2, 5);
    println!("(5 * 2) * 2 = {}", result);
}
