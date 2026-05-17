use crate::application;

pub fn create_address() -> String {
    format!("0.0.0.0:{}", application::util::constants::DEFAULT_PORT)
}



