#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: BdtWorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Area")]
    pub area: BdtDecimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: BdtDateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: BdtPositiveInteger3digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "FertilizerVolumeOrdered")]
    pub fertilizer_volume_ordered: BdtPositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasured")]
    pub fertilizer_volume_measured: BdtPositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasuredText", skip_serializing_if = "Option::is_none")]
    pub fertilizer_volume_measured_text: Option<BdtString200Type>,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "FertilizationTotalAudit")]
    pub fertilization_total_audit: BdtWorkingQualityType,
    #[serde(rename = "FertilizationTotalAuditText", skip_serializing_if = "Option::is_none")]
    pub fertilization_total_audit_text: Option<BdtString200Type>,
    #[serde(rename = "TreeOrGroundDamages")]
    pub tree_or_ground_damages: BdtYesNoType,
    #[serde(rename = "TreeOrGroundDamagesText", skip_serializing_if = "Option::is_none")]
    pub tree_or_ground_damages_text: Option<BdtString200Type>,
    #[serde(rename = "EnvironmentCleanlinessNoticed")]
    pub environment_cleanliness_noticed: BdtYesNoType,
    #[serde(rename = "EnvironmentCleanlinessNoticedText", skip_serializing_if = "Option::is_none")]
    pub environment_cleanliness_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "DrainStorageAsInstructed")]
    pub drain_storage_as_instructed: BdtYesNoType,
    #[serde(rename = "DrainStorageAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub drain_storage_as_instructed_text: Option<BdtString200Type>,
    #[serde(rename = "AirdromeAsInstructed", skip_serializing_if = "Option::is_none")]
    pub airdrome_as_instructed: Option<BdtYesNoType>,
    #[serde(rename = "AirdromeAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub airdrome_as_instructed_text: Option<BdtString200Type>,
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: BdtYesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "NewEnvironmentalObjects")]
    pub new_environmental_objects: BdtYesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "EnvironmentalObjectNoticed")]
    pub environmental_object_noticed: BdtYesNoType,
    #[serde(rename = "EnvironmentalObjectNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_object_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WaterEconomySystemNoticed")]
    pub water_economy_system_noticed: BdtYesNoType,
    #[serde(rename = "WaterEconomySystemNoticedText", skip_serializing_if = "Option::is_none")]
    pub water_economy_system_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WaterSystemProtection")]
    pub water_system_protection: BdtYesNoType,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingSafetyNoticed")]
    pub working_safety_noticed: BdtYesNoType,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "WorkingInstructionsSufficient")]
    pub working_instructions_sufficient: BdtYesNoType,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<BdtFinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: BdtFinalAuditerTypeType,
    #[serde(rename = "FinalAuditerId")]
    pub final_auditer_id: BdtString20Type,
    #[serde(rename = "FinalAuditerName")]
    pub final_auditer_name: BdtString50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: BdtTimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: BdtYesNoType,
}

