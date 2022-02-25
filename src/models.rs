//! Data structure for postal parcel record.
//!
//! You can check the data type of response [here](https://www.notion.so/API-47ab1a425d9e48aaad5b34b4f703c718#72bcd8015613468abb39567d2b620344).
//!
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct PostalBlock {
    /// JIS code
    pub jisx0402: String,
    /// Old postal code (ex: 100)
    pub old_code: String,
    /// Latest postal code.7digits.(ex: 1008105)
    pub postal_code: String,
    /// Prefectures described with full width kana (ex: トウキョウト)
    pub prefecture_kana: String,
    /// City described with full width kana (ex: チヨダク)
    pub city_kana: String,
    /// Town described with full width kana (ex: チヨダ)
    /// It has a [know issue](https://www.notion.so/a75d99e4ce864ada9ad627a6ff8558f1#6fba9308958a480cbbebefb11f332054)
    pub town_kana: String,
    /// Town on the postal code database that described by full width kana (ex: チヨダ)
    pub town_kana_raw: String,
    /// Prefecture (ex: 東京都)
    pub prefecture: String,
    /// Municipality
    pub city: String,
    /// Based on a name of the town with brackets excluded and joined multiple lines
    pub town: String,
    /// Koaza (ex: 2丁目)
    pub koaza: String,
    /// Kyoto specific street
    pub kyoto_street: String,
    /// Name of the building (ex: オフィスタワーX)
    pub building: String,
    /// Floor of the building (ex: 3階)
    pub floor: String,
    /// If the area of the town has more then 2 postal code, return `true`.
    pub town_partial: bool,
    /// Wide town area (大域) has multiple Koaza(小字) and each Koaza has specific `house number`,
    /// thus people can not specify the address from postcal code and an address.
    pub town_addressed_koaza: bool,
    /// Town area(町域) that owns district number(丁目)
    pub town_chome: bool,
    /// In some case, single postal code has multiple wotn area(町域) and return true in that case
    pub town_multi: bool,
    /// Name of the town area on postcal code database
    pub town_raw: String,
    /// If the data matches [individual corporate number](https://www.post.japanpost.jp/zipcode/dl/jigyosyo/readme.html),
    /// an additional data will be returned.
    pub corporation: Option<Corporation>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Corporation {
    /// Name of the office
    pub name: String,
    /// Name of the office with Kana
    pub name_kana: String,
    /// Block lot
    pub block_lot: String,
    pub post_office: String,
    /// Code type is desribed with `0` or `1`.
    /// 0: 大口事業所
    /// 1: 私書箱
    pub code_type: u8,
}

/// `GET /postalcode/:postal_code`
#[derive(Debug, Deserialize, PartialEq)]
pub struct PostalCodeResponse {
    /// Version of the data.
    pub version: String,
    /// Data is returned with json in array.
    pub data: Vec<PostalBlock>,
}
