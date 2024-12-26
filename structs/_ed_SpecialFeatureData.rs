#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureData {
    #[serde(flatten)]
    pub special_feature_data: SpecialFeatureDataType,
}

