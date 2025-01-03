#[derive(Debug, Serialize, Deserialize)]
pub struct SawinghoursDataType {
    #[serde(rename = "Hours")]
    pub hours: PositiveInteger2digitsType,
    #[serde(rename = "Minutes")]
    pub minutes: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTimeType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "StartTime")]
    pub start_time: TimeStampType,
    #[serde(rename = "EndTime")]
    pub end_time: TimeStampType,
    #[serde(rename = "SavingTime")]
    pub saving_time: TimeStampType,
    #[serde(rename = "Sawinghours", skip_serializing_if = "Option::is_none")]
    pub sawinghours: Option<SawinghoursDataType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
}

