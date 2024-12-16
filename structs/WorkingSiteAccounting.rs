#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: BdtString20Type,
    #[serde(rename = "Code")]
    pub code: BdtString50Type,
    #[serde(rename = "Name")]
    pub name: BdtString50Type,
    #[serde(rename = "PlannedVolume")]
    pub planned_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "HarvestedVolume")]
    pub harvested_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "HarvestedVolumeAccounted")]
    pub harvested_volume_accounted: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ForwardedVolume")]
    pub forwarded_volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ForwardedVolumeAccounted")]
    pub forwarded_volume_accounted: BdtDecimal3FractionDigitsType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: BdtPositiveInteger4digitsType,
    #[serde(rename = "ForestHaulageDistanceContinued", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance_continued: Option<BdtPositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeDataType {
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "AmountPlanned")]
    pub amount_planned: BdtDecimal3FractionDigitsType,
    #[serde(rename = "AmountNotified")]
    pub amount_notified: BdtDecimal3FractionDigitsType,
    #[serde(rename = "AmountAccounted")]
    pub amount_accounted: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccountingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ContarctorId")]
    pub contarctor_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ServiceType")]
    pub service_type: BdtServiceTypeType,
    #[serde(rename = "AccountingDate")]
    pub accounting_date: BdtTimeStampType,
    #[serde(rename = "FinalAccounting")]
    pub final_accounting: BdtYesNoType,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

