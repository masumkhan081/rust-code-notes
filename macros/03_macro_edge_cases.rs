/*
    Topic: Macros - Edge Cases
    
    1. Macro Hygiene (How macros avoid variable name collisions).
    2. Debugging Macros (log_syntax, recursive limits).
*/

// Hygiene Example
// If we define a variable 'x' in macro, it shouldn't conflict with 'x' outside unless we want it to.
macro_rules! make_x {
    () => {
        let x = 42; 
        // This 'x' is distinct from any caller's 'x' (hygiene)
        println!("Macro x: {}", x);
    };
}

pub fn main() {
    println!("--- 03 Macro Edge Cases ---");
    let x = 100;
    make_x!();
    println!("Outer x: {}", x); // Still 100
}
