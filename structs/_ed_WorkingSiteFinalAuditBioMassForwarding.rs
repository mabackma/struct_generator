#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassQualityText {
    #[serde(flatten)]
    pub bio_mass_quality_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingBiomass {
    #[serde(flatten)]
    pub remaining_biomass: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestEnergySuitable {
    #[serde(flatten)]
    pub forest_energy_suitable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingBiomassText {
    #[serde(flatten)]
    pub remaining_biomass_text: BdtString200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestEnergySuitableText {
    #[serde(flatten)]
    pub forest_energy_suitable_text: BdtString200Type,
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

