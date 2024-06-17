use futures::TryStreamExt;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use crate::models::{InventoryConfig, SdmWmsRow};
use std::borrow::Cow;



pub async fn get_sdm_wms(plant: InventoryConfig) -> Result<Vec<SdmWmsRow>, anyhow::Error> {
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
                let record = SdmWmsRow::from_row(&row)?;
                println!("{:?}", record);
                rows.push(record);
    }
    Ok(rows)
}
