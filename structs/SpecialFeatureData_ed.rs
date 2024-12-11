#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureData {
    #[serde(flatten)]
    pub special_feature_data: SpecialFeatureDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatures {
    #[serde(flatten)]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeature {
    #[serde(flatten)]
    pub special_feature: LocatedSpecialFeature1Type,
}

