// Functions in Rust
// =================
// Functions are defined with 'fn' and use snake_case naming.

fn main() {
    println!("=== Functions in Rust ===");
    
    // Basic function call
    greet();
    
    // Function with parameters
    greet_person("Alice");
    greet_person_age("Bob", 25);
    
    // Functions with return values
    let sum = add(5, 3);
    println!("Sum: {}", sum);
    
    let product = multiply(4, 7);
    println!("Product: {}", product);
    
    // Different return styles
    let result1 = explicit_return(10);
    let result2 = implicit_return(10);
    println!("Explicit return: {}, Implicit return: {}", result1, result2);
    
    // Early returns
    let check1 = check_positive(5);
    let check2 = check_positive(-3);
    println!("Check positive 5: {}, Check positive -3: {}", check1, check2);
    
    // Multiple return values with tuples
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} remainder {}", quotient, remainder);
    
    // Functions that don't return values (return unit type)
    print_separator();
    
    // Function expressions
    function_expressions();
    
    // Nested functions
    nested_functions_demo();
    
    // Recursion
    let factorial_5 = factorial(5);
    println!("5! = {}", factorial_5);
    
    let fib_10 = fibonacci(10);
    println!("10th Fibonacci number: {}", fib_10);
}

// Basic function - no parameters, no return value
fn greet() {
    println!("Hello, world!");
}

// Function with one parameter
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Function with multiple parameters
fn greet_person_age(name: &str, age: u32) {
    println!("Hello, {}! You are {} years old.", name, age);
}

// Function with return value (explicit type)
fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon = return value
}

// Another function with return value
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// Explicit return statement
fn explicit_return(x: i32) -> i32 {
    return x * 2; // Using 'return' keyword
}

// Implicit return (more idiomatic)
fn implicit_return(x: i32) -> i32 {
    x * 2 // No semicolon
}

// Early return
fn check_positive(num: i32) -> bool {
    if num <= 0 {
        return false; // Early return
    }
    true // Implicit return
}

// Multiple return values using tuple
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

// Function that prints (returns unit type ())
fn print_separator() {
    println!("------------------------");
}

// Unit type function (explicit)
fn do_nothing() -> () {
    // This function returns the unit type ()
}

// Function expressions and statements
fn function_expressions() {
    println!("\n--- Function Expressions vs Statements ---");
    
    // Statement (doesn't return a value)
    let x = 5; // This is a statement
    
    // Expression (returns a value)
    let y = {
        let inner = 3;
        inner + 1 // Expression, no semicolon
    };
    
    println!("x: {}, y: {}", x, y);
    
    // If as expression
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number: {}", number);
    
    // Match as expression
    let value = 3;
    let description = match value {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("Description: {}", description);
}

// Nested functions
fn nested_functions_demo() {
    println!("\n--- Nested Functions ---");
    
    fn outer_function() {
        println!("This is the outer function");
        
        fn inner_function() {
            println!("This is the inner function");
        }
        
        inner_function(); // Call inner function
    }
    
    outer_function();
}

// Recursive function
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Another recursive function
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Function with complex logic
fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None; // Early return for edge case
    }
    
    let mut max = numbers[0];
    for &number in numbers.iter().skip(1) {
        if number > max {
            max = number;
        }
    }
    Some(max)
}

// Function with multiple parameters of different types
fn format_person_info(name: &str, age: u32, height: f32, is_student: bool) -> String {
    let status = if is_student { "student" } else { "not a student" };
    format!(
        "{} is {} years old, {:.1}m tall, and is {}",
        name, age, height, status
    )
}

// Function that takes a function pointer
fn apply_operation(x: i32, y: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(x, y)
}

// Functions to use with function pointers
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn power(base: i32, exp: i32) -> i32 {
    base.pow(exp as u32)
}

// Demonstrate function pointers
fn function_pointers_demo() {
    println!("\n--- Function Pointers ---");
    
    let result1 = apply_operation(10, 3, add);
    let result2 = apply_operation(10, 3, subtract);
    let result3 = apply_operation(2, 3, power);
    
    println!("10 + 3 = {}", result1);
    println!("10 - 3 = {}", result2);
    println!("2 ^ 3 = {}", result3);
}

// Function with default behavior using Option
fn greet_with_title(name: &str, title: Option<&str>) {
    match title {
        Some(t) => println!("Hello, {} {}!", t, name),
        None => println!("Hello, {}!", name),
    }
}

// Demonstrating various function patterns
fn function_patterns_demo() {
    println!("\n--- Function Patterns ---");
    
    // Option return type
    let numbers = [1, 5, 3, 9, 2];
    match find_max(&numbers) {
        Some(max) => println!("Maximum: {}", max),
        None => println!("No maximum (empty array)"),
    }
    
    // Complex formatting
    let info = format_person_info("Alice", 25, 1.65, true);
    println!("{}", info);
    
    // Function pointers
    function_pointers_demo();
    
    // Optional parameters
    greet_with_title("Alice", Some("Dr."));
    greet_with_title("Bob", None);
}

// Function documentation (doc comments)
/// Calculates the area of a rectangle.
/// 
/// # Arguments
/// 
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
/// 
/// # Examples
/// 
/// ```
/// let area = calculate_area(5.0, 3.0);
/// assert_eq!(area, 15.0);
/// ```
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

/// Checks if a number is prime.
/// 
/// # Arguments
/// 
/// * `n` - The number to check
/// 
/// # Returns
/// 
/// Returns `true` if the number is prime, `false` otherwise.
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

// Test the documented functions
fn test_documented_functions() {
    println!("\n--- Documented Functions ---");
    
    let area = calculate_area(5.0, 3.0);
    println!("Area of 5x3 rectangle: {}", area);
    
    for i in 1..=20 {
        if is_prime(i) {
            print!("{} ", i);
        }
    }
    println!("(prime numbers 1-20)");
}
