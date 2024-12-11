#[derive(Debug, Serialize, Deserialize)]
pub struct Envelopes {
    #[serde(flatten)]
    pub envelopes: EnvelopesType,
}

