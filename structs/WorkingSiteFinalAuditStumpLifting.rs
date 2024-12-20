#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLiftingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: BdtString20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<BdtFinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: BdtFinalAuditerTypeType,
    #[serde(rename = "FinalAuditer")]
    pub final_auditer: BdtString50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: BdtTimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: BdtYesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: BdtYesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed: Option<BdtYesNoType>,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "LimitsToWaterSystem", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system: Option<BdtYesNoType>,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<BdtString200Type>,
    #[serde(rename = "WaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub water_system_protection: Option<BdtYesNoType>,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingSafetyNoticed", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed: Option<BdtYesNoType>,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingInstructionsSufficient", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient: Option<BdtYesNoType>,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<BdtString200Type>,
    #[serde(rename = "StumpLiftingSuitable")]
    pub stump_lifting_suitable: BdtYesNoType,
    #[serde(rename = "StumpLiftingSuitableText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_suitable_text: Option<BdtString200Type>,
    #[serde(rename = "StumpCuttingAsInstructed")]
    pub stump_cutting_as_instructed: BdtYesNoType,
    #[serde(rename = "StumpCuttingAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub stump_cutting_as_instructed_text: Option<BdtString200Type>,
    #[serde(rename = "StumpTidiness", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness: Option<BdtYesNoType>,
    #[serde(rename = "StumpTidinessText", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness_text: Option<BdtString200Type>,
    #[serde(rename = "RemainingStumpCount", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count: Option<BdtYesNoType>,
    #[serde(rename = "RemainingStumpCountText", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count_text: Option<BdtString200Type>,
    #[serde(rename = "StumpLiftingQuality", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality: Option<BdtWorkingQualityType>,
    #[serde(rename = "StumpLiftingQualityText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality_text: Option<BdtString200Type>,
}

