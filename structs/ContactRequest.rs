use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPropertyDataSet {
    #[serde(flatten)]
    pub forest_property_data_set: ForestPropertyDataSetType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDate {
    #[serde(flatten)]
    pub create_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactMunicipality {
    #[serde(flatten)]
    pub contact_municipality: MunicipalityNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpirationDate {
    #[serde(flatten)]
    pub expiration_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactLocationInformation {
    #[serde(flatten)]
    pub contact_location_information: AlternativeGeometriesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCode {
    #[serde(flatten)]
    pub request_code: RequestCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreferredContactingMethods {
    #[serde(flatten)]
    pub preferred_contacting_methods: PreferredContactingMethodsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestInfo {
    #[serde(flatten)]
    pub request_info: String2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactRequest {
    #[serde(flatten)]
    pub contact_request: ContactRequestType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreferredContactingMethod {
    #[serde(flatten)]
    pub preferred_contacting_method: PreferredContactingMethodType,
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
    pub contact_municipality: MunicipalityNumberType,
    #[serde(rename = "ContactLocationInformation")]
    pub contact_location_information: AlternativeGeometriesType,
    #[serde(rename = "RequestCode")]
    pub request_code: RequestCodeType,
    #[serde(rename = "RequestInfo", skip_serializing_if = "Option::is_none")]
    pub request_info: Option<String2000Type>,
    #[serde(rename = "PreferredContactingMethods", skip_serializing_if = "Option::is_none")]
    pub preferred_contacting_methods: Option<PreferredContactingMethodsType>,
    #[serde(rename = "CreateDate")]
    pub create_date: DateType,
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateType>,
    #[serde(rename = "ForestPropertyDataSet", skip_serializing_if = "Option::is_none")]
    pub forest_property_data_set: Option<ForestPropertyDataSetType>,
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<Documents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSetType {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: Vec<ForestPropertyDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodsType {
    #[serde(rename = "PreferredContactingMethod")]
    pub preferred_contacting_method: Vec<PreferredContactingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCodeType {
    #[serde(rename = "request_code_type.base")]
    pub base: String,
}

