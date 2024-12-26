#[derive(Debug, Serialize, Deserialize)]
pub struct DrainStorageAsInstructed {
    #[serde(flatten)]
    pub drain_storage_as_instructed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentCleanlinessNoticed {
    #[serde(flatten)]
    pub environment_cleanliness_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilization {
    #[serde(flatten)]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeOrGroundDamagesText {
    #[serde(flatten)]
    pub tree_or_ground_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdromeAsInstructed {
    #[serde(flatten)]
    pub airdrome_as_instructed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeMeasuredText {
    #[serde(flatten)]
    pub fertilizer_volume_measured_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeOrGroundDamages {
    #[serde(flatten)]
    pub tree_or_ground_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemNoticedText {
    #[serde(flatten)]
    pub water_economy_system_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdromeAsInstructedText {
    #[serde(flatten)]
    pub airdrome_as_instructed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationTotalAudit {
    #[serde(flatten)]
    pub fertilization_total_audit: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationTotalAuditText {
    #[serde(flatten)]
    pub fertilization_total_audit_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainStorageAsInstructedText {
    #[serde(flatten)]
    pub drain_storage_as_instructed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentCleanlinessNoticedText {
    #[serde(flatten)]
    pub environment_cleanliness_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeOrdered {
    #[serde(flatten)]
    pub fertilizer_volume_ordered: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemNoticed {
    #[serde(flatten)]
    pub water_economy_system_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeMeasured {
    #[serde(flatten)]
    pub fertilizer_volume_measured: BdtPositiveIntegerType,
}

