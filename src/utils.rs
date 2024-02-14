use once_cell::sync::Lazy;

pub static SECRET_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "0123".repeat(16)));

const SALT: &[u8] = b"securesalt";
