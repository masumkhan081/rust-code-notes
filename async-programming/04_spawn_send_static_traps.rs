// Async: tokio::spawn — Send + 'static traps
// ============================================
// tokio::spawn requires the future to be Send + 'static.
// This file shows the common traps and how to fix them.

use tokio::sync::Mutex;
use std::sync::Arc;

// ── TRAP 1: spawning a future that holds a non-Send type ─────────────────────
// std::rc::Rc is !Send.  Holding it across an .await makes the future !Send.
// Fix: replace Rc with Arc.

async fn trap_rc_not_send() {
    // BAD (does not compile — uncomment to see E0277):
    // let rc = std::rc::Rc::new(42);
    // tokio::spawn(async move {
    //     tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    //     println!("{}", rc);  // rc is Rc → future is !Send
    // });

    // GOOD: use Arc instead
    let arc = Arc::new(42);
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        println!("arc value: {}", arc);
    });
}

// ── TRAP 2: spawning a future that borrows a local (not 'static) ──────────────
// Spawned tasks must own all their data; borrows of locals are not 'static.
// Fix: clone or Arc-wrap the data before moving into the task.

async fn trap_borrow_not_static() {
    let data = String::from("hello");

    // BAD (does not compile — uncomment to see E0597 / lifetime error):
    // tokio::spawn(async {
    //     println!("{}", data);  // borrows `data` from enclosing scope
    // });

    // GOOD: move or clone into the task
    let data_clone = data.clone();
    let handle = tokio::spawn(async move {
        println!("task sees: {}", data_clone);
    });
    handle.await.unwrap();
    println!("original still here: {}", data);
}

// ── TRAP 3: holding a MutexGuard across an .await ────────────────────────────
// std::sync::MutexGuard is !Send.  Keeping a guard alive across .await blocks
// the thread and can deadlock under tokio's work-stealing scheduler.
// Fix: drop the guard before .await, or use tokio::sync::Mutex.

async fn trap_std_mutex_guard_across_await() {
    let m = Arc::new(std::sync::Mutex::new(0u32));

    // BAD pattern (compiles but can deadlock / panic at runtime):
    // let guard = m.lock().unwrap();
    // some_async_fn().await;        // guard still held → thread blocked
    // drop(guard);

    // GOOD: drop the guard in a separate scope before the await
    {
        let mut guard = m.lock().unwrap();
        *guard += 1;
    } // guard dropped here
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;

    // ALSO GOOD: use tokio::sync::Mutex (its guard IS Send)
    let tm = Arc::new(Mutex::new(0u32));
    {
        let mut guard = tm.lock().await;
        *guard += 1;
    }
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    println!("tokio mutex value: {}", *tm.lock().await);
}

// ── TRAP 4: JoinHandle must be awaited or the task is silently dropped ────────

async fn trap_dropped_join_handle() {
    // If you drop the JoinHandle without awaiting it the task IS still running
    // (tokio detaches it), but any panic inside is swallowed silently.
    let handle = tokio::spawn(async {
        // panics are caught by tokio and turned into JoinError
        42u32
    });

    // Always check the result:
    match handle.await {
        Ok(v)  => println!("task returned: {}", v),
        Err(e) => println!("task panicked: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    trap_rc_not_send().await;
    trap_borrow_not_static().await;
    trap_std_mutex_guard_across_await().await;
    trap_dropped_join_handle().await;
    println!("all spawn / Send / 'static demos done");
}
