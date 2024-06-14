pub mod all_sites {
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub struct InventoryConfig {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<ObjectId>,
        pub whse: String,
        pub sdm: i32,
        pub ma: i32,
        pub ebs: i32,
        pub yms: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exclusion_skus: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exclusion_subs: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exclusion_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sdm_server: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sdm_db: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sdm_ohq_query: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ma_ohq_query: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sdm_yms_query: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ma_yms_query: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub one_party_logistics: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ebs_query: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sdm_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sdm_pass: Option<String>,
    }
}
