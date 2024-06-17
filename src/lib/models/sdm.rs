pub mod sdm_inventory {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    fn deserialize_naive_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let parsed_datetime = DateTime::parse_from_str(&s, "%Y-%m-%d")
                    .map_err(serde::de::Error::custom)?;
                Ok(parsed_datetime.with_timezone(&Utc))
            }
            None => Ok(Utc::now()),
        }
    }
    fn serialize_datetime<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let iso_string = datetime.to_rfc3339();
        serializer.serialize_str(&iso_string)
    }

    #[derive(Debug, Deserialize,Serialize, Clone)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub struct SdmWmsRow {
        #[serde(deserialize_with = "deserialize_naive_date_time", serialize_with = "serialize_datetime")]
        pub(crate) log_dttm: DateTime<Utc>,
        pub(crate) whse: Option<String>,
        pub(crate) wms_system: Option<String>,
        pub(crate) ohq_location: Option<String>,
        pub(crate) item_name: Option<String>,
        pub(crate) description: Option<String>,
        pub(crate) prod_type: Option<String>,
        pub(crate) item_type: Option<String>,
        pub(crate) lpn: Option<String>,
        pub(crate) sdm_location: Option<String>,
        pub(crate) lot_number: Option<String>,
        pub(crate) sdm_ohq: Option<f32>,
    }

    impl SdmWmsRow {
        pub fn from_row(row: &tiberius::Row) -> Result<Self, tiberius::error::Error> {
            Ok(SdmWmsRow {
                log_dttm: Utc::now(),
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
                sdm_ohq: row.try_get::<f32, _>("SDM_OHQ").ok().flatten(),
            })
        }
    }
}