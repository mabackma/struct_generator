#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTime {
    #[serde(flatten)]
    pub working_site_work_time: WorkingSiteWorkTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hours {
    #[serde(flatten)]
    pub hours: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTime {
    #[serde(flatten)]
    pub end_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTime {
    #[serde(flatten)]
    pub start_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sawinghours {
    #[serde(flatten)]
    pub sawinghours: SawinghoursDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minutes {
    #[serde(flatten)]
    pub minutes: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingTime {
    #[serde(flatten)]
    pub saving_time: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: BdtString200Type,
}

