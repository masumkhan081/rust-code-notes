/*
    Topic: Concurrency - Message Passing (Channels)
    
    Go-style concurrency in Rust using channels.
    
    Concepts:
    1. mpsc (Multiple Producer, Single Consumer) channel.
*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    println!("--- 03 Message Passing ---");

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi from thread 1");
        tx1.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("hi from thread 2");
        thread::sleep(Duration::from_millis(100)); // Simulate work
        tx.send(val).unwrap();
    });

    // Receiver iterates over messages
    // The loop likely ends when all transmitters are dropped
    for received in rx {
        println!("Got: {}", received);
    }
}
