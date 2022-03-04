use std::env;

use crate::errors::KenallError;

#[derive(Clone, Debug)]
pub struct KenallConfig {
    pub base_url: String,
    pub api_key: String,
}

/// Loading current baseurl of kenall API.
/// Pease see the [detail](https://kenall.jp/docs/API/preparation/)
pub fn load_config() -> Result<KenallConfig, KenallError> {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "https://api.kenall.jp/v1".to_string());
    let api_key = env::var("API_KEY").unwrap_or_else(|_| panic!("API_KEY must be set"));

    Ok(KenallConfig { base_url, api_key })
}
