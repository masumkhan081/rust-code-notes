ROLE: You are GitHub Copilot working as a senior Rust engineer.
GOAL: Make this rust-code-notes repo “interview-proof” by (1) making every example compile/run cleanly, (2) adding missing senior-level topics with runnable code, and (3) adding a minimal real backend project. Do NOT just list topics—add/replace files with full runnable code.

HARD GATES:

- Every .rs file must compile (unless explicitly marked “INTENTIONAL COMPILE FAIL” and paired with a fixed version).
- No committed binaries (.exe, ./pattern, ./lifetimes, etc.) and no target/ directory in repo.
- Keep examples realistic (backend-oriented), avoid toy snippets.
- No misleading code: avoid shadowing std types (Option/Result/Iterator) and avoid naming collisions.
- Each new file must be runnable via `cargo run --bin <name>` OR via a workspace member crate (backend example).
- Add a top-level TODO checklist and a repo-wide sanity script.

STEP 0 — REPO CLEANUP (must do first)

1. Delete all compiled artifacts currently in the repo:
   - root/main.exe, root/pattern.exe, root/ownership_demo, function-module/pattern, ownership-borrowing-lifetimes/lifetimes and any similar binaries.
2. Update .gitignore to include:
   - /target
   - \*.exe
   - \*.out
   - \*.bin
   - .DS_Store

STEP 1 — MAKE EXISTING NOTES COMPILE (replace broken examples)
A) Replace these files with compile-safe versions:

- iterators-and-closures/01_iterators.rs
- traits-and-generics/02_traits.rs
- macros/01_declarative_macros.rs

B) Fix or refactor any other file that fails compilation:

- Remove custom enums named Option/Result unless namespaced or renamed (e.g., MyOption/MyResult).
- Remove any custom trait named Iterator; never shadow std::iter::Iterator.
- Ensure module examples reference real module files; if modules-and-visibility/03_module_usage_example.rs declares `mod front_of_house;`, create `modules-and-visibility/front_of_house.rs` (or rewrite to use inline modules).

STEP 2 — ADD “SENIOR RUST GAP” FILES (new runnable examples)
Create the following new files with complete runnable code, each demonstrating the concept + one production pitfall:

concurrency/
06_atomics_memory_order.rs - Demonstrate Acquire/Release handshake using AtomicBool + AtomicUsize. - Explain via code structure (not comments) why Relaxed isn’t enough for the flag.
07_once_lock_singleton.rs - Use OnceLock for config singleton. Show get_or_init and environment parsing.
08_scoped_threads_borrowing.rs - Use std::thread::scope to borrow stack slices across threads safely.

ownership-borrowing-lifetimes/
06_partial_moves_take.rs - Show partial move trap, fix with ref pattern, Option::take, mem::take.
07_self_referential_fix_with_index.rs - Explain self-referential struct problem by implementing index/range-based workaround and safe getter.

traits-and-generics/
05_orphan_rule_newtype.rs - Show orphan rule via newtype wrapper implementing Display.
06_object_safety.rs - Show object-safe trait used as dyn Trait and a non-object-safe trait (generic method) that cannot be dyn.

async-programming/ (add missing async traps)
04_spawn_send_static_traps.rs - Show tokio::spawn requires Send + 'static future; demonstrate a failing pattern in a commented block and then fix it.
05_lock_across_await_pitfall.rs - Show deadlock/perf trap: holding mutex guard across await; fix by dropping guard before await.

memory-runtime/
01_drop_order_raii.rs - Demonstrate drop order, RAII, mem::forget, ManuallyDrop boundary (safely).
02_vec_realloc_invalidation.rs - Show how pushing to Vec can reallocate and invalidate references; fix by reserving, using indices, or split ownership.

STEP 3 — ADD A REAL BACKEND MINI-PROJECT (axum)
Create a new folder:
backend-patterns/axum_minimal/
Cargo.toml
src/main.rs

Requirements:

- Axum 0.7 + Tokio + Serde + thiserror + tracing + tracing-subscriber.
- Endpoints:
  - GET /healthz -> "ok"
  - POST /items {name} -> creates item with atomic ID
  - GET /items/:id -> returns item or 404
- Use AppState { AtomicU64, RwLock<HashMap<..>> }.
- Use a typed ApiError implementing IntoResponse with JSON {error}.
- Add tracing logs at startup and on request handling.

STEP 4 — WIRE IT TO CARGO (so everything runs easily)
Option A (preferred): Convert the repo into a Cargo workspace:

- root Cargo.toml becomes [workspace] with members:
  - notes (a crate containing bins for each .rs example OR grouped crates)
  - backend-patterns/axum_minimal
- For notes: configure each file as a bin target (e.g., src/bin/...).
  Option B: Keep single crate but add `[[bin]]` entries for each example file and ensure paths align.

Choose the approach that requires the least restructuring while making:

- `cargo test` succeed
- `cargo run --bin <example>` possible for each example and for axum_minimal.

STEP 5 — ADD TODO + SANITY CHECK SCRIPT

1. Add top-level TODO.md with checkboxes:

- Compile gate passed
- Senior gap files added
- Backend project runs
- No artifacts committed

2. Add scripts/sanity.sh (bash) that:

- runs `cargo fmt --check`
- runs `cargo clippy -- -D warnings`
- runs `cargo test`
- optionally runs a small subset of bins to ensure runtime behavior.

OUTPUT REQUIREMENTS:

- Implement everything above in the repo.
- Ensure all new files compile.
- Ensure no duplicate macro names, no missing deps, no missing modules.
- Keep code production-shaped (structured errors, correct concurrency patterns).
- After changes, provide a brief report in TODO.md: what was changed, what was added, and how to run examples.

START NOW:

- First make the repo compile-clean and remove artifacts.
- Then add gap files.
- Then add the backend mini-project.
- Then add TODO + sanity script.
