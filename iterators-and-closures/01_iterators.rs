// Iterators in Rust (compile-safe version)
// ========================================

fn main() {
    basics_iter_iter_mut();
    adaptor_vs_consumer();
    try_collect_result_pattern();
    custom_iterator_demo();
}

fn basics_iter_iter_mut() {
    // iter(): &T
    let v = vec![1, 2, 3];
    let sum: i32 = v.iter().copied().sum();
    println!("iter sum = {sum}, v still usable = {:?}", v);

    // into_iter(): T (moves)
    let v2 = vec![1, 2, 3];
    let owned: Vec<i32> = v2.into_iter().map(|x| x * 10).collect();
    println!("into_iter collected = {:?}", owned);

    // iter_mut(): &mut T
    let mut v3 = vec![1, 2, 3];
    for x in v3.iter_mut() {
        *x *= 100;
    }
    println!("iter_mut mutated = {:?}", v3);
}

fn adaptor_vs_consumer() {
    let v = vec![1, 2, 3, 4, 5, 6];

    // Adaptors are lazy
    let it = v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x);

    // Consumer triggers evaluation
    let evens_squared: Vec<i32> = it.collect();
    println!("evens_squared = {:?}", evens_squared);
}

fn try_collect_result_pattern() {
    let inputs = ["10", "20", "x", "40"];

    // Interview-grade pattern: parsing pipeline that returns Result<Vec<_>, _>
    let parsed: Result<Vec<i32>, _> = inputs
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();

    println!("parsed = {:?}", parsed);

    // If you want "best effort" parsing:
    let only_ok: Vec<i32> = inputs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("only_ok = {:?}", only_ok);
}

struct Counter {
    cur: u32,
    end: u32,
}

impl Counter {
    fn new(end: u32) -> Self {
        Self { cur: 0, end }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            None
        } else {
            let out = self.cur;
            self.cur += 1;
            Some(out)
        }
    }
}

fn custom_iterator_demo() {
    let c = Counter::new(5);

    let v: Vec<u32> = c
        .skip(1)
        .take(3)
        .map(|x| x * 2)
        .collect();

    println!("custom iterator result = {:?}", v);
}
