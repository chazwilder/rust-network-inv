pub mod sdm_inventory {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serialize,Serializer};

    fn deserialize_naive_date_time<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => NaiveDateTime::parse_from_str(&s, "%Y-%m-%d").map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
    fn serialize_naive_date_time<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match date {
            Some(date) => serializer.serialize_some(&date.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => serializer.serialize_none(),
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub struct SdmWmsRow {
        #[serde(deserialize_with = "deserialize_naive_date_time", serialize_with = "serialize_naive_date_time")]
        pub(crate) log_dttm: Option<NaiveDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) whse: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) wms_system: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) ohq_location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) item_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) prod_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) item_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) lpn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) sdm_location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) lot_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) sdm_ohq: Option<i32>,
    }
}