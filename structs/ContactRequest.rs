use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestInfo {
    #[serde(flatten)]
    pub request_info: CoString2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDate {
    #[serde(flatten)]
    pub create_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreferredContactingMethod {
    #[serde(flatten)]
    pub preferred_contacting_method: CoPreferredContactingMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactRequest {
    #[serde(flatten)]
    pub contact_request: ContactRequestType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactMunicipality {
    #[serde(flatten)]
    pub contact_municipality: CoMunicipalityNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreferredContactingMethods {
    #[serde(flatten)]
    pub preferred_contacting_methods: PreferredContactingMethodsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpirationDate {
    #[serde(flatten)]
    pub expiration_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCode {
    #[serde(flatten)]
    pub request_code: RequestCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPropertyDataSet {
    #[serde(flatten)]
    pub forest_property_data_set: ForestPropertyDataSetType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactLocationInformation {
    #[serde(flatten)]
    pub contact_location_information: GdtAlternativeGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCodeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "ContactInformation")]
    pub contact_information: ContactInformationType,
    #[serde(rename = "ContactMunicipality")]
    pub contact_municipality: CoMunicipalityNumberType,
    #[serde(rename = "ContactLocationInformation")]
    pub contact_location_information: GdtAlternativeGeometriesType,
    #[serde(rename = "RequestCode")]
    pub request_code: RequestCodeType,
    #[serde(rename = "RequestInfo", skip_serializing_if = "Option::is_none")]
    pub request_info: Option<CoString2000Type>,
    #[serde(rename = "PreferredContactingMethods", skip_serializing_if = "Option::is_none")]
    pub preferred_contacting_methods: Option<PreferredContactingMethodsType>,
    #[serde(rename = "CreateDate")]
    pub create_date: CoDateType,
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<CoDateType>,
    #[serde(rename = "ForestPropertyDataSet", skip_serializing_if = "Option::is_none")]
    pub forest_property_data_set: Option<ForestPropertyDataSetType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<WtcoDocuments>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSetType {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: Vec<ForestPropertyDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodsType {
    #[serde(rename = "PreferredContactingMethod")]
    pub preferred_contacting_method: Vec<CoPreferredContactingMethodType>,
}

