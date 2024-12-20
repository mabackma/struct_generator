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

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "SilvicultureMethodCorrect")]
    pub silviculture_method_correct: BdtYesNoType,
    #[serde(rename = "SilvicultureMethodCorrectText", skip_serializing_if = "Option::is_none")]
    pub silviculture_method_correct_text: Option<BdtString200Type>,
    #[serde(rename = "SoilConditioningMethod", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method: Option<BdtWorkCodeType>,
    #[serde(rename = "SoilConditioningMethodCorrect", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method_correct: Option<BdtYesNoType>,
    #[serde(rename = "SoilConditioningMethodCorrectText", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method_correct_text: Option<BdtString200Type>,
    #[serde(rename = "PlantStorageManagement")]
    pub plant_storage_management: BdtYesNoType,
    #[serde(rename = "PlantStorageManagementText", skip_serializing_if = "Option::is_none")]
    pub plant_storage_management_text: Option<BdtString200Type>,
    #[serde(rename = "SoilConditioningQuality")]
    pub soil_conditioning_quality: BdtWorkingQualityType,
    #[serde(rename = "SoilConditioningQualityText", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_quality_text: Option<BdtString200Type>,
    #[serde(rename = "WaterEconomySystemsNoticed")]
    pub water_economy_systems_noticed: BdtYesNoType,
    #[serde(rename = "WaterEconomySystemsNoticedText", skip_serializing_if = "Option::is_none")]
    pub water_economy_systems_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: BdtYesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<BdtString200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed")]
    pub environmental_objects_noticed: BdtYesNoType,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "EnvironmentalTidinessNoticed")]
    pub environmental_tidiness_noticed: BdtYesNoType,
    #[serde(rename = "EnvironmentalTidinessNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_tidiness_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "SilvicultureQuality")]
    pub silviculture_quality: BdtWorkingQualityType,
    #[serde(rename = "SilvicultureQualityText", skip_serializing_if = "Option::is_none")]
    pub silviculture_quality_text: Option<BdtString200Type>,
    #[serde(rename = "LimitsToWaterSystem")]
    pub limits_to_water_system: BdtYesNoType,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<BdtString200Type>,
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
    #[serde(rename = "BioMassCollectionDone", skip_serializing_if = "Option::is_none")]
    pub bio_mass_collection_done: Option<BdtYesNoType>,
    #[serde(rename = "BioMassCollectionDoneText", skip_serializing_if = "Option::is_none")]
    pub bio_mass_collection_done_text: Option<BdtString200Type>,
}

