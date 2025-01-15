use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentCleanlinessNoticed {
    #[serde(flatten)]
    pub environment_cleanliness_noticed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentCleanlinessNoticedText {
    #[serde(flatten)]
    pub environment_cleanliness_noticed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirdromeAsInstructed {
    #[serde(flatten)]
    pub airdrome_as_instructed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterEconomySystemNoticedText {
    #[serde(flatten)]
    pub water_economy_system_noticed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeOrGroundDamages {
    #[serde(flatten)]
    pub tree_or_ground_damages: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizerVolumeMeasured {
    #[serde(flatten)]
    pub fertilizer_volume_measured: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterEconomySystemNoticed {
    #[serde(flatten)]
    pub water_economy_system_noticed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrainStorageAsInstructed {
    #[serde(flatten)]
    pub drain_storage_as_instructed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizerVolumeOrdered {
    #[serde(flatten)]
    pub fertilizer_volume_ordered: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizationTotalAuditText {
    #[serde(flatten)]
    pub fertilization_total_audit_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizerVolumeMeasuredText {
    #[serde(flatten)]
    pub fertilizer_volume_measured_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeOrGroundDamagesText {
    #[serde(flatten)]
    pub tree_or_ground_damages_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertilizationTotalAudit {
    #[serde(flatten)]
    pub fertilization_total_audit: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditFertilization {
    #[serde(flatten)]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirdromeAsInstructedText {
    #[serde(flatten)]
    pub airdrome_as_instructed_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrainStorageAsInstructedText {
    #[serde(flatten)]
    pub drain_storage_as_instructed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditerId")]
    pub final_auditer_id: String20Type,
    #[serde(rename = "FinalAuditerName")]
    pub final_auditer_name: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "FertilizationTotalAudit")]
    pub fertilization_total_audit: WorkingQualityType,
    #[serde(rename = "FertilizationTotalAuditText", skip_serializing_if = "Option::is_none")]
    pub fertilization_total_audit_text: Option<String200Type>,
    #[serde(rename = "TreeOrGroundDamages")]
    pub tree_or_ground_damages: YesNoType,
    #[serde(rename = "TreeOrGroundDamagesText", skip_serializing_if = "Option::is_none")]
    pub tree_or_ground_damages_text: Option<String200Type>,
    #[serde(rename = "EnvironmentCleanlinessNoticed")]
    pub environment_cleanliness_noticed: YesNoType,
    #[serde(rename = "EnvironmentCleanlinessNoticedText", skip_serializing_if = "Option::is_none")]
    pub environment_cleanliness_noticed_text: Option<String200Type>,
    #[serde(rename = "DrainStorageAsInstructed")]
    pub drain_storage_as_instructed: YesNoType,
    #[serde(rename = "DrainStorageAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub drain_storage_as_instructed_text: Option<String200Type>,
    #[serde(rename = "AirdromeAsInstructed", skip_serializing_if = "Option::is_none")]
    pub airdrome_as_instructed: Option<YesNoType>,
    #[serde(rename = "AirdromeAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub airdrome_as_instructed_text: Option<String200Type>,
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: YesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "NewEnvironmentalObjects")]
    pub new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectNoticed")]
    pub environmental_object_noticed: YesNoType,
    #[serde(rename = "EnvironmentalObjectNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_object_noticed_text: Option<String200Type>,
    #[serde(rename = "WaterEconomySystemNoticed")]
    pub water_economy_system_noticed: YesNoType,
    #[serde(rename = "WaterEconomySystemNoticedText", skip_serializing_if = "Option::is_none")]
    pub water_economy_system_noticed_text: Option<String200Type>,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: PositiveInteger3digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "FertilizerVolumeOrdered")]
    pub fertilizer_volume_ordered: PositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasured")]
    pub fertilizer_volume_measured: PositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasuredText", skip_serializing_if = "Option::is_none")]
    pub fertilizer_volume_measured_text: Option<String200Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

