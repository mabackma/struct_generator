#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: CoVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountType {
    #[serde(flatten)]
    pub base: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrataType {
    #[serde(rename = "DeadTreeStratum")]
    pub dead_tree_stratum: Vec<DeadTreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeTypeType {
    #[serde(flatten)]
    pub base: CoDeadTreeTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratumType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "DeadTreeType")]
    pub dead_tree_type: DeadTreeTypeType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(flatten)]
    pub base: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterType {
    #[serde(flatten)]
    pub base: CoDiameterType,
}

