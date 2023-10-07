use log::info;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::data::awattarpricelist::{AwattarPriceList, Datum};

pub async fn update_price_list(db_url: &str, updatelist: &AwattarPriceList) -> Result<(), surrealdb::Error> {
    let db = Surreal::new::<Ws>(db_url).await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("awattar").use_db("awattar_prices").await?;
    for datum in updatelist.data.iter() {
        match db.create(("price", datum.start_timestamp.timestamp_millis())).content(datum).await {
            Ok(thing) => {
                info!("Created thing: {:?}", thing as Option<Vec<Datum>>);
            },
            Err(e) => {
                info!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}
