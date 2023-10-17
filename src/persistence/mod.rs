use anyhow::{anyhow, Result};
use log::{info, warn};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::data::awattarpricelist::{AwattarPriceList, Datum};

pub async fn update_price_list(db_url: &str, db_user: &str, db_pass: &str, updatelist: &AwattarPriceList) -> Result<()> {
    let db = Surreal::new::<Ws>(db_url).await?;

    db.signin(Root {
        username: db_user,
        password: db_pass,
    }).await?;

    db.use_ns("awattar").use_db("awattar_prices").await?;

    let mut errors :Vec<surrealdb::Error> = Vec::new();
    let mut processed_items:u32 = 0;

    for datum in updatelist.data.iter() {
        processed_items = processed_items + 1;
        match db.create(("price", datum.start_timestamp.timestamp_millis())).content(datum).await {
            Ok(thing) => {
                info!("Created thing: {:?}", thing as Option<Vec<Datum>>);
            },
            Err(e) => {
                warn!("Could not insert {:?}", datum);
                warn!("Error: {:?}", e);
                errors.push(e);
            }
        }
    }

    if errors.len() > 0 {
        Err(anyhow!(format!("{} errors while inserting {} items", errors.len(), processed_items)))
    } else {
        Ok(())
    }
}
