#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "Status5")]
    pub status5: ChangeStateType,
    #[serde(rename = "TreeNumber")]
    pub tree_number: string,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeDiameter", skip_serializing_if = "Option::is_none")]
    pub tree_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeHeight", skip_serializing_if = "Option::is_none")]
    pub tree_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "TreeCount", skip_serializing_if = "Option::is_none")]
    pub tree_count: Option<StemCountType>,
    #[serde(rename = "WorkQuality", skip_serializing_if = "Option::is_none")]
    pub work_quality: Option<VirtaWorkQualityType>,
    #[serde(rename = "DamageClass", skip_serializing_if = "Option::is_none")]
    pub damage_class: Option<VirtaDamageClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaDamageClassType {
    #[serde(flatten)]
    pub base: VirtaDamageClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaWorkQualityType {
    #[serde(flatten)]
    pub base: VirtaWorkQualityType,
}

