use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationId {
    #[serde(flatten)]
    pub registration_id: ReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyCode {
    #[serde(flatten)]
    pub reply_code: ReplyCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessages {
    #[serde(flatten)]
    pub error_messages: ErrorMessagesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyInfoAsText {
    #[serde(flatten)]
    pub key_info_as_text: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceType {
    #[serde(flatten)]
    pub reference_type: ForestCentreMessageReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyElementName {
    #[serde(flatten)]
    pub key_element_name: String200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    #[serde(flatten)]
    pub error_message: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyElementNS {
    #[serde(flatten)]
    pub key_element_n_s: String500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessageData {
    #[serde(flatten)]
    pub error_message_data: ErrorMessageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreReply {
    #[serde(flatten)]
    pub forest_centre_reply: ForestCentreReplyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorCode {
    #[serde(flatten)]
    pub error_code: String25Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyElementId {
    #[serde(flatten)]
    pub key_element_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageType {
    #[serde(flatten)]
    pub message_type: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwnerType {
    #[serde(rename = "CiNameAndOrganizationGroup")]
    pub ci_name_and_organization_group: Vec<NameAndOrganizationGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageDataType {
    #[serde(rename = "ReferenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<ForestCentreMessageReferenceType>,
    #[serde(rename = "Reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceType>,
    #[serde(rename = "KeyElementNS", skip_serializing_if = "Option::is_none")]
    pub key_element_n_s: Option<String500Type>,
    #[serde(rename = "KeyElementName", skip_serializing_if = "Option::is_none")]
    pub key_element_name: Option<String200Type>,
    #[serde(rename = "KeyElementId", skip_serializing_if = "Option::is_none")]
    pub key_element_id: Option<IdStringNotEmptyType>,
    #[serde(rename = "KeyInfoAsText", skip_serializing_if = "Option::is_none")]
    pub key_info_as_text: Option<String2000Type>,
    #[serde(rename = "ErrorCode")]
    pub error_code: String25Type,
    #[serde(rename = "ErrorMessage")]
    pub error_message: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessagesType {
    #[serde(rename = "ErrorMessageData")]
    pub error_message_data: Vec<ErrorMessageDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreReplyType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: TimeStampType,
    #[serde(rename = "Acceptance")]
    pub acceptance: AcceptanceType,
    #[serde(rename = "ReplyCode")]
    pub reply_code: ReplyCodeType,
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference: Option<ReferenceType>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<DateType>,
    #[serde(rename = "RegistrationId", skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<ReferenceType>,
    #[serde(rename = "ErrorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<ErrorMessagesType>,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String1000Type>,
}

