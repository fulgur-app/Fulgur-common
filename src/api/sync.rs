use serde::{Deserialize, Serialize};

use crate::api::shares;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginResponse {
    pub device_name: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialSynchronizationPayload {
    pub public_key: String,
}
