#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWithPublicity {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CallForOffer")]
    pub call_for_offer: Vec<CallForOfferType>,
    #[serde(rename = "Publicity")]
    pub publicity: PublicityType,
    #[serde(rename = "PublicityOrganizations", skip_serializing_if = "Option::is_none")]
    pub publicity_organizations: Option<OrganizationsType>,
}
