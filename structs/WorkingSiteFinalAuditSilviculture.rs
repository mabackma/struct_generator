use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantStorageManagement {
    #[serde(flatten)]
    pub plant_storage_management: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: BdtFinalAuditerTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureMethodCorrect {
    #[serde(flatten)]
    pub silviculture_method_correct: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantStorageManagementText {
    #[serde(flatten)]
    pub plant_storage_management_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalTidinessNoticedText {
    #[serde(flatten)]
    pub environmental_tidiness_noticed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterEconomySystemsNoticedText {
    #[serde(flatten)]
    pub water_economy_systems_noticed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: BdtFinalAuditTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeQualifier {
    #[serde(flatten)]
    pub work_code_qualifier: BdtWorkCodeQualifierType1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotCountRequired {
    #[serde(flatten)]
    pub sample_plot_count_required: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningQuality {
    #[serde(flatten)]
    pub soil_conditioning_quality: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitsToWaterSystemText {
    #[serde(flatten)]
    pub limits_to_water_system_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningMethod {
    #[serde(flatten)]
    pub soil_conditioning_method: BdtWorkCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterSystemProtectionText {
    #[serde(flatten)]
    pub water_system_protection_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditSilviculture {
    #[serde(flatten)]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotCount {
    #[serde(flatten)]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectsNoticedText {
    #[serde(flatten)]
    pub environmental_objects_noticed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    #[serde(flatten)]
    pub images: BdtPositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct hasFoundNewEnvironmentalObjects {
    #[serde(flatten)]
    pub has_found_new_environmental_objects: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BioMassCollectionDone {
    #[serde(flatten)]
    pub bio_mass_collection_done: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureQualityText {
    #[serde(flatten)]
    pub silviculture_quality_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: BdtWorkCodeGroupType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningQualityText {
    #[serde(flatten)]
    pub soil_conditioning_quality_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureMethodCorrectText {
    #[serde(flatten)]
    pub silviculture_method_correct_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalObjectsNoticed {
    #[serde(flatten)]
    pub environmental_objects_noticed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningMethodCorrectText {
    #[serde(flatten)]
    pub soil_conditioning_method_correct_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTime {
    #[serde(flatten)]
    pub working_time: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderId {
    #[serde(flatten)]
    pub order_id: WctERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SilvicultureQuality {
    #[serde(flatten)]
    pub silviculture_quality: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BioMassCollectionDoneText {
    #[serde(flatten)]
    pub bio_mass_collection_done_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterSystemProtection {
    #[serde(flatten)]
    pub water_system_protection: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterEconomySystemsNoticed {
    #[serde(flatten)]
    pub water_economy_systems_noticed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilConditioningMethodCorrect {
    #[serde(flatten)]
    pub soil_conditioning_method_correct: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitsToWaterSystem {
    #[serde(flatten)]
    pub limits_to_water_system: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentalTidinessNoticed {
    #[serde(flatten)]
    pub environmental_tidiness_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSilvicultureBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilvicultureBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<WctERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditer")]
    pub final_auditer: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "SilvicultureMethodCorrect")]
    pub silviculture_method_correct: YesNoType,
    #[serde(rename = "SilvicultureMethodCorrectText", skip_serializing_if = "Option::is_none")]
    pub silviculture_method_correct_text: Option<String200Type>,
    #[serde(rename = "SoilConditioningMethod", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method: Option<WorkCodeType>,
    #[serde(rename = "SoilConditioningMethodCorrect", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method_correct: Option<YesNoType>,
    #[serde(rename = "SoilConditioningMethodCorrectText", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method_correct_text: Option<String200Type>,
    #[serde(rename = "PlantStorageManagement")]
    pub plant_storage_management: YesNoType,
    #[serde(rename = "PlantStorageManagementText", skip_serializing_if = "Option::is_none")]
    pub plant_storage_management_text: Option<String200Type>,
    #[serde(rename = "SoilConditioningQuality")]
    pub soil_conditioning_quality: WorkingQualityType,
    #[serde(rename = "SoilConditioningQualityText", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_quality_text: Option<String200Type>,
    #[serde(rename = "WaterEconomySystemsNoticed")]
    pub water_economy_systems_noticed: YesNoType,
    #[serde(rename = "WaterEconomySystemsNoticedText", skip_serializing_if = "Option::is_none")]
    pub water_economy_systems_noticed_text: Option<String200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed")]
    pub environmental_objects_noticed: YesNoType,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalTidinessNoticed")]
    pub environmental_tidiness_noticed: YesNoType,
    #[serde(rename = "EnvironmentalTidinessNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_tidiness_noticed_text: Option<String200Type>,
    #[serde(rename = "SilvicultureQuality")]
    pub silviculture_quality: WorkingQualityType,
    #[serde(rename = "SilvicultureQualityText", skip_serializing_if = "Option::is_none")]
    pub silviculture_quality_text: Option<String200Type>,
    #[serde(rename = "LimitsToWaterSystem")]
    pub limits_to_water_system: YesNoType,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<String200Type>,
    #[serde(rename = "WaterSystemProtection")]
    pub water_system_protection: YesNoType,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<String200Type>,
    #[serde(rename = "WorkingSafetyNoticed")]
    pub working_safety_noticed: YesNoType,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<String200Type>,
    #[serde(rename = "WorkingInstructionsSufficient")]
    pub working_instructions_sufficient: YesNoType,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<String200Type>,
    #[serde(rename = "BioMassCollectionDone", skip_serializing_if = "Option::is_none")]
    pub bio_mass_collection_done: Option<YesNoType>,
    #[serde(rename = "BioMassCollectionDoneText", skip_serializing_if = "Option::is_none")]
    pub bio_mass_collection_done_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditSilvicultureBaseType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<BdtWorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<BdtDateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

