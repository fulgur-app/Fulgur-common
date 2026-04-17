use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub public_key: Option<String>,
    pub created_at: String,
    pub expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevicesResponse {
    pub devices: Vec<DeviceResponse>,
    #[serde(default)]
    pub max_file_size_bytes: Option<u64>,
}
