// Foreign Function Interface (FFI) in Rust
// =========================================
// FFI allows Rust to call functions written in other languages (mainly C)
// and to be called from other languages.

fn main() {
    // Basic C function calls
    basic_c_calls();
    
    // Working with C strings
    c_string_handling();
    
    // Passing structs to C
    struct_passing();
    
    // Callbacks from C to Rust
    callback_demo();
    
    // Error handling in FFI
    ffi_error_handling();
    
    // Creating a Rust library for C
    // (This would be in a separate crate with crate-type = ["cdylib"])
    rust_for_c_demo();
}

// External C functions (from libc)
extern "C" {
    fn strlen(s: *const i8) -> usize;
    fn strcmp(s1: *const i8, s2: *const i8) -> i32;
    fn malloc(size: usize) -> *mut std::ffi::c_void;
    fn free(ptr: *mut std::ffi::c_void);
    fn printf(format: *const i8, ...) -> i32;
}

fn basic_c_calls() {
    println!("=== Basic C Function Calls ===");
    
    // Using C string functions
    let hello = std::ffi::CString::new("Hello, World!").unwrap();
    
    unsafe {
        let len = strlen(hello.as_ptr());
        println!("String length (from C): {}", len);
        
        // Compare strings
        let world = std::ffi::CString::new("Hello, World!").unwrap();
        let comparison = strcmp(hello.as_ptr(), world.as_ptr());
        println!("String comparison result: {}", comparison);
    }
}

fn c_string_handling() {
    println!("\n=== C String Handling ===");
    
    // Rust String to C string
    let rust_string = "Hello from Rust!";
    let c_string = std::ffi::CString::new(rust_string).unwrap();
    
    unsafe {
        // Pass to C function that expects null-terminated string
        let len = strlen(c_string.as_ptr());
        println!("C string length: {}", len);
        
        // Print using C printf
        let format = std::ffi::CString::new("C printf: %s\n").unwrap();
        printf(format.as_ptr(), c_string.as_ptr());
    }
    
    // Creating C string from raw pointer (dangerous!)
    unsafe {
        let raw_str = "Raw string\0".as_ptr() as *const i8;
        let len = strlen(raw_str);
        println!("Raw string length: {}", len);
        
        // Convert back to Rust string
        let c_str = std::ffi::CStr::from_ptr(raw_str);
        let rust_str = c_str.to_str().unwrap();
        println!("Back to Rust: {}", rust_str);
    }
    
    // Handling C strings with potential invalid UTF-8
    let invalid_utf8 = b"Hello\xFF\xFEWorld\0";
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(invalid_utf8.as_ptr() as *const i8);
        match c_str.to_str() {
            Ok(s) => println!("Valid UTF-8: {}", s),
            Err(e) => {
                println!("Invalid UTF-8: {:?}", e);
                // Use to_string_lossy for fallback
                let lossy = c_str.to_string_lossy();
                println!("Lossy conversion: {}", lossy);
            }
        }
    }
}

// C-compatible struct representation
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

#[repr(C)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// Mock C functions (in real code, these would be in a C library)
extern "C" {
    // These would be actual C functions
    // For demo purposes, we'll implement them as Rust functions
    // fn calculate_distance(p1: *const Point, p2: *const Point) -> f64;
    // fn print_rectangle(rect: *const Rectangle);
}

// Implementation of "C" functions for demo
#[no_mangle]
extern "C" fn calculate_distance(p1: *const Point, p2: *const Point) -> f64 {
    unsafe {
        let p1 = &*p1;
        let p2 = &*p2;
        ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
    }
}

#[no_mangle]
extern "C" fn print_rectangle(rect: *const Rectangle) {
    unsafe {
        let rect = &*rect;
        println!("Rectangle: ({}, {}) to ({}, {})", 
                 rect.top_left.x, rect.top_left.y,
                 rect.bottom_right.x, rect.bottom_right.y);
    }
}

fn struct_passing() {
    println!("\n=== Passing Structs to C ===");
    
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 4.0, y: 6.0 };
    
    unsafe {
        let distance = calculate_distance(&p1, &p2);
        println!("Distance between points: {}", distance);
    }
    
    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 10.0, y: 5.0 },
    };
    
    unsafe {
        print_rectangle(&rect);
    }
}

// Function pointer types for callbacks
type CallbackFn = extern "C" fn(i32) -> i32;

// Mock C function that takes a callback
#[no_mangle]
extern "C" fn process_array(arr: *const i32, len: usize, callback: CallbackFn) {
    unsafe {
        for i in 0..len {
            let value = *arr.add(i);
            let result = callback(value);
            println!("Processed {} -> {}", value, result);
        }
    }
}

// Rust callback function
extern "C" fn double_value(x: i32) -> i32 {
    x * 2
}

extern "C" fn square_value(x: i32) -> i32 {
    x * x
}

fn callback_demo() {
    println!("\n=== Callbacks from C to Rust ===");
    
    let numbers = [1, 2, 3, 4, 5];
    
    unsafe {
        println!("Doubling values:");
        process_array(numbers.as_ptr(), numbers.len(), double_value);
        
        println!("Squaring values:");
        process_array(numbers.as_ptr(), numbers.len(), square_value);
    }
    
    // Closure to function pointer (limited cases)
    let multiplier = 3;
    let triple: extern "C" fn(i32) -> i32 = {
        extern "C" fn triple_impl(x: i32) -> i32 { x * 3 }
        triple_impl
    };
    
    unsafe {
        println!("Tripling values:");
        process_array(numbers.as_ptr(), numbers.len(), triple);
    }
}

