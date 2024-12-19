#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesType {
    #[serde(rename = "Envelope")]
    pub envelope: Vec<EnvelopeType>,
}

