#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamagesText {
    #[serde(flatten)]
    pub road_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTreesLeftText {
    #[serde(flatten)]
    pub save_trees_left_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterId {
    #[serde(flatten)]
    pub harvester_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamages {
    #[serde(flatten)]
    pub road_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageText {
    #[serde(flatten)]
    pub storage_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: BdtWorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditer {
    #[serde(flatten)]
    pub final_auditer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    #[serde(flatten)]
    pub audit: AuditionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwarderId {
    #[serde(flatten)]
    pub forwarder_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingText {
    #[serde(flatten)]
    pub pre_clearing_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audits {
    #[serde(flatten)]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtectionText {
    #[serde(flatten)]
    pub water_system_protection_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficient {
    #[serde(flatten)]
    pub working_instructions_sufficient: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatment {
    #[serde(flatten)]
    pub stump_treatment: BdtYesNoNotNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighStumps {
    #[serde(flatten)]
    pub high_stumps: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearing {
    #[serde(flatten)]
    pub pre_clearing: BdtYesNoNotNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjectsText {
    #[serde(flatten)]
    pub has_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTreesLeft {
    #[serde(flatten)]
    pub save_trees_left: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditRequired {
    #[serde(flatten)]
    pub final_audit_required: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: BdtWorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditerType {
    #[serde(flatten)]
    pub final_auditer_type: BdtFinalAuditerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressuresText {
    #[serde(flatten)]
    pub vehicle_path_pressures_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystemText {
    #[serde(flatten)]
    pub limits_to_water_system_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditDate {
    #[serde(flatten)]
    pub final_audit_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasFoundNewEnvironmentalObjects {
    #[serde(flatten)]
    pub has_found_new_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hasEnvironmentalObjects {
    #[serde(flatten)]
    pub has_environmental_objects: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamages {
    #[serde(flatten)]
    pub tree_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesFromMapSymbols {
    #[serde(flatten)]
    pub spare_trees_from_map_symbols: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitsToWaterSystem {
    #[serde(flatten)]
    pub limits_to_water_system: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticed {
    #[serde(flatten)]
    pub working_safety_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticedText {
    #[serde(flatten)]
    pub environmental_objects_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEnvironmentalObjectsText {
    #[serde(flatten)]
    pub new_environmental_objects_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectsNoticed {
    #[serde(flatten)]
    pub environmental_objects_noticed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatmentText {
    #[serde(flatten)]
    pub stump_treatment_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSystemProtection {
    #[serde(flatten)]
    pub water_system_protection: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingInstructionsSufficientText {
    #[serde(flatten)]
    pub working_instructions_sufficient_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvesting {
    #[serde(flatten)]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyNoticedText {
    #[serde(flatten)]
    pub working_safety_noticed_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressures {
    #[serde(flatten)]
    pub vehicle_path_pressures: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractId {
    #[serde(flatten)]
    pub purchase_contract_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighStumpsText {
    #[serde(flatten)]
    pub high_stumps_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditType {
    #[serde(flatten)]
    pub final_audit_type: BdtFinalAuditTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamagesText {
    #[serde(flatten)]
    pub tree_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeparateSpareTrees {
    #[serde(flatten)]
    pub separate_spare_trees: WctSpareTreesByCategoryType,
}

