use std::thread;

fn main() {
    // Scoped threads: borrow stack data safely (no 'static requirement).
    let mut data = vec![1, 2, 3, 4];

    thread::scope(|s| {
        let (left, right) = data.split_at_mut(2);

        s.spawn(move || {
            for x in left {
                *x *= 10;
            }
        });

        s.spawn(move || {
            for x in right {
                *x *= 100;
            }
        });
    });

    println!("data = {:?}", data);
}
// Interview payoff: "how do you parallelize work without cloning everything?"
