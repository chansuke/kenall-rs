//! # kenall-rs: A command line tool of [Kenall](https://kenall.jp/).
//!
//! Kenall is a efficient service for searching japanese address from postal code and making
//! complexity simple and straightforward.
//! This is an **unofficial** [cli tool](https://github.com/chansuke/kenall-rs) for the service.
//!
//! If you want to check the detail of the API, please check the [official document](https://www.notion.so/API-47ab1a425d9e48aaad5b34b4f703c718)
//!
//! # Getting Started
//!
//! First, you need to (signup)[https://kenall.jp/signup] to Kenall and get an API key.
//! Once you have an API key, please write your API key into `.env` file.
//! You can create the `.env` file by copying `.env.example` file.
//!
//! ```sh
//! $ cp example.env .env
//! $ vi .env
//! ```
//!
//! # Usage
//!
//! Please enter valid japanese post number and you will get the address.
//!
//! ```sh
//! kenall-rs 100-0001
//! ```
//!
//! or
//!
//! ```sh
//! kenall-rs 100-0001
//! ```

pub mod api;
pub mod cli;
pub mod errors;
pub mod model;
pub mod ui;

pub use crate::api::fetch_address;
pub use crate::cli::run;
pub use crate::errors::KenallError;
pub use crate::model::PostalCodeResponse;
pub use crate::ui::Ui;
