# Rust Code Notes

This repository contains comprehensive notes and examples for essential Rust programming concepts.

## Topics Covered

### 1. Ownership, Borrowing, and Lifetimes
**Location**: `ownership-borrowing-lifetimes/`

- **ownership.rs**: Core ownership rules, move semantics, Copy vs Clone
- **borrowing.rs**: References, mutable borrowing, borrowing rules, string slices
- **lifetimes.rs**: Lifetime annotations, lifetime elision, generic lifetimes

### 2. Traits and Generics
**Location**: `traits-and-generics/`

- **traits.rs**: Defining traits, trait implementations, trait objects, associated types
- **generics.rs**: Generic functions/structs/enums, type parameters, where clauses

### 3. Macros
**Location**: `macros/`

- **declarative_macros.rs**: macro_rules!, pattern matching, variadic macros, DSLs
- **procedural_macros.rs**: Derive macros, attribute macros, function-like macros

### 4. Smart Pointers
**Location**: `smart-pointers/`

- **box_pointer.rs**: Heap allocation, recursive types, trait objects with Box
- **rc_pointer.rs**: Reference counting, shared ownership, weak references
- **refcell_interior_mutability.rs**: Interior mutability, runtime borrow checking

### 5. Modules and Visibility
**Location**: `modules-and-visibility/`

- **main.rs**: Module system, visibility rules, use statements, paths
- **front_of_house.rs**: Example of separate module file
- **package_organization.rs**: Crates, packages, workspaces, documentation

### 6. Iterators and Closures
**Location**: `iterators-and-closures/`

- **iterators.rs**: Iterator trait, lazy evaluation, map/filter/collect, custom iterators
- **closures.rs**: Closure syntax, capturing environment, Fn/FnMut/FnOnce traits, move closures

### 7. Unsafe Rust
**Location**: `unsafe-rust/`

- **unsafe_basics.rs**: Unsafe blocks, raw pointers, unsafe functions, memory management
- **ffi_examples.rs**: Foreign Function Interface, calling C from Rust, callbacks

## Existing Topics (Already in your workspace)

- **async-programming/**: Async/await, futures, tokio
- **collection/**: Vec, HashMap, HashSet, iterators
- **concurrency/**: Threads, channels, Arc, Mutex
- **error-handling/**: Result, Option, error propagation
- **file-handling/**: File I/O, serialization
- **function-module/**: Functions, closures, modules
- **struct-enum/**: Structs, enums, pattern matching
- **types-and-control-flow/**: Basic types, control structures

## How to Use

Each folder contains:
1. **Complete code examples** with detailed comments
2. **Practical patterns** and use cases
3. **Best practices** and performance notes
4. **Common pitfalls** and how to avoid them

## Running the Examples

```bash
# Compile and run any example
rustc path/to/file.rs
./executable_name

# Or add to main.rs and run with cargo
cargo run
```

## Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
