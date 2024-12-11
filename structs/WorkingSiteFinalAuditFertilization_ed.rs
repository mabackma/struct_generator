#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerId {
    #[serde(flatten)]
    pub final_auditer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerName {
    #[serde(flatten)]
    pub final_auditer_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemNoticedText {
    #[serde(flatten)]
    pub water_economy_system_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: WorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjects {
    #[serde(flatten)]
    pub new_environmental_objects: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentCleanlinessNoticedText {
    #[serde(flatten)]
    pub environment_cleanliness_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdromeAsInstructedText {
    #[serde(flatten)]
    pub airdrome_as_instructed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainStorageAsInstructed {
    #[serde(flatten)]
    pub drain_storage_as_instructed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationTotalAuditText {
    #[serde(flatten)]
    pub fertilization_total_audit_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCountRequired {
    #[serde(flatten)]
    pub sample_plot_count_required: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeOrdered {
    #[serde(flatten)]
    pub fertilizer_volume_ordered: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentCleanlinessNoticed {
    #[serde(flatten)]
    pub environment_cleanliness_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeOrGroundDamages {
    #[serde(flatten)]
    pub tree_or_ground_damages: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticed {
    #[serde(flatten)]
    pub environmental_object_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtection {
    #[serde(flatten)]
    pub water_system_protection: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizationTotalAudit {
    #[serde(flatten)]
    pub fertilization_total_audit: WorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeMeasuredText {
    #[serde(flatten)]
    pub fertilizer_volume_measured_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdromeAsInstructed {
    #[serde(flatten)]
    pub airdrome_as_instructed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectNoticedText {
    #[serde(flatten)]
    pub environmental_object_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtectionText {
    #[serde(flatten)]
    pub water_system_protection_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: FinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemNoticed {
    #[serde(flatten)]
    pub water_economy_system_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeOrGroundDamagesText {
    #[serde(flatten)]
    pub tree_or_ground_damages_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertilizerVolumeMeasured {
    #[serde(flatten)]
    pub fertilizer_volume_measured: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: FinalAuditerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainStorageAsInstructedText {
    #[serde(flatten)]
    pub drain_storage_as_instructed_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilization {
    #[serde(flatten)]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
}

