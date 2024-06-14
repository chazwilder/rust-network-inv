pub mod ma_inventory {
    use serde::{Deserialize, Deserializer};
    use chrono::NaiveDateTime;
    use oracle::Row;

    fn deserialize_naive_date_time<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub struct ManhattanWmsRow {
        #[serde(deserialize_with = "deserialize_naive_date_time")]
        log_dttm: Option<NaiveDateTime>,
        whse: Option<String>,
        wms_system: Option<String>,
        ohq_location: Option<String>,
        item_name: Option<String>,
        lpn: Option<String>,
        ma_location: Option<String>,
        description: Option<String>,
        prod_type: Option<String>,
        item_type: Option<String>,
        lot_number: Option<String>,
        man_ohq: Option<f64>,
    }
    impl ManhattanWmsRow {
        pub fn from_row(row: Row) -> Result<Self, anyhow::Error> {
            Ok(ManhattanWmsRow {
                log_dttm: parse_naive_date_time(row.get("LOG_DTTM")?)?.into(),
                whse: row.get("WHSE")?,
                wms_system: row.get("WMS_SYSTEM")?,
                ohq_location: row.get("OHQ_LOCATION")?,
                item_name: row.get("ITEM_NAME")?,
                lpn: row.get("LPN")?,
                ma_location: row.get("MA_LOCATION")?,
                description: row.get("DESCRIPTION")?,
                prod_type: row.get("PROD_TYPE")?,
                item_type: row.get("ITEM_TYPE")?,
                lot_number: row.get("LOT_NUMBER")?,
                man_ohq: row.get("MAN_OHQ")?,
            })
        }
    }
    fn parse_naive_date_time(date_string: String) -> Result<NaiveDateTime, anyhow::Error> {
        NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").map_err(|e| e.into())
    }

}

