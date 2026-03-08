use std::sync::OnceLock;

#[derive(Debug)]
struct Config {
    app_name: String,
    pool_size: usize,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "my-app".to_string());
        let pool_size = std::env::var("POOL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        Config { app_name, pool_size }
    })
}

fn main() {
    std::env::set_var("APP_NAME", "notes");
    std::env::set_var("POOL_SIZE", "32");

    println!("cfg1 = {:?}", config());
    println!("cfg2 = {:?}", config()); // same instance
}
