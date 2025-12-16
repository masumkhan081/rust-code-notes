// Basic Error Handling
// ====================
// Rust groups errors into recoverable (Result) and unrecoverable (panic!) errors.

fn main() {
    println!("=== Error Handling Basics ===");
    
    // Panic and unrecoverable errors
    panic_demo();
    
    // Result type for recoverable errors
    result_demo();
    
    // Option type for handling None values
    option_demo();
    
    // Error propagation
    propagation_demo();
    
    // Practical error handling patterns
    practical_patterns();
}

fn panic_demo() {
    println!("\n--- Panic! (Unrecoverable Errors) ---");
    
    // Direct panic
    // panic!("Crash and burn"); // Uncomment to see panic
    
    // Panic with formatting
    let x = 5;
    if x < 10 {
        println!("x is less than 10, no panic needed");
    } else {
        // panic!("x is {} which is >= 10", x); // Uncomment to see panic
    }
    
    // Array out of bounds causes panic
    let v = vec![1, 2, 3];
    // let element = v[99]; // This would panic
    
    // Better way to access array elements safely
    match v.get(99) {
        Some(element) => println!("Element at index 99: {}", element),
        None => println!("No element at index 99"),
    }
    
    // Using unwrap (panics on error)
    let some_value: Option<i32> = Some(5);
    let value = some_value.unwrap(); // Safe because we know it's Some
    println!("Unwrapped value: {}", value);
    
    // let none_value: Option<i32> = None;
    // let value = none_value.unwrap(); // This would panic
    
    // Using expect (panic with custom message)
    let some_value: Option<i32> = Some(10);
    let value = some_value.expect("Should have a value");
    println!("Expected value: {}", value);
    
    // Backtrace environment variable
    println!("Set RUST_BACKTRACE=1 to see backtraces on panic");
}

fn result_demo() {
    println!("\n--- Result<T, E> (Recoverable Errors) ---");
    
    use std::fs::File;
    use std::io::ErrorKind;
    
    // Basic Result handling
    let result = File::open("hello.txt");
    
    match result {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);
        }
        Err(error) => {
            println!("Failed to open file: {:?}", error);
        }
    }
    
    // Different ways to handle specific errors
    let result = File::open("hello.txt");
    
    let _file = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found, creating it...");
                match File::create("hello.txt") {
                    Ok(fc) => {
                        println!("File created successfully");
                        fc
                    }
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    
    // Using unwrap_or_else for cleaner error handling
    let _file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    // Creating your own Result-returning functions
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    // Testing the divide function
    let results = vec![
        divide(10.0, 2.0),
        divide(10.0, 0.0),
        divide(15.0, 3.0),
    ];
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("Result {}: {}", i + 1, value),
            Err(error) => println!("Error {}: {}", i + 1, error),
        }
    }
    
    // Using map and map_err
    let result = divide(20.0, 4.0)
        .map(|x| x * 2.0)
        .map_err(|e| format!("Math error: {}", e));
    
    println!("Mapped result: {:?}", result);
    
    // Using and_then for chaining operations
    fn sqrt(x: f64) -> Result<f64, String> {
        if x < 0.0 {
            Err("Cannot take square root of negative number".to_string())
        } else {
            Ok(x.sqrt())
        }
    }
    
    let result = divide(16.0, 4.0)
        .and_then(|x| sqrt(x));
    
    println!("Chained operations result: {:?}", result);
    
    // Using or_else for fallback
    let result1 = divide(10.0, 0.0)
        .or_else(|_| Ok(0.0)); // Fallback to 0.0 on error
    
    println!("With fallback: {:?}", result1);
}

