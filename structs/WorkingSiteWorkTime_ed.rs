#[derive(Debug, Serialize, Deserialize)]
pub struct EndTime {
    #[serde(flatten)]
    pub end_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minutes {
    #[serde(flatten)]
    pub minutes: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTime {
    #[serde(flatten)]
    pub start_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTime {
    #[serde(flatten)]
    pub working_site_work_time: WorkingSiteWorkTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingTime {
    #[serde(flatten)]
    pub saving_time: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sawinghours {
    #[serde(flatten)]
    pub sawinghours: SawinghoursDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hours {
    #[serde(flatten)]
    pub hours: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

