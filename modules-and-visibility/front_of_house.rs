// front_of_house.rs - Separate module file
// =========================================
// This file is loaded by the declaration `mod front_of_house;` in main.rs

pub mod hosting {
    pub fn add_to_waitlist() {
        println!("Adding customer to waitlist");
    }

    pub fn seat_at_table() {
        println!("Seating customer at table");
    }
    
    // Private function
    fn clean_table() {
        println!("Cleaning table");
    }
    
    // Function that uses private function
    pub fn prepare_table() {
        clean_table();
        println!("Table is ready");
    }
}

pub mod serving {
    pub fn take_order() {
        println!("Taking order");
    }

    pub fn serve_order() {
        println!("Serving order");
    }

    pub fn take_payment() {
        println!("Taking payment");
    }
    
    // Using function from sibling module
    pub fn full_service() {
        super::hosting::add_to_waitlist();
        take_order();
        serve_order();
        take_payment();
    }
}

// Module with structs and enums
pub mod menu {
    pub struct MenuItem {
        pub name: String,
        pub price: f64,
        category: MenuCategory, // Private field
    }
    
    #[derive(Debug)]
    pub enum MenuCategory {
        Appetizer,
        MainCourse,
        Dessert,
        Beverage,
    }
    
    impl MenuItem {
        pub fn new(name: String, price: f64, category: MenuCategory) -> MenuItem {
            MenuItem {
                name,
                price,
                category,
            }
        }
        
        pub fn get_category(&self) -> &MenuCategory {
            &self.category
        }
        
        pub fn apply_discount(&mut self, discount_percent: f64) {
            self.price *= (100.0 - discount_percent) / 100.0;
        }
    }
    
    // Constants
    pub const TAX_RATE: f64 = 0.08;
    
    // Module-level function
    pub fn calculate_total(items: &[MenuItem]) -> f64 {
        let subtotal: f64 = items.iter().map(|item| item.price).sum();
        subtotal * (1.0 + TAX_RATE)
    }
}

// Re-export commonly used items
pub use menu::{MenuItem, MenuCategory};
pub use hosting::add_to_waitlist;
pub use serving::take_order;
