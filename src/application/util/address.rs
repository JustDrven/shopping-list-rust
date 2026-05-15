const DEFAULT_PORT: u32 = 9090;

pub fn create_address() -> String {
    format!("0.0.0.0:{}", DEFAULT_PORT)
}