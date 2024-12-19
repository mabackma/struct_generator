#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    #[serde(flatten)]
    pub envelope: EnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub message: PayloadType,
}

