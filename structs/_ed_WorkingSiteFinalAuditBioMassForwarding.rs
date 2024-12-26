#[derive(Debug, Serialize, Deserialize)]
pub struct ForestEnergySuitable {
    #[serde(flatten)]
    pub forest_energy_suitable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingBiomass {
    #[serde(flatten)]
    pub remaining_biomass: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamagesText {
    #[serde(flatten)]
    pub road_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadDamages {
    #[serde(flatten)]
    pub road_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingBiomassText {
    #[serde(flatten)]
    pub remaining_biomass_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassQualityText {
    #[serde(flatten)]
    pub bio_mass_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamagesText {
    #[serde(flatten)]
    pub tree_damages_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassQuality {
    #[serde(flatten)]
    pub bio_mass_quality: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageText {
    #[serde(flatten)]
    pub storage_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamages {
    #[serde(flatten)]
    pub tree_damages: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressuresText {
    #[serde(flatten)]
    pub vehicle_path_pressures_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathPressures {
    #[serde(flatten)]
    pub vehicle_path_pressures: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestEnergySuitableText {
    #[serde(flatten)]
    pub forest_energy_suitable_text: BdtString200Type,
}

