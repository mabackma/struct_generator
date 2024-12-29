#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(rename = "base")]
    pub base: String,
}

