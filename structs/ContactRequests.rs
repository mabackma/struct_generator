#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestsType {
    #[serde(flatten)]
    pub contact_requests_type: ContactRequests,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequests {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CrContactRequest")]
    pub cr_contact_request: Vec<ContactRequest>,
}

