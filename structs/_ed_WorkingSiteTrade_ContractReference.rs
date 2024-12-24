#[derive(Debug, Serialize, Deserialize)]
pub struct ContractText {
    #[serde(flatten)]
    pub contract_text: CoString1500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEndingDate {
    #[serde(flatten)]
    pub contract_ending_date: ContractEndingDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSites {
    #[serde(flatten)]
    pub contract_working_sites: ContractWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractId {
    #[serde(flatten)]
    pub contract_id: ContractIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractBeginningDate {
    #[serde(flatten)]
    pub contract_beginning_date: ContractBeginningDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    #[serde(flatten)]
    pub contract: ContractType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSiteDetails {
    #[serde(flatten)]
    pub contract_working_site_details: ContractWorkingSiteDetailsType,
}

