/*
    Topic: Functions - Interview Questions
    
    1. Explain `Fn` vs `FnMut` vs `FnOnce`.
    2. Write a function that curries another function.
*/

// Interview Q1: Fn Traits
// FnOnce: Consumes context (can be called once)
// FnMut: Mutates context
// Fn: Reads context

pub fn main() {
    println!("--- 03 Function Interview ---");
    
    let mut x = 0;
    
    // FnMut closure
    let mut increment = || {
        x += 1; 
        println!("Incremented to {}", x);
    };
    
    call_mut(&mut increment);
    call_mut(&mut increment);
}

fn call_mut<F>(mut f: F) 
where F: FnMut() 
{
    f()
}
