#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalMessage {
    #[serde(flatten)]
    pub original_message: BdtString1000Type,
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
pub struct SenderUserId {
    #[serde(flatten)]
    pub sender_user_id: BdtString20Type,
}

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
pub struct SmsOperatorStatus {
    #[serde(flatten)]
    pub sms_operator_status: SmsOperatorStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientTypeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatusType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String20Type,
    #[serde(rename = "NotificationType")]
    pub notification_type: NotificationTypeType,
    #[serde(rename = "RecipientType")]
    pub recipient_type: RecipientTypeType,
    #[serde(rename = "SenderUserId")]
    pub sender_user_id: String20Type,
    #[serde(rename = "SendTimestamp")]
    pub send_timestamp: TimeStampType,
    #[serde(rename = "StatusTimestamp")]
    pub status_timestamp: TimeStampType,
    #[serde(rename = "StatusCode")]
    pub status_code: StatusCodeType,
    #[serde(rename = "OriginalMessage")]
    pub original_message: String1000Type,
    #[serde(rename = "StatusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCodeType {
    #[serde(rename = "base")]
    pub base: String,
}

