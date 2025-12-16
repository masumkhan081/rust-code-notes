// Modules and Visibility in Rust
// ===============================
// Modules help organize code and control privacy.
// The module system includes packages, crates, modules, and paths.

// This is the main module (crate root)

// Declaring modules
mod front_of_house; // Looks for front_of_house.rs or front_of_house/mod.rs

// Inline module
mod back_of_house {
    // Private by default
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Call function in parent module
    }

    fn cook_order() {}

    // Public struct
    pub struct Breakfast {
        pub toast: String,     // Public field
        seasonal_fruit: String, // Private field
    }

    impl Breakfast {
        // Public associated function
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Public enum - all variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Function in crate root
fn deliver_order() {}

// Using modules
pub fn eat_at_restaurant() {
    // Absolute path
    crate::back_of_house::Breakfast::summer("Rye");

    // Relative path
    back_of_house::Breakfast::summer("Wheat");

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // Can modify public field
    // meal.seasonal_fruit = String::from("blueberries"); // Error! Private field

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Re-exporting with pub use
pub use crate::back_of_house::Appetizer;

fn main() {
    eat_at_restaurant();
    
    // Now we can use Appetizer directly
    let appetizer = Appetizer::Soup;
    
    // Module organization examples
    module_organization_examples();
    
    // Visibility examples
    visibility_examples();
    
    // Path examples
    path_examples();
    
    // Use statement examples
    use_statement_examples();
}

// Nested modules
mod sound {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                println!("Playing clarinet");
                super::super::breathe(); // Call parent's parent function
            }
        }
        
        pub mod brass {
            pub fn trumpet() {
                println!("Playing trumpet");
            }
        }
    }
    
    fn breathe() {
        println!("Taking a breath");
    }
}

// Module with different visibility levels
mod library {
    // Public module
    pub mod books {
        pub struct Book {
            pub title: String,
            isbn: String, // Private
        }
        
        impl Book {
            pub fn new(title: String, isbn: String) -> Book {
                Book { title, isbn }
            }
            
            pub fn get_isbn(&self) -> &str {
                &self.isbn
            }
        }
    }
    
    // Private module (default)
    mod internal {
        pub fn secret_function() {
            println!("This is secret");
        }
    }
    
    // Function that can access private module
    pub fn access_internal() {
        internal::secret_function();
    }
}

fn module_organization_examples() {
    // Using nested modules
    sound::instrument::woodwind::clarinet();
    sound::instrument::brass::trumpet();
    
    // Using library module
    let book = library::books::Book::new(
        "The Rust Programming Language".to_string(),
        "978-1718500440".to_string(),
    );
    println!("Book: {}, ISBN: {}", book.title, book.get_isbn());
    
    // Access private module through public function
    library::access_internal();
}

// Visibility examples
mod visibility_examples_module {
    // Private (default)
    fn private_function() {}
    
    // Public
    pub fn public_function() {
        private_function(); // Can call private from same module
    }
    
    // Public within crate
    pub(crate) fn crate_visible_function() {}
    
    // Public within parent module
    pub(super) fn parent_visible_function() {}
    
    // Public within this module only
    pub(self) fn self_visible_function() {}
    
    pub mod nested {
        pub fn call_parent_functions() {
            super::private_function(); // Can access parent's private
            super::parent_visible_function(); // Can access pub(super)
        }
    }
}

fn visibility_examples() {
    visibility_examples_module::public_function();
    visibility_examples_module::crate_visible_function();
    // visibility_examples_module::private_function(); // Error!
    // visibility_examples_module::parent_visible_function(); // Error!
    
    visibility_examples_module::nested::call_parent_functions();
}

// Path examples
mod path_examples_module {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    
    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
        
        fn serve_order_internal() {
            // Relative path
            take_order();
            
            // Absolute path
            crate::path_examples_module::hosting::add_to_waitlist();
            
            // Using super
            super::hosting::seat_at_table();
        }
    }
}

fn path_examples() {
    // Absolute paths start with crate
    crate::path_examples_module::hosting::add_to_waitlist();
    
    // Relative paths start from current module
    path_examples_module::serving::take_order();
}

// Use statement examples
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt;
use std::io;

// Nested paths
use std::{cmp::Ordering, io::Write};

// Glob operator
use std::collections::*;

// Renaming with as
use std::fmt::Result;
use std::io::Result as IoResult;

// Re-exporting
pub use std::collections::HashSet;

fn use_statement_examples() {
    // Using HashMap without full path
    let mut map = HashMap::new();
    map.insert("key", "value");
    
    // Using BTreeMap
    let mut btree = BTreeMap::new();
    btree.insert(1, "one");
    
    // Using renamed types
    fn returns_fmt_result() -> Result {
        Ok(())
    }
    
    fn returns_io_result() -> IoResult<()> {
        Ok(())
    }
}

// External crate usage (if these were in Cargo.toml)
// use rand::Rng;
// use serde::{Serialize, Deserialize};

// Conditional compilation
#[cfg(target_os = "windows")]
mod windows_specific {
    pub fn do_windows_thing() {
        println!("Windows-specific code");
    }
}

#[cfg(target_os = "linux")]
mod linux_specific {
    pub fn do_linux_thing() {
        println!("Linux-specific code");
    }
}

// Feature flags
#[cfg(feature = "advanced")]
mod advanced_features {
    pub fn advanced_function() {
        println!("Advanced feature enabled");
    }
}

// Testing module
#[cfg(test)]
mod tests {
    use super::*; // Import everything from parent module
    
    #[test]
    fn test_eat_at_restaurant() {
        eat_at_restaurant(); // Test passes if no panic
    }
    
    #[test]
    fn test_library_book() {
        let book = library::books::Book::new(
            "Test Book".to_string(),
            "123-456".to_string(),
        );
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.get_isbn(), "123-456");
    }
}

// Binary crate structure example (if this were a binary crate)
/*
src/
├── main.rs (crate root)
├── lib.rs (library crate root, if it exists)
├── front_of_house.rs
├── front_of_house/
│   ├── mod.rs
│   └── hosting.rs
└── back_of_house/
    ├── mod.rs
    ├── kitchen.rs
    └── serving.rs
*/

// Library crate structure example
/*
src/
├── lib.rs (crate root)
├── utils.rs
├── utils/
│   ├── mod.rs
│   ├── string_utils.rs
│   └── math_utils.rs
├── models/
│   ├── mod.rs
│   ├── user.rs
│   └── post.rs
└── services/
    ├── mod.rs
    ├── user_service.rs
    └── post_service.rs
*/

// Workspace structure example
/*
Cargo.toml (workspace manifest)
├── frontend/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── backend/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── shared/
    ├── Cargo.toml
    └── src/
        └── lib.rs
*/

// Best practices
fn module_best_practices() {
    println!("=== Module Best Practices ===");
    println!("1. Start with everything private, make public as needed");
    println!("2. Use pub(crate) for internal APIs");
    println!("3. Group related functionality in modules");
    println!("4. Use re-exports (pub use) to create clean APIs");
    println!("5. Prefer nested paths in use statements");
    println!("6. Use meaningful module names");
    println!("7. Keep modules focused and cohesive");
}
