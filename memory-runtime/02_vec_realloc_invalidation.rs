// Memory & Runtime: Vec reallocation and reference invalidation
// =============================================================
// Vec<T> stores elements on the heap.  When it grows beyond its capacity it
// reallocates — copying all elements to a new allocation.  Any raw pointer or
// reference to the old buffer is dangling after the reallocation.
// Rust's borrow checker catches the iterator/reference invalidation case at
// compile time; raw pointer invalidation requires unsafe and is your responsibility.

fn reallocation_basics() {
    println!("=== Vec reallocation basics ===");

    let mut v: Vec<i32> = Vec::with_capacity(2);
    let ptr_before = v.as_ptr();
    println!("  capacity: {}, ptr: {:p}", v.capacity(), ptr_before);

    v.push(1);
    v.push(2);
    // Next push exceeds capacity → realloc
    v.push(3);

    let ptr_after = v.as_ptr();
    println!("  capacity: {}, ptr: {:p}", v.capacity(), ptr_after);

    if ptr_before != ptr_after {
        println!("  *** pointer changed — reallocation happened ***");
    } else {
        println!("  pointer unchanged (allocator reused same block)");
    }
}

// ── Borrow-checker prevents iterator invalidation ────────────────────────
// The classic C++ bug:
//   for (auto& x : vec) { vec.push_back(x); }  // UB — iterator invalidated
//
// Rust won't compile an equivalent:
fn borrow_checker_prevents_invalidation() {
    println!("=== Borrow checker blocks push-while-iterating ===");
    let mut v = vec![1i32, 2, 3];

    // This would not compile — uncomment to see E0502:
    // for x in &v {
    //     v.push(*x);  // ERROR: cannot borrow `v` as mutable because it is
    //                  //        also borrowed as immutable
    // }

    // Safe alternative: collect indices first, then mutate
    let len = v.len();
    for i in 0..len {
        let val = v[i];
        v.push(val * 2);
    }
    println!("  doubled elements appended: {:?}", v);
}

// ── Pre-allocate to avoid repeated reallocations ─────────────────────────
fn preallocate_to_avoid_realloc() {
    println!("=== Pre-allocate with reserve ===");
    let n = 1_000usize;

    // Without pre-allocation: O(log n) reallocations
    let mut v1: Vec<u64> = Vec::new();
    for i in 0..n { v1.push(i as u64); }

    // With pre-allocation: zero reallocations
    let mut v2: Vec<u64> = Vec::with_capacity(n);
    for i in 0..n { v2.push(i as u64); }

    // Reserve additional space dynamically
    let mut v3: Vec<u64> = Vec::new();
    v3.reserve(n);
    for i in 0..n { v3.push(i as u64); }

    println!("  v1 final capacity: {}", v1.capacity());
    println!("  v2 final capacity: {} (no reallocs)", v2.capacity());
    println!("  v3 final capacity: {}", v3.capacity());
}

// ── Raw pointer invalidation (unsafe) — DO NOT do this ───────────────────
// This is commented out because executing it is undefined behaviour.
// It demonstrates why you must not cache raw pointers across push().
fn raw_pointer_invalidation_example() {
    println!("=== Raw pointer invalidation (unsafe — demonstration only) ===");

    // NEVER do this:
    // let mut v = vec![1i32, 2, 3];
    // let p: *const i32 = &v[0];   // raw pointer to element 0
    // v.push(4);                   // may reallocate — `p` is now dangling
    // println!("{}", unsafe { *p }); // UB: use-after-free

    // Safe: re-borrow after mutation
    let mut v = vec![1i32, 2, 3];
    v.push(4);
    let first = &v[0]; // fresh borrow after push — always valid
    println!("  first element (re-borrowed after push): {}", first);
}

// ── shrink_to_fit: release excess capacity ───────────────────────────────
fn shrink_demo() {
    println!("=== shrink_to_fit ===");
    let mut v: Vec<i32> = Vec::with_capacity(1000);
    v.extend(0..10);
    println!("  len={}, cap={}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("  after shrink_to_fit → len={}, cap={}", v.len(), v.capacity());
}

fn main() {
    reallocation_basics();
    println!();
    borrow_checker_prevents_invalidation();
    println!();
    preallocate_to_avoid_realloc();
    println!();
    raw_pointer_invalidation_example();
    println!();
    shrink_demo();
}