fn option_demo() {
    println!("\n--- Option<T> ---");
    
    // Option represents a value that might or might not exist
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    // Pattern matching with Option
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("Got None"),
    }
    
    match absent_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("Got None"),
    }
    
    // Using if let for simple cases
    if let Some(n) = some_number {
        println!("if let: Got number {}", n);
    }
    
    // Option methods
    let x = Some(2);
    let y: Option<i32> = None;
    
    // unwrap_or provides a default value
    println!("x.unwrap_or(0): {}", x.unwrap_or(0));
    println!("y.unwrap_or(0): {}", y.unwrap_or(0));
    
    // unwrap_or_else computes default value
    println!("y.unwrap_or_else(|| 42): {}", y.unwrap_or_else(|| 42));
    
    // map transforms the value if it exists
    let doubled = x.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);
    
    let doubled_none = y.map(|n| n * 2);
    println!("Doubled None: {:?}", doubled_none);
    
    // and_then for chaining operations that return Option
    fn checked_divide(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
    
    let result = Some(20)
        .and_then(|x| checked_divide(x, 4))
        .and_then(|x| checked_divide(x, 2));
    
    println!("Chained division: {:?}", result);
    
    // filter keeps value only if predicate is true
    let number = Some(4);
    let even = number.filter(|&n| n % 2 == 0);
    let odd = number.filter(|&n| n % 2 == 1);
    
    println!("Even filter: {:?}", even);
    println!("Odd filter: {:?}", odd);
    
    // Working with collections and Option
    let numbers = vec![1, 2, 3, 4, 5];
    
    // find returns Option
    let found = numbers.iter().find(|&&x| x > 3);
    println!("First number > 3: {:?}", found);
    
    // position returns Option
    let position = numbers.iter().position(|&x| x == 3);
    println!("Position of 3: {:?}", position);
    
    // Converting between Option and Result
    let opt = Some(42);
    let res: Result<i32, &str> = opt.ok_or("No value");
    println!("Option to Result: {:?}", res);
    
    let res: Result<i32, &str> = Ok(42);
    let opt = res.ok();
    println!("Result to Option: {:?}", opt);
}

