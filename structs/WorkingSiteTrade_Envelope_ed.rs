#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub message: WorkingSiteTradeMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelope {
    #[serde(flatten)]
    pub working_site_trade_envelope: WorkingSiteTradeEnvelopeType,
}

