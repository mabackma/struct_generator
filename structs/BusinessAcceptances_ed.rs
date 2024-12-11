#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptances {
    #[serde(flatten)]
    pub business_acceptances: BusinessAcceptancesType,
}

