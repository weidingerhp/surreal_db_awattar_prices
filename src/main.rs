use std::alloc::System;
use env_logger::Env;
use log::{info, warn};
use reqwest::Client;
use chrono::{DateTime, Local};

#[global_allocator]
static A: System = System;

use crate::data::awattarpricelist::AwattarPriceList;

pub mod data;
pub mod persistence;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let surrealdb_url = std::env::var("SURREALDB_URL").unwrap_or("localhost:8000".to_string());
    let surrealdb_user = std::env::var("SURREALDB_USER").unwrap_or("root".to_string());
    let surrealdb_pass = std::env::var("SURREALDB_PASS").unwrap_or("root".to_string());
    let debug_read_data = std::env::var("DEBUG_READ_DATA").unwrap_or("false".to_string()).parse::<bool>().unwrap_or(false);

    let client = Client::new();
    match client.get("https://api.awattar.de/v1/marketdata").send().await {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<AwattarPriceList>().await {
                    Ok(prices) => {
                        info!("Success fetching {} prices", prices.data.len());
                        if debug_read_data {
                            debug_print_pricelist(&prices).await;
                        }
                        persistence::update_price_list(&surrealdb_url, &surrealdb_user, &surrealdb_pass, &prices).await.unwrap();
                    },
                    Err(e) => {
                        warn!("Error: {:?}", e);
                    }
                }
            } else {
                warn!("Error: {:?}", res.status());
            }
                }
        Err(e) => {
            warn!("Error: {:?}", e);
        }
    }
}

async fn debug_print_pricelist(pricelist: &AwattarPriceList) {
    info!("object: {}", pricelist.object);
    info!("url: {}", pricelist.url);
    for datum in &pricelist.data {
        info!("from {} to {}: {} {:?}", 
            DateTime::<Local>::from(datum.start_timestamp), 
            DateTime::<Local>::from(datum.end_timestamp), 
            datum.marketprice, 
            datum.unit);
    }

}
