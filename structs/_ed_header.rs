#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    #[serde(flatten)]
    pub header: HeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTag {
    #[serde(flatten)]
    pub entity_tag: CoEntityTagType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(flatten)]
    pub action: CoActionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransmissionTime {
    #[serde(flatten)]
    pub transmission_time: XsdateTime,
}

