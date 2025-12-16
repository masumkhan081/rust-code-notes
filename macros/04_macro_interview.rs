/*
    Topic: Macros - Interview Questions
    
    1. Declarative vs Procedural Macros (Differences).
    2. Write a macro that vectors like `vec!`.
*/

// Interview Q2: Simple Vec Macro
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn main() {
    println!("--- 04 Macro Interview ---");
    let v = my_vec![1, 2, 3];
    println!("Created vector via macro: {:?}", v);
}
