// Closures in Rust
// =================
// Closures are anonymous functions that can capture variables from their environment.
// They implement the Fn, FnMut, or FnOnce traits depending on how they capture variables.

fn main() {
    // Basic closure syntax
    let add_one = |x| x + 1;
    let result = add_one(5);
    println!("5 + 1 = {}", result);
    
    // Closure with type annotations
    let add: fn(i32, i32) -> i32 = |x, y| x + y;
    println!("3 + 4 = {}", add(3, 4));
    
    // Closures capturing environment
    environment_capture();
    
    // Closure traits: Fn, FnMut, FnOnce
    closure_traits();
    
    // Move closures
    move_closures();
    
    // Closures with iterators
    closures_with_iterators();
    
    // Higher-order functions
    higher_order_functions();
    
    // Function pointers vs closures
    function_pointers_vs_closures();
    
    // Advanced closure patterns
    advanced_closure_patterns();
}

fn environment_capture() {
    println!("\n=== Environment Capture ===");
    
    let x = 10;
    let y = 20;
    
    // Closure captures variables from environment
    let capture_by_ref = || {
        println!("Captured x: {}, y: {}", x, y);
    };
    
    capture_by_ref();
    
    // Still can use x and y
    println!("x and y still available: {}, {}", x, y);
    
    // Capture mutable reference
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    
    increment();
    increment();
    increment();
    
    // count is borrowed mutably by closure
    println!("Final count: {}", count);
}

fn closure_traits() {
    println!("\n=== Closure Traits ===");
    
    // Fn - can be called multiple times, captures by immutable reference
    let x = 10;
    let fn_closure = || x + 1;
    
    call_fn_closure(&fn_closure);
    call_fn_closure(&fn_closure); // Can call multiple times
    
    // FnMut - can be called multiple times, captures by mutable reference
    let mut count = 0;
    let mut fn_mut_closure = || {
        count += 1;
        count
    };
    
    call_fn_mut_closure(&mut fn_mut_closure);
    call_fn_mut_closure(&mut fn_mut_closure);
    
    // FnOnce - can only be called once, takes ownership
    let data = vec![1, 2, 3];
    let fn_once_closure = || {
        println!("Data: {:?}", data);
        data // Takes ownership, can only be called once
    };
    
    call_fn_once_closure(fn_once_closure);
    // call_fn_once_closure(fn_once_closure); // Error! Already consumed
}

fn call_fn_closure<F>(f: &F) where F: Fn() -> i32 {
    let result = f();
    println!("Fn result: {}", result);
}

fn call_fn_mut_closure<F>(f: &mut F) where F: FnMut() -> i32 {
    let result = f();
    println!("FnMut result: {}", result);
}

fn call_fn_once_closure<F>(f: F) where F: FnOnce() -> Vec<i32> {
    let result = f();
    println!("FnOnce result: {:?}", result);
}

fn move_closures() {
    println!("\n=== Move Closures ===");
    
    let data = vec![1, 2, 3, 4, 5];
    
    // Normal closure borrows
    let closure_borrow = || {
        println!("Borrowed data: {:?}", data);
    };
    
    closure_borrow();
    println!("Data still available: {:?}", data); // Still can use data
    
    // Move closure takes ownership
    let data2 = vec![6, 7, 8, 9, 10];
    let closure_move = move || {
        println!("Moved data: {:?}", data2);
        data2.len() // Takes ownership
    };
    
    let len = closure_move();
    println!("Length: {}", len);
    // println!("Data2: {:?}", data2); // Error! data2 was moved
    
    // Move closures with threads
    let shared_data = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        println!("Thread data: {:?}", shared_data);
        shared_data.len()
    });
    
    let result = handle.join().unwrap();
    println!("Thread result: {}", result);
}

fn closures_with_iterators() {
    println!("\n=== Closures with Iterators ===");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Map with closure
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Filter with closure
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
    
    // Complex closure capturing environment
    let threshold = 5;
    let filtered: Vec<&i32> = numbers
        .iter()
        .filter(|&x| x > &threshold)
        .collect();
    println!("Above {}: {:?}", threshold, filtered);
    
    // Closure that modifies captured variable
    let mut sum = 0;
    numbers.iter().for_each(|x| {
        sum += x;
    });
    println!("Sum: {}", sum);
    
    // Find with closure
    let found = numbers.iter().find(|&x| x > &7);
    println!("First > 7: {:?}", found);
    
    // Fold with closure
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);
}

fn higher_order_functions() {
    println!("\n=== Higher-Order Functions ===");
    
    // Function that takes a closure
    fn apply_operation<F>(x: i32, y: i32, op: F) -> i32 
    where 
        F: Fn(i32, i32) -> i32 
    {
        op(x, y)
    }
    
    let add = |a, b| a + b;
    let multiply = |a, b| a * b;
    let max = |a, b| if a > b { a } else { b };
    
    println!("Add: {}", apply_operation(5, 3, add));
    println!("Multiply: {}", apply_operation(5, 3, multiply));
    println!("Max: {}", apply_operation(5, 3, max));
    
    // Function that returns a closure
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }
    
    let add_5 = make_adder(5);
    let add_10 = make_adder(10);
    
    println!("7 + 5 = {}", add_5(7));
    println!("7 + 10 = {}", add_10(7));
    
    // Function that takes different closure types
    fn process_data<F>(data: Vec<i32>, processor: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        data.into_iter().map(processor).collect()
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = process_data(numbers.clone(), |x| x * 2);
    let squared = process_data(numbers, |x| x * x);
    
    println!("Doubled: {:?}", doubled);
    println!("Squared: {:?}", squared);
}

