use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingStumpCount {
    #[serde(flatten)]
    pub remaining_stump_count: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingStumpCountText {
    #[serde(flatten)]
    pub remaining_stump_count_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingQualityText {
    #[serde(flatten)]
    pub stump_lifting_quality_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingQuality {
    #[serde(flatten)]
    pub stump_lifting_quality: BdtWorkingQualityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingSuitableText {
    #[serde(flatten)]
    pub stump_lifting_suitable_text: BdtString200Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StumpLiftingSuitable {
    #[serde(flatten)]
    pub stump_lifting_suitable: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteFinalAuditStumpLifting {
    #[serde(flatten)]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: YesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed: Option<YesNoType>,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<String200Type>,
    #[serde(rename = "LimitsToWaterSystem", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system: Option<YesNoType>,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<String200Type>,
    #[serde(rename = "WaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub water_system_protection: Option<YesNoType>,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<String200Type>,
    #[serde(rename = "WorkingSafetyNoticed", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed: Option<YesNoType>,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<String200Type>,
    #[serde(rename = "WorkingInstructionsSufficient", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient: Option<YesNoType>,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<String200Type>,
    #[serde(rename = "StumpLiftingSuitable")]
    pub stump_lifting_suitable: YesNoType,
    #[serde(rename = "StumpLiftingSuitableText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_suitable_text: Option<String200Type>,
    #[serde(rename = "StumpCuttingAsInstructed")]
    pub stump_cutting_as_instructed: YesNoType,
    #[serde(rename = "StumpCuttingAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub stump_cutting_as_instructed_text: Option<String200Type>,
    #[serde(rename = "StumpTidiness", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness: Option<YesNoType>,
    #[serde(rename = "StumpTidinessText", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness_text: Option<String200Type>,
    #[serde(rename = "RemainingStumpCount", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count: Option<YesNoType>,
    #[serde(rename = "RemainingStumpCountText", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count_text: Option<String200Type>,
    #[serde(rename = "StumpLiftingQuality", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality: Option<WorkingQualityType>,
    #[serde(rename = "StumpLiftingQualityText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLiftingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
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

