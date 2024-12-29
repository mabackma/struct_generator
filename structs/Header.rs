#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "MessageType")]
    pub message_type: CoMessageType,
    #[serde(rename = "base")]
    pub base: String,
}

