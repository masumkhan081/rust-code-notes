// Ownership in Rust
// ==================
// Ownership is Rust's most unique feature and enables memory safety without garbage collection.
// Each value in Rust has an owner, and there can only be one owner at a time.

fn main() {
    // RULE 1: Each value in Rust has an owner
    let s1 = String::from("hello"); // s1 owns the string
    
    // RULE 2: There can only be one owner at a time
    let s2 = s1; // ownership moved from s1 to s2
    // println!("{}", s1); // This would cause a compile error! s1 is no longer valid
    println!("{}", s2); // s2 is the owner now
    
    // RULE 3: When the owner goes out of scope, the value is dropped
    {
        let s3 = String::from("temporary");
        println!("{}", s3);
    } // s3 goes out of scope and is dropped here
    
    // Ownership with functions
    let s4 = String::from("function");
    takes_ownership(s4); // s4's value moves into the function
    // println!("{}", s4); // This would error! s4 is no longer valid
    
    let x = 5;
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x
    println!("{}", x); // x is still valid because i32 implements Copy trait
    
    // Getting ownership back from functions
    let s5 = gives_ownership(); // gives_ownership moves its return value into s5
    let s6 = String::from("hello");
    let s7 = takes_and_gives_back(s6); // s6 is moved into takes_and_gives_back, which also moves its return value into s7
    
    println!("s5: {}, s7: {}", s5, s7);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

// Clone vs Copy
fn ownership_clone_copy() {
    // Clone: explicit deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Expensive operation - creates a new String
    println!("s1 = {}, s2 = {}", s1, s2); // Both are valid
    
    // Copy: implicit copy for simple types
    let x = 5;
    let y = x; // Copy happens automatically for i32
    println!("x = {}, y = {}", x, y); // Both are valid
}
