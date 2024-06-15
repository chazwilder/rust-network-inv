use inventory_health::db::connections::{oracle, mongodb, mssql};
use inventory_health::models::{InventoryConfig, ManhattanWmsRow};
use tokio;

#[tokio::main]
async fn main()  {
    let sites:Vec<InventoryConfig> = mongodb::get_sites().await.unwrap();
    let sdm_data = mssql::get_sdm_wms(sites[1].clone()).await;
    // if let Some(ma_ohq_query) = sites[0].ma_ohq_query.as_deref() {
    //     let ma_data = oracle::get_ma_wms(ma_ohq_query).unwrap();
    //     for row in ma_data {
    //         println!("{:#?}", row);
    //     };
    // }
}
