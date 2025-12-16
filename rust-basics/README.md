# Rust Basics

This folder contains fundamental Rust concepts that every Rust developer should understand. These examples are designed for beginners and cover the essential building blocks of the Rust programming language.

## 📁 Files Overview

### 1. `variables.rs` - Variables and Mutability
- **Immutability by default**: Variables are immutable unless declared with `mut`
- **Variable shadowing**: Redeclaring variables with the same name
- **Constants**: Compile-time constants with `const`
- **Scope and lifetime**: Understanding when variables are valid

**Key Concepts:**
```rust
let x = 5;          // Immutable
let mut y = 10;     // Mutable
const MAX: i32 = 100; // Constant
let x = x + 1;      // Shadowing
```

### 2. `data_types.rs` - Data Types and Type System
- **Scalar types**: integers, floats, booleans, characters
- **Compound types**: tuples and arrays
- **Type inference**: Rust's ability to deduce types
- **Type conversions**: Safe and explicit type casting

**Key Concepts:**
```rust
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = 'R';
let tuple: (i32, f64, char) = (42, 3.14, 'R');
let array: [i32; 5] = [1, 2, 3, 4, 5];
```

### 3. `functions.rs` - Functions and Control
- **Function definitions**: Parameters and return types
- **Function expressions vs statements**
- **Return values**: Implicit and explicit returns
- **Function documentation**: Using doc comments

**Key Concepts:**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // Expression (no semicolon)
}

fn greet(name: &str) {
    println!("Hello, {}!", name);  // Statement
}
```

### 4. `control_flow.rs` - Control Flow Structures
- **Conditional statements**: `if`, `else if`, `else`
- **Loops**: `loop`, `while`, `for`
- **Pattern matching**: `match` expressions
- **Loop control**: `break` and `continue`

**Key Concepts:**
```rust
if condition {
    // code
} else {
    // code
}

for item in collection {
    // code
}

match value {
    pattern1 => result1,
    pattern2 => result2,
    _ => default_result,
}
```

### 5. `structs_enums.rs` - Structs and Enums
- **Struct definitions**: Grouping related data
- **Methods and associated functions**: `impl` blocks
- **Enum definitions**: Types with multiple variants
- **Pattern matching**: Destructuring structs and enums

**Key Concepts:**
```rust
struct User {
    name: String,
    age: u32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl User {
    fn new(name: String, age: u32) -> User {
        User { name, age }
    }
}
```

### 6. `ownership_borrowing.rs` - Ownership and Borrowing
- **Ownership rules**: Each value has one owner
- **Moving vs copying**: Stack vs heap data
- **References**: Borrowing without taking ownership
- **Mutable references**: Rules and restrictions

**Key Concepts:**
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved, no longer valid

let s3 = String::from("world");
let len = calculate_length(&s3);  // Borrow s3

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop the value
```

### 7. `error_handling.rs` - Error Handling
- **Panic**: Unrecoverable errors with `panic!`
- **Result type**: Recoverable errors with `Result<T, E>`
- **Option type**: Handling optional values
- **Error propagation**: Using the `?` operator

**Key Concepts:**
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

let result = divide(10.0, 2.0)?;  // Propagate error
```

## 🚀 How to Run the Examples

Each file contains a complete, runnable example with a `main()` function. You can run them individually:

```bash
# Run a specific example
rustc variables.rs && ./variables

# Or using cargo (if you have a Cargo.toml in the parent directory)
cargo run --bin variables
```

## 📚 Learning Path

**Recommended order for learning:**

1. **Start here**: `variables.rs` - Understand mutability and basic syntax
2. **Type system**: `data_types.rs` - Learn about Rust's type system
3. **Functions**: `functions.rs` - Understand function definitions and calls
4. **Control flow**: `control_flow.rs` - Master conditional logic and loops
5. **Data structures**: `structs_enums.rs` - Learn to organize data
6. **Memory management**: `ownership_borrowing.rs` - Rust's core concept
7. **Error handling**: `error_handling.rs` - Handle errors gracefully

## 🎯 Key Learning Objectives

After working through these examples, you should understand:

- ✅ Why Rust variables are immutable by default
- ✅ How Rust's type system prevents common programming errors
- ✅ The difference between expressions and statements
- ✅ How to use pattern matching effectively
- ✅ How to define and use structs and enums
- ✅ Rust's ownership system and borrowing rules
- ✅ How to handle errors without exceptions

## 🔗 Next Steps

Once you're comfortable with these basics, you can explore the advanced topics in the parent directory:

- **ownership-borrowing-lifetimes/**: Deep dive into memory management
- **traits-and-generics/**: Code reuse and abstraction
- **error-handling/**: Advanced error handling patterns
- **concurrency/**: Safe concurrent programming
- **macros/**: Code generation and metaprogramming

## 💡 Tips for Success

1. **Run the code**: Don't just read - compile and run each example
2. **Experiment**: Modify the examples to see what happens
3. **Break things**: Try to create compile errors to understand the rules
4. **Practice**: Write your own variations of these patterns
5. **Ask questions**: The Rust compiler error messages are very helpful

## 📖 Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust documentation
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get you used to Rust

Happy learning! 🦀
