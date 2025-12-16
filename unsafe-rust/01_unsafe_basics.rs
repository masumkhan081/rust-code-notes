// Unsafe Rust
// ============
// Unsafe Rust allows you to bypass Rust's safety guarantees when necessary.
// Use unsafe blocks only when you know what you're doing and can guarantee safety.

fn main() {
    // Basic unsafe operations
    unsafe_basics();
    
    // Raw pointers
    raw_pointers_demo();
    
    // Unsafe functions and methods
    unsafe_functions_demo();
    
    // Mutable static variables
    mutable_statics_demo();
    
    // Unsafe traits
    unsafe_traits_demo();
    
    // Foreign Function Interface (FFI)
    // ffi_demo(); // Commented out as it requires external C library
    
    // Memory management with unsafe
    memory_management_demo();
    
    // Safe abstractions over unsafe code
    safe_abstractions_demo();
    
    // Common unsafe patterns
    common_unsafe_patterns();
}

fn unsafe_basics() {
    println!("=== Unsafe Basics ===");
    
    // Raw pointers
    let mut num = 5;
    
    // Creating raw pointers is safe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // But dereferencing them requires unsafe
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        
        // Modifying through mutable raw pointer
        *r2 = 10;
        println!("num is now: {}", num);
    }
    
    // Creating arbitrary raw pointers (dangerous!)
    let address = 0x012345usize;
    let _r = address as *const i32;
    // Don't dereference this! It would likely crash or access invalid memory
}

fn raw_pointers_demo() {
    println!("\n=== Raw Pointers ===");
    
    let mut v = vec![1, 2, 3, 4, 5];
    let ptr = v.as_mut_ptr();
    
    unsafe {
        // Access elements through raw pointer
        for i in 0..v.len() {
            let element = ptr.add(i);
            println!("Element {}: {}", i, *element);
            
            // Modify through raw pointer
            *element *= 2;
        }
    }
    
    println!("Modified vector: {:?}", v);
    
    // Creating a slice from raw parts
    let slice = unsafe {
        std::slice::from_raw_parts(ptr, v.len())
    };
    println!("Slice from raw parts: {:?}", slice);
    
    // Raw pointer arithmetic
    unsafe {
        let first = ptr;
        let second = ptr.add(1);
        let third = ptr.add(2);
        
        println!("First: {}", *first);
        println!("Second: {}", *second);
        println!("Third: {}", *third);
        
        // Pointer difference
        let diff = second.offset_from(first);
        println!("Offset difference: {}", diff);
    }
}

// Unsafe function
unsafe fn dangerous() {
    println!("This is an unsafe function");
}

// Safe wrapper around unsafe operation
fn safe_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_functions_demo() {
    println!("\n=== Unsafe Functions ===");
    
    // Calling unsafe function requires unsafe block
    unsafe {
        dangerous();
    }
    
    // Using our safe wrapper
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = safe_split_at_mut(&mut v, 3);
    println!("Left: {:?}, Right: {:?}", left, right);
    
    // Modifying both parts simultaneously (normally not allowed)
    left[0] = 10;
    right[0] = 20;
    println!("Modified vector: {:?}", v);
}

// Global mutable state (requires unsafe to access)
static mut COUNTER: usize = 0;

fn mutable_statics_demo() {
    println!("\n=== Mutable Statics ===");
    
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
        
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
    
    // Safe alternative: use atomic types
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    static ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(0);
    
    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Atomic counter: {}", ATOMIC_COUNTER.load(Ordering::SeqCst));
}

// Unsafe traits
unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}

// Implementing unsafe trait requires unsafe impl
unsafe impl UnsafeTrait for i32 {
    fn unsafe_method(&self) {
        println!("Unsafe method called on: {}", self);
    }
}

fn unsafe_traits_demo() {
    println!("\n=== Unsafe Traits ===");
    
    let num = 42i32;
    num.unsafe_method();
    
    // Example: Send and Sync are unsafe traits
    // They're automatically implemented by the compiler for safe types
    // But you can manually implement them for types containing raw pointers
}

// FFI (Foreign Function Interface) example
#[link(name = "c")]
extern "C" {
    fn strlen(s: *const i8) -> usize;
}

fn ffi_demo() {
    println!("\n=== FFI Demo ===");
    
    let c_string = std::ffi::CString::new("Hello from C!").unwrap();
    
    unsafe {
        let len = strlen(c_string.as_ptr());
        println!("String length from C: {}", len);
    }
}

fn memory_management_demo() {
    println!("\n=== Memory Management ===");
    
    // Manual memory allocation
    use std::alloc::{alloc, dealloc, Layout};
    
    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        
        // Initialize the memory
        *ptr = 42;
        println!("Allocated value: {}", *ptr);
        
        // Must manually deallocate
        dealloc(ptr as *mut u8, layout);
    }
    
    // Using Box for comparison (safe heap allocation)
    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed);
    // Box automatically deallocates when it goes out of scope
    
    // Working with uninitialized memory
    use std::mem::MaybeUninit;
    
    let mut uninit: MaybeUninit<i32> = MaybeUninit::uninit();
    
    unsafe {
        // Initialize the memory
        uninit.as_mut_ptr().write(100);
        
        // Now it's safe to assume it's initialized
        let initialized = uninit.assume_init();
        println!("Initialized value: {}", initialized);
    }
}

