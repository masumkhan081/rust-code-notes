// Data Types in Rust
// ==================
// Rust is statically typed - must know types at compile time.

fn main() {
    println!("=== Rust Data Types ===");
    
    // Scalar Types
    scalar_types();
    
    // Compound Types
    compound_types();
    
    // Type conversion
    type_conversion();
    
    // Type inference
    type_inference();
}

fn scalar_types() {
    println!("\n--- Scalar Types ---");
    
    // Integer types
    let decimal = 98_222;           // i32 (default)
    let hex = 0xff;                 // hexadecimal
    let octal = 0o77;               // octal
    let binary = 0b1111_0000;       // binary
    let byte = b'A';                // u8 only
    
    println!("Decimal: {}", decimal);
    println!("Hex: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", binary);
    println!("Byte: {}", byte);
    
    // Explicit integer types
    let small: i8 = -128;           // 8-bit signed (-128 to 127)
    let big: i64 = 9_223_372_036_854_775_807; // 64-bit signed
    let unsigned: u32 = 4_294_967_295; // 32-bit unsigned
    let size_type: usize = 100;     // pointer-sized
    
    println!("i8: {}, i64: {}, u32: {}, usize: {}", small, big, unsigned, size_type);
    
    // Floating-point types
    let float32: f32 = 3.14159;     // 32-bit float
    let float64 = 2.718281828;      // f64 (default)
    
    println!("f32: {}, f64: {}", float32, float64);
    
    // Boolean type
    let is_true = true;
    let is_false: bool = false;
    
    println!("Boolean: {}, {}", is_true, is_false);
    
    // Character type (Unicode scalar values)
    let letter = 'A';
    let emoji = '😻';
    let chinese = '中';
    
    println!("Characters: '{}', '{}', '{}'", letter, emoji, chinese);
    
    // Character vs string
    let char_a = 'A';       // char - single Unicode scalar
    let string_a = "A";     // &str - string slice
    println!("Char: {}, String: {}", char_a, string_a);
}

fn compound_types() {
    println!("\n--- Compound Types ---");
    
    // Tuples - group different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("Tuple elements: {}, {}, {}", x, y, z);
    
    // Access by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("By index: {}, {}, {}", five_hundred, six_point_four, one);
    
    // Unit tuple (empty tuple)
    let unit = ();
    println!("Unit tuple: {:?}", unit);
    
    // Arrays - same type, fixed size
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type and size
    let arr3 = [3; 5]; // [3, 3, 3, 3, 3]
    
    println!("Arrays: {:?}, {:?}, {:?}", arr1, arr2, arr3);
    
    // Array access
    let first = arr1[0];
    let second = arr1[1];
    println!("Array elements: {}, {}", first, second);
    
    // Array length
    println!("Array length: {}", arr1.len());
    
    // Slices - view into arrays
    let slice = &arr1[1..4]; // elements 1, 2, 3
    println!("Slice: {:?}", slice);
    
    // String types
    let string_literal = "Hello, world!";     // &str (string slice)
    let owned_string = String::from("Hello"); // String (owned)
    
    println!("String literal: {}", string_literal);
    println!("Owned string: {}", owned_string);
    
    // Vectors - dynamic arrays
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(6);
    println!("Vector: {:?}", vec);
}

fn type_conversion() {
    println!("\n--- Type Conversion ---");
    
    // Explicit casting with 'as'
    let integer = 42u8;
    let float = integer as f64;
    let back_to_int = float as i32;
    
    println!("u8: {} -> f64: {} -> i32: {}", integer, float, back_to_int);
    
    // Parsing strings to numbers
    let string_number = "42";
    let parsed: i32 = string_number.parse().expect("Not a number!");
    println!("Parsed string '{}' to number: {}", string_number, parsed);
    
    // Safe parsing with Result
    let maybe_number = "42".parse::<i32>();
    match maybe_number {
        Ok(n) => println!("Successfully parsed: {}", n),
        Err(e) => println!("Failed to parse: {}", e),
    }
    
    // Converting numbers to strings
    let num = 42;
    let num_string = num.to_string();
    let formatted = format!("Number: {}", num);
    
    println!("Number as string: '{}', formatted: '{}'", num_string, formatted);
    
    // Byte conversions
    let text = "Hello";
    let bytes = text.as_bytes();
    println!("Text '{}' as bytes: {:?}", text, bytes);
    
    // Character to number and back
    let ch = 'A';
    let ascii_value = ch as u8;
    let back_to_char = ascii_value as char;
    
    println!("'{}' -> {} -> '{}'", ch, ascii_value, back_to_char);
}

fn type_inference() {
    println!("\n--- Type Inference ---");
    
    // Rust can infer types
    let inferred_int = 42;          // i32
    let inferred_float = 3.14;      // f64
    let inferred_bool = true;       // bool
    let inferred_char = 'A';        // char
    
    println!("Inferred types: {}, {}, {}, {}", 
             inferred_int, inferred_float, inferred_bool, inferred_char);
    
    // Context helps with inference
    let mut vec = Vec::new();       // Can't infer element type yet
    vec.push(1);                    // Now Rust knows it's Vec<i32>
    println!("Vector: {:?}", vec);
    
    // Sometimes you need to specify
    let parsed: u32 = "42".parse().expect("Not a number");
    // or
    let parsed2 = "42".parse::<u32>().expect("Not a number");
    
    println!("Parsed with annotation: {}, with turbofish: {}", parsed, parsed2);
    
    // Function return type inference
    fn get_number() -> i32 { 42 }
    let number = get_number(); // i32 inferred from function signature
    println!("Function return: {}", number);
}

// Demonstrate numeric operations
fn numeric_operations() {
    println!("\n--- Numeric Operations ---");
    
    // Arithmetic
    let a = 10;
    let b = 3;
    
    println!("a = {}, b = {}", a, b);
    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);
    println!("Multiplication: {}", a * b);
    println!("Division: {}", a / b);        // Integer division
    println!("Remainder: {}", a % b);
    
    // Floating point division
    let x = 10.0;
    let y = 3.0;
    println!("Float division: {}", x / y);
    
    // Overflow behavior (debug vs release)
    let max_u8 = 255u8;
    // let overflow = max_u8 + 1; // Would panic in debug mode
    
    // Safe arithmetic
    let result = max_u8.checked_add(1);
    match result {
        Some(n) => println!("Result: {}", n),
        None => println!("Overflow occurred!"),
    }
    
    // Wrapping arithmetic
    let wrapped = max_u8.wrapping_add(1);
    println!("Wrapped: {}", wrapped); // 0
    
    // Saturating arithmetic
    let saturated = max_u8.saturating_add(1);
    println!("Saturated: {}", saturated); // 255
}
