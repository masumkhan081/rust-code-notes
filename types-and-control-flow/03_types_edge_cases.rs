/*
    Topic: Types and Control Flow - Edge Cases
    
    1. Overflowing integers (debug vs release).
    2. Floating point weirdness (NaN).
*/

pub fn main() {
    println!("--- 03 Types Edge Cases ---");
    
    // Overflow wrap
    let mut i: u8 = 255;
    // In release mode, this wraps to 0. In debug, it panics.
    // i = i.wrapping_add(1); 
    println!("Wrapped u8: {}", i.wrapping_add(1));
}
