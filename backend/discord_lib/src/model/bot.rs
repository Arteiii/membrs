use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Guild {
    pub id: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner: Option<bool>,
    pub permissions: Option<String>,
    #[serde(rename = "permissions_new")]
    pub permissions_new: Option<String>,
    pub features: Option<Vec<String>>,
}
