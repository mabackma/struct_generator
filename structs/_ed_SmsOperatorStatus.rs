#[derive(Debug, Serialize, Deserialize)]
pub struct StatusTimestamp {
    #[serde(flatten)]
    pub status_timestamp: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationType {
    #[serde(flatten)]
    pub notification_type: NotificationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderUserId {
    #[serde(flatten)]
    pub sender_user_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientType {
    #[serde(flatten)]
    pub recipient_type: RecipientTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendTimestamp {
    #[serde(flatten)]
    pub send_timestamp: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatus {
    #[serde(flatten)]
    pub sms_operator_status: SmsOperatorStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalMessage {
    #[serde(flatten)]
    pub original_message: BdtString1000Type,
}

