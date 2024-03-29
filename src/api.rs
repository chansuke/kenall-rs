//! Handle API connection.
//!
//! An `official` Kenall API has single endpoint that return an address from postal code.
//!
//! `GET /postalcode/:postal_code`.
//!
//! Postal codes in Japan are 7-digit numeric codes using the format NNN-NNNN and
//! `kenall-rs` accpet the codes in 2 ways.
//!
//! NNNNNNN and NNN-NNNN
//!
//! ## Example
//! ```sh
//! kenall-rs 100-0005
//! ```
//!
//! ```sh
//! kenall-rs 1000005
//! ```

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

use crate::{KenallError, PostalCodeResponse};

/// This is a current base url for kenall API.
/// Pease see the [detail](https://www.notion.so/API-47ab1a425d9e48aaad5b34b4f703c718#d371ccaa035840418ee2c35f23bb9dca)
const BASE_URL: &str = "https://api.kenall.jp/v1/postalcode";

/// User specific apikey as a token.
/// You need to [signup](https://kenall.jp/signup) to grab the key.
fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let api_key = dotenv::var("API_KEY").expect("API_KEY must be set");
    let token = format!("Token {}", api_key);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers
}

pub fn fetch_address(postal_code: &str) -> Result<PostalCodeResponse, KenallError> {
    let endpoint = format!("{}/{}", BASE_URL, postal_code);

    let client = reqwest::blocking::Client::new();
    let result = client
        .get(&endpoint)
        .headers(construct_headers())
        .send()?
        .json::<PostalCodeResponse>()?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn standard_postalcode_with_corporation() {
        let postal_code = "1008105";
        let address = r#"
        {
          "version": "2022-01-31",
          "data": [
            {
              "jisx0402": "13101",
              "old_code": "100",
              "postal_code": "1008105",
              "prefecture_kana": "",
              "city_kana": "",
              "town_kana": "",
              "town_kana_raw": "",
              "prefecture": "東京都",
              "city": "千代田区",
              "town": "大手町",
              "koaza": "",
              "kyoto_street": "",
              "building": "新大手町ビル",
              "floor": "",
              "town_partial": false,
              "town_addressed_koaza": false,
              "town_chome": false,
              "town_multi": false,
              "town_raw": "大手町",
              "corporation": {
                "name": "チッソ　株式会社",
                "name_kana": "チツソ　カブシキガイシヤ",
                "block_lot": "２丁目２－１（新大手町ビル）",
                "post_office": "銀座",
                "code_type": 0
              }
            }
          ]
        }
      "#;
        let address_object: PostalCodeResponse = serde_json::from_str(address).unwrap();
        let result = fetch_address(postal_code).unwrap();
        assert_eq!(result, address_object);
    }

    // https://twitter.com/tchov/status/1358752899522695178
    // Feb 12: Seems API fixed the problem.
    #[test]
    fn koaza_exception() {
        let postal_code = "4850801";
        let result = fetch_address(postal_code).unwrap();
        assert_eq!(result.data[0].koaza, "");
    }

    #[test]
    fn separated_block() {
        let postal_code = "0282504";
        let result = fetch_address(postal_code).unwrap();
        assert_eq!(result.data[0].koaza, "第２地割");
        assert_eq!(result.data[1].koaza, "第３地割");
        assert_eq!(result.data[2].koaza, "第４地割");
    }

    /// You can find the good example in the [demo page](https://codepen.io/kenall/pen/NWbPYda).
    /// > 京都の通り名を含む例
    #[test]
    fn include_kyoto_streetname_one() {
        let postal_code = "6048063";
        let result = fetch_address(postal_code).unwrap();
        assert_eq!(result.data[1].kyoto_street, "蛸薬師通麸屋町西入");
    }

    #[test]
    fn include_kyoto_streetname_two() {
        let postal_code = "6048012";
        let result = fetch_address(postal_code).unwrap();
        assert_eq!(result.data[1].kyoto_street, "先斗町通三条下る");
    }

    #[test]
    fn include_name_of_building_and_floor() {
        let postal_code = "1046001";
        let result = fetch_address(postal_code).unwrap();
        let building = &result.data[0].building;
        assert_eq!(building, "オフィスタワーＸ");
        let floor = &result.data[0].floor;
        assert_eq!(floor, "１階");
    }

    #[test]
    fn include_individual_corporate_number() {
        let postal_code = "1008926";
        let result = fetch_address(postal_code).unwrap();
        let cooporation = result.data[0].corporation.as_ref().unwrap();
        assert_eq!(cooporation.name, "総務省");
        assert_eq!(cooporation.code_type, 0);
        assert_eq!(cooporation.block_lot, "２丁目１－２");
    }

    /// Parse `甲、乙`
    #[test]
    fn parse_comma_delimiter() {
        let postal_code = "7614103";
        let result = fetch_address(postal_code).unwrap();
        let town_raw = &result.data[0].town_raw;
        assert_eq!(town_raw, "甲、乙（大木戸）");
    }

    #[test]
    fn parse_north_award() {
        let postal_code = "0993211";
        let result = fetch_address(postal_code).unwrap();
        let town_raw = &result.data[0].town_raw;
        assert_eq!(town_raw, "東藻琴（北１区）");
    }

    /// This area has a unique case that includes `屋敷` for historical reason
    #[test]
    fn includes_yashiki() {
        let postal_code = "4411336";
        let result = fetch_address(postal_code).unwrap();
        let town_raw = &result.data[0].town_raw;
        assert_eq!(town_raw, "富岡（○○屋敷）");
    }
}
