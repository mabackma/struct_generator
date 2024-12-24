#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreasType {
    #[serde(rename = "WorkingArea", skip_serializing_if = "Option::is_none")]
    pub working_area: Option<Vec<WorkingAreaType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<Vec<BdtWorkCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreaType {
    #[serde(rename = "OperationalRegion")]
    pub operational_region: BdtString50Type,
    #[serde(rename = "Name")]
    pub name: BdtString100Type,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ContractId")]
    pub contract_id: BdtString20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ContractCode", skip_serializing_if = "Option::is_none")]
    pub contract_code: Option<BdtString50Type>,
    #[serde(rename = "ValidityDateBegin")]
    pub validity_date_begin: BdtDateType,
    #[serde(rename = "ValidityDateEnd", skip_serializing_if = "Option::is_none")]
    pub validity_date_end: Option<BdtDateType>,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "WorkCodeGroups")]
    pub work_code_groups: WorkCodeGroupsType,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "ContractInfo", skip_serializing_if = "Option::is_none")]
    pub contract_info: Option<BdtString1000Type>,
    #[serde(rename = "MeasureDeviceCheckRequired")]
    pub measure_device_check_required: BdtYesNoType,
    #[serde(rename = "CompanyMode", skip_serializing_if = "Option::is_none")]
    pub company_mode: Option<BdtCompanyModeType>,
    #[serde(rename = "SubContractorsAllowed")]
    pub sub_contractors_allowed: BdtYesNoType,
    #[serde(rename = "WorkingAreas", skip_serializing_if = "Option::is_none")]
    pub working_areas: Option<WorkingAreasType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<BdtServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<BdtWorkCodeGroupType>,
}

