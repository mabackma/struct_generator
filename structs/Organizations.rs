#[derive(Serialize, Deserialize)]
pub struct Services {
    #[serde(flatten)]
    pub services: ServicesType,
}

#[derive(Serialize, Deserialize)]
pub struct Role {
    #[serde(flatten)]
    pub role: OrganizationRoleType,
}

#[derive(Serialize, Deserialize)]
pub struct Roles {
    #[serde(flatten)]
    pub roles: RolesType,
}

#[derive(Serialize, Deserialize)]
pub struct Service {
    #[serde(flatten)]
    pub service: OrganizationServiceType,
}

#[derive(Serialize, Deserialize)]
pub struct Organizations {
    #[serde(flatten)]
    pub organizations: OrganizationsType,
}

#[derive(Serialize, Deserialize)]
pub struct Organization {
    #[serde(flatten)]
    pub organization: OrganizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationServiceType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "BusinessId")]
    pub business_id: String,
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
    #[serde(rename = "Roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RolesType>>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServicesType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationRoleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationsType {
    #[serde(rename = "Organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Vec<OrganizationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesType {
    #[serde(rename = "Role")]
    pub role: Vec<OrganizationRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesType {
    #[serde(rename = "Service")]
    pub service: Vec<OrganizationServiceType>,
}

