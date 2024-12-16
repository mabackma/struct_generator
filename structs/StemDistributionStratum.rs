#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesType {
    #[serde(flatten)]
    pub base: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrataType {
    #[serde(rename = "StemDistributionStratum")]
    pub stem_distribution_stratum: Vec<StemDistributionStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratumType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<CoStratumNumberType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: BasalAreaType,
    #[serde(rename = "CddDistributionModelGroup")]
    pub cdd_distribution_model_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaType {
    #[serde(flatten)]
    pub base: CoBasalAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeType {
    #[serde(flatten)]
    pub base: CoAgeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreyType {
    #[serde(flatten)]
    pub base: CoStoreyType,
}

