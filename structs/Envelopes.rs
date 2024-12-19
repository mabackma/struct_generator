#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesType {
    #[serde(rename = "EnvlEnvelope")]
    pub envl_envelope: Vec<String>,
}

