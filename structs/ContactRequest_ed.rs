#[derive(Debug, Serialize, Deserialize)]
pub struct RequestInfo {
    #[serde(flatten)]
    pub request_info: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDate {
    #[serde(flatten)]
    pub create_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactLocationInformation {
    #[serde(flatten)]
    pub contact_location_information: AlternativeGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethod {
    #[serde(flatten)]
    pub preferred_contacting_method: PreferredContactingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    #[serde(flatten)]
    pub contact_information: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSet {
    #[serde(flatten)]
    pub forest_property_data_set: ForestPropertyDataSetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactMunicipality {
    #[serde(flatten)]
    pub contact_municipality: MunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCode {
    #[serde(flatten)]
    pub request_code: RequestCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpirationDate {
    #[serde(flatten)]
    pub expiration_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyData {
    #[serde(flatten)]
    pub forest_property_data: ForestPropertyDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequest {
    #[serde(flatten)]
    pub contact_request: ContactRequestType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethods {
    #[serde(flatten)]
    pub preferred_contacting_methods: PreferredContactingMethodsType,
}

