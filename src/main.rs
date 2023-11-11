use std::alloc::System;
use env_logger::Env;
use log::{info, warn};
use reqwest::Client;
use chrono::{DateTime, Local};

#[global_allocator]
static A: System = System;

use crate::data::awattarpricelist::AwattarPriceList;
use crate::messaging::MessagingParams;

pub mod data;
pub mod persistence;
mod messaging;
use messaging::MessageQueue;


#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let surrealdb_url = std::env::var("SURREALDB_URL").unwrap_or("localhost:8000".to_string());
    let surrealdb_user = std::env::var("SURREALDB_USER").unwrap_or("root".to_string());
    let surrealdb_pass = std::env::var("SURREALDB_PASS").unwrap_or("root".to_string());
    let debug_read_data = std::env::var("DEBUG_READ_DATA").unwrap_or("false".to_string()).parse::<bool>().unwrap_or(false);
    let awattar_api_url = std::env::var("AWATTAR_API_URL").unwrap_or("https://api.awattar.at/v1/marketdata".to_string());
    let mq_rabbit_url = std::env::var("RABBIT_URL").unwrap_or("amqp://guest:guest@rabbitmq:5672".to_string());
    let mq_rabbit_route_key = std::env::var("RABBIT_ROUTING_KEY").unwrap_or("awattar_price_data".to_string());
    let messaging = messaging::init(MessagingParams::Rabbit {
        connection_string: mq_rabbit_url,
        routing_key: mq_rabbit_route_key }).await;

    if let Err(e) = &messaging {
        warn!("Connection to MQ-Server failed: {:?}", e);
    }

    let client = Client::new();
    match client.get(awattar_api_url).send().await {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<AwattarPriceList>().await {
                    Ok(prices) => {
                        info!("Success fetching {} prices", prices.data.len());
                        if debug_read_data {
                            debug_print_pricelist(&prices).await;
                        }
                        if let Ok(mut mq) = messaging {
                            match mq.send_new_data(&prices) {
                                Ok(()) => {
                                    info!("MQ: Success sending prices");
                                },
                                Err(e) => {
                                    warn!("MQ: Error: {:?}", e);
                                }
                            }
                        }

                        match persistence::update_price_list(&surrealdb_url, &surrealdb_user, &surrealdb_pass, &prices).await {
                            Ok(_) => {
                                info!("Success updating prices");
                            },
                            Err(e) => {
                                warn!("Error: {:?}", e);
                                std::process::exit(exitcode::DATAERR);
                            }
                        }
                    },
                    Err(e) => {
                        warn!("Error: {:?}", e);
                        std::process::exit(exitcode::DATAERR);
                    }
                }
            } else {
                warn!("Error: {:?}", res.status());
                std::process::exit(exitcode::DATAERR);
            }
        }
        Err(e) => {
            warn!("Error: {:?}", e);
            std::process::exit(exitcode::PROTOCOL);
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
