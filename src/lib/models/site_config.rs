pub mod all_sites {
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InventoryConfig {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub ID: Option<ObjectId>,
        pub WHSE: String,
        pub SDM: i32,
        pub MA: i32,
        pub EBS: i32,
        pub YMS: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub EXCLUSION_SKUS: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub EXCLUSION_SUBS: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub EXCLUSION_TYPES: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub SDM_SERVER: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub SDM_DB: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub SDM_OHQ_QUERY: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub MA_OHQ_QUERY: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub SDM_YMS_QUERY: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub MA_YMS_QUERY: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ONE_PARTY_LOGISTICS: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub EBS_QUERY: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub SDM_USER: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub SDM_PASS: Option<String>,
    }
}