fn function_pointers_vs_closures() {
    println!("\n=== Function Pointers vs Closures ===");
    
    // Regular function
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    // Function pointer
    let fn_ptr: fn(i32) -> i32 = add_one;
    
    // Closure
    let closure = |x| x + 1;
    
    // Function that accepts both
    fn apply_fn<F>(x: i32, f: F) -> i32 
    where 
        F: Fn(i32) -> i32 
    {
        f(x)
    }
    
    println!("Function pointer: {}", apply_fn(5, fn_ptr));
    println!("Closure: {}", apply_fn(5, closure));
    
    // Function pointer can be stored in collections
    let operations: Vec<fn(i32) -> i32> = vec![
        |x| x + 1,
        |x| x * 2,
        |x| x * x,
    ];
    
    for (i, op) in operations.iter().enumerate() {
        println!("Operation {}: 5 -> {}", i, op(5));
    }
}

fn advanced_closure_patterns() {
    println!("\n=== Advanced Closure Patterns ===");
    
    // Closure composition
    let add_one = |x| x + 1;
    let double = |x| x * 2;
    
    fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C 
    where 
        F: Fn(A) -> B,
        G: Fn(B) -> C,
    {
        move |x| g(f(x))
    }
    
    let add_then_double = compose(add_one, double);
    println!("(5 + 1) * 2 = {}", add_then_double(5));
    
    // Partial application
    fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }
    
    let add_10 = curry_add(10);
    println!("Curried add: 5 + 10 = {}", add_10(5));
    
    // Memoization pattern
    use std::collections::HashMap;
    
    struct Memoized<F> {
        func: F,
        cache: HashMap<i32, i32>,
    }
    
    impl<F> Memoized<F> 
    where 
        F: Fn(i32) -> i32,
    {
        fn new(func: F) -> Self {
            Memoized {
                func,
                cache: HashMap::new(),
            }
        }
        
        fn call(&mut self, arg: i32) -> i32 {
            if let Some(&result) = self.cache.get(&arg) {
                println!("Cache hit for {}", arg);
                result
            } else {
                println!("Computing for {}", arg);
                let result = (self.func)(arg);
                self.cache.insert(arg, result);
                result
            }
        }
    }
    
    let fibonacci = |n| {
        match n {
            0 => 0,
            1 => 1,
            _ => n, // Simplified for demo
        }
    };
    
    let mut memoized_fib = Memoized::new(fibonacci);
    println!("Fib(5): {}", memoized_fib.call(5));
    println!("Fib(5): {}", memoized_fib.call(5)); // Cache hit
    
    // Lazy evaluation with closures
    struct Lazy<F, T> {
        func: Option<F>,
        value: Option<T>,
    }
    
    impl<F, T> Lazy<F, T> 
    where 
        F: FnOnce() -> T,
    {
        fn new(func: F) -> Self {
            Lazy {
                func: Some(func),
                value: None,
            }
        }
        
        fn get(&mut self) -> &T {
            if self.value.is_none() {
                let func = self.func.take().unwrap();
                self.value = Some(func());
            }
            self.value.as_ref().unwrap()
        }
    }
    
    let mut lazy_computation = Lazy::new(|| {
        println!("Expensive computation happening...");
        42
    });
    
    println!("Before accessing lazy value");
    println!("Lazy value: {}", lazy_computation.get());
    println!("Lazy value again: {}", lazy_computation.get()); // No recomputation
}

// Error handling with closures
fn error_handling_closures() {
    println!("\n=== Error Handling with Closures ===");
    
    let numbers = vec!["1", "2", "not_a_number", "4"];
    
    // Using closure with Result
    let parsed: Vec<Result<i32, _>> = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    println!("All results: {:?}", parsed);
    
    // Filter successful parses
    let successful: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    println!("Successful parses: {:?}", successful);
    
    // Custom error handling
    let process_result = |input: &str| -> Result<i32, String> {
        input.parse::<i32>()
            .map(|n| n * 2)
            .map_err(|e| format!("Parse error: {}", e))
    };
    
    for number_str in &numbers {
        match process_result(number_str) {
            Ok(result) => println!("{} -> {}", number_str, result),
            Err(e) => println!("{} -> Error: {}", number_str, e),
        }
    }
}

// Performance considerations
fn performance_considerations() {
    println!("\n=== Performance Considerations ===");
    
    use std::time::Instant;
    
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // Closure with no captures (should be optimized to function pointer)
    let no_capture = |x: &i32| x * 2;
    
    let start = Instant::now();
    let _: Vec<i32> = data.iter().map(no_capture).collect();
    let no_capture_time = start.elapsed();
    
    // Closure with captures
    let multiplier = 2;
    let with_capture = |x: &i32| x * multiplier;
    
    let start = Instant::now();
    let _: Vec<i32> = data.iter().map(with_capture).collect();
    let with_capture_time = start.elapsed();
    
    println!("No capture time: {:?}", no_capture_time);
    println!("With capture time: {:?}", with_capture_time);
    println!("Both should be similar due to optimizations!");
}
