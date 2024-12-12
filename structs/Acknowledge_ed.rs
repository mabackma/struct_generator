#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessage {
    #[serde(flatten)]
    pub status_message: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalMessageType {
    #[serde(flatten)]
    pub original_message_type: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acknowledge {
    #[serde(flatten)]
    pub acknowledge: AcknowledgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyTo {
    #[serde(flatten)]
    pub reply_to: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCode {
    #[serde(flatten)]
    pub status_code: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessages {
    #[serde(flatten)]
    pub status_messages: StatusMessageLanguageType,
}

