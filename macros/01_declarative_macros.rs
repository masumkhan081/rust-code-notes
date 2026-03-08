// Declarative macros that compile (no duplicate names, no paste dependency)
// NOTE: macro_rules! macros must be defined before use in the same file.

use std::collections::HashMap;

// ── macro definitions (must come before main) ────────────────────────────────

macro_rules! say_hello {
    () => { println!("Hello!"); };
    (to $name:expr) => { println!("Hello, {}!", $name); };
    (times $count:expr) => {
        for _ in 0..$count { println!("Hello!"); }
    };
    (to $name:expr, times $count:expr) => {
        for _ in 0..$count { println!("Hello, {}!", $name); }
    };
}

macro_rules! my_vec {
    () => { Vec::new() };
    ($elem:expr; $n:expr) => {{
        let mut v = Vec::new();
        v.resize($n, $elem);
        v
    }};
    ($($elem:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $( v.push($elem); )+
        v
    }};
}

macro_rules! hashmap {
    () => { HashMap::new() };
    ($($k:expr => $v:expr),+ $(,)?) => {{
        let mut m = HashMap::new();
        $( m.insert($k, $v); )+
        m
    }};
}

// Getter/setter without paste: require explicit function names.
macro_rules! getter_setter {
    ($get:ident, $set:ident, $field:ident, $ty:ty) => {
        fn $get(s: &Config) -> &$ty { &s.$field }
        fn $set(s: &mut Config, v: impl Into<$ty>) { s.$field = v.into(); }
    };
}

struct Config {
    host: String,
}

getter_setter!(get_host, set_host, host, String);

// ── main ─────────────────────────────────────────────────────────────────────

fn main() {
    say_hello!();
    say_hello!(to "World");
    say_hello!(times 2);
    say_hello!(to "Rust", times 2);

    let v1 = my_vec![1, 2, 3];
    let v2 = my_vec![42; 4];
    println!("v1={:?} v2={:?}", v1, v2);

    let map = hashmap! {
        "a" => 1,
        "b" => 2,
    };
    println!("map={:?}", map);

    let mut cfg = Config { host: "localhost".to_string() };
    set_host(&mut cfg, "api.local");
    println!("cfg host={}", get_host(&cfg));
}
