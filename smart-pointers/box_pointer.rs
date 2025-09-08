// Box<T> - Smart Pointer for Heap Allocation
// ===========================================
// Box<T> allows you to store data on the heap rather than the stack.
// Useful for: large data, recursive types, trait objects.

use std::ops::Deref;

fn main() {
    // Basic Box usage
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Boxes allow recursive types
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive list created successfully");
    
    // Box with large data
    let large_data = Box::new([0; 1000000]); // 1 million zeros on the heap
    println!("Large data allocated on heap");
    
    // Box for trait objects
    trait_objects_with_box();
    
    // Custom Box implementation demo
    custom_box_demo();
    
    // Deref coercion demo
    deref_coercion_demo();
}

// Recursive data structure - only possible with Box
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// Using Box for trait objects
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

fn trait_objects_with_box() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];
    
    for shape in shapes {
        shape.draw();
    }
}

// Custom smart pointer to understand Box better
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn custom_box_demo() {
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref trait allows this
    
    println!("Custom box works!");
}

// Deref coercion examples
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn deref_coercion_demo() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Deref coercion: &MyBox<String> -> &String -> &str
    
    // Without deref coercion, we'd need:
    // hello(&(*m)[..]);
}

// Box vs other allocations
fn box_vs_stack() {
    // Stack allocation - limited size, very fast
    let stack_array = [0; 1000];
    
    // Heap allocation with Box - unlimited size (within memory limits), slightly slower
    let heap_array = Box::new([0; 1000000]);
    
    println!("Stack array: {} bytes", std::mem::size_of_val(&stack_array));
    println!("Box pointer: {} bytes", std::mem::size_of_val(&heap_array));
    println!("Heap array: {} bytes", std::mem::size_of_val(&*heap_array));
}

// Box with custom Drop
struct CustomDrop {
    data: String,
}

impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("Dropping CustomDrop with data: {}", self.data);
    }
}

fn drop_demo() {
    let cd = Box::new(CustomDrop {
        data: String::from("some data"),
    });
    
    // Box and its contents are dropped at end of scope
    println!("CustomDrop created");
} // Drop is called here

// Performance considerations
fn performance_notes() {
    println!("=== Box Performance Notes ===");
    println!("1. Single heap allocation");
    println!("2. No runtime overhead after allocation");
    println!("3. Move semantics - no copying of large data");
    println!("4. Cache-friendly for small data");
    println!("5. Enables zero-cost abstractions");
}

// Common Box patterns
fn common_patterns() {
    // Pattern 1: Recursive data structures
    type TreeBox = Option<Box<TreeNode>>;
    
    struct TreeNode {
        value: i32,
        left: TreeBox,
        right: TreeBox,
    }
    
    let tree = Box::new(TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: None,
        })),
    });
    
    // Pattern 2: Large structs
    struct LargeStruct {
        data: [u8; 1024 * 1024], // 1MB
    }
    
    let large = Box::new(LargeStruct {
        data: [0; 1024 * 1024],
    });
    
    // Pattern 3: Unknown size at compile time
    fn create_data(size: usize) -> Box<[u8]> {
        vec![0; size].into_boxed_slice()
    }
    
    let dynamic_data = create_data(1000);
    
    println!("Common patterns demonstrated");
}

// Box vs Vec
fn box_vs_vec() {
    // Box<[T]> - fixed size, heap allocated
    let boxed_slice: Box<[i32]> = vec![1, 2, 3, 4].into_boxed_slice();
    
    // Vec<T> - dynamic size, heap allocated with capacity
    let vec: Vec<i32> = vec![1, 2, 3, 4];
    
    println!("Boxed slice len: {}", boxed_slice.len());
    println!("Vec len: {}, capacity: {}", vec.len(), vec.capacity());
    
    // Vec uses more memory due to capacity
    println!("Boxed slice size: {}", std::mem::size_of_val(&*boxed_slice));
    println!("Vec data size: {}", std::mem::size_of_val(vec.as_slice()));
}
