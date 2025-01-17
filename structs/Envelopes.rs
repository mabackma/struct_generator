use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
--pub struct Envelopes {
    #[serde(flatten)]
    pub envelopes: EnvelopesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesType {
    #[serde(rename = "Envelope")]
    pub envl_envelope: Vec<EnvlEnvelope>,
}

