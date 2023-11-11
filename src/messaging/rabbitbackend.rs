use amiquip::{Connection, Exchange, Publish};
use anyhow::Result;

use crate::data::awattarpricelist::AwattarPriceList;
use crate::messaging::MessageQueue;

pub struct RabbitQueue {
    connection: Connection,
    routing_key: String,
}

pub async fn init(connection_string: &str, routing_key: &str) -> Result<impl MessageQueue> {
    // init the queue as described in https://docs.rs/amiquip/latest/amiquip/
    let routing_key = routing_key.to_string();
    let connection = Connection::insecure_open(connection_string)?;

    let rabbit_queue = RabbitQueue {
        connection,
        routing_key,
    };
    Ok(rabbit_queue)
}

impl MessageQueue for RabbitQueue {
    fn send_new_data(&mut self, data: &AwattarPriceList) -> Result<()> {
        // Open a channel - None says let the library choose the channel ID.
        let channel = self.connection.open_channel(None)?;
        let exchange = Exchange::direct(&channel);

        for entry in &data.data {
            exchange.publish(Publish::new(serde_json::to_string(&entry)?.as_bytes(), &self.routing_key))?;
        }
        Ok(())

    }
}