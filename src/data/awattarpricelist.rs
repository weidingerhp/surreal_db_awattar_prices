// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::AwattarPriceList;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: AwattarPriceList = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct AwattarPriceList {
    pub object: String,
    pub data: Vec<Datum>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    #[serde(with = "ts_milliseconds")]
    pub start_timestamp: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub end_timestamp: DateTime<Utc>,
    pub marketprice: f64,
    pub unit: String,
}
