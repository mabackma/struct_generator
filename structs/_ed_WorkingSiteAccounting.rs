#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistanceContinued {
    #[serde(flatten)]
    pub forest_haulage_distance_continued: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContarctorId {
    #[serde(flatten)]
    pub contarctor_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountingDate {
    #[serde(flatten)]
    pub accounting_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountPlanned {
    #[serde(flatten)]
    pub amount_planned: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolume {
    #[serde(flatten)]
    pub harvested_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountAccounted {
    #[serde(flatten)]
    pub amount_accounted: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAccounting {
    #[serde(flatten)]
    pub final_accounting: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedVolumeAccounted {
    #[serde(flatten)]
    pub harvested_volume_accounted: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountNotified {
    #[serde(flatten)]
    pub amount_notified: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolume {
    #[serde(flatten)]
    pub forwarded_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardedVolumeAccounted {
    #[serde(flatten)]
    pub forwarded_volume_accounted: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedVolume {
    #[serde(flatten)]
    pub planned_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccounting {
    #[serde(flatten)]
    pub working_site_accounting: WorkingSiteAccountingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: BdtString20Type,
}

