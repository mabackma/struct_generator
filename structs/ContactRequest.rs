#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCodeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSetType {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: Vec<FdForestPropertyDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@source")]
    pub source: Xsstring,
    #[serde(rename = "ContactInformation")]
    pub contact_information: CiContactInformationType,
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
    #[serde(rename = "WtcoDocuments", skip_serializing_if = "Option::is_none")]
    pub wtco_documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodsType {
    #[serde(rename = "PreferredContactingMethod")]
    pub preferred_contacting_method: Vec<CoPreferredContactingMethodType>,
}

