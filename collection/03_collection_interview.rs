/*
    Topic: Collections - Interview Questions
    
    1. Merge two sorted vectors into one sorted vector (without sort()).
    2. Implement an LRU Cache (Conceptual/Basic).
*/

pub fn main() {
    println!("--- 03 Collection Interview ---");
    
    let v1 = vec![1, 3, 5];
    let v2 = vec![2, 4, 6];
    let merged = merge_sorted_arrays(&v1, &v2);
    println!("Merged: {:?}", merged);
}

// Interview Q1: Merge Sorted
fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }
    
    // Append remaining
    if i < a.len() { result.extend_from_slice(&a[i..]); }
    if j < b.len() { result.extend_from_slice(&b[j..]); }
    
    result
}
