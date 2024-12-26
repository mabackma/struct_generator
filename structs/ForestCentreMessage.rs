#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: CoForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: CoForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: CoDateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Message")]
    pub message: CoMessageType,
    #[serde(rename = "SenderEmail", skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<CiEmailAddressType>,
    #[serde(rename = "ForestUseDeclaration")]
    pub forest_use_declaration: FudForestUseDeclarationType,
}

