#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementNS {
    #[serde(flatten)]
    pub key_element_n_s: CoString500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreReply {
    #[serde(flatten)]
    pub forest_centre_reply: ForestCentreReplyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageData {
    #[serde(flatten)]
    pub error_message_data: ErrorMessageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    #[serde(flatten)]
    pub message_type: CoMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acceptance {
    #[serde(flatten)]
    pub acceptance: CoAcceptanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyElementId {
    #[serde(flatten)]
    pub key_element_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyInfoAsText {
    #[serde(flatten)]
    pub key_info_as_text: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationId {
    #[serde(flatten)]
    pub registration_id: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCode {
    #[serde(flatten)]
    pub reply_code: CoReplyCodeType,
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
pub struct ErrorMessages {
    #[serde(flatten)]
    pub error_messages: ErrorMessagesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(flatten)]
    pub reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    #[serde(flatten)]
    pub error_message: CoString1000Type,
}

