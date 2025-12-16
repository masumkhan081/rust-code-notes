/*
    Topic: Iterators and Closures - Edge Cases
    
    1. Modifying a collection while iterating (not allowed directly).
    2. Infinite iterators (taking care not to collect them).
    3. Closure lifetime capture issues.
*/

pub fn main() {
    println!("--- 03 Iterator Edge Cases ---");
    
    // Infinite iterator
    let numbers = 1..; // Infinite range
    let five_nums: Vec<_> = numbers.take(5).collect();
    println!("Five nums: {:?}", five_nums);
}
