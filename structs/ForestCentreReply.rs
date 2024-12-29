#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementId {
    #[serde(flatten)]
    pub key_element_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreReply {
    #[serde(flatten)]
    pub forest_centre_reply: ForestCentreReplyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementNS {
    #[serde(flatten)]
    pub key_element_n_s: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyInfoAsText {
    #[serde(flatten)]
    pub key_info_as_text: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageData {
    #[serde(flatten)]
    pub error_message_data: ErrorMessageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationId {
    #[serde(flatten)]
    pub registration_id: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acceptance {
    #[serde(flatten)]
    pub acceptance: CoAcceptanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(flatten)]
    pub reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementName {
    #[serde(flatten)]
    pub key_element_name: CoString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorCode {
    #[serde(flatten)]
    pub error_code: CoString25Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    #[serde(flatten)]
    pub message_type: CoMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    #[serde(flatten)]
    pub error_message: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCode {
    #[serde(flatten)]
    pub reply_code: CoReplyCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessages {
    #[serde(flatten)]
    pub error_messages: ErrorMessagesType,
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
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Acceptance")]
    pub acceptance: CoAcceptanceType,
    #[serde(rename = "ReplyCode")]
    pub reply_code: CoReplyCodeType,
    #[serde(rename = "MessageType")]
    pub message_type: CoMessageType,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference: Option<CoReferenceType>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<CoDateType>,
    #[serde(rename = "RegistrationId", skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<CoReferenceType>,
    #[serde(rename = "ErrorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<ErrorMessagesType>,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessagesType {
    #[serde(rename = "ErrorMessageData")]
    pub error_message_data: Vec<ErrorMessageDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwnerType {
    #[serde(rename = "CiNameAndOrganizationGroup")]
    pub ci_name_and_organization_group: Vec<NameAndOrganizationGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageDataType {
    #[serde(rename = "ReferenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<CoForestCentreMessageReferenceType>,
    #[serde(rename = "Reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<CoReferenceType>,
    #[serde(rename = "KeyElementNS", skip_serializing_if = "Option::is_none")]
    pub key_element_n_s: Option<CoString500Type>,
    #[serde(rename = "KeyElementName", skip_serializing_if = "Option::is_none")]
    pub key_element_name: Option<CoString200Type>,
    #[serde(rename = "KeyElementId", skip_serializing_if = "Option::is_none")]
    pub key_element_id: Option<CoIdStringNotEmptyType>,
    #[serde(rename = "KeyInfoAsText", skip_serializing_if = "Option::is_none")]
    pub key_info_as_text: Option<CoString2000Type>,
    #[serde(rename = "ErrorCode")]
    pub error_code: CoString25Type,
    #[serde(rename = "ErrorMessage")]
    pub error_message: CoString1000Type,
}

