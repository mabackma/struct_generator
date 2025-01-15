use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    pub base: String,
}

