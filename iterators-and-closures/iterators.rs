// Iterators in Rust
// ==================
// Iterators are lazy and provide zero-cost abstractions for processing collections.
// They implement the Iterator trait and use functional programming patterns.

fn main() {
    // Basic iterator creation
    let vec = vec![1, 2, 3, 4, 5];
    
    // Three ways to create iterators
    let iter1 = vec.iter();        // &T - immutable references
    let iter2 = vec.into_iter();   // T - takes ownership
    // let iter3 = vec.iter_mut();  // &mut T - mutable references (can't use after into_iter)
    
    // Iterator methods are lazy - nothing happens until consumed
    let doubled: Vec<i32> = vec![1, 2, 3]
        .iter()
        .map(|x| x * 2)     // Lazy - returns iterator
        .collect();         // Consumer - executes the chain
    
    println!("Doubled: {:?}", doubled);
    
    // Common iterator methods
    iterator_methods_demo();
    
    // Iterator adaptors vs consumers
    adaptors_vs_consumers();
    
    // For loops are syntactic sugar for iterators
    for_loop_vs_iterators();
    
    // Performance comparison
    performance_comparison();
    
    // Custom iterators
    custom_iterator_demo();
    
    // Advanced iterator patterns
    advanced_patterns();
}

fn iterator_methods_demo() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Map - transform each element
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Filter - keep elements that match predicate
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
    
    // Chain operations
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)    // Keep evens
        .map(|x| x * x)             // Square them
        .collect();
    println!("Even squares: {:?}", result);
    
    // Enumerate - get index and value
    for (index, value) in numbers.iter().enumerate() {
        if index < 3 {
            println!("Index {}: {}", index, value);
        }
    }
    
    // Take and skip
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_first_three: Vec<&i32> = numbers.iter().skip(3).collect();
    println!("First 3: {:?}", first_three);
    println!("Skip 3: {:?}", skip_first_three);
    
    // Find - returns Option<T>
    let found = numbers.iter().find(|&x| x > &5);
    println!("First > 5: {:?}", found);
    
    // Any and all
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);
    
    // Fold and reduce
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Sum: {}, Product: {}", sum, product);
    
    // Reduce (similar to fold but uses first element as initial value)
    let sum2 = numbers.iter().reduce(|acc, x| acc + x);
    println!("Sum with reduce: {:?}", sum2);
    
    // Collect into different types
    let as_vec: Vec<_> = numbers.iter().collect();
    let as_string = numbers.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
    println!("As string: {}", as_string);
}

fn adaptors_vs_consumers() {
    println!("\n=== Iterator Adaptors vs Consumers ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Adaptors (lazy) - these don't do anything until consumed
    let _lazy_iter = numbers
        .iter()
        .map(|x| {
            println!("Processing {}", x); // This won't print!
            x * 2
        })
        .filter(|&x| x > &4);
    
    println!("Created lazy iterator - nothing processed yet");
    
    // Consumers - these execute the chain
    let result: Vec<_> = numbers
        .iter()
        .map(|x| {
            println!("Actually processing {}", x); // This will print!
            x * 2
        })
        .filter(|&x| x > &4)
        .collect(); // Consumer!
    
    println!("Result: {:?}", result);
    
    // Other consumers
    let sum: i32 = numbers.iter().sum();                    // Consumer
    let max = numbers.iter().max();                         // Consumer
    let count = numbers.iter().count();                     // Consumer
    let found = numbers.iter().find(|&x| x == &3);         // Consumer
    
    println!("Sum: {}, Max: {:?}, Count: {}, Found: {:?}", sum, max, count, found);
}

fn for_loop_vs_iterators() {
    println!("\n=== For Loop vs Iterators ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Traditional for loop (actually uses iterators under the hood)
    for num in &numbers {
        println!("For loop: {}", num);
    }
    
    // Equivalent iterator approach
    numbers.iter().for_each(|num| println!("Iterator: {}", num));
    
    // For loop with index
    for (i, num) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, num);
    }
    
    // Different iteration types
    for num in numbers.iter() {        // Borrowing
        println!("Borrowed: {}", num);
    }
    
    for num in numbers.clone().into_iter() {  // Taking ownership
        println!("Owned: {}", num);
    }
    
    let mut numbers_mut = vec![1, 2, 3];
    for num in numbers_mut.iter_mut() {  // Mutable borrowing
        *num *= 2;
    }
    println!("After mutation: {:?}", numbers_mut);
}

