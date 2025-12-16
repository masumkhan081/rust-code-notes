/*
    Topic: Smart Pointers - Interview Questions
    
    1. Deref Coercion: Why can I pass &String to a function expecting &str?
    2. Implement a custom "Box" (MyBox) to understand Deref trait.
    3. Drop trait: When is it called?
*/

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn main() {
    println!("--- 05 Smart Pointer Interview ---");
    
    // Q2: MyBox
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref magic happens here
    println!("MyBox deref worked.");
    
    // Q3: Drop
    let _c = CustomSmartPointer { data: String::from("stuff") };
    println!("CustomSmartPointer created.");
    // "Dropping..." will print after main ends
}
