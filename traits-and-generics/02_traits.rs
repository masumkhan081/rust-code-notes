// Traits in Rust (compile-safe + senior traps covered)
// ====================================================

use std::fmt::{self, Debug, Display};

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "people".to_string(),
    };

    let article = NewsArticle {
        headline: "Rust wins again".to_string(),
        author: "Ferris".to_string(),
    };

    println!("tweet: {}", tweet.summarize());
    println!("article: {}", article.summarize());

    notify(&tweet);
    notify(&article);

    notify_display(&tweet);

    trait_object_demo();
    iterator_assoc_type_demo();
    orphan_rule_preview();
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Clone, Debug)]
struct Tweet {
    username: String,
    content: String,
}

#[derive(Clone, Debug)]
struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.summarize())
    }
}

fn notify(item: &impl Summary) {
    println!("notify: {}", item.summarize());
}

fn notify_display(item: &(impl Summary + Display)) {
    println!("notify_display: {}", item);
}

fn trait_object_demo() {
    // dyn Trait is runtime polymorphism (vtable)
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Tweet {
            username: "a".to_string(),
            content: "b".to_string(),
        }),
        Box::new(NewsArticle {
            headline: "h".to_string(),
            author: "x".to_string(),
        }),
    ];

    for it in items {
        println!("dyn Summary => {}", it.summarize());
    }
}

fn iterator_assoc_type_demo() {
    // Associated types are the real Iterator pattern
    struct Counter(u32);

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            let n = self.0;
            self.0 += 1;
            Some(n)
        }
    }

    let sum: u32 = Counter(1).take(5).sum();
    println!("iterator sum = {sum}");
}

fn orphan_rule_preview() {
    // You cannot implement a foreign trait for a foreign type:
    // impl Display for Vec<i32> { ... }  // <- orphan rule (won't compile)
    // Newtype solves it (see 05_orphan_rule_newtype.rs).
}