fn performance_comparison() {
    println!("\n=== Performance Comparison ===");
    
    use std::time::Instant;
    
    let numbers: Vec<i32> = (0..1_000_000).collect();
    
    // Traditional loop
    let start = Instant::now();
    let mut sum1 = 0;
    for i in 0..numbers.len() {
        sum1 += numbers[i] * 2;
    }
    let loop_time = start.elapsed();
    
    // Iterator approach
    let start = Instant::now();
    let sum2: i32 = numbers.iter().map(|x| x * 2).sum();
    let iter_time = start.elapsed();
    
    println!("Loop sum: {}, time: {:?}", sum1, loop_time);
    println!("Iterator sum: {}, time: {:?}", sum2, iter_time);
    println!("Performance difference: iterators are usually as fast or faster!");
}

// Custom iterator implementation
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

fn custom_iterator_demo() {
    println!("\n=== Custom Iterator ===");
    
    let counter = Counter::new(5);
    
    for num in counter {
        println!("Count: {}", num);
    }
    
    // Use iterator methods on custom iterator
    let sum: usize = Counter::new(10).sum();
    let evens: Vec<usize> = Counter::new(10).filter(|x| x % 2 == 0).collect();
    
    println!("Sum 0-9: {}", sum);
    println!("Evens 0-9: {:?}", evens);
}

fn advanced_patterns() {
    println!("\n=== Advanced Iterator Patterns ===");
    
    // Chaining iterators
    let iter1 = vec![1, 2, 3];
    let iter2 = vec![4, 5, 6];
    let chained: Vec<i32> = iter1.iter().chain(iter2.iter()).cloned().collect();
    println!("Chained: {:?}", chained);
    
    // Zip - combine two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("Pairs: {:?}", pairs);
    
    // Partition - split into two collections
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (evens, odds): (Vec<_>, Vec<_>) = numbers.iter().partition(|&x| x % 2 == 0);
    println!("Evens: {:?}, Odds: {:?}", evens, odds);
    
    // Group by (using itertools crate pattern)
    let words = vec!["apple", "banana", "apricot", "blueberry"];
    let mut grouped: std::collections::HashMap<char, Vec<&str>> = std::collections::HashMap::new();
    for word in words {
        grouped.entry(word.chars().next().unwrap()).or_insert(Vec::new()).push(word);
    }
    println!("Grouped by first letter: {:?}", grouped);
    
    // Flat map - flatten nested structures
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = nested.iter().flat_map(|v| v.iter()).cloned().collect();
    println!("Flattened: {:?}", flattened);
    
    // Scan - stateful map
    let numbers = vec![1, 2, 3, 4, 5];
    let running_sum: Vec<i32> = numbers.iter().scan(0, |acc, x| {
        *acc += x;
        Some(*acc)
    }).collect();
    println!("Running sum: {:?}", running_sum);
    
    // Step by (every nth element)
    let every_second: Vec<_> = (0..10).step_by(2).collect();
    println!("Every second: {:?}", every_second);
    
    // Cycle - infinite repetition
    let repeated: Vec<_> = vec![1, 2, 3].iter().cycle().take(10).collect();
    println!("Cycled: {:?}", repeated);
}

// Working with Results and Options in iterators
fn result_option_patterns() {
    println!("\n=== Result/Option Patterns ===");
    
    // Filter map - filter and transform in one step
    let strings = vec!["1", "2", "not_a_number", "4"];
    let numbers: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Parsed numbers: {:?}", numbers);
    
    // Collect into Result
    let strings = vec!["1", "2", "3"];
    let result: Result<Vec<i32>, _> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("All parsed: {:?}", result);
    
    // With error
    let strings_with_error = vec!["1", "2", "not_a_number"];
    let result_with_error: Result<Vec<i32>, _> = strings_with_error
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("With error: {:?}", result_with_error);
}

// Iterator trait bounds and generic functions
fn generic_iterator_functions() {
    fn process_iter<I, T>(iter: I) -> Vec<T> 
    where 
        I: Iterator<Item = T>,
        T: Clone,
    {
        iter.collect()
    }
    
    fn sum_iter<I>(iter: I) -> i32
    where
        I: Iterator<Item = i32>,
    {
        iter.sum()
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    let processed = process_iter(numbers.iter().cloned());
    let sum = sum_iter(numbers.iter().cloned());
    
    println!("Processed: {:?}, Sum: {}", processed, sum);
}
