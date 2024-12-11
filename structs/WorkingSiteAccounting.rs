#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Name")]
    pub name: String50Type,
    #[serde(rename = "PlannedVolume")]
    pub planned_volume: Decimal3FractionDigitsType,
    #[serde(rename = "HarvestedVolume")]
    pub harvested_volume: Decimal3FractionDigitsType,
    #[serde(rename = "HarvestedVolumeAccounted")]
    pub harvested_volume_accounted: Decimal3FractionDigitsType,
    #[serde(rename = "ForwardedVolume")]
    pub forwarded_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ForwardedVolumeAccounted")]
    pub forwarded_volume_accounted: Decimal3FractionDigitsType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: PositiveInteger4digitsType,
    #[serde(rename = "ForestHaulageDistanceContinued", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance_continued: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeDataType {
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "AmountPlanned")]
    pub amount_planned: Decimal3FractionDigitsType,
    #[serde(rename = "AmountNotified")]
    pub amount_notified: Decimal3FractionDigitsType,
    #[serde(rename = "AmountAccounted")]
    pub amount_accounted: Decimal3FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccountingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContarctorId")]
    pub contarctor_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "AccountingDate")]
    pub accounting_date: TimeStampType,
    #[serde(rename = "FinalAccounting")]
    pub final_accounting: YesNoType,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeDataType>,
}

