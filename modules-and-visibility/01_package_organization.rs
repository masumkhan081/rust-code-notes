// Package and Crate Organization Examples
// ========================================
// This file demonstrates different ways to organize Rust code in packages and crates.

// Package structure:
// A package contains one or more crates
// A package has a Cargo.toml file
// A package can have at most one library crate
// A package can have multiple binary crates

/*
Example package structure:

restaurant/
├── Cargo.toml
├── src/
│   ├── main.rs          (binary crate root)
│   ├── lib.rs           (library crate root)
│   ├── bin/
│   │   ├── server.rs    (additional binary)
│   │   └── client.rs    (additional binary)
│   └── modules/
│       ├── mod.rs
│       ├── kitchen.rs
│       └── dining.rs
├── tests/
│   ├── integration_test.rs
│   └── common/
│       └── mod.rs
├── examples/
│   └── example_usage.rs
└── benches/
    └── benchmark.rs
*/

// This would be lib.rs in a library crate
pub mod restaurant {
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() { println!("Adding customer to waitlist"); }
            pub fn seat_at_table()   { println!("Seating customer at table"); }
        }
        pub mod serving {
            pub fn take_order()  { println!("Taking order"); }
            pub fn serve_order() { println!("Serving order"); }
        }
        pub mod menu {
            pub struct MenuItem { pub name: String, pub price: f64 }
            pub enum MenuCategory { Appetizer, MainCourse, Dessert, Beverage }
        }
    }
    pub mod back_of_house {
        pub mod kitchen {
            pub fn prepare_order() { println!("Preparing order"); }
        }
    }
    pub use front_of_house::hosting;
    pub use back_of_house::kitchen;
}

// Public API of the library
pub use restaurant::front_of_house::menu::{MenuItem, MenuCategory};
pub use restaurant::hosting;

// Crate-level documentation
// # Restaurant Library
// 
// This crate provides functionality for managing a restaurant,
// including order management, menu handling, and customer service.
// 
// ## Examples
// 
// ```
// use restaurant::hosting;
// use restaurant::{MenuItem, MenuCategory};
// 
// hosting::add_to_waitlist();
// 
// let item = MenuItem::new(
//     "Pasta".to_string(),
//     12.99,
//     MenuCategory::MainCourse
// );
// ```

// Example of how main.rs would use the library
fn main() {
    // Using the library crate
    // This would work if we had a proper lib.rs
    println!("Restaurant management system starting...");
    
    demonstrate_crate_usage();
    demonstrate_workspace_concepts();
}

fn demonstrate_crate_usage() {
    println!("=== Crate Usage Examples ===");
    
    // Binary crate can use library crate
    // use restaurant_lib::hosting;
    // hosting::add_to_waitlist();
    
    // External crates (would be in Cargo.toml)
    // use serde::{Serialize, Deserialize};
    // use tokio::main;
    // use clap::Parser;
}

fn demonstrate_workspace_concepts() {
    println!("=== Workspace Concepts ===");
    
    /*
    Workspace Cargo.toml example:
    
    [workspace]
    members = [
        "restaurant-lib",
        "restaurant-server", 
        "restaurant-client",
        "restaurant-common"
    ]
    
    [workspace.dependencies]
    serde = "1.0"
    tokio = "1.0"
    
    Package Cargo.toml example:
    
    [package]
    name = "restaurant-lib"
    version = "0.1.0"
    edition = "2021"
    
    [dependencies]
    serde = { workspace = true }
    tokio = { workspace = true }
    
    [lib]
    name = "restaurant"
    path = "src/lib.rs"
    
    [[bin]]
    name = "restaurant-server"
    path = "src/bin/server.rs"
    
    [[bin]]  
    name = "restaurant-client"
    path = "src/bin/client.rs"
    */
}

// Integration test example (would be in tests/integration_test.rs)
#[cfg(test)]
mod integration_tests {
    // use restaurant_lib::hosting;
    
    #[test]
    fn test_full_workflow() {
        // Test the public API of the library
        // hosting::add_to_waitlist();
        // assert!(true);
    }
}

// Example of conditional compilation for different targets
#[cfg(target_os = "windows")]
fn platform_specific_code() {
    println!("Running on Windows");
}

#[cfg(target_os = "linux")]
fn platform_specific_code() {
    println!("Running on Linux");
}

#[cfg(target_os = "macos")]
fn platform_specific_code() {
    println!("Running on macOS");
}

// Feature flags example
#[cfg(feature = "logging")]
mod logging {
    pub fn setup_logger() {
        println!("Logger setup (feature enabled)");
    }
}

#[cfg(not(feature = "logging"))]
mod logging {
    pub fn setup_logger() {
        println!("No logging (feature disabled)");
    }
}

// Development dependencies and testing
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_specific() {
        platform_specific_code();
    }
    
    #[test]
    fn test_logging_feature() {
        logging::setup_logger();
    }
}

// Benchmark example (would be in benches/benchmark.rs)
/*
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use restaurant_lib::menu::calculate_total;

fn benchmark_calculate_total(c: &mut Criterion) {
    let items = vec![
        MenuItem::new("Item 1".to_string(), 10.0, MenuCategory::MainCourse),
        MenuItem::new("Item 2".to_string(), 5.0, MenuCategory::Appetizer),
    ];
    
    c.bench_function("calculate_total", |b| {
        b.iter(|| calculate_total(black_box(&items)))
    });
}

criterion_group!(benches, benchmark_calculate_total);
criterion_main!(benches);
*/

// Example usage (would be in examples/example_usage.rs)
/*
use restaurant_lib::{hosting, MenuItem, MenuCategory};

fn main() {
    println!("Restaurant Example");
    
    hosting::add_to_waitlist();
    
    let mut item = MenuItem::new(
        "Grilled Salmon".to_string(),
        18.99,
        MenuCategory::MainCourse
    );
    
    println!("Original price: ${:.2}", item.price);
    item.apply_discount(10.0);
    println!("Discounted price: ${:.2}", item.price);
}
*/

// Documentation examples
/// This function demonstrates how to document code
/// 
/// # Arguments
/// 
/// * `name` - The name of the customer
/// * `party_size` - Number of people in the party
/// 
/// # Examples
/// 
/// ```
/// let result = add_customer("Alice", 4);
/// assert_eq!(result, "Added Alice (party of 4) to waitlist");
/// ```
/// 
/// # Panics
/// 
/// This function will panic if `party_size` is 0.
/// 
/// # Safety
/// 
/// This function is safe to call with any valid string and positive integer.
pub fn add_customer(name: &str, party_size: usize) -> String {
    if party_size == 0 {
        panic!("Party size cannot be zero");
    }
    
    format!("Added {} (party of {}) to waitlist", name, party_size)
}
