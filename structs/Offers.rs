#[derive(Debug, Serialize, Deserialize)]
pub struct Offers {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "Offer", skip_serializing_if = "Option::is_none")]
    pub offer: Option<Vec<OfferType>>,
}

