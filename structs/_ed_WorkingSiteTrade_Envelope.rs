#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelope {
    #[serde(flatten)]
    pub working_site_trade_envelope: WorkingSiteTradeEnvelopeType,
}

