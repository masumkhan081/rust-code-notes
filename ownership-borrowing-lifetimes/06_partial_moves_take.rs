#[derive(Debug)]
struct User {
    id: u64,
    email: String,
    token: Option<String>,
}

fn main() {
    let mut u = User {
        id: 1,
        email: "a@b.com".to_string(),
        token: Some("secret".to_string()),
    };

    // Move a field out safely: Option::take replaces it with None.
    let token = u.token.take().expect("token exists");
    println!("moved token={token}");
    println!("user after take = {:?}", u);

    let u2 = User {
        id: 2,
        email: "x@y.com".to_string(),
        token: None,
    };
    // Partial move trap: let User { email, .. } = u2; // moves email; u2 becomes partially moved

    // Fix: borrow fields in pattern
    let User { ref email, .. } = u2;
    println!("borrowed email={email}, still can read id={}", u2.id);

    // mem::take pattern: drain a value by replacing with Default
    let mut payload = String::from("request-body");
    let moved = std::mem::take(&mut payload);
    println!("moved={moved}, payload now='{payload}'");
}
// Classic interview pitfall: partial move and "how do I move out of a field?"
