use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusMessage {
    #[serde(flatten)]
    pub status_message: BdtString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Acknowledge {
    #[serde(flatten)]
    pub acknowledge: AcknowledgeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyTo {
    #[serde(flatten)]
    pub reply_to: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalMessageType {
    #[serde(flatten)]
    pub original_message_type: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusCode {
    #[serde(flatten)]
    pub status_code: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusMessages {
    #[serde(flatten)]
    pub status_messages: StatusMessageLanguageType,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessageLanguageType {
    #[serde(rename = "@LanguageCode")]
    pub language_code: LanguageCodeType,
    #[serde(rename = "StatusMessage")]
    pub status_message: BdtString1000Type,
    #[serde(flatten)]
    pub base: BdtString1000Type,
}

