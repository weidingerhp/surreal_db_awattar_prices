use anyhow::Result;

use crate::data::awattarpricelist::AwattarPriceList;

pub mod rabbitbackend;

pub enum MessagingParams {
    Rabbit { connection_string: String, routing_key: String},
}

pub trait MessageQueue {
    fn send_new_data(&mut self, data: &AwattarPriceList) -> Result<()>;
}

pub async fn init(params: MessagingParams) -> Result<impl MessageQueue> {
    match params {
        MessagingParams::Rabbit {connection_string, routing_key} => {
            rabbitbackend::init(&connection_string, &routing_key).await
        },
    }

}