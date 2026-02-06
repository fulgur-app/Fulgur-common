use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareFilePayload {
    pub content: String,
    pub file_name: String,
    pub device_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub deduplication_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareFileResponse {
    pub message: String,
    pub expiration_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedFileResponse {
    pub id: String,
    pub source_device_id: String,
    pub file_name: String,
    pub file_size: i32,
    pub content: String, // Encrypted content (base64)
    pub created_at: String,
    pub expires_at: String,
}
