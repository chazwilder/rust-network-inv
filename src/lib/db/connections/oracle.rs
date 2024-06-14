use dotenvy::dotenv;
use std::env;
use oracle::Connection;
use tokio;
use crate::models::ManhattanWmsRow;

#[tokio::main]
pub async fn get_ma(query: &str) -> Result<Vec<ManhattanWmsRow>, anyhow::Error> {
    dotenv().ok();
    let username = env::var("MA_USER").unwrap();
    let password = env::var("MA_PASS").unwrap();
    let connection_string = env::var("MA_PROD").unwrap();
    let conn = Connection::connect(username, password, connection_string)?;
    let sql = query;

    let mut manhattan_wms_rows = Vec::new();

    let rows = conn.query(sql, &[])?;
    for row_result in rows {
        let row = row_result?;
        let manhattan_wms_row: ManhattanWmsRow = ManhattanWmsRow::from_row(row)?;
        manhattan_wms_rows.push(manhattan_wms_row);
    }
    Ok(manhattan_wms_rows)
}
