// Generics in Rust
// =================
// Generics allow us to write code that works with many different types while maintaining type safety.

use std::cmp::PartialOrd;

fn main() {
    // Generic functions
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    
    // Generic structs
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = PointMixed { x: 5, y: 4.0 };
    
    println!("integer point: ({}, {})", integer.x, integer.y);
    println!("float point: ({}, {})", float.x, float.y);
    println!("mixed point: ({}, {})", mixed.x, mixed.y);
    
    // Generic methods
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    
    // Generic enums
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    match some_number {
        Some(value) => println!("Got a value: {}", value),
        None => println!("Got nothing"),
    }
    
    // Result enum
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    // Advanced generics
    advanced_generics_example();
    
    // Where clauses
    where_clause_example();
}

// Generic function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Alternative implementation using references to avoid Copy requirement
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Generic struct with one type parameter
struct Point<T> {
    x: T,
    y: T,
}

// Generic struct with multiple type parameters
struct PointMixed<T, U> {
    x: T,
    y: U,
}

// Implementing methods on generic structs
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implementing methods for specific types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic methods
impl<T, U> PointMixed<T, U> {
    fn mixup<V, W>(self, other: PointMixed<V, W>) -> PointMixed<T, W> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

// Generic enums (standard library examples)
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Custom generic enum
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    
    fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
}

// Advanced generics with trait bounds
fn advanced_generics_example() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = add_all(&numbers);
    println!("Sum: {}", sum);
    
    let words = vec!["hello", "world"];
    let concatenated = concatenate_all(&words);
    println!("Concatenated: {}", concatenated);
}

// Generic function with trait bounds
fn add_all<T>(items: &[T]) -> T 
where 
    T: std::ops::Add<Output = T> + Copy + Default
{
    let mut sum = T::default();
    for &item in items {
        sum = sum + item;
    }
    sum
}

fn concatenate_all<T>(items: &[T]) -> String 
where 
    T: std::fmt::Display
{
    let mut result = String::new();
    for item in items {
        result.push_str(&format!("{} ", item));
    }
    result.trim().to_string()
}

// Complex where clauses
fn where_clause_example() {
    let container = Container { value: 42 };
    complex_function(&container, &container);
}

struct Container<T> {
    value: T,
}

fn complex_function<T, U>(item1: &T, item2: &U) 
where 
    T: std::fmt::Display + std::fmt::Debug + Clone,
    U: std::fmt::Display + std::fmt::Debug + Clone,
{
    println!("Item 1: {:?}", item1);
    println!("Item 2: {:?}", item2);
}

// Generic associated types
trait Collect<T> {
    type Output;
    fn collect(self) -> Self::Output;
}

struct VecCollector<T> {
    items: Vec<T>,
}

impl<T> Collect<T> for VecCollector<T> {
    type Output = Vec<T>;
    
    fn collect(self) -> Self::Output {
        self.items
    }
}

// Higher-ranked trait bounds (HRTB)
fn closure_example() {
    let closure = |x: &i32| x + 1;
    
    // This function accepts any closure that can work with references of any lifetime
    fn apply_to_all<F>(f: F) -> i32 
    where 
        F: for<'a> Fn(&'a i32) -> i32
    {
        let value = 42;
        f(&value)
    }
    
    let result = apply_to_all(closure);
    println!("Closure result: {}", result);
}

// Generic with lifetime parameters
struct Ref<'a, T> {
    value: &'a T,
}

impl<'a, T> Ref<'a, T> 
where 
    T: std::fmt::Display
{
    fn print_value(&self) {
        println!("Value: {}", self.value);
    }
}

// Zero-cost abstractions example
fn zero_cost_example() {
    // This generic code compiles to the same assembly as hand-written specific code
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    
    // Monomorphization: the compiler generates specific versions for each type
    // Point<i32> and Point<f64> become separate types at compile time
}

// Phantom types (types that don't store data but carry type information)
use std::marker::PhantomData;

struct PhantomPoint<T> {
    x: f64,
    y: f64,
    _marker: PhantomData<T>,
}

struct Meters;
struct Feet;

impl PhantomPoint<Meters> {
    fn new_meters(x: f64, y: f64) -> Self {
        PhantomPoint {
            x,
            y,
            _marker: PhantomData,
        }
    }
}

impl PhantomPoint<Feet> {
    fn new_feet(x: f64, y: f64) -> Self {
        PhantomPoint {
            x,
            y,
            _marker: PhantomData,
        }
    }
}
