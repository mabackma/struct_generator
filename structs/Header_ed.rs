#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    #[serde(flatten)]
    pub priority: PriorityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransmissionTime {
    #[serde(flatten)]
    pub transmission_time: dateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(flatten)]
    pub action: ActionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTag {
    #[serde(flatten)]
    pub entity_tag: EntityTagType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    #[serde(flatten)]
    pub header: HeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

