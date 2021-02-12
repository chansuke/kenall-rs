use std::process;

use crate::{api::fetch_address, UI};

pub fn run(code: &str) {
    let postal_code = code.replace("-", "");
    let result = fetch_address(&postal_code).unwrap();
    if result.data.is_empty() {
        eprintln!("Please enter the postcode with 7digit like following: `kenall-rs 1000000` or `kenall-rs 100-0000`");
        process::exit(1);
    } else {
        UI::display_address(&result);
    }
}
