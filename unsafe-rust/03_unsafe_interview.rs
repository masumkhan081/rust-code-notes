/*
    Topic: Unsafe Rust - Interview Questions
    
    1. When should you use `unsafe`?
    2. What guarantees does `unsafe` remove? (Borrow checker doesn't turn off, but you can deferencing raw pointers).
    3. How to split_at_mut using unsafe (why is it necessary?).
*/

use std::slice;

// Q3 demo: mutable split
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn main() {
    println!("--- 03 Unsafe Interview ---");
    let mut v = vec![1, 2, 3, 4];
    let (a, b) = split_at_mut(&mut v, 2);
    println!("Split: {:?} and {:?}", a, b);
}
