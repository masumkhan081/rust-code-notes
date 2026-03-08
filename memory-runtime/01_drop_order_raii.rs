// Memory & Runtime: drop order, RAII, mem::forget, ManuallyDrop
// ==============================================================
// In Rust, values are dropped in reverse declaration order (LIFO).
// Understanding this is critical for locks, files, and custom destructors.

use std::mem::{self, ManuallyDrop};

// ── 1. Drop order demonstration ───────────────────────────────────────────
struct Droppable(&'static str);

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("  dropping: {}", self.0);
    }
}

fn drop_order_demo() {
    println!("=== Drop Order (LIFO) ===");
    let _a = Droppable("a  (declared first)");
    let _b = Droppable("b");
    let _c = Droppable("c  (declared last → dropped first)");
    // prints: c, b, a
}

// ── 2. RAII: resource tied to a struct's lifetime ─────────────────────────
struct FileGuard {
    name: String,
}

impl FileGuard {
    fn open(name: &str) -> Self {
        println!("  [FileGuard] opening '{}'", name);
        FileGuard { name: name.to_string() }
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("  [FileGuard] closing '{}'", self.name);
    }
}

fn raii_demo() {
    println!("=== RAII ===");
    let _f = FileGuard::open("data.txt");
    println!("  doing work...");
    // _f dropped here — 'close' is guaranteed even on early return / panic
}

// ── 3. mem::forget — opt out of drop (use carefully) ─────────────────────
// mem::forget prevents Drop from running.  Useful when transferring ownership
// to FFI or when you've already cleaned up manually.
fn forget_demo() {
    println!("=== mem::forget ===");
    let d = Droppable("forgotten — will NOT print drop message");
    mem::forget(d); // Drop::drop is never called
    println!("  mem::forget returned — no drop message above");
}

// ── 4. ManuallyDrop — explicit control without consuming the value ─────────
// ManuallyDrop wraps a value and inhibits automatic dropping.
// You must call ManuallyDrop::drop() yourself (unsafe).
fn manually_drop_demo() {
    println!("=== ManuallyDrop ===");
    let mut md = ManuallyDrop::new(Droppable("manually-dropped"));
    println!("  doing work with the value...");
    // SAFETY: we are the sole owner and never use `md` after this call.
    unsafe { ManuallyDrop::drop(&mut md) };
    println!("  manual drop done");
}

// ── 5. Drop order in structs ──────────────────────────────────────────────
// Fields are dropped in declaration order (first-to-last), which is the
// opposite of local variables.
struct Connection {
    _socket: Droppable,   // dropped second
    _buffer: Droppable,   // dropped first
}

fn struct_field_drop_order() {
    println!("=== Struct field drop order (declaration order) ===");
    let _conn = Connection {
        _socket: Droppable("socket"),
        _buffer: Droppable("buffer (declared last → dropped last)"),
    };
    // Drops: socket, then buffer
    // Note: opposite of local variable LIFO!
}

// ── 6. Early drop with std::mem::drop ────────────────────────────────────
fn early_drop_demo() {
    println!("=== Early explicit drop ===");
    let lock = std::sync::Mutex::new(42u32);
    let guard = lock.lock().unwrap();
    println!("  lock held, value = {}", *guard);
    drop(guard); // release lock early — same as closing a scope block
    println!("  lock released before end of function");
}

fn main() {
    drop_order_demo();
    println!();
    raii_demo();
    println!();
    forget_demo();
    println!();
    manually_drop_demo();
    println!();
    struct_field_drop_order();
    println!();
    early_drop_demo();
}
