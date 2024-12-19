#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessMessageTimeStamp {
    #[serde(flatten)]
    pub business_message_time_stamp: BusinessMessageTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptance {
    #[serde(flatten)]
    pub business_acceptance: BusinessAcceptanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceActor {
    #[serde(flatten)]
    pub business_acceptance_actor: BusinessAcceptanceActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceId {
    #[serde(flatten)]
    pub business_acceptance_id: BusinessAcceptanceIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceDate {
    #[serde(flatten)]
    pub business_acceptance_date: BusinessAcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceStatus {
    #[serde(flatten)]
    pub business_acceptance_status: CoBusinessAcceptanceStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInformation {
    #[serde(flatten)]
    pub additional_information: AdditionalInformationType,
}

