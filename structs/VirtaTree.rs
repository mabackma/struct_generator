#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaWorkQualityType {
    #[serde(flatten)]
    pub base: CoVirtaWorkQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaDamageClassType {
    #[serde(flatten)]
    pub base: CoVirtaDamageClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "Status5")]
    pub status5: CoChangeStateType,
    #[serde(rename = "TreeNumber")]
    pub tree_number: Xsstring,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "TreeDiameter", skip_serializing_if = "Option::is_none")]
    pub tree_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeHeight", skip_serializing_if = "Option::is_none")]
    pub tree_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<CoTreeClassType>,
    #[serde(rename = "TreeCount", skip_serializing_if = "Option::is_none")]
    pub tree_count: Option<CoStemCountType>,
    #[serde(rename = "WorkQuality", skip_serializing_if = "Option::is_none")]
    pub work_quality: Option<VirtaWorkQualityType>,
    #[serde(rename = "DamageClass", skip_serializing_if = "Option::is_none")]
    pub damage_class: Option<VirtaDamageClassType>,
}

