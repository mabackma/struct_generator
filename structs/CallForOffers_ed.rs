#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOffersType {
    #[serde(flatten)]
    pub call_for_offers_type: CallForOffers,
}

