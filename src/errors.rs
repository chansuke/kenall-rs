//! Custom errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KenallError {
    #[error("Failed to get response: {0}")]
    Reqwest(#[from] reqwest::Error),
}
