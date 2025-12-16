// Basic Ownership and Borrowing
// =============================
// Rust's ownership system ensures memory safety without garbage collection.

fn main() {
    println!("=== Ownership and Borrowing Basics ===");
    
    // Ownership rules
    ownership_rules();
    
    // Moving and copying
    move_copy_demo();
    
    // References and borrowing
    borrowing_demo();
    
    // Mutable references
    mutable_references_demo();
    
    // String vs &str
    string_vs_str_demo();
    
    // Common ownership patterns
    common_patterns_demo();
}

fn ownership_rules() {
    println!("\n--- Ownership Rules ---");
    println!("1. Each value has an owner");
    println!("2. There can only be one owner at a time");
    println!("3. When the owner goes out of scope, the value is dropped");
    
    {
        let s = String::from("hello"); // s owns the string
        println!("s: {}", s);
    } // s goes out of scope and is dropped here
    
    // println!("s: {}", s); // This would be an error - s is no longer valid
    
    // Variable scope
    {
        let x = 5; // x comes into scope
        println!("x in inner scope: {}", x);
    } // x goes out of scope
    
    // println!("x: {}", x); // This would be an error
    
    let y = 10; // y comes into scope
    println!("y: {}", y);
    // y will go out of scope at the end of the function
}

fn move_copy_demo() {
    println!("\n--- Move vs Copy ---");
    
    // Copy types (stored on stack)
    let x = 5;
    let y = x; // x is copied to y
    println!("x: {}, y: {} (both valid because i32 implements Copy)", x, y);
    
    // Move types (stored on heap)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This would be an error - s1 is no longer valid
    
    // Clone to avoid move
    let s3 = String::from("world");
    let s4 = s3.clone(); // Deep copy
    println!("s3: {}, s4: {} (both valid because we cloned)", s3, s4);
    
    // Types that implement Copy
    let tuple1 = (1, 2);
    let tuple2 = tuple1; // Copied
    println!("tuple1: {:?}, tuple2: {:?}", tuple1, tuple2);
    
    let array1 = [1, 2, 3];
    let array2 = array1; // Copied
    println!("array1: {:?}, array2: {:?}", array1, array2);
    
    // Functions and ownership
    fn takes_ownership(some_string: String) {
        println!("Function received: {}", some_string);
    } // some_string goes out of scope and is dropped
    
    fn makes_copy(some_integer: i32) {
        println!("Function received: {}", some_integer);
    } // some_integer goes out of scope but nothing special happens
    
    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function
    // println!("s: {}", s); // This would be an error
    
    let x = 5;
    makes_copy(x); // x is copied into the function
    println!("x is still valid: {}", x); // x is still valid
    
    // Returning values transfers ownership
    fn gives_ownership() -> String {
        let some_string = String::from("yours");
        some_string // returned and moves out
    }
    
    fn takes_and_gives_back(a_string: String) -> String {
        a_string // returned and moves out
    }
    
    let s1 = gives_ownership(); // Function moves return value into s1
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved, return value moves into s3
    
    println!("s1: {}", s1);
    println!("s3: {}", s3);
    // println!("s2: {}", s2); // This would be an error
}

fn borrowing_demo() {
    println!("\n--- References and Borrowing ---");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow s1
    println!("The length of '{}' is {}", s1, len); // s1 is still valid
    
    fn calculate_length(s: &String) -> usize {
        s.len()
    } // s goes out of scope but doesn't drop the value because it doesn't own it
    
    // Multiple immutable references are allowed
    let s = String::from("hello world");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    
    // References must always be valid
    // let reference_to_nothing = dangle(); // This would be an error
    
    // fn dangle() -> &String { // This function would return a reference to a String
    //     let s = String::from("hello");
    //     &s // We return a reference to s
    // } // s goes out of scope and is dropped. Its memory goes away. Danger!
    
    // Correct version - return the String directly
    fn no_dangle() -> String {
        let s = String::from("hello");
        s // Return the String directly
    }
    
    let string = no_dangle();
    println!("Valid string: {}", string);
    
    // Borrowing rules demonstration
    let s = String::from("hello");
    
    // This is fine - many immutable references
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &s; // This is fine
    println!("r3: {}", r3);
    
    // String slices
    let s = String::from("hello world");
    let hello = &s[0..5];  // or &s[..5]
    let world = &s[6..11]; // or &s[6..]
    let whole = &s[..];    // entire string
    
    println!("hello: {}", hello);
    println!("world: {}", world);
    println!("whole: {}", whole);
    
    // First word function using slices
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: {}", word);
    
    // Works with string literals too
    let literal = "hello rust world";
    let word = first_word(literal);
    println!("First word of literal: {}", word);
}

