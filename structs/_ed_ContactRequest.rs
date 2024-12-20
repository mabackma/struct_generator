#[derive(Debug, Serialize, Deserialize)]
pub struct RequestInfo {
    #[serde(flatten)]
    pub request_info: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCode {
    #[serde(flatten)]
    pub request_code: RequestCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactLocationInformation {
    #[serde(flatten)]
    pub contact_location_information: GdtAlternativeGeometriesType,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDate {
    #[serde(flatten)]
    pub create_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactMunicipality {
    #[serde(flatten)]
    pub contact_municipality: CoMunicipalityNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethod {
    #[serde(flatten)]
    pub preferred_contacting_method: CoPreferredContactingMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpirationDate {
    #[serde(flatten)]
    pub expiration_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSet {
    #[serde(flatten)]
    pub forest_property_data_set: ForestPropertyDataSetType,
}

