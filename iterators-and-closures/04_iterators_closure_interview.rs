/*
    Topic: Iterators and Closures - Interview Questions
    
    1. Implement a custom Iterator.
    2. Explain the difference between `iter()`, `iter_mut()`, and `into_iter()`.
    
    Answer:
    - iter(): Borrows (creates &T)
    - iter_mut(): Mutably borrows (creates &mut T)
    - into_iter(): Consumes (creates T)
*/

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Interview Q1: Implement Iterator
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn main() {
    println!("--- 04 Iterator Interview ---");
    
    let mut counter = Counter::new();
    while let Some(v) = counter.next() {
        print!("{} ", v);
    }
    println!();
}