fn mutable_references_demo() {
    println!("\n--- Mutable References ---");
    
    let mut s = String::from("hello");
    change(&mut s);
    println!("After change: {}", s);
    
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    
    // Only one mutable reference at a time
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // This would be an error
    println!("r1: {}", r1);
    
    // After r1 is done being used, we can create another mutable reference
    let r2 = &mut s;
    println!("r2: {}", r2);
    
    // Cannot have mutable and immutable references simultaneously
    let mut s = String::from("hello");
    let r1 = &s; // No problem
    let r2 = &s; // No problem
    // let r3 = &mut s; // BIG PROBLEM - would be an error if r1 and r2 were still used
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // No problem
    println!("r3: {}", r3);
    
    // Mutable slice example
    let mut numbers = [1, 2, 3, 4, 5];
    let slice = &mut numbers[1..4];
    slice[0] = 10;
    slice[1] = 20;
    slice[2] = 30;
    
    println!("Modified array: {:?}", numbers);
}

fn string_vs_str_demo() {
    println!("\n--- String vs &str ---");
    
    // String literals are &str
    let literal: &str = "Hello, world!";
    println!("String literal: {}", literal);
    
    // String is owned
    let owned: String = String::from("Hello, world!");
    println!("Owned string: {}", owned);
    
    // Converting between them
    let from_literal = literal.to_string();
    let from_owned = owned.as_str();
    
    println!("From literal to String: {}", from_literal);
    println!("From String to &str: {}", from_owned);
    
    // Function parameters - prefer &str for flexibility
    fn print_string(s: &str) {
        println!("Function received: {}", s);
    }
    
    print_string("string literal");
    print_string(&owned);
    print_string(&from_literal);
    
    // String methods
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("world!");
    println!("Built string: {}", s);
    
    // String concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    println!("Concatenated: {}", s3);
    
    // format! macro doesn't take ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Formatted: {}", s);
    println!("s1 still valid: {}", s1); // s1 is still valid
}

fn common_patterns_demo() {
    println!("\n--- Common Ownership Patterns ---");
    
    // Pattern 1: Pass by reference for read-only access
    fn analyze_text(text: &str) -> (usize, usize) {
        let word_count = text.split_whitespace().count();
        let char_count = text.len();
        (word_count, char_count)
    }
    
    let text = String::from("Hello world from Rust");
    let (words, chars) = analyze_text(&text);
    println!("Text '{}' has {} words and {} characters", text, words, chars);
    
    // Pattern 2: Pass by mutable reference for modification
    fn capitalize_first_letter(text: &mut String) {
        if let Some(first_char) = text.chars().next() {
            let rest = text.chars().skip(1).collect::<String>();
            *text = format!("{}{}", first_char.to_uppercase(), rest);
        }
    }
    
    let mut greeting = String::from("hello rust");
    println!("Before: {}", greeting);
    capitalize_first_letter(&mut greeting);
    println!("After: {}", greeting);
    
    // Pattern 3: Return owned data
    fn create_greeting(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    
    let name = "Alice";
    let greeting = create_greeting(name);
    println!("Created greeting: {}", greeting);
    
    // Pattern 4: Take ownership and transform
    fn into_uppercase(mut text: String) -> String {
        text = text.to_uppercase();
        text
    }
    
    let message = String::from("hello world");
    let upper_message = into_uppercase(message);
    // message is no longer valid here
    println!("Uppercase: {}", upper_message);
    
    // Pattern 5: Option and ownership
    fn find_word(text: &str, target: &str) -> Option<String> {
        if text.contains(target) {
            Some(format!("Found '{}' in text", target))
        } else {
            None
        }
    }
    
    let text = "Rust is awesome";
    match find_word(text, "Rust") {
        Some(message) => println!("{}", message),
        None => println!("Word not found"),
    }
    
    // Pattern 6: Vec and ownership
    let mut words = Vec::new();
    words.push(String::from("hello"));
    words.push(String::from("world"));
    
    // Iterating without taking ownership
    for word in &words {
        println!("Word: {}", word);
    }
    
    // Taking ownership during iteration
    for word in words {
        println!("Owned word: {}", word);
    }
    // words is no longer valid here
    
    // Pattern 7: Clone when you need multiple owners
    let original = String::from("important data");
    let backup = original.clone();
    
    // Both are valid
    println!("Original: {}", original);
    println!("Backup: {}", backup);
}