// Error handling in FFI
#[repr(C)]
enum CResult {
    Success = 0,
    ErrorInvalidInput = 1,
    ErrorOutOfMemory = 2,
    ErrorUnknown = 3,
}

#[no_mangle]
extern "C" fn safe_divide(a: f64, b: f64, result: *mut f64) -> CResult {
    if result.is_null() {
        return CResult::ErrorInvalidInput;
    }
    
    if b == 0.0 {
        return CResult::ErrorInvalidInput;
    }
    
    unsafe {
        *result = a / b;
    }
    
    CResult::Success
}

fn ffi_error_handling() {
    println!("\n=== Error Handling in FFI ===");
    
    let mut result: f64 = 0.0;
    
    unsafe {
        // Successful division
        match safe_divide(10.0, 2.0, &mut result) {
            CResult::Success => println!("10.0 / 2.0 = {}", result),
            error => println!("Error: {:?}", error as i32),
        }
        
        // Division by zero
        match safe_divide(10.0, 0.0, &mut result) {
            CResult::Success => println!("Result: {}", result),
            CResult::ErrorInvalidInput => println!("Error: Invalid input (division by zero)"),
            error => println!("Other error: {:?}", error as i32),
        }
        
        // Null pointer
        match safe_divide(10.0, 2.0, std::ptr::null_mut()) {
            CResult::Success => println!("This shouldn't print"),
            CResult::ErrorInvalidInput => println!("Error: Null pointer passed"),
            error => println!("Other error: {:?}", error as i32),
        }
    }
}

// Creating Rust functions that can be called from C
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_string_length(s: *const i8) -> usize {
    if s.is_null() {
        return 0;
    }
    
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(s);
        c_str.to_bytes().len()
    }
}

#[no_mangle]
pub extern "C" fn rust_allocate_array(size: usize) -> *mut i32 {
    if size == 0 {
        return std::ptr::null_mut();
    }
    
    let layout = std::alloc::Layout::array::<i32>(size).unwrap();
    unsafe {
        let ptr = std::alloc::alloc(layout) as *mut i32;
        if !ptr.is_null() {
            // Initialize array
            for i in 0..size {
                *ptr.add(i) = i as i32;
            }
        }
        ptr
    }
}

#[no_mangle]
pub extern "C" fn rust_free_array(ptr: *mut i32, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }
    
    unsafe {
        let layout = std::alloc::Layout::array::<i32>(size).unwrap();
        std::alloc::dealloc(ptr as *mut u8, layout);
    }
}

fn rust_for_c_demo() {
    println!("\n=== Rust Functions for C ===");
    
    // Test our exported functions
    let sum = rust_add(5, 3);
    println!("rust_add(5, 3) = {}", sum);
    
    let test_str = std::ffi::CString::new("Hello, C!").unwrap();
    let len = rust_string_length(test_str.as_ptr());
    println!("String length: {}", len);
    
    unsafe {
        // Test array allocation
        let size = 5;
        let array = rust_allocate_array(size);
        
        if !array.is_null() {
            println!("Allocated array:");
            for i in 0..size {
                print!("{} ", *array.add(i));
            }
            println!();
            
            rust_free_array(array, size);
            println!("Array freed");
        }
    }
}

// Advanced FFI patterns
mod advanced_ffi {
    use super::*;
    
    // Opaque types (useful for hiding implementation details)
    #[repr(C)]
    pub struct OpaqueHandle {
        _private: [u8; 0],
    }
    
    // Functions working with opaque handles
    #[no_mangle]
    pub extern "C" fn create_handle() -> *mut OpaqueHandle {
        let data = Box::new(String::from("Secret data"));
        Box::into_raw(data) as *mut OpaqueHandle
    }
    
    #[no_mangle]
    pub extern "C" fn use_handle(handle: *mut OpaqueHandle) -> i32 {
        if handle.is_null() {
            return -1;
        }
        
        unsafe {
            let data = &*(handle as *mut String);
            data.len() as i32
        }
    }
    
    #[no_mangle]
    pub extern "C" fn destroy_handle(handle: *mut OpaqueHandle) {
        if handle.is_null() {
            return;
        }
        
        unsafe {
            let _data = Box::from_raw(handle as *mut String);
            // Box is automatically dropped
        }
    }
    
    // Variable argument functions (limited support)
    extern "C" {
        fn printf(format: *const i8, ...) -> i32;
    }
    
    pub fn variable_args_demo() {
        unsafe {
            let format = std::ffi::CString::new("Number: %d, String: %s\n").unwrap();
            let text = std::ffi::CString::new("Hello").unwrap();
            printf(format.as_ptr(), 42, text.as_ptr());
        }
    }
}

// Bindgen example (in real code, you'd use the bindgen crate)
/*
// build.rs example for generating bindings
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=mylib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
*/

// Safety guidelines for FFI
fn ffi_safety_guidelines() {
    println!("\n=== FFI Safety Guidelines ===");
    println!("1. Always validate pointers before dereferencing");
    println!("2. Use #[repr(C)] for structs passed to C");
    println!("3. Handle string encoding carefully (UTF-8 vs null-terminated)");
    println!("4. Match C calling conventions exactly");
    println!("5. Be careful with memory management across FFI boundary");
    println!("6. Document ownership transfer clearly");
    println!("7. Use bindgen for complex C APIs");
    println!("8. Test thoroughly, especially error conditions");
    println!("9. Consider using safer alternatives when possible");
    println!("10. Wrap unsafe FFI calls in safe Rust APIs");
}
