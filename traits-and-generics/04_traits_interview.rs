/*
    Topic: Traits and Generics - Interview Questions
    
    1. Associated Types vs Generics (When to use which?).
    2. Blanket Implementations (Implementing a trait for ALL types implementing another trait).
*/

// Q1 Example: Associated Type (Iterator uses this)
trait IteratorLike {
    type Item; // Only one Item type per implementation
    fn next(&mut self) -> Option<Self::Item>;
}

// Q1 Example: Generics
trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>; // Can implement multiple times for different T
}

pub fn main() {
    println!("--- 04 Traits Interview ---");
    println!("Key difference: use Associated Types when there should only be ONE implementation for a given type.");
}
