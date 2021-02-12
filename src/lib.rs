//! # kenall-rs: A command line tool of [Kenall](https://kenall.jp/).
//!
//! Kenall is a efficient service for searching japanese address from postal code and making
//! complexity simple and straightforward.
//! This is a *unofficial* [cli tool](https://github.com/chansuke/kenall-rs) of for that.
//!
//! If you want to check the detail of the API, please check the page below
//!
//! https://www.notion.so/API-47ab1a425d9e48aaad5b34b4f703c718

pub mod api;
pub mod cli;
pub mod errors;
pub mod model;
pub mod ui;

pub use crate::api::fetch_address;
pub use crate::cli::run;
pub use crate::errors::KenallError;
pub use crate::model::PostalCodeResponse;
pub use crate::ui::UI;