fn propagation_demo() {
    println!("\n--- Error Propagation ---");
    
    use std::fs::File;
    use std::io::{self, Read};
    
    // Manual error propagation
    fn read_username_from_file_v1() -> Result<String, io::Error> {
        let f = File::open("username.txt");
        
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        
        let mut s = String::new();
        
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    
    // Using ? operator for error propagation
    fn read_username_from_file_v2() -> Result<String, io::Error> {
        let mut f = File::open("username.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    
    // Even shorter with chaining
    fn read_username_from_file_v3() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("username.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    
    // Using fs::read_to_string (built-in function)
    fn read_username_from_file_v4() -> Result<String, io::Error> {
        std::fs::read_to_string("username.txt")
    }
    
    // Test all versions
    let versions = vec![
        ("Manual propagation", read_username_from_file_v1()),
        ("Using ? operator", read_username_from_file_v2()),
        ("Chained ? operator", read_username_from_file_v3()),
        ("Built-in function", read_username_from_file_v4()),
    ];
    
    for (name, result) in versions {
        match result {
            Ok(content) => println!("{}: Read {} characters", name, content.len()),
            Err(error) => println!("{}: Error - {:?}", name, error),
        }
    }
    
    // The ? operator can also be used with Option
    fn parse_and_add() -> Option<i32> {
        let num1: i32 = "5".parse().ok()?;
        let num2: i32 = "10".parse().ok()?;
        Some(num1 + num2)
    }
    
    println!("Parse and add result: {:?}", parse_and_add());
    
    // Custom error propagation
    fn divide_strings(a: &str, b: &str) -> Result<f64, String> {
        let a_num: f64 = a.parse()
            .map_err(|_| format!("'{}' is not a valid number", a))?;
        let b_num: f64 = b.parse()
            .map_err(|_| format!("'{}' is not a valid number", b))?;
        
        if b_num == 0.0 {
            return Err("Cannot divide by zero".to_string());
        }
        
        Ok(a_num / b_num)
    }
    
    let test_cases = vec![
        ("10", "2"),
        ("invalid", "2"),
        ("10", "0"),
        ("15", "3"),
    ];
    
    for (a, b) in test_cases {
        match divide_strings(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(error) => println!("Error dividing {} by {}: {}", a, b, error),
        }
    }
}

fn practical_patterns() {
    println!("\n--- Practical Error Handling Patterns ---");
    
    // Pattern 1: Early return with ?
    fn process_data(input: &str) -> Result<String, String> {
        if input.is_empty() {
            return Err("Input cannot be empty".to_string());
        }
        
        let trimmed = input.trim();
        if trimmed.len() < 3 {
            return Err("Input too short".to_string());
        }
        
        let uppercase = trimmed.to_uppercase();
        Ok(format!("Processed: {}", uppercase))
    }
    
    let inputs = vec!["", "hi", "hello world", "  rust  "];
    
    for input in inputs {
        match process_data(input) {
            Ok(result) => println!("Success: {}", result),
            Err(error) => println!("Error processing '{}': {}", input, error),
        }
    }
    
    // Pattern 2: Default values for errors
    fn get_config_value(key: &str) -> String {
        // Simulate reading from config file
        match key {
            "database_url" => "localhost:5432".to_string(),
            "port" => "8080".to_string(),
            _ => "default_value".to_string(), // Default for unknown keys
        }
    }
    
    let config_keys = vec!["database_url", "port", "unknown_key"];
    for key in config_keys {
        println!("Config {}: {}", key, get_config_value(key));
    }
    
    // Pattern 3: Collecting errors
    fn validate_inputs(inputs: &[&str]) -> Result<Vec<i32>, Vec<String>> {
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        for input in inputs {
            match input.parse::<i32>() {
                Ok(num) => results.push(num),
                Err(_) => errors.push(format!("'{}' is not a valid number", input)),
            }
        }
        
        if errors.is_empty() {
            Ok(results)
        } else {
            Err(errors)
        }
    }
    
    let test_inputs = vec!["1", "2", "invalid", "4", "also_invalid", "6"];
    match validate_inputs(&test_inputs) {
        Ok(numbers) => println!("All valid numbers: {:?}", numbers),
        Err(errors) => {
            println!("Validation errors:");
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
    
    // Pattern 4: Chain of operations with early exit
    fn calculate_average(numbers: &[&str]) -> Result<f64, String> {
        if numbers.is_empty() {
            return Err("Cannot calculate average of empty list".to_string());
        }
        
        let mut sum = 0.0;
        for num_str in numbers {
            let num: f64 = num_str.parse()
                .map_err(|_| format!("'{}' is not a valid number", num_str))?;
            sum += num;
        }
        
        Ok(sum / numbers.len() as f64)
    }
    
    let number_sets = vec![
        vec!["1", "2", "3", "4", "5"],
        vec!["10", "20", "invalid"],
        vec![],
        vec!["1.5", "2.5", "3.5"],
    ];
    
    for numbers in number_sets {
        match calculate_average(&numbers) {
            Ok(avg) => println!("Average of {:?}: {:.2}", numbers, avg),
            Err(error) => println!("Error calculating average of {:?}: {}", numbers, error),
        }
    }
    
    // Pattern 5: Graceful degradation
    fn get_user_preference(user_id: u32, key: &str) -> String {
        // Simulate database lookup that might fail
        if user_id == 999 {
            // User not found, return default
            return get_default_preference(key);
        }
        
        match key {
            "theme" => "dark".to_string(),
            "language" => "en".to_string(),
            _ => get_default_preference(key),
        }
    }
    
    fn get_default_preference(key: &str) -> String {
        match key {
            "theme" => "light".to_string(),
            "language" => "en".to_string(),
            "notifications" => "enabled".to_string(),
            _ => "unknown".to_string(),
        }
    }
    
    let test_cases = vec![
        (1, "theme"),
        (1, "language"),
        (999, "theme"), // User not found
        (1, "unknown_setting"),
    ];
    
    for (user_id, key) in test_cases {
        let preference = get_user_preference(user_id, key);
        println!("User {} preference for '{}': {}", user_id, key, preference);
    }
    
    println!("\nError handling summary:");
    println!("- Use panic! for unrecoverable errors");
    println!("- Use Result<T, E> for recoverable errors");
    println!("- Use Option<T> for values that might not exist");
    println!("- Use ? operator for error propagation");
    println!("- Provide meaningful error messages");
    println!("- Consider graceful degradation patterns");
}
