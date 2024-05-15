use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserData {
    pub id: String,
    pub username: Option<String>,
    pub discriminator: Option<String>, // the old #0001
    pub avatar: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u64>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub premium_type: Option<u8>,
    pub public_flags: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PremiumTypes {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3,
}
