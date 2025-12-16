/*
    Topic: Smart Pointers - Edge Cases (Ref Cycles)
    
    Concepts:
    1. Reference Cycles using Rc and RefCell (Introduction to memory leaks).
    2. Using `Weak<T>` to break cycles.
*/

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    // Use Weak to prevent cycle if child points back to parent
    parent: RefCell<Weak<Node>>,
}

pub fn main() {
    println!("--- 04 Smart Pointer Edge Cases ---");
    
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    
    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());
}
