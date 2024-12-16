#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCodeExtensionsType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureAdditionalCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalFeatureCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

