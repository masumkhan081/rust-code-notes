// Self-referential structs are not allowed safely in Rust.
// Real fix pattern: store indices/ranges, not &str references into self.

#[derive(Debug)]
struct ParsedLine {
    buf: String,
    first_word_end: usize,
}

impl ParsedLine {
    fn new(input: impl Into<String>) -> Self {
        let buf = input.into();
        let first_word_end = buf
            .as_bytes()
            .iter()
            .position(|&b| b == b' ')
            .unwrap_or(buf.len());

        Self { buf, first_word_end }
    }

    fn first_word(&self) -> &str {
        &self.buf[..self.first_word_end]
    }
}

fn main() {
    let p = ParsedLine::new("hello world");
    println!("parsed={:?}", p);
    println!("first_word={}", p.first_word());
}
// Production relevance: parsing, tokenization, avoiding "borrowed field inside owned struct".
