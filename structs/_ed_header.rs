#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    #[serde(flatten)]
    pub priority: CoPriorityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(flatten)]
    pub action: CoActionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransmissionTime {
    #[serde(flatten)]
    pub transmission_time: chrono::NaiveDateTime,
}

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
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: WctTaxNumberType,
}

