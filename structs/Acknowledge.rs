#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessageLanguageType {
    #[serde(rename = "@LanguageCode")]
    pub language_code: LanguageCodeType,
    #[serde(rename = "StatusMessage")]
    pub status_message: String1000Type,
    #[serde(flatten)]
    pub base: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeType {
    #[serde(rename = "ReplyTo")]
    pub reply_to: String50Type,
    #[serde(rename = "StatusCode")]
    pub status_code: PositiveInteger3digitsType,
    #[serde(rename = "StatusMessage")]
    pub status_message: String1000Type,
    #[serde(rename = "OriginalMessageType")]
    pub original_message_type: String50Type,
    #[serde(rename = "StatusMessages", skip_serializing_if = "Option::is_none")]
    pub status_messages: Option<StatusMessageLanguageType>,
}

