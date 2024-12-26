#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvestingType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseHarvestingType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "HarvesterId", skip_serializing_if = "Option::is_none")]
    pub harvester_id: Option<BdtString20Type>,
    #[serde(rename = "ForwarderId", skip_serializing_if = "Option::is_none")]
    pub forwarder_id: Option<BdtString20Type>,
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
pub struct SelfMonitoringFinalAuditHarvestingType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseHarvestingType,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
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
pub struct WorkingSiteFinalAuditBaseHarvestingType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "HarvesterId", skip_serializing_if = "Option::is_none")]
    pub harvester_id: Option<BdtString20Type>,
    #[serde(rename = "ForwarderId", skip_serializing_if = "Option::is_none")]
    pub forwarder_id: Option<BdtString20Type>,
    #[serde(rename = "PurchaseContractId", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_id: Option<BdtString20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<BdtWorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<BdtWorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "SaveTreesLeft")]
    pub save_trees_left: BdtYesNoType,
    #[serde(rename = "SaveTreesLeftText", skip_serializing_if = "Option::is_none")]
    pub save_trees_left_text: Option<BdtString200Type>,
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
    #[serde(rename = "StumpTreatment", skip_serializing_if = "Option::is_none")]
    pub stump_treatment: Option<BdtYesNoNotNeededType>,
    #[serde(rename = "StumpTreatmentText", skip_serializing_if = "Option::is_none")]
    pub stump_treatment_text: Option<BdtString200Type>,
    #[serde(rename = "HighStumps", skip_serializing_if = "Option::is_none")]
    pub high_stumps: Option<BdtYesNoType>,
    #[serde(rename = "HighStumpsText", skip_serializing_if = "Option::is_none")]
    pub high_stumps_text: Option<BdtString200Type>,
    #[serde(rename = "Storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<BdtYesNoType>,
    #[serde(rename = "StorageText", skip_serializing_if = "Option::is_none")]
    pub storage_text: Option<BdtString200Type>,
    #[serde(rename = "RoadDamages", skip_serializing_if = "Option::is_none")]
    pub road_damages: Option<BdtYesNoType>,
    #[serde(rename = "RoadDamagesText", skip_serializing_if = "Option::is_none")]
    pub road_damages_text: Option<BdtString200Type>,
    #[serde(rename = "PreClearing", skip_serializing_if = "Option::is_none")]
    pub pre_clearing: Option<BdtYesNoNotNeededType>,
    #[serde(rename = "PreClearingText", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_text: Option<BdtString200Type>,
    #[serde(rename = "TreeDamages", skip_serializing_if = "Option::is_none")]
    pub tree_damages: Option<BdtYesNoType>,
    #[serde(rename = "TreeDamagesText", skip_serializing_if = "Option::is_none")]
    pub tree_damages_text: Option<BdtString200Type>,
    #[serde(rename = "VehiclePathPressures", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_pressures: Option<BdtYesNoType>,
    #[serde(rename = "VehiclePathPressuresText", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_pressures_text: Option<BdtString200Type>,
    #[serde(rename = "SeparateSpareTrees", skip_serializing_if = "Option::is_none")]
    pub separate_spare_trees: Option<WctSpareTreesByCategoryType>,
    #[serde(rename = "SpareTreesFromMapSymbols", skip_serializing_if = "Option::is_none")]
    pub spare_trees_from_map_symbols: Option<WctSpareTreesByCategoryType>,
}

