use std::fmt::format;
use crate::util::constant::PORT;

pub fn create_address() -> String {
    format(format_args!("0.0.0.0:{}", PORT))
}
