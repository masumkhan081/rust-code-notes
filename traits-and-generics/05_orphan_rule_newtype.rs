use std::fmt;

// Orphan rule: you can't implement a foreign trait for a foreign type.
// Newtype pattern: wrap the type you don't own.

struct Csv<'a>(&'a [i32]);

impl fmt::Display for Csv<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, x) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }
            write!(f, "{x}")?;
        }
        Ok(())
    }
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{}", Csv(&v));
}
// "Rust trait system maturity" — interviewers absolutely ask this.
