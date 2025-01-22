use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractWorkingSiteDetails {
    #[serde(flatten)]
    pub contract_working_site_details: ContractWorkingSiteDetailsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractText {
    #[serde(flatten)]
    pub contract_text: CoString1500Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractEndingDate {
    #[serde(flatten)]
    pub contract_ending_date: ContractEndingDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractBeginningDate {
    #[serde(flatten)]
    pub contract_beginning_date: ContractBeginningDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractWorkingSites {
    #[serde(flatten)]
    pub contract_working_sites: ContractWorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractBeginningDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSitesType {
    #[serde(rename = "ContractWorkingSiteDetails")]
    pub contract_working_site_details: Vec<ContractWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: i32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: CoTimeStamp,
    #[serde(rename = "ContractId")]
    pub contract_id: ContractIdType,
    #[serde(rename = "ContractBeginningDate")]
    pub contract_beginning_date: ContractBeginningDateType,
    #[serde(rename = "ContractEndingDate")]
    pub contract_ending_date: ContractEndingDateType,
    #[serde(rename = "ContractText", skip_serializing_if = "Option::is_none")]
    pub contract_text: Option<CoString1500Type>,
    #[serde(rename = "ContractWorkingSites", skip_serializing_if = "Option::is_none")]
    pub contract_working_sites: Option<ContractWorkingSitesType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<WtcoDocuments>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEndingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSiteDetailsType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: i32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<CoString1500Type>,
    #[serde(rename = "AssortmentClasses")]
    pub as_assortment_classes: AsAssortmentClasses,
}

