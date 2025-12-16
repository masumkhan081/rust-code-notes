/*
    Topic: Concurrency - Interview Questions
    
    Common concurrency challenges.
    
    1. Producer-Consumer Problem (Classic)
    2. Implement a thread-safe Singleton (Lazy) in Rust.
*/

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::collections::VecDeque;

// Interview Q1: Producer-Consumer using Condvar
// Problem: Implement a bounded buffer where producers wait if full, consumers wait if empty.
struct BoundedBuffer {
    queue: Mutex<VecDeque<i32>>,
    cond: Condvar,
    capacity: usize,
}

impl BoundedBuffer {
    fn new(capacity: usize) -> Self {
        BoundedBuffer {
            queue: Mutex::new(VecDeque::new()),
            cond: Condvar::new(),
            capacity,
        }
    }

    fn produce(&self, item: i32) {
        let mut queue = self.queue.lock().unwrap();
        while queue.len() >= self.capacity {
            queue = self.cond.wait(queue).unwrap();
        }
        queue.push_back(item);
        println!("Produced: {}", item);
        self.cond.notify_all();
    }

    fn consume(&self) -> i32 {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.cond.wait(queue).unwrap();
        }
        let item = queue.pop_front().unwrap();
        println!("Consumed: {}", item);
        self.cond.notify_all();
        item
    }
}

pub fn main() {
    println!("--- 05 Concurrency Interview - Producer/Consumer ---");
    let buffer = Arc::new(BoundedBuffer::new(5));

    let p_buffer = Arc::clone(&buffer);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            p_buffer.produce(i);
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let c_buffer = Arc::clone(&buffer);
    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            c_buffer.consume();
            thread::sleep(std::time::Duration::from_millis(100)); // Slower consumer
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
