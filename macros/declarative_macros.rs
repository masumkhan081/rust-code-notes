// Declarative Macros in Rust
// ===========================
// Macros allow you to write code that writes other code (metaprogramming).
// Declarative macros are defined using macro_rules! and use pattern matching.

fn main() {
    // Using built-in macros
    println!("Hello, world!"); // println! is a macro
    vec![1, 2, 3]; // vec! is a macro
    
    // Using custom macros
    say_hello!();
    say_hello!(to "World");
    say_hello!(times 3);
    say_hello!(to "Rust", times 2);
    
    // Vector creation macro
    let v1 = my_vec![1, 2, 3, 4];
    let v2 = my_vec![42; 5]; // Create vector with 5 elements, all 42
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    
    // Hash map creation macro
    let map = hashmap! {
        "key1" => "value1",
        "key2" => "value2",
        "key3" => "value3",
    };
    println!("HashMap: {:?}", map);
    
    // Multiple patterns macro
    calculate!(add 2, 3);
    calculate!(multiply 4, 5);
    calculate!(subtract 10, 3);
    
    // Variable argument macro
    print_all!("Hello", "World", "from", "Rust");
    
    // Debug macro
    debug_vars!(v1, v2, map);
}

// Simple macro without arguments
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// Macro with optional arguments and different patterns
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    (to $name:expr) => {
        println!("Hello, {}!", $name);
    };
    (times $count:expr) => {
        for _ in 0..$count {
            println!("Hello!");
        }
    };
    (to $name:expr, times $count:expr) => {
        for _ in 0..$count {
            println!("Hello, {}!", $name);
        }
    };
}

// Vector creation macro (simplified version of vec!)
macro_rules! my_vec {
    // Empty vector
    () => {
        Vec::new()
    };
    // Vector with repeated element
    ($elem:expr; $n:expr) => {
        {
            let mut v = Vec::new();
            for _ in 0..$n {
                v.push($elem);
            }
            v
        }
    };
    // Vector with list of elements
    ($($elem:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($elem);
            )+
            v
        }
    };
}

// HashMap creation macro
use std::collections::HashMap;

macro_rules! hashmap {
    // Empty hashmap
    () => {
        HashMap::new()
    };
    // Hashmap with key-value pairs
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

// Macro with multiple patterns for different operations
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        {
            let result = $a + $b;
            println!("{} + {} = {}", $a, $b, result);
            result
        }
    };
    (multiply $a:expr, $b:expr) => {
        {
            let result = $a * $b;
            println!("{} * {} = {}", $a, $b, result);
            result
        }
    };
    (subtract $a:expr, $b:expr) => {
        {
            let result = $a - $b;
            println!("{} - {} = {}", $a, $b, result);
            result
        }
    };
}

// Variadic macro (variable number of arguments)
macro_rules! print_all {
    ($($arg:expr),*) => {
        {
            print!("Arguments: ");
            $(
                print!("{} ", $arg);
            )*
            println!();
        }
    };
}

// Debug macro that prints variable names and values
macro_rules! debug_vars {
    ($($var:ident),*) => {
        {
            println!("Debug variables:");
            $(
                println!("  {} = {:?}", stringify!($var), $var);
            )*
        }
    };
}

// Conditional compilation macro
macro_rules! log {
    ($level:ident, $msg:expr) => {
        {
            #[cfg(debug_assertions)]
            {
                println!("[{}] {}", stringify!($level), $msg);
            }
            #[cfg(not(debug_assertions))]
            {
                // In release mode, only log errors
                if stringify!($level) == "ERROR" {
                    println!("[{}] {}", stringify!($level), $msg);
                }
            }
        }
    };
}

fn logging_example() {
    log!(INFO, "This is an info message");
    log!(ERROR, "This is an error message");
    log!(DEBUG, "This is a debug message");
}

// Macro for creating getter and setter methods
macro_rules! getter_setter {
    ($field:ident, $field_type:ty) => {
        paste::paste! {
            pub fn [<get_ $field>](&self) -> &$field_type {
                &self.$field
            }
            
            pub fn [<set_ $field>](&mut self, value: $field_type) {
                self.$field = value;
            }
        }
    };
}

// DSL (Domain Specific Language) macro
macro_rules! html {
    // Self-closing tag
    ($tag:ident) => {
        format!("<{} />", stringify!($tag))
    };
    // Tag with content
    ($tag:ident { $($content:tt)* }) => {
        format!("<{}>{}</{}>", stringify!($tag), html!($($content)*), stringify!($tag))
    };
    // Text content
    ($text:expr) => {
        $text.to_string()
    };
    // Multiple elements
    ($($element:tt)*) => {
        {
            let mut result = String::new();
            $(
                result.push_str(&html!($element));
            )*
            result
        }
    };
}

fn html_example() {
    let html_output = html! {
        div {
            h1 { "Welcome to my page" }
            p { "This is a paragraph" }
            br
        }
    };
    println!("Generated HTML: {}", html_output);
}

// Macro with complex pattern matching
macro_rules! match_ast {
    // Match expressions
    (expr $e:expr) => {
        println!("Expression: {}", stringify!($e));
    };
    // Match statements
    (stmt $s:stmt) => {
        println!("Statement: {}", stringify!($s));
    };
    // Match patterns
    (pat $p:pat) => {
        println!("Pattern: {}", stringify!($p));
    };
    // Match types
    (ty $t:ty) => {
        println!("Type: {}", stringify!($t));
    };
}

// Recursive macro
macro_rules! count {
    () => (0usize);
    ($head:tt $($tail:tt)*) => (1usize + count!($($tail)*));
}

fn recursive_macro_example() {
    let count = count!(a b c d e);
    println!("Count: {}", count);
}

// Macro for implementing traits
macro_rules! impl_add {
    ($type:ty) => {
        impl std::ops::Add for $type {
            type Output = Self;
            
            fn add(self, other: Self) -> Self::Output {
                Self(self.0 + other.0)
            }
        }
    };
}

struct MyInt(i32);
impl_add!(MyInt);

// Advanced: Macro that generates different code based on input
macro_rules! generate_function {
    // Generate a function that returns a constant
    (const $name:ident -> $value:expr) => {
        fn $name() -> i32 {
            $value
        }
    };
    // Generate a function that adds two numbers
    (add $name:ident) => {
        fn $name(a: i32, b: i32) -> i32 {
            a + b
        }
    };
    // Generate a function that multiplies two numbers
    (multiply $name:ident) => {
        fn $name(a: i32, b: i32) -> i32 {
            a * b
        }
    };
}

generate_function!(const get_answer -> 42);
generate_function!(add add_numbers);
generate_function!(multiply multiply_numbers);

fn generated_functions_example() {
    println!("Answer: {}", get_answer());
    println!("Add: {}", add_numbers(5, 3));
    println!("Multiply: {}", multiply_numbers(4, 6));
}

// Hygiene example - macro variables don't interfere with surrounding code
macro_rules! hygiene_test {
    ($e:expr) => {
        {
            let x = 42; // This x doesn't interfere with x in the calling scope
            $e + x
        }
    };
}

fn hygiene_example() {
    let x = 10;
    let result = hygiene_test!(x); // Uses both the outer x and the macro's x
    println!("Hygiene result: {}", result); // Should be 10 + 42 = 52
}
