#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SfLocatedSpecialFeature1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "SpecialFeatures")]
    pub special_features: SpecialFeaturesType,
}

