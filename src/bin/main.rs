use inventory_health::db::connections::{oracle, mongodb};
use inventory_health::models::{InventoryConfig, ManhattanWmsRow};
fn main()  {
    let sites:Vec<InventoryConfig> = mongodb::get_sites().unwrap();
    if let Some(ma_ohq_query) = sites[0].ma_ohq_query.as_deref() {
        let ma_data = oracle::get_ma(ma_ohq_query).unwrap();
        for row in ma_data {
            println!("{:#?}", row);
        };
    }
}
