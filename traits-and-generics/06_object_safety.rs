// Object safety: why some traits can be dyn Trait and some cannot.

trait Store {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, val: String);
}

// Not object safe (generic method makes it impossible to build a vtable):
// trait NotObjectSafe {
//     fn parse<T: std::str::FromStr>(&self, s: &str) -> T;
//             ^ generic method => cannot be called through dyn
// }

struct MemStore {
    data: std::collections::HashMap<String, String>,
}

impl Store for MemStore {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: String, val: String) {
        self.data.insert(key, val);
    }
}

fn main() {
    let mut s: Box<dyn Store> = Box::new(MemStore { data: Default::default() });
    s.set("k".to_string(), "v".to_string());
    println!("k => {:?}", s.get("k"));
}
// If you can explain object safety cleanly, you sound senior immediately.
