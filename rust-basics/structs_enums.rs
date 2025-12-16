// Basic Structs and Enums
// ========================
// Structs group related data, enums define types with multiple variants.

fn main() {
    println!("=== Structs and Enums ===");
    
    // Struct examples
    struct_examples();
    
    // Enum examples
    enum_examples();
    
    // Methods and associated functions
    methods_demo();
    
    // Pattern matching with structs and enums
    pattern_matching_demo();
}

fn struct_examples() {
    println!("\n--- Structs ---");
    
    // Define a struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    // Create an instance
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // Access struct fields
    println!("User: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign-in count: {}", user1.sign_in_count);
    
    // Mutable struct
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };
    
    // Modify field (entire struct must be mutable)
    user2.email = String::from("newemail@example.com");
    println!("Updated email: {}", user2.email);
    
    // Function that creates a user
    fn build_user(email: String, username: String) -> User {
        User {
            email,           // Shorthand when variable and field have same name
            username,        // Shorthand
            active: true,
            sign_in_count: 1,
        }
    }
    
    let user3 = build_user(
        String::from("test@example.com"),
        String::from("testuser"),
    );
    println!("Built user: {}", user3.username);
    
    // Struct update syntax
    let user4 = User {
        email: String::from("different@example.com"),
        username: String::from("differentuser"),
        ..user3  // Use remaining fields from user3
    };
    println!("User4 active: {}", user4.active);
    
    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // Unit struct (no fields)
    struct UnitStruct;
    let _unit = UnitStruct;
    
    // Structs with different field types
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Rectangle: {}x{}", rect.width, rect.height);
    println!("Area: {}", rect.width * rect.height);
    
    // Debug trait for structs
    #[derive(Debug)]
    struct DebugRect {
        width: u32,
        height: u32,
    }
    
    let debug_rect = DebugRect {
        width: 30,
        height: 50,
    };
    
    println!("Debug rect: {:?}", debug_rect);
    println!("Pretty debug: {:#?}", debug_rect);
}

fn enum_examples() {
    println!("\n--- Enums ---");
    
    // Basic enum
    enum IpAddrKind {
        V4,
        V6,
    }
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // Function that takes any IP address kind
    fn route(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => println!("IPv4 routing"),
            IpAddrKind::V6 => println!("IPv6 routing"),
        }
    }
    
    route(four);
    route(six);
    
    // Enum with data
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    match home {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
    
    match loopback {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
    
    // Enum with various data types
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },   // Named fields
        Write(String),              // Single value
        ChangeColor(i32, i32, i32), // Multiple values
    }
    
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 15 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for message in messages {
        match message {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
    
    // Option enum (built-in)
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("Got None"),
    }
    
    // Working with Option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("plus_one(Some(5)): {:?}", six);
    println!("plus_one(None): {:?}", none);
    
    // Result enum (built-in)
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }
    
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);
    
    match result1 {
        Ok(value) => println!("10.0 / 2.0 = {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    match result2 {
        Ok(value) => println!("10.0 / 0.0 = {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

fn methods_demo() {
    println!("\n--- Methods and Associated Functions ---");
    
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // Associated function (like static method)
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        
        // Another associated function
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
        
        // Method (takes &self)
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // Method that takes another Rectangle
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        
        // Mutable method
        fn double_size(&mut self) {
            self.width *= 2;
            self.height *= 2;
        }
        
        // Method that takes ownership
        fn into_square(self) -> Rectangle {
            let size = std::cmp::max(self.width, self.height);
            Rectangle::square(size)
        }
    }
    
    // Using associated functions
    let rect1 = Rectangle::new(30, 50);
    let square1 = Rectangle::square(25);
    
    println!("Rectangle: {:?}", rect1);
    println!("Square: {:?}", square1);
    
    // Using methods
    println!("Area of rect1: {}", rect1.area());
    println!("Area of square1: {}", square1.area());
    
    let rect2 = Rectangle::new(10, 40);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    
    // Mutable method
    let mut rect3 = Rectangle::new(5, 10);
    println!("Before doubling: {:?}", rect3);
    rect3.double_size();
    println!("After doubling: {:?}", rect3);
    
    // Method that takes ownership
    let rect4 = Rectangle::new(10, 20);
    let square2 = rect4.into_square();
    println!("Converted to square: {:?}", square2);
    // rect4 is no longer valid here
    
    // Multiple impl blocks (allowed)
    impl Rectangle {
        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }
        
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }
    
    println!("Square perimeter: {}", square1.perimeter());
    println!("Is square1 a square? {}", square1.is_square());
    println!("Is rect1 a square? {}", rect1.is_square());
}

fn pattern_matching_demo() {
    println!("\n--- Pattern Matching ---");
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { base: f64, height: f64 },
    }
    
    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle { width, height } => width * height,
                Shape::Triangle { base, height } => 0.5 * base * height,
            }
        }
        
        fn describe(&self) -> String {
            match self {
                Shape::Circle { radius } => format!("Circle with radius {}", radius),
                Shape::Rectangle { width, height } => {
                    format!("Rectangle {}x{}", width, height)
                }
                Shape::Triangle { base, height } => {
                    format!("Triangle with base {} and height {}", base, height)
                }
            }
        }
    }
    
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 10.0, height: 20.0 },
        Shape::Triangle { base: 8.0, height: 12.0 },
    ];
    
    for shape in &shapes {
        println!("{}: area = {:.2}", shape.describe(), shape.area());
    }
    
    // Pattern matching with structs
    let point = Point { x: 0, y: 7 };
    
    match point {
        Point { x: 0, y } => println!("On the Y-axis at y = {}", y),
        Point { x, y: 0 } => println!("On the X-axis at x = {}", x),
        Point { x, y } => println!("Point at ({}, {})", x, y),
    }
    
    // if let for simple pattern matching
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("Got the value 3!");
    }
    
    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    // Destructuring in let statements
    let tuple = (1, 2, 3);
    let (a, b, c) = tuple;
    println!("Destructured tuple: a={}, b={}, c={}", a, b, c);
    
    let Point { x, y } = Point { x: 5, y: 10 };
    println!("Destructured point: x={}, y={}", x, y);
}
