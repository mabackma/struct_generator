#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyData {
    #[serde(flatten)]
    pub forest_property_data: ForestPropertyDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "ReRealEstates", skip_serializing_if = "Option::is_none")]
    pub re_real_estates: Option<RealEstates>,
    #[serde(rename = "StStands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<Stands>,
}

