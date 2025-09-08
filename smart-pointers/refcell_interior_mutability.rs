// RefCell<T> and Interior Mutability
// ===================================
// RefCell<T> provides interior mutability - allows mutation of data even when there are immutable references.
// Enforces borrowing rules at runtime instead of compile time.

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Basic RefCell usage
    let data = RefCell::new(5);
    
    {
        let mut borrowed = data.borrow_mut();
        *borrowed += 10;
    } // Mutable borrow is dropped here
    
    println!("Data: {}", data.borrow());
    
    // RefCell with Rc for shared mutable data
    shared_mutable_data();
    
    // Mock object pattern
    mock_object_example();
    
    // Interior mutability patterns
    interior_mutability_patterns();
    
    // Runtime borrow checking
    // runtime_borrow_panic(); // Commented out to avoid panic
    
    // Performance considerations
    performance_comparison();
}

fn shared_mutable_data() {
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    
    *a.borrow_mut() += 10;
    *b.borrow_mut() += 5;
    
    println!("Final value: {}", value.borrow());
}

// Mock object example - useful for testing
trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // Even though send takes &self, we can mutate sent_messages
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

fn mock_object_example() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    
    limit_tracker.set_value(80);
    
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    println!("Mock test passed! Sent: {:?}", mock_messenger.sent_messages.borrow());
}

// Interior mutability patterns
struct Counter {
    count: RefCell<usize>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: RefCell::new(0),
        }
    }
    
    // Method takes &self but can mutate internal state
    fn increment(&self) {
        let mut count = self.count.borrow_mut();
        *count += 1;
    }
    
    fn get(&self) -> usize {
        *self.count.borrow()
    }
}

// Cache example with interior mutability
use std::collections::HashMap;

struct Cache {
    data: RefCell<HashMap<String, String>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: RefCell::new(HashMap::new()),
        }
    }
    
    fn get(&self, key: &str) -> Option<String> {
        self.data.borrow().get(key).cloned()
    }
    
    fn set(&self, key: String, value: String) {
        self.data.borrow_mut().insert(key, value);
    }
}

fn interior_mutability_patterns() {
    // Counter pattern
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Counter: {}", counter.get());
    
    // Cache pattern
    let cache = Cache::new();
    cache.set("key1".to_string(), "value1".to_string());
    cache.set("key2".to_string(), "value2".to_string());
    
    println!("Cached value: {:?}", cache.get("key1"));
}

// Demonstrating runtime borrow checking (this would panic)
fn runtime_borrow_panic() {
    let data = RefCell::new(5);
    
    let _borrow1 = data.borrow(); // Immutable borrow
    let _borrow2 = data.borrow(); // Another immutable borrow (OK)
    
    // This would panic! Cannot borrow mutably while immutably borrowed
    // let _borrow3 = data.borrow_mut(); // PANIC!
    
    println!("This line would not be reached if we uncommented the panic line");
}

// Safe runtime borrow checking
fn safe_runtime_borrowing() {
    let data = RefCell::new(5);
    
    // Use try_borrow and try_borrow_mut for safe borrowing
    match data.try_borrow_mut() {
        Ok(mut borrowed) => {
            *borrowed += 10;
            println!("Successfully borrowed and modified: {}", *borrowed);
        }
        Err(e) => {
            println!("Failed to borrow: {:?}", e);
        }
    }
}

// Performance comparison: RefCell vs regular references
fn performance_comparison() {
    use std::time::Instant;
    
    // Regular mutable reference
    let start = Instant::now();
    let mut data = 0;
    for i in 0..1_000_000 {
        data += i;
    }
    let regular_time = start.elapsed();
    
    // RefCell
    let start = Instant::now();
    let data = RefCell::new(0);
    for i in 0..1_000_000 {
        *data.borrow_mut() += i;
    }
    let refcell_time = start.elapsed();
    
    println!("Regular reference time: {:?}", regular_time);
    println!("RefCell time: {:?}", refcell_time);
    println!("RefCell overhead: {:?}", refcell_time.saturating_sub(regular_time));
}

// Tree structure with RefCell for mutable children
#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(TreeNode {
            value,
            children: RefCell::new(Vec::new()),
        })
    }
    
    fn add_child(self: &Rc<Self>, child: Rc<TreeNode>) {
        self.children.borrow_mut().push(child);
    }
    
    fn print_tree(&self, depth: usize) {
        println!("{:indent$}{}", "", self.value, indent = depth * 2);
        for child in self.children.borrow().iter() {
            child.print_tree(depth + 1);
        }
    }
}

fn tree_example() {
    let root = TreeNode::new(1);
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);
    let grandchild = TreeNode::new(4);
    
    child1.add_child(grandchild);
    root.add_child(child1);
    root.add_child(child2);
    
    println!("Tree structure:");
    root.print_tree(0);
}

// Multiple borrowing examples
fn multiple_borrowing_examples() {
    let data = RefCell::new(vec![1, 2, 3, 4, 5]);
    
    // Multiple immutable borrows are allowed
    {
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        
        println!("First borrow: {:?}", *borrow1);
        println!("Second borrow: {:?}", *borrow2);
    } // Borrows dropped here
    
    // Now we can have a mutable borrow
    {
        let mut borrow_mut = data.borrow_mut();
        borrow_mut.push(6);
        println!("After mutation: {:?}", *borrow_mut);
    }
}

// Cell vs RefCell comparison
use std::cell::Cell;

fn cell_vs_refcell() {
    // Cell: for Copy types only, moves values
    let cell_data = Cell::new(5);
    let old_value = cell_data.replace(10);
    println!("Cell - old: {}, new: {}", old_value, cell_data.get());
    
    // RefCell: for any type, borrows values
    let refcell_data = RefCell::new(vec![1, 2, 3]);
    refcell_data.borrow_mut().push(4);
    println!("RefCell: {:?}", refcell_data.borrow());
}

// Thread safety note
fn thread_safety_note() {
    println!("=== Thread Safety ===");
    println!("RefCell is NOT thread-safe!");
    println!("For thread-safe interior mutability, use:");
    println!("  - Mutex<T> for exclusive access");
    println!("  - RwLock<T> for shared read/exclusive write");
    println!("  - AtomicT types for lock-free operations");
}
