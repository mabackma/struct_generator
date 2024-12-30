use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct Envelopes {
    #[serde(flatten)]
    pub envelopes: EnvelopesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesType {
    #[serde(rename = "EnvlEnvelope")]
    pub envl_envelope: Vec<Envelope>,
}

