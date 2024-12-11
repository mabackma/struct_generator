#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceIdType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTypeType {
    #[serde(flatten)]
    pub base: MessageTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessMessageTimeStampType {
    #[serde(flatten)]
    pub base: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInformationType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceActorType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceType {
    #[serde(rename = "@message")]
    pub message: MessageTypeType,
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "BusinessMessageTimeStamp")]
    pub business_message_time_stamp: BusinessMessageTimeStampType,
    #[serde(rename = "BusinessAcceptanceActor")]
    pub business_acceptance_actor: BusinessAcceptanceActorType,
    #[serde(rename = "BusinessAcceptanceStatus")]
    pub business_acceptance_status: BusinessAcceptanceStatusType,
    #[serde(rename = "BusinessAcceptanceId", skip_serializing_if = "Option::is_none")]
    pub business_acceptance_id: Option<BusinessAcceptanceIdType>,
    #[serde(rename = "AdditionalInformation")]
    pub additional_information: AdditionalInformationType,
    #[serde(rename = "BusinessAcceptanceDate")]
    pub business_acceptance_date: BusinessAcceptanceDateType,
}

