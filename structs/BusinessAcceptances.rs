#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptancesType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "BaBusinessAcceptance", skip_serializing_if = "Option::is_none")]
    pub ba_business_acceptance: Option<Vec<BusinessAcceptance>>,
}

