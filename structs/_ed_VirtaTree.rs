#[derive(Debug, Serialize, Deserialize)]
pub struct DamageClass {
    #[serde(flatten)]
    pub damage_class: VirtaDamageClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeHeight {
    #[serde(flatten)]
    pub tree_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDiameter {
    #[serde(flatten)]
    pub tree_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeCount {
    #[serde(flatten)]
    pub tree_count: StemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkQuality {
    #[serde(flatten)]
    pub work_quality: VirtaWorkQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status5 {
    #[serde(flatten)]
    pub status5: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubDiameter {
    #[serde(flatten)]
    pub stub_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

