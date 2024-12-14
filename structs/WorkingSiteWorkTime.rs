#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTimeType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "StartTime")]
    pub start_time: BdtTimeStampType,
    #[serde(rename = "EndTime")]
    pub end_time: BdtTimeStampType,
    #[serde(rename = "SavingTime")]
    pub saving_time: BdtTimeStampType,
    #[serde(rename = "Sawinghours", skip_serializing_if = "Option::is_none")]
    pub sawinghours: Option<SawinghoursDataType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawinghoursDataType {
    #[serde(rename = "Hours")]
    pub hours: BdtPositiveInteger2digitsType,
    #[serde(rename = "Minutes")]
    pub minutes: BdtPositiveInteger2digitsType,
}

