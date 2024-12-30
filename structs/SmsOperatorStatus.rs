use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct SenderUserId {
    #[serde(flatten)]
    pub sender_user_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusMessage {
    #[serde(flatten)]
    pub status_message: String100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationType {
    #[serde(flatten)]
    pub notification_type: NotificationTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalMessage {
    #[serde(flatten)]
    pub original_message: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmsOperatorStatus {
    #[serde(flatten)]
    pub sms_operator_status: SmsOperatorStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipientType {
    #[serde(flatten)]
    pub recipient_type: RecipientTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusCode {
    #[serde(flatten)]
    pub status_code: StatusCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendTimestamp {
    #[serde(flatten)]
    pub send_timestamp: TimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusTimestamp {
    #[serde(flatten)]
    pub status_timestamp: TimeStampType,
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
pub struct RecipientTypeType {
    #[serde(rename = "recipient_type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCodeType {
    #[serde(rename = "status_code_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeType {
    #[serde(rename = "notification_type_type.base")]
    pub base: String,
}

