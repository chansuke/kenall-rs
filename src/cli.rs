use std::process;

use crate::{api::fetch_address, KenallError, PostalCodeResponse};

pub fn run(code: &str) -> Result<PostalCodeResponse, KenallError> {
    let postal_code = code.replace("-", "");

    if postal_code.len() != 7 {
        eprintln!("Please enter the postcode with 7digit like following: `kenall-rs 1000000` or `kenall-rs 100-0000`");
        process::exit(1);
    }

    let result = fetch_address(&postal_code)?;

    Ok(result)
}
