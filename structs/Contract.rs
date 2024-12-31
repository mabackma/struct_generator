use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidityDateEnd {
    #[serde(flatten)]
    pub validity_date_end: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingAreas {
    #[serde(flatten)]
    pub working_areas: WorkingAreasType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidityDateBegin {
    #[serde(flatten)]
    pub validity_date_begin: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractCode {
    #[serde(flatten)]
    pub contract_code: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingArea {
    #[serde(flatten)]
    pub working_area: WorkingAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractorsAllowed {
    #[serde(flatten)]
    pub sub_contractors_allowed: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInfo {
    #[serde(flatten)]
    pub contract_info: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasureDeviceCheckRequired {
    #[serde(flatten)]
    pub measure_device_check_required: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCodeGroups {
    #[serde(flatten)]
    pub work_code_groups: WorkCodeGroupsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceTypes {
    #[serde(flatten)]
    pub service_types: ServiceTypesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyMode {
    #[serde(flatten)]
    pub company_mode: CompanyModeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    #[serde(flatten)]
    pub contract: ContractType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreaType {
    #[serde(rename = "OperationalRegion")]
    pub operational_region: String50Type,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContractId")]
    pub contract_id: String20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ContractCode", skip_serializing_if = "Option::is_none")]
    pub contract_code: Option<String50Type>,
    #[serde(rename = "ValidityDateBegin")]
    pub validity_date_begin: DateType,
    #[serde(rename = "ValidityDateEnd", skip_serializing_if = "Option::is_none")]
    pub validity_date_end: Option<DateType>,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "WorkCodeGroups")]
    pub work_code_groups: WorkCodeGroupsType,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "ContractInfo", skip_serializing_if = "Option::is_none")]
    pub contract_info: Option<String1000Type>,
    #[serde(rename = "MeasureDeviceCheckRequired")]
    pub measure_device_check_required: YesNoType,
    #[serde(rename = "CompanyMode", skip_serializing_if = "Option::is_none")]
    pub company_mode: Option<CompanyModeType>,
    #[serde(rename = "SubContractorsAllowed")]
    pub sub_contractors_allowed: YesNoType,
    #[serde(rename = "WorkingAreas", skip_serializing_if = "Option::is_none")]
    pub working_areas: Option<WorkingAreasType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreasType {
    #[serde(rename = "WorkingArea", skip_serializing_if = "Option::is_none")]
    pub working_area: Option<Vec<WorkingAreaType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<Vec<WorkCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<WorkCodeGroupType>,
}

