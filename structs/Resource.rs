use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct Removed {
    #[serde(flatten)]
    pub removed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutOfServiceStartDate {
    #[serde(flatten)]
    pub out_of_service_start_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractorVerified {
    #[serde(flatten)]
    pub sub_contractor_verified: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadRating {
    #[serde(flatten)]
    pub load_rating: WctLoadRatingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrdererResponsibilityDocumentsChecked {
    #[serde(flatten)]
    pub orderer_responsibility_documents_checked: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceStartDate {
    #[serde(flatten)]
    pub service_start_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HavesterModelYear {
    #[serde(flatten)]
    pub havester_model_year: BdtYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnWeight {
    #[serde(flatten)]
    pub own_weight: BdtPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalSystemInUse {
    #[serde(flatten)]
    pub external_system_in_use: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalSystemName {
    #[serde(flatten)]
    pub external_system_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommunicationType {
    #[serde(flatten)]
    pub communication_type: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentMonth {
    #[serde(flatten)]
    pub deployment_month: BdtString5Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeripheralCode {
    #[serde(flatten)]
    pub peripheral_code: BdtPeripheralCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Productivity {
    #[serde(flatten)]
    pub productivity: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveDate {
    #[serde(flatten)]
    pub remove_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manufacturer {
    #[serde(flatten)]
    pub manufacturer: BdtMachineManufacturerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtinguisherVerificationDate {
    #[serde(flatten)]
    pub extinguisher_verification_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubContractorResource {
    #[serde(flatten)]
    pub sub_contractor_resource: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasuringDeviceLastControl {
    #[serde(flatten)]
    pub measuring_device_last_control: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentYear {
    #[serde(flatten)]
    pub deployment_year: BdtYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasuringDeviceVersion {
    #[serde(flatten)]
    pub measuring_device_version: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvesterModel {
    #[serde(flatten)]
    pub harvester_model: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelYear {
    #[serde(flatten)]
    pub model_year: BdtYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductivityUnit {
    #[serde(flatten)]
    pub productivity_unit: BdtWorkCodeUnitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutOfServiceEndDate {
    #[serde(flatten)]
    pub out_of_service_end_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peripherals {
    #[serde(flatten)]
    pub peripherals: PeripheralsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoaderScaleModel {
    #[serde(flatten)]
    pub loader_scale_model: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    #[serde(flatten)]
    pub model: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ElevatorCertificate {
    #[serde(flatten)]
    pub elevator_certificate: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingWeight {
    #[serde(flatten)]
    pub working_weight: BdtWorkingWeightType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoaderScaleModelYear {
    #[serde(flatten)]
    pub loader_scale_model_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeInfoType {
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Productivity")]
    pub productivity: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
    #[serde(rename = "ProductivityUnit")]
    pub productivity_unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceType {
    #[serde(rename = "Contractors")]
    pub contractors: ContractorsType,
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<String100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<String20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceTypeType,
    #[serde(rename = "ResourceName")]
    pub resource_name: String50Type,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "WorkCodeGroups")]
    pub work_code_groups: WorkCodeGroupsType,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<MachineManufacturerType>,
    #[serde(rename = "Model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String50Type>,
    #[serde(rename = "ModelYear", skip_serializing_if = "Option::is_none")]
    pub model_year: Option<YearType>,
    #[serde(rename = "HarvesterModel", skip_serializing_if = "Option::is_none")]
    pub harvester_model: Option<String50Type>,
    #[serde(rename = "HavesterModelYear", skip_serializing_if = "Option::is_none")]
    pub havester_model_year: Option<YearType>,
    #[serde(rename = "DeploymentYear", skip_serializing_if = "Option::is_none")]
    pub deployment_year: Option<YearType>,
    #[serde(rename = "DeploymentMonth", skip_serializing_if = "Option::is_none")]
    pub deployment_month: Option<String5Type>,
    #[serde(rename = "OwnWeight", skip_serializing_if = "Option::is_none")]
    pub own_weight: Option<PositiveInteger6digitsType>,
    #[serde(rename = "WorkingWeight", skip_serializing_if = "Option::is_none")]
    pub working_weight: Option<WorkingWeightType>,
    #[serde(rename = "CommunicationType", skip_serializing_if = "Option::is_none")]
    pub communication_type: Option<String100Type>,
    #[serde(rename = "MeasuringDeviceVersion", skip_serializing_if = "Option::is_none")]
    pub measuring_device_version: Option<String50Type>,
    #[serde(rename = "MeasuringDeviceLastControl", skip_serializing_if = "Option::is_none")]
    pub measuring_device_last_control: Option<DateType>,
    #[serde(rename = "LoaderScaleModel", skip_serializing_if = "Option::is_none")]
    pub loader_scale_model: Option<String50Type>,
    #[serde(rename = "LoaderScaleModelYear", skip_serializing_if = "Option::is_none")]
    pub loader_scale_model_year: Option<YearType>,
    #[serde(rename = "ServiceStartDate")]
    pub service_start_date: DateType,
    #[serde(rename = "OutOfServiceStartDate", skip_serializing_if = "Option::is_none")]
    pub out_of_service_start_date: Option<DateType>,
    #[serde(rename = "OutOfServiceEndDate", skip_serializing_if = "Option::is_none")]
    pub out_of_service_end_date: Option<DateType>,
    #[serde(rename = "LoadRating", skip_serializing_if = "Option::is_none")]
    pub load_rating: Option<LoadRatingType>,
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<PointGeometryType>,
    #[serde(rename = "ElevatorCertificate", skip_serializing_if = "Option::is_none")]
    pub elevator_certificate: Option<YesNoType>,
    #[serde(rename = "ExtinguisherVerificationDate", skip_serializing_if = "Option::is_none")]
    pub extinguisher_verification_date: Option<DateType>,
    #[serde(rename = "Telephone", skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String50Type>,
    #[serde(rename = "SubContractorResource", skip_serializing_if = "Option::is_none")]
    pub sub_contractor_resource: Option<YesNoType>,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: String20Type,
    #[serde(rename = "SubContractorVerified", skip_serializing_if = "Option::is_none")]
    pub sub_contractor_verified: Option<YesNoType>,
    #[serde(rename = "Removed", skip_serializing_if = "Option::is_none")]
    pub removed: Option<YesNoType>,
    #[serde(rename = "RemoveDate", skip_serializing_if = "Option::is_none")]
    pub remove_date: Option<DateType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "OrdererResponsibilityDocumentsChecked", skip_serializing_if = "Option::is_none")]
    pub orderer_responsibility_documents_checked: Option<YesNoType>,
    #[serde(rename = "Peripherals", skip_serializing_if = "Option::is_none")]
    pub peripherals: Option<PeripheralsType>,
    #[serde(rename = "ExternalSystemInUse", skip_serializing_if = "Option::is_none")]
    pub external_system_in_use: Option<YesNoType>,
    #[serde(rename = "ExternalSystemName", skip_serializing_if = "Option::is_none")]
    pub external_system_name: Option<String50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralsType {
    #[serde(rename = "PeripheralCode", skip_serializing_if = "Option::is_none")]
    pub peripheral_code: Option<Vec<PeripheralCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<WorkCodeGroupType>,
}

