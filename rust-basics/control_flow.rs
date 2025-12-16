// Control Flow in Rust
// ====================
// if expressions, loops, and match statements for controlling program flow.

fn main() {
    println!("=== Control Flow in Rust ===");
    
    // If expressions
    if_expressions();
    
    // Loops
    loop_examples();
    
    // Match expressions
    match_expressions();
    
    // Advanced control flow
    advanced_control_flow();
}

fn if_expressions() {
    println!("\n--- If Expressions ---");
    
    // Basic if
    let number = 6;
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }
    
    // if is an expression - can return values
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number from if expression: {}", number);
    
    // Both arms must return same type
    let result = if number > 0 {
        "positive"
    } else {
        "zero or negative"
    };
    println!("Number is: {}", result);
    
    // Nested if expressions
    let x = 10;
    let y = 20;
    let comparison = if x > y {
        "x is greater"
    } else if x < y {
        "y is greater"
    } else {
        "they are equal"
    };
    println!("Comparison: {}", comparison);
    
    // if let pattern matching
    let some_option = Some(5);
    if let Some(value) = some_option {
        println!("Got a value: {}", value);
    } else {
        println!("Got None");
    }
    
    // Complex conditions
    let age = 18;
    let has_license = true;
    let has_car = false;
    
    if age >= 18 && has_license {
        if has_car {
            println!("Can drive own car");
        } else {
            println!("Can drive but needs to borrow a car");
        }
    } else {
        println!("Cannot drive");
    }
}

fn loop_examples() {
    println!("\n--- Loops ---");
    
    // 1. loop - infinite loop
    println!("Infinite loop (with break):");
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            println!("Breaking at counter = {}", counter);
            break;
        }
        println!("Counter: {}", counter);
    }
    
    // loop with return value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("Result from loop: {}", result);
    
    // 2. while loop
    println!("\nWhile loop:");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // while let pattern matching
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    // 3. for loop
    println!("\nFor loops:");
    
    // Iterate over array
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("Element: {}", element);
    }
    
    // Iterate over range
    for number in 1..4 {
        println!("Number: {}", number);
    }
    
    // Inclusive range
    for number in 1..=3 {
        println!("Inclusive: {}", number);
    }
    
    // Reverse range
    for number in (1..4).rev() {
        println!("Reverse: {}", number);
    }
    
    // Enumerate (get index and value)
    let names = ["Alice", "Bob", "Charlie"];
    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name);
    }
    
    // Iterate over collection references
    let vec = vec![1, 2, 3, 4, 5];
    for item in &vec {
        println!("Reference: {}", item);
    }
    
    // Iterate and take ownership
    let vec2 = vec![1, 2, 3];
    for item in vec2 {
        println!("Owned: {}", item);
    }
    // vec2 is no longer valid here
    
    // Nested loops with labels
    println!("\nNested loops with labels:");
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                println!("Breaking outer loop at i={}, j={}", i, j);
                break 'outer;
            }
            println!("i={}, j={}", i, j);
        }
    }
    
    // continue in loops
    println!("\nUsing continue:");
    for i in 1..=5 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        println!("Odd number: {}", i);
    }
}

fn match_expressions() {
    println!("\n--- Match Expressions ---");
    
    // Basic match
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"), // Default case
    }
    
    // Match with return values
    let number = 2;
    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("Description: {}", description);
    
    // Match with multiple patterns
    let number = 4;
    match number {
        1 | 2 => println!("One or two"),
        3..=5 => println!("Three through five"),
        6 | 7 | 8 => println!("Six, seven, or eight"),
        _ => println!("Something else"),
    }
    
    // Match with guards (additional conditions)
    let number = 4;
    let is_even = true;
    match number {
        x if x < 5 && is_even => println!("{} is less than 5 and even", x),
        x if x < 5 => println!("{} is less than 5 but odd", x),
        _ => println!("Number is 5 or greater"),
    }
    
    // Match with Option
    let some_number = Some(5);
    match some_number {
        Some(n) if n > 0 => println!("Positive number: {}", n),
        Some(n) => println!("Non-positive number: {}", n),
        None => println!("No number"),
    }
    
    // Match with Result
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    // Destructuring with match
    let point = (3, 5);
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On Y-axis at {}", y),
        (x, 0) => println!("On X-axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
    
    // Match with structs
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("On Y-axis at y = {}", y),
        Point { x, y: 0 } => println!("On X-axis at x = {}", x),
        Point { x, y } => println!("Point at ({}, {})", x, y),
    }
    
    // Match with enums
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    let direction = Direction::North;
    match direction {
        Direction::North => println!("Going north"),
        Direction::South => println!("Going south"),
        Direction::East => println!("Going east"),
        Direction::West => println!("Going west"),
    }
    
    // Match with enum variants that hold data
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let message = Message::Write(String::from("hello"));
    match message {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}

fn advanced_control_flow() {
    println!("\n--- Advanced Control Flow ---");
    
    // Combining if let with other patterns
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    
    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    // for loop with pattern matching
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    
    // let statements with patterns
    let (x, y, z) = (1, 2, 3);
    println!("Destructured: x={}, y={}, z={}", x, y, z);
    
    // Function parameters with patterns
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    
    let point = (3, 5);
    print_coordinates(&point);
    
    // Match with @ bindings
    let msg = Some(5);
    match msg {
        Some(n @ 1..=12) => println!("Found a number between 1 and 12: {}", n),
        Some(n @ 13..=19) => println!("Found a teen number: {}", n),
        Some(n) => println!("Found some other number: {}", n),
        None => println!("Found no number"),
    }
    
    // Ignoring values with ..
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("First: {}, Last: {}", first, last);
        }
    }
    
    // Breaking from nested loops with labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {}", count);
}
