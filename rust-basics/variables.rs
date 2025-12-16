// Variables and Mutability in Rust
// =================================
// Variables are immutable by default in Rust for safety and concurrency.

fn main() {
    println!("=== Variables and Mutability ===");
    
    // Immutable variables (default)
    let x = 5;
    println!("The value of x is: {}", x);
    
    // x = 6; // This would cause a compile error!
    // println!("The value of x is: {}", x);
    
    // Mutable variables
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // This is allowed because y is mutable
    println!("The value of y is: {}", y);
    
    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
    
    // Shadowing - reusing variable names
    let z = 5;
    let z = z + 1; // Shadows the previous z
    {
        let z = z * 2; // Shadows again in this scope
        println!("The value of z in inner scope is: {}", z); // 12
    }
    println!("The value of z is: {}", z); // 6
    
    // Shadowing with different types
    let spaces = "   ";           // string type
    let spaces = spaces.len();    // number type
    println!("Number of spaces: {}", spaces);
    
    // This wouldn't work with mut:
    // let mut spaces = "   ";
    // spaces = spaces.len(); // Error! Can't change type of mutable variable
    
    // Variable binding patterns
    let (a, b) = (1, 2);
    println!("a = {}, b = {}", a, b);
    
    let [first, second, third] = [1, 2, 3];
    println!("Array elements: {}, {}, {}", first, second, third);
    
    // Unused variables (prefix with _ to avoid warnings)
    let _unused_variable = 42;
    
    // Multiple assignments
    let (mut x1, mut y1) = (1, 2);
    println!("Before swap: x1 = {}, y1 = {}", x1, y1);
    let temp = x1;
    x1 = y1;
    y1 = temp;
    println!("After swap: x1 = {}, y1 = {}", x1, y1);
}

// Global constants
const MAX_POINTS: u32 = 100_000;

// Static variables (global mutable state - requires unsafe to modify)
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: usize = 0;

fn demonstrate_globals() {
    println!("Max points: {}", MAX_POINTS);
    println!("Static string: {}", HELLO_WORLD);
    
    // Accessing mutable static requires unsafe
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
