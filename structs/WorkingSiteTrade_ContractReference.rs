#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEndingDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSitesType {
    #[serde(rename = "ContractWorkingSiteDetails")]
    pub contract_working_site_details: Vec<ContractWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSiteDetailsType {
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: int,
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<String1500Type>,
    #[serde(rename = "AssortmentClasses")]
    pub assortment_classes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractType {
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: int,
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "ContractId")]
    pub contract_id: ContractIdType,
    #[serde(rename = "ContractBeginningDate")]
    pub contract_beginning_date: ContractBeginningDateType,
    #[serde(rename = "ContractEndingDate")]
    pub contract_ending_date: ContractEndingDateType,
    #[serde(rename = "ContractText", skip_serializing_if = "Option::is_none")]
    pub contract_text: Option<String1500Type>,
    #[serde(rename = "ContractWorkingSites", skip_serializing_if = "Option::is_none")]
    pub contract_working_sites: Option<ContractWorkingSitesType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractBeginningDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractIdType {
    #[serde(flatten)]
    pub base: string,
}

