use once_cell::sync::Lazy;

const COMPILED_ENV: &str = include_str!("../default.env");

/// Initialize environment variables from .env file
/// or default.env baked into executable.
pub fn init_env() {
    match dotenvy::dotenv() {
        Ok(_) =>
            eprintln!("Cultivators: using settings from: compiled binary + .env file + overrides in environment"),
        Err(_) =>
            eprintln!("Cultivators: no .env file exists, using settings from: compiled binary + overrides in environment"),
    }
    dotenvy::from_read(COMPILED_ENV.as_bytes()).unwrap();
}

pub fn print_default_env() {
    println!("{}", COMPILED_ENV);
}

fn str_to_bool(s: String) -> bool {
    matches!(
        s.to_lowercase().as_str(),
        "1" | "t" | "y" | "yes" | "true" | "on" | "enabled"
    )
}

pub fn bool_from_env(key: &str) -> bool {
    std::env::var(key).is_ok_and(str_to_bool)
}

pub fn bind_address() -> String {
    std::env::var("CULTIVATORS_BIND_ADDRESS").expect("CULTIVATORS_BIND_ADDRESS must be set")
}

pub fn use_tls() -> bool {
    bool_from_env("CULTIVATORS_USE_TLS")
}

fn base_url_from_bind_address() -> String {
    let mut bind_address = bind_address();
    let tls = use_tls();
    let scheme = if tls { "https" } else { "http" };
    // if bind_Address starts with 127.0.0.1, replace that with localhost:
    if bind_address.starts_with("127.0.0.1") {
        bind_address = format!("localhost{}", bind_address.trim_start_matches("127.0.0.1"));
    }
    let s = format!("{scheme}://{bind_address}");
    if !tls && bind_address.ends_with(":80") {
        s.trim_end_matches(":80").to_string()
    } else if tls && bind_address.ends_with(":443") {
        s.trim_end_matches(":443").to_string()
    } else {
        s
    }
}

pub fn base_url() -> String {
    std::env::var("CULTIVATORS_BASE_URL")
        .unwrap_or_else(|_| base_url_from_bind_address())
}

pub static BASE_URL_CACHED: Lazy<String> = Lazy::new(base_url);

pub fn base_url_domain() -> String {
    url::Url::parse(&BASE_URL_CACHED)
        .expect("CULTIVATORS_BASE_URL (or generated from CULTIVATORS_BIND_ADDRESS) must be a valid URL")
        .host_str()
        .expect("CULTIVATORS_BASE_URL (or generated from CULTIVATORS_BIND_ADDRESS) must have a host")
        .to_string()
}

pub fn override_enable_tls() {
    std::env::set_var("CULTIVATORS_USE_TLS", "true");
}
