#[derive(Debug, Serialize, Deserialize)]
pub struct ContractCode {
    #[serde(flatten)]
    pub contract_code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityDateEnd {
    #[serde(flatten)]
    pub validity_date_end: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreas {
    #[serde(flatten)]
    pub working_areas: WorkingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDeviceCheckRequired {
    #[serde(flatten)]
    pub measure_device_check_required: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingArea {
    #[serde(flatten)]
    pub working_area: WorkingAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyMode {
    #[serde(flatten)]
    pub company_mode: CompanyModeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityDateBegin {
    #[serde(flatten)]
    pub validity_date_begin: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsAllowed {
    #[serde(flatten)]
    pub sub_contractors_allowed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    #[serde(flatten)]
    pub contract_info: String1000Type,
}

