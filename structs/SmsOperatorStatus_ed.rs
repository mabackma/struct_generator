#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(flatten)]
    pub phone_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationType {
    #[serde(flatten)]
    pub notification_type: NotificationTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderUserId {
    #[serde(flatten)]
    pub sender_user_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendTimestamp {
    #[serde(flatten)]
    pub send_timestamp: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCode {
    #[serde(flatten)]
    pub status_code: StatusCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalMessage {
    #[serde(flatten)]
    pub original_message: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientType {
    #[serde(flatten)]
    pub recipient_type: RecipientTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusTimestamp {
    #[serde(flatten)]
    pub status_timestamp: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatus {
    #[serde(flatten)]
    pub sms_operator_status: SmsOperatorStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessage {
    #[serde(flatten)]
    pub status_message: String100Type,
}

