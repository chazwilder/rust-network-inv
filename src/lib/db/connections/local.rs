use futures::{AsyncRead, AsyncWrite, TryStreamExt};
use tiberius::{AuthMethod, Client, Config, TokenRow, ColumnData};
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncWriteCompatExt, TokioAsyncReadCompatExt};
use crate::models::{SdmWmsRow};
use dotenvy::dotenv
use std::env;




pub async fn save(plant_data: Vec<SdmWmsRow>, table: &str) -> Result<(), anyhow::Error> {
    dotenv().ok();
    let mut config = Config::new();
    config.host(env::var("INV_SERVER").unwrap());
    config.port(1433);
    config.database(env::var("INV_DB").unwrap());
    config.authentication(AuthMethod::Integrated);
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    bulk_insert_sdm_wms_rows(&mut client, plant_data, table).await?;

    Ok(())
}

async fn bulk_insert_sdm_wms_rows(
    client: &mut Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    rows: Vec<SdmWmsRow>,
    table: &str,
) -> Result<(), tiberius::error::Error> {
    let mut req = client.bulk_insert(table).await?;

    for row in rows {
        let mut token_row = TokenRow::new();
        token_row.push(ColumnData::DateTime(Some(row.log_dttm)));
        token_row.push(ColumnData::String(row.whse.map(|s| s.into())));
        token_row.push(ColumnData::String(row.wms_system.map(|s| s.into())));
        token_row.push(ColumnData::String(row.ohq_location.map(|s| s.into())));
        token_row.push(ColumnData::String(row.item_name.map(|s| s.into())));
        token_row.push(ColumnData::String(row.description.map(|s| s.into())));
        token_row.push(ColumnData::String(row.prod_type.map(|s| s.into())));
        token_row.push(ColumnData::String(row.item_type.map(|s| s.into())));
        token_row.push(ColumnData::String(row.lpn.map(|s| s.into())));
        token_row.push(ColumnData::String(row.sdm_location.map(|s| s.into())));
        token_row.push(ColumnData::String(row.lot_number.map(|s| s.into())));
        token_row.push(ColumnData::F32(row.sdm_ohq));

        req.send(token_row).await?;
    }

    let res = req.finalize().await?;
    println!("Bulk inserted {} rows", res.total());

    Ok(())
}