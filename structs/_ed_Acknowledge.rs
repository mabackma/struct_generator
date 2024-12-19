#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessages {
    #[serde(flatten)]
    pub status_messages: StatusMessageLanguageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyTo {
    #[serde(flatten)]
    pub reply_to: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalMessageType {
    #[serde(flatten)]
    pub original_message_type: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acknowledge {
    #[serde(flatten)]
    pub acknowledge: AcknowledgeType,
}

