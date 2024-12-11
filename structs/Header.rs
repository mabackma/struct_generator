#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(flatten)]
    pub base: string,
}

