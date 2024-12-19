#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingBiomass {
    #[serde(flatten)]
    pub remaining_biomass: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwarding {
    #[serde(flatten)]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestEnergySuitable {
    #[serde(flatten)]
    pub forest_energy_suitable: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassQuality {
    #[serde(flatten)]
    pub bio_mass_quality: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestEnergySuitableText {
    #[serde(flatten)]
    pub forest_energy_suitable_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioMassQualityText {
    #[serde(flatten)]
    pub bio_mass_quality_text: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingBiomassText {
    #[serde(flatten)]
    pub remaining_biomass_text: String200Type,
}

