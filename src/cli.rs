use std::process;

use crate::{api::fetch_address, Ui};

pub fn run(code: &str) {
    let postal_code = code.replace("-", "");

    if postal_code.len() != 7 {
        eprintln!("Please enter the postcode with 7digit like following: `kenall-rs 1000000` or `kenall-rs 100-0000`");
        process::exit(1);
    }

    let result = fetch_address(&postal_code).unwrap();

    if result.data.is_empty() {
        eprintln!("Sorry, There was no address associated with the post code");
        process::exit(1);
    }

    Ui::display_address(&result);
}
