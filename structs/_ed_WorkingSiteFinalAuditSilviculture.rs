#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderId {
    #[serde(flatten)]
    pub order_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasFoundNewEnvironmentalObjects {
    #[serde(flatten)]
    pub has_found_new_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningQualityText {
    #[serde(flatten)]
    pub soil_conditioning_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureMethodCorrect {
    #[serde(flatten)]
    pub silviculture_method_correct: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: BdtWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassCollectionDoneText {
    #[serde(flatten)]
    pub bio_mass_collection_done_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantStorageManagementText {
    #[serde(flatten)]
    pub plant_storage_management_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystem {
    #[serde(flatten)]
    pub limits_to_water_system: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: BdtFinalAuditerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethod {
    #[serde(flatten)]
    pub soil_conditioning_method: BdtWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemsNoticed {
    #[serde(flatten)]
    pub water_economy_systems_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureQuality {
    #[serde(flatten)]
    pub silviculture_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: BdtWorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantStorageManagement {
    #[serde(flatten)]
    pub plant_storage_management: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilviculture {
    #[serde(flatten)]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterEconomySystemsNoticedText {
    #[serde(flatten)]
    pub water_economy_systems_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtection {
    #[serde(flatten)]
    pub water_system_protection: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalTidinessNoticedText {
    #[serde(flatten)]
    pub environmental_tidiness_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticedText {
    #[serde(flatten)]
    pub environmental_objects_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystemText {
    #[serde(flatten)]
    pub limits_to_water_system_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: BdtFinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtectionText {
    #[serde(flatten)]
    pub water_system_protection_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethodCorrect {
    #[serde(flatten)]
    pub soil_conditioning_method_correct: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalTidinessNoticed {
    #[serde(flatten)]
    pub environmental_tidiness_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCountRequired {
    #[serde(flatten)]
    pub sample_plot_count_required: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeQualifier {
    #[serde(flatten)]
    pub work_code_qualifier: BdtWorkCodeQualifierType1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningQuality {
    #[serde(flatten)]
    pub soil_conditioning_quality: BdtWorkingQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassCollectionDone {
    #[serde(flatten)]
    pub bio_mass_collection_done: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureMethodCorrectText {
    #[serde(flatten)]
    pub silviculture_method_correct_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningMethodCorrectText {
    #[serde(flatten)]
    pub soil_conditioning_method_correct_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticed {
    #[serde(flatten)]
    pub environmental_objects_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureQualityText {
    #[serde(flatten)]
    pub silviculture_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

