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
    #[serde(rename = "EnvironmentalObjectsNoticed", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed: Option<BdtYesNoType>,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<BdtString200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: BdtYesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<BdtString200Type>,
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
    #[serde(rename = "Storage")]
    pub storage: BdtYesNoType,
    #[serde(rename = "StorageText", skip_serializing_if = "Option::is_none")]
    pub storage_text: Option<BdtString200Type>,
    #[serde(rename = "RoadDamages", skip_serializing_if = "Option::is_none")]
    pub road_damages: Option<BdtYesNoType>,
    #[serde(rename = "RoadDamagesText", skip_serializing_if = "Option::is_none")]
    pub road_damages_text: Option<BdtString200Type>,
    #[serde(rename = "TreeDamages", skip_serializing_if = "Option::is_none")]
    pub tree_damages: Option<BdtYesNoType>,
    #[serde(rename = "TreeDamagesText", skip_serializing_if = "Option::is_none")]
    pub tree_damages_text: Option<BdtString200Type>,
    #[serde(rename = "VehiclePathPressures", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_pressures: Option<BdtYesNoType>,
    #[serde(rename = "VehiclePathPressuresText", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_pressures_text: Option<BdtString200Type>,
    #[serde(rename = "BioMassQuality")]
    pub bio_mass_quality: BdtYesNoType,
    #[serde(rename = "BioMassQualityText", skip_serializing_if = "Option::is_none")]
    pub bio_mass_quality_text: Option<BdtString200Type>,
    #[serde(rename = "ForestEnergySuitable", skip_serializing_if = "Option::is_none")]
    pub forest_energy_suitable: Option<BdtYesNoType>,
    #[serde(rename = "ForestEnergySuitableText", skip_serializing_if = "Option::is_none")]
    pub forest_energy_suitable_text: Option<BdtString200Type>,
    #[serde(rename = "RemainingBiomass", skip_serializing_if = "Option::is_none")]
    pub remaining_biomass: Option<BdtYesNoType>,
    #[serde(rename = "RemainingBiomassText", skip_serializing_if = "Option::is_none")]
    pub remaining_biomass_text: Option<BdtString200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwardingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
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

