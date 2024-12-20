#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    #[serde(flatten)]
    pub envelope: EnvelopeType,
}

