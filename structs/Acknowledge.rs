#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessageLanguageType {
    #[serde(rename = "@LanguageCode")]
    pub language_code: BdtLanguageCodeType,
    #[serde(rename = "StatusMessage")]
    pub status_message: BdtString1000Type,
    #[serde(flatten)]
    pub base: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeType {
    #[serde(rename = "ReplyTo")]
    pub reply_to: BdtString50Type,
    #[serde(rename = "StatusCode")]
    pub status_code: BdtPositiveInteger3digitsType,
    #[serde(rename = "StatusMessage")]
    pub status_message: BdtString1000Type,
    #[serde(rename = "OriginalMessageType")]
    pub original_message_type: BdtString50Type,
    #[serde(rename = "StatusMessages", skip_serializing_if = "Option::is_none")]
    pub status_messages: Option<StatusMessageLanguageType>,
}

