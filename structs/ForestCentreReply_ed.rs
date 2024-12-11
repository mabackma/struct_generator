#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessages {
    #[serde(flatten)]
    pub error_messages: ErrorMessagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCode {
    #[serde(flatten)]
    pub reply_code: ReplyCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementName {
    #[serde(flatten)]
    pub key_element_name: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageReference {
    #[serde(flatten)]
    pub forest_centre_message_reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    #[serde(flatten)]
    pub error_message: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageData {
    #[serde(flatten)]
    pub error_message_data: ErrorMessageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorCode {
    #[serde(flatten)]
    pub error_code: String25Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfo {
    #[serde(flatten)]
    pub additional_info: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyInfoAsText {
    #[serde(flatten)]
    pub key_info_as_text: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationId {
    #[serde(flatten)]
    pub registration_id: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementNS {
    #[serde(flatten)]
    pub key_element_n_s: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreReply {
    #[serde(flatten)]
    pub forest_centre_reply: ForestCentreReplyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    #[serde(flatten)]
    pub message_type: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acceptance {
    #[serde(flatten)]
    pub acceptance: AcceptanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(flatten)]
    pub reference: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementId {
    #[serde(flatten)]
    pub key_element_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(flatten)]
    pub reference_type: ForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalDate {
    #[serde(flatten)]
    pub arrival_date: DateType,
}

