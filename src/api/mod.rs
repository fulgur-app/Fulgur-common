pub mod devices;
pub mod shares;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginResponse {
    pub device_name: String,
    pub encryption_key: String,
    pub shares: Vec<shares::SharedFileResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingResponse {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionKeyResponse {
    pub encryption_key: String,
}
