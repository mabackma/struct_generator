#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodes {
    #[serde(flatten)]
    pub work_codes: WorkCodesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyMode {
    #[serde(flatten)]
    pub company_mode: CompanyModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroups {
    #[serde(flatten)]
    pub work_code_groups: WorkCodeGroupsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingArea {
    #[serde(flatten)]
    pub working_area: WorkingAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractId {
    #[serde(flatten)]
    pub contract_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsAllowed {
    #[serde(flatten)]
    pub sub_contractors_allowed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityDateEnd {
    #[serde(flatten)]
    pub validity_date_end: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationalRegion {
    #[serde(flatten)]
    pub operational_region: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreas {
    #[serde(flatten)]
    pub working_areas: WorkingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    #[serde(flatten)]
    pub contract: ContractType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractCode {
    #[serde(flatten)]
    pub contract_code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypes {
    #[serde(flatten)]
    pub service_types: ServiceTypesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: PolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroup {
    #[serde(flatten)]
    pub work_code_group: WorkCodeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityDateBegin {
    #[serde(flatten)]
    pub validity_date_begin: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDeviceCheckRequired {
    #[serde(flatten)]
    pub measure_device_check_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(flatten)]
    pub service_type: ServiceTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    #[serde(flatten)]
    pub contract_info: String1000Type,
}

