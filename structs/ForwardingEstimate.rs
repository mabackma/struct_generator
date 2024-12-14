#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<WctERPIdType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Code")]
    pub code: BdtString50Type,
    #[serde(rename = "Volume")]
    pub volume: BdtPositiveInteger4digitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<BdtWorkCodeUnitType>,
    #[serde(rename = "Loads")]
    pub loads: BdtPositiveInteger3digitsType,
    #[serde(rename = "Day")]
    pub day: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimateType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: WctERPIdType,
    #[serde(rename = "StartTime")]
    pub start_time: BdtTimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

