#[derive(Debug, Serialize, Deserialize)]
pub struct OffersType {
    #[serde(flatten)]
    pub offers_type: Offers,
}