// Safe wrapper around unsafe operations
struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec {
            ptr: std::ptr::NonNull::dangling().as_ptr(),
            len: 0,
            capacity: 0,
        }
    }
    
    fn push(&mut self, item: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            std::ptr::write(self.ptr.add(self.len), item);
        }
        
        self.len += 1;
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }
    
    fn grow(&mut self) {
        use std::alloc::{alloc, realloc, Layout};
        
        let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        
        unsafe {
            let new_ptr = if self.capacity == 0 {
                alloc(new_layout)
            } else {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                realloc(self.ptr as *mut u8, old_layout, new_layout.size())
            };
            
            if new_ptr.is_null() {
                panic!("Allocation failed");
            }
            
            self.ptr = new_ptr as *mut T;
            self.capacity = new_capacity;
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        use std::alloc::{dealloc, Layout};
        
        if self.capacity != 0 {
            unsafe {
                for i in 0..self.len {
                    std::ptr::drop_in_place(self.ptr.add(i));
                }
                
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

fn safe_abstractions_demo() {
    println!("\n=== Safe Abstractions ===");
    
    let mut vec = MyVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    println!("MyVec[0]: {:?}", vec.get(0));
    println!("MyVec[1]: {:?}", vec.get(1));
    println!("MyVec[2]: {:?}", vec.get(2));
    println!("MyVec[10]: {:?}", vec.get(10)); // None
}

fn common_unsafe_patterns() {
    println!("\n=== Common Unsafe Patterns ===");
    
    // Pattern 1: Transmuting between types
    let num: u32 = 42;
    let bytes: [u8; 4] = unsafe {
        std::mem::transmute(num)
    };
    println!("u32 {} as bytes: {:?}", num, bytes);
    
    // Pattern 2: Uninitialized memory for performance
    let mut buffer: [u8; 1024] = unsafe {
        std::mem::MaybeUninit::uninit().assume_init()
    };
    
    // Initialize part of the buffer
    for i in 0..10 {
        buffer[i] = i as u8;
    }
    println!("First 10 bytes: {:?}", &buffer[0..10]);
    
    // Pattern 3: Aliasing with different lifetimes
    fn extend_lifetime<'a, 'b, T>(r: &'a T) -> &'b T {
        unsafe { std::mem::transmute(r) }
    }
    
    // DON'T DO THIS! It's just an example of what's possible
    // let extended = extend_lifetime(&42);
    
    // Pattern 4: Bypassing borrow checker for self-referential structs
    struct SelfReferential {
        data: String,
        pointer: *const u8,
    }
    
    impl SelfReferential {
        fn new(data: String) -> Self {
            let mut s = SelfReferential {
                data,
                pointer: std::ptr::null(),
            };
            
            s.pointer = s.data.as_ptr();
            s
        }
        
        fn get_pointer_value(&self) -> Option<u8> {
            unsafe {
                if !self.pointer.is_null() {
                    Some(*self.pointer)
                } else {
                    None
                }
            }
        }
    }
    
    let self_ref = SelfReferential::new("Hello".to_string());
    println!("First byte: {:?}", self_ref.get_pointer_value());
}

// Guidelines for using unsafe code
fn unsafe_guidelines() {
    println!("\n=== Unsafe Guidelines ===");
    println!("1. Minimize unsafe code - use it only when necessary");
    println!("2. Encapsulate unsafe code in safe APIs");
    println!("3. Document safety invariants clearly");
    println!("4. Test unsafe code thoroughly");
    println!("5. Consider alternatives like atomic types, Arc/Mutex");
    println!("6. Use tools like Miri for checking unsafe code");
    println!("7. Never expose raw pointers in public APIs if avoidable");
    println!("8. Always check for null pointers before dereferencing");
    println!("9. Ensure memory is properly aligned");
    println!("10. Don't violate aliasing rules (no mutable aliasing)");
}

// Memory safety violations to avoid
fn what_not_to_do() {
    println!("\n=== What NOT to Do ===");
    
    // DON'T: Use after free
    /*
    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        dealloc(ptr as *mut u8, layout);
        // *ptr = 42; // Use after free!
    }
    */
    
    // DON'T: Double free
    /*
    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut u8;
        dealloc(ptr, layout);
        // dealloc(ptr, layout); // Double free!
    }
    */
    
    // DON'T: Access out of bounds
    /*
    unsafe {
        let arr = [1, 2, 3];
        let ptr = arr.as_ptr();
        // let value = *ptr.add(10); // Out of bounds!
    }
    */
    
    // DON'T: Create invalid references
    /*
    unsafe {
        let ptr = 0x123 as *const i32;
        // let reference = &*ptr; // Invalid reference!
    }
    */
    
    println!("Remember: unsafe code should never violate memory safety!");
}

// Tools and techniques for working with unsafe code
fn unsafe_tools_and_techniques() {
    println!("\n=== Tools and Techniques ===");
    println!("1. Miri - interpreter for detecting undefined behavior");
    println!("2. AddressSanitizer - runtime memory error detector");
    println!("3. Valgrind - memory debugging tool");
    println!("4. Static analysis tools like Clippy");
    println!("5. Fuzzing with tools like cargo-fuzz");
    println!("6. Property-based testing with QuickCheck");
    println!("7. Formal verification tools");
    println!("8. Code review with focus on safety invariants");
}
