#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatusType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: BdtString20Type,
    #[serde(rename = "NotificationType")]
    pub notification_type: NotificationTypeType,
    #[serde(rename = "RecipientType")]
    pub recipient_type: RecipientTypeType,
    #[serde(rename = "SenderUserId")]
    pub sender_user_id: BdtString20Type,
    #[serde(rename = "SendTimestamp")]
    pub send_timestamp: BdtTimeStampType,
    #[serde(rename = "StatusTimestamp")]
    pub status_timestamp: BdtTimeStampType,
    #[serde(rename = "StatusCode")]
    pub status_code: StatusCodeType,
    #[serde(rename = "OriginalMessage")]
    pub original_message: BdtString1000Type,
    #[serde(rename = "StatusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<BdtString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

