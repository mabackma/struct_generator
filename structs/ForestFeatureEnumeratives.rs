#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeExtensionsType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCodeType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTypeType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalFeatureCodeType {
    #[serde(flatten)]
    pub base: String,
}

