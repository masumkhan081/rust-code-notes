/*
    Topic: Types and Control Flow - Interview Questions
    
    1. Scalar vs Compound types.
    2. Stack vs Heap (Where do primitives live?).
    3. Array vs Tuple vs Slice.
*/

pub fn main() {
    println!("--- 04 Types Interview ---");
    
    let a = [1, 2, 3]; // Array (Stack, fixed)
    let b = &a[1..];   // Slice (View into array)
    let c = (1, "hello"); // Tuple (Fixed size, mixed types)
    
    println!("Types demo done.");
}
