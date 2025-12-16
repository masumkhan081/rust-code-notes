/*
    Topic: Traits and Generics - Edge Cases
    
    1. Trait Objects (dynamic dispatch) vs Generics (static dispatch).
    2. Object Safety rules (why some traits can't be objects).
    3. Supertraits (Inheritance-like behavior).
*/

trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Edge case: Using 'impl Trait' in return position
fn make_summary() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    }
}

pub fn main() {
    println!("--- 03 Traits Edge Cases ---");
    let s = make_summary();
    println!("Summary: {}", s.summarize());
}
