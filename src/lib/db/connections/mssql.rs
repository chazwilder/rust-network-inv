use futures::TryStreamExt;
use tiberius::{AuthMethod, Client, Config, QueryItem,Row,FromSql};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use crate::models::{InventoryConfig, SdmWmsRow};
use std::borrow::Cow;
use chrono::{NaiveDateTime, NaiveDate};



pub async fn get_sdm_wms(plant: InventoryConfig) -> Result<(), anyhow::Error> {
    let mut config = Config::new();
    config.host(plant.sdm_server.unwrap_or_default());
    config.port(1433);
    config.authentication(AuthMethod::sql_server(plant.sdm_user.unwrap_or_default(), plant.sdm_pass.unwrap_or_default()));
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let sql: Cow<'_, str> = plant.sdm_ohq_query.unwrap_or_default().into();
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    let mut rows:Vec<SdmWmsRow> = Vec::new();
    let mut stream = client.query(sql, &[]).await?.into_first_result().await?;
    for row in stream {
        let log_dttm: Option<NaiveDateTime> = row.try_get("LOG_DTTM").ok().and_then(|date_opt: Option<i32>| {
            date_opt.map(|days_since_base| {
                NaiveDate::from_ymd_opt(1900, 1, 1).expect("REASON").and_hms_opt(0, 0, 0).expect("REASON").checked_add_signed(chrono::Duration::days(days_since_base as i64)).unwrap()
            })
        });

        let record = SdmWmsRow {
            log_dttm,
            whse: row.try_get::<&str, _>("WHSE").ok().flatten().map(|s| s.to_string()),
            wms_system: row.try_get::<&str, _>("WMS_SYSTEM").ok().flatten().map(|s| s.to_string()),
            ohq_location: row.try_get::<&str, _>("OHQ_LOCATION").ok().flatten().map(|s| s.to_string()),
            item_name: row.try_get::<&str, _>("ITEM_NAME").ok().flatten().map(|s| s.to_string()),
            description: row.try_get::<&str, _>("DESCRIPTION").ok().flatten().map(|s| s.to_string()),
            prod_type: row.try_get::<&str, _>("PROD_TYPE").ok().flatten().map(|s| s.to_string()),
            item_type: row.try_get::<&str, _>("ITEM_TYPE").ok().flatten().map(|s| s.to_string()),
            lpn: row.try_get::<&str, _>("LPN").ok().flatten().map(|s| s.to_string()),
            sdm_location: row.try_get::<&str, _>("SDM_LOCATION").ok().flatten().map(|s| s.to_string()),
            lot_number: row.try_get::<&str, _>("LOT_NUMBER").ok().flatten().map(|s| s.to_string()),
            sdm_ohq: row.try_get::<i32, _>("SDM_OHQ").ok().flatten(),
        };
        println!("{:?}", record);
        rows.push(record);
    }
    Ok(())
}
