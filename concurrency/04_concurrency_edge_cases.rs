/*
    Topic: Concurrency - Edge Cases & Deadlocks
    
    Tricky situations in concurrent programming.
    
    Scenarios:
    1. Deadlock: Two threads waiting for each other's locks.
    2. Poisoned Mutex: What happens if a thread panics while holding a lock?
*/

use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    println!("--- 04 Concurrency Edge Cases ---");
    
    demo_poisoned_mutex();
    // demo_deadlock(); // Uncomment to see the deadlock (program will hang)
}

fn demo_poisoned_mutex() {
    println!("\n[Demo] Mutex Poisoning");
    let lock = Arc::new(Mutex::new(0));
    let lock_clone = Arc::clone(&lock);

    let _ = thread::spawn(move || {
        let _guard = lock_clone.lock().unwrap();
        panic!("Oops! I panicked while holding the lock.");
    }).join();

    // Now the main thread tries to access the lock
    match lock.lock() {
        Ok(_) => println!("Lock acquired successfully (Unexpected)"),
        Err(poisoned) => {
            println!("Error: Mutex is poisoned! Recovering data...");
            let data = poisoned.into_inner();
            println!("Recovered data: {}", data);
        }
    }
}

// Unused to prevent hanging the notes run
#[allow(dead_code)]
fn demo_deadlock() {
    let lock1 = Arc::new(Mutex::new(1));
    let lock2 = Arc::new(Mutex::new(2));

    let l1 = Arc::clone(&lock1);
    let l2 = Arc::clone(&lock2);

    let t1 = thread::spawn(move || {
        let _g1 = l1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(100));
        let _g2 = l2.lock().unwrap(); // Waiting for lock 2
    });

    let l3 = Arc::clone(&lock1);
    let l4 = Arc::clone(&lock2);

    let t2 = thread::spawn(move || {
        let _g1 = l4.lock().unwrap(); // Acquired lock 2
        thread::sleep(std::time::Duration::from_millis(100));
        let _g2 = l3.lock().unwrap(); // Waiting for lock 1
    });
    
    t1.join().unwrap();
    t2.join().unwrap();
}
