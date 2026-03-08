// Async: MutexGuard held across .await — the classic deadlock pitfall
// =====================================================================
// Holding std::sync::MutexGuard across an .await is a compile error
// (future becomes !Send) or a runtime deadlock on a single-threaded executor.
// This file shows the pitfall, the fixes, and the tradeoffs.

use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::{sleep, Duration};

// ── Pitfall: std Mutex guard held across await ─────────────────────────────
// The code below would NOT compile with tokio::spawn (future is !Send):
//
//   async fn broken() {
//       let m = Arc::new(Mutex::new(0u32));
//       let mut g = m.lock().unwrap();   // holds guard
//       sleep(Duration::from_millis(10)).await;  // ERROR: MutexGuard is !Send
//       *g += 1;
//   }

// ── Fix 1: drop the guard before the await ────────────────────────────────
async fn fix_drop_before_await(counter: Arc<Mutex<u32>>) {
    {
        let mut g = counter.lock().unwrap();
        *g += 1;
    } // guard dropped here — .await is now safe
    sleep(Duration::from_millis(10)).await;
    println!("[fix1] counter = {}", counter.lock().unwrap());
}

// ── Fix 2: use tokio::sync::Mutex (async-aware, guard is Send) ────────────
async fn fix_tokio_mutex(counter: Arc<AsyncMutex<u32>>) {
    let mut g = counter.lock().await; // async lock — no thread blocking
    *g += 1;
    sleep(Duration::from_millis(10)).await; // guard is still held — this is OK
    println!("[fix2] counter = {}", *g);
    // guard auto-dropped here
}

// ── Fix 3: clone data out of the lock before await ────────────────────────
async fn fix_clone_out(data: Arc<Mutex<String>>) {
    let snapshot = data.lock().unwrap().clone(); // hold lock briefly, clone
    sleep(Duration::from_millis(10)).await;      // no lock held
    println!("[fix3] snapshot = {}", snapshot);
}

// ── Tradeoff note ──────────────────────────────────────────────────────────
// tokio::sync::Mutex is slower than std::sync::Mutex for CPU-bound critical
// sections. Prefer std::Mutex + drop-before-await when the locked section is
// short and purely synchronous. Use tokio::Mutex only when you genuinely need
// to hold the lock across .await points.

// ── Bonus: watch for nested locks (recipe for deadlock) ───────────────────
async fn nested_lock_danger(a: Arc<Mutex<u32>>, b: Arc<Mutex<u32>>) {
    // Always acquire locks in a consistent order to prevent deadlock.
    let mut ga = a.lock().unwrap();
    let mut gb = b.lock().unwrap();
    *ga += 1;
    *gb += 1;
    println!("[nested] a={} b={}", *ga, *gb);
}

#[tokio::main]
async fn main() {
    let std_counter = Arc::new(Mutex::new(0u32));
    let async_counter = Arc::new(AsyncMutex::new(0u32));
    let data = Arc::new(Mutex::new(String::from("hello")));
    let a = Arc::new(Mutex::new(1u32));
    let b = Arc::new(Mutex::new(2u32));

    fix_drop_before_await(Arc::clone(&std_counter)).await;
    fix_tokio_mutex(Arc::clone(&async_counter)).await;
    fix_clone_out(Arc::clone(&data)).await;
    nested_lock_danger(Arc::clone(&a), Arc::clone(&b)).await;

    println!("all lock-across-await demos done");
}
