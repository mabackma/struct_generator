#[derive(Debug, Serialize, Deserialize)]
pub struct PublicityOrganizations {
    #[serde(flatten)]
    pub publicity_organizations: OOrganizationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWithPublicityType {
    #[serde(flatten)]
    pub call_for_offer_with_publicity_type: CallForOfferWithPublicity,
}

