/*
    Topic: Structs and Enums - Edge Cases
    
    Concepts:
    1. Methods on Enums.
    2. The "Update Syntax" for structs.
    3. Zero-sized types (Unit structs).
*/

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
}

impl Message {
    fn call(&self) {
        println!("Message processed");
    }
}

pub fn main() {
    println!("--- 02 Struct Enum Edge Cases ---");
    
    let p1 = Point { x: 1, y: 2 };
    // Update syntax: copy remaining fields from p1
    let p2 = Point { x: 5, ..p1 };
    
    println!("p2: {:?}", p2);
    
    let m = Message::Move { x: 10, y: 10 };
    m.call();
}
