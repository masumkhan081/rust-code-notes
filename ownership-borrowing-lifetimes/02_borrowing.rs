// Borrowing in Rust
// ==================
// Borrowing allows you to refer to some value without taking ownership of it.
// References are created with & and dereferenced with *

fn main() {
    // Immutable References
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 creates a reference to s1
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid!
    
    // Mutable References
    let mut s2 = String::from("hello");
    change(&mut s2); // &mut s2 creates a mutable reference
    println!("{}", s2);
    
    // Reference Rules Demo
    reference_rules();
    
    // Dangling References (this won't compile)
    // let reference_to_nothing = dangle(); // This would cause a compile error
    let valid_reference = no_dangle(); // This works
    println!("{}", valid_reference);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn reference_rules() {
    let mut s = String::from("hello");
    
    // Rule 1: You can have either one mutable reference or any number of immutable references
    
    // Multiple immutable references are allowed
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    
    // One mutable reference
    let r3 = &mut s; // no problem
    println!("{}", r3);
    
    // Rule 2: References must always be valid
    // The following would cause a compile error:
    /*
    let r4 = &s; // no problem
    let r5 = &mut s; // BIG PROBLEM! Cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, {}", r4, r5);
    */
    
    // Scopes help with borrowing
    {
        let r4 = &mut s; // no problem
        println!("{}", r4);
    } // r4 goes out of scope here, so we can make a new reference with no problems.
    
    let r5 = &s; // no problem
    println!("{}", r5);
}

// This function would create a dangling reference (won't compile)
/*
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s // Return the String directly, transferring ownership
}

// String slices - a special kind of reference
fn string_slices() {
    let s = String::from("hello world");
    
    let hello = &s[0..5]; // Reference to part of the string
    let world = &s[6..11];
    // let world = &s[6..]; // From index 6 to the end
    // let hello = &s[..5]; // From the start to index 5
    // let slice = &s[..]; // The entire string
    
    println!("{} {}", hello, world);
    
    // String literals are slices
    let s = "Hello, world!"; // This is a &str, a slice pointing to a specific point in the binary
    
    // First word function using slices
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // This would cause a compile error! Can't modify s while word is borrowed
    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Better version that works with both String and &str
fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
