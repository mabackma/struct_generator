#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecreaseType {
    #[serde(flatten)]
    pub base: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberExtensionType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfoType {
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: CoInfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: CiOrganizationNameType,
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<CoIdentifierType>,
}

