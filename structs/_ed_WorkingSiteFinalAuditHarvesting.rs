#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTreesLeft {
    #[serde(flatten)]
    pub save_trees_left: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressuresText {
    #[serde(flatten)]
    pub vehicle_path_pressures_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesFromMapSymbols {
    #[serde(flatten)]
    pub spare_trees_from_map_symbols: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighStumpsText {
    #[serde(flatten)]
    pub high_stumps_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamages {
    #[serde(flatten)]
    pub road_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTreesLeftText {
    #[serde(flatten)]
    pub save_trees_left_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageText {
    #[serde(flatten)]
    pub storage_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighStumps {
    #[serde(flatten)]
    pub high_stumps: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressures {
    #[serde(flatten)]
    pub vehicle_path_pressures: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamages {
    #[serde(flatten)]
    pub tree_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamagesText {
    #[serde(flatten)]
    pub road_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatmentText {
    #[serde(flatten)]
    pub stump_treatment_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwarderId {
    #[serde(flatten)]
    pub forwarder_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvesting {
    #[serde(flatten)]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeparateSpareTrees {
    #[serde(flatten)]
    pub separate_spare_trees: WctSpareTreesByCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StumpTreatment {
    #[serde(flatten)]
    pub stump_treatment: BdtYesNoNotNeededType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingText {
    #[serde(flatten)]
    pub pre_clearing_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamagesText {
    #[serde(flatten)]
    pub tree_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractId {
    #[serde(flatten)]
    pub purchase_contract_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterId {
    #[serde(flatten)]
    pub harvester_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearing {
    #[serde(flatten)]
    pub pre_clearing: BdtYesNoNotNeededType,
}

