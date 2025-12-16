// Lifetimes in Rust
// ==================
// Lifetimes ensure that references are valid as long as we need them to be.
// They prevent dangling references and ensure memory safety.

fn main() {
    // Basic lifetime example
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(&string1, string2);
    println!("The longest string is {}", result);
    
    // Lifetime with structs
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {}", i.part);
    
    // Method with lifetimes
    println!("Announcement: {}", i.announce_and_return_part("Breaking news!"));
    
    // Static lifetime
    static_lifetime_example();
}

// Function with explicit lifetime annotations
// The lifetime 'a means that the returned reference will be valid as long as both input references are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime annotations
// This struct holds a reference, so we need a lifetime parameter
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Implementation block with lifetimes
impl<'a> ImportantExcerpt<'a> {
    // Method that doesn't need explicit lifetime annotation due to lifetime elision rules
    fn level(&self) -> i32 {
        3
    }
    
    // Method with explicit lifetime annotation
    // The returned reference has the same lifetime as self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Lifetime Elision Rules
// ========================
// The compiler can often infer lifetimes, so you don't always need to write them explicitly

// Rule 1: Each parameter that is a reference gets its own lifetime parameter
fn first_word_elision(s: &str) -> &str { // Actually: fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Rule 2: If there is exactly one input lifetime parameter, 
// that lifetime is assigned to all output lifetime parameters
fn get_first_char(s: &str) -> &str { // Actually: fn get_first_char<'a>(s: &'a str) -> &'a str
    &s[0..1]
}

// Rule 3: If there are multiple input lifetime parameters, but one of them is &self or &mut self,
// the lifetime of self is assigned to all output lifetime parameters
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self) -> &str { // Actually: fn return_part(&self) -> &'a str
        self.part
    }
}

// When you need explicit lifetime annotations
// This function needs explicit annotations because the compiler can't determine which input lifetime the output should have
fn longest_with_context<'a>(x: &'a str, y: &'a str, context: &str) -> &'a str {
    println!("Context: {}", context);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Static lifetime
fn static_lifetime_example() {
    let s: &'static str = "I have a static lifetime.";
    // String literals always have 'static lifetime because they're stored in the program's binary
    println!("{}", s);
    
    // Static variables also have 'static lifetime
    static HELLO_WORLD: &str = "Hello, world!";
    println!("{}", HELLO_WORLD);
}

// Lifetime bounds
fn lifetime_bounds() {
    // T: 'a means that T must live at least as long as 'a
    fn ref_x<'a, T>(x: &'a T) -> &'a T 
    where 
        T: 'a  // T must live at least as long as 'a
    {
        x
    }
}

// Common lifetime patterns
fn lifetime_patterns() {
    // Pattern 1: Multiple references with same lifetime
    fn compare_strings<'a>(s1: &'a str, s2: &'a str) -> bool {
        s1.len() == s2.len()
    }
    
    // Pattern 2: Input and output have different lifetimes
    fn get_default<'a>(input: &'a str, default: &'static str) -> &'a str {
        if input.is_empty() {
            default // This won't compile! 'static doesn't necessarily live as long as 'a
        } else {
            input
        }
    }
    
    // Correct version:
    fn get_default_correct<'a>(input: &'a str, default: &'a str) -> &'a str {
        if input.is_empty() {
            default
        } else {
            input
        }
    }
}
