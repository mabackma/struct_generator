#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    #[serde(flatten)]
    pub organization: OrganizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Roles {
    #[serde(flatten)]
    pub roles: RolesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    #[serde(flatten)]
    pub role: OrganizationRoleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organizations {
    #[serde(flatten)]
    pub organizations: OrganizationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(flatten)]
    pub service: OrganizationServiceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Services {
    #[serde(flatten)]
    pub services: ServicesType,
}

