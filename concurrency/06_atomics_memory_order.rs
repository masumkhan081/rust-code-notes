use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

static READY: AtomicBool = AtomicBool::new(false);
static VALUE: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let producer = thread::spawn(|| {
        // Publish data first...
        VALUE.store(123, Ordering::Relaxed);
        // ...then publish the flag with Release.
        READY.store(true, Ordering::Release);
    });

    let consumer = thread::spawn(|| {
        // Acquire pairs with producer's Release.
        while !READY.load(Ordering::Acquire) {
            std::hint::spin_loop();
        }
        // After Acquire, reads that happened-before Release are visible.
        let v = VALUE.load(Ordering::Relaxed);
        println!("Observed VALUE={v}");
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
// Interview payoff: "what ordering do you need here and why?"
