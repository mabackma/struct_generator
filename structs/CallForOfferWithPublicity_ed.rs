#[derive(Debug, Serialize, Deserialize)]
pub struct Publicity {
    #[serde(flatten)]
    pub publicity: PublicityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicityOrganizations {
    #[serde(flatten)]
    pub publicity_organizations: OrganizationsType,
}

