#[derive(Debug, Serialize, Deserialize)]
pub struct ElevatorCertificate {
    #[serde(flatten)]
    pub elevator_certificate: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderScaleModelYear {
    #[serde(flatten)]
    pub loader_scale_model_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStartDate {
    #[serde(flatten)]
    pub service_start_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HavesterModelYear {
    #[serde(flatten)]
    pub havester_model_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveDate {
    #[serde(flatten)]
    pub remove_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrdererResponsibilityDocumentsChecked {
    #[serde(flatten)]
    pub orderer_responsibility_documents_checked: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasuringDeviceLastControl {
    #[serde(flatten)]
    pub measuring_device_last_control: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtinguisherVerificationDate {
    #[serde(flatten)]
    pub extinguisher_verification_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentYear {
    #[serde(flatten)]
    pub deployment_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterModel {
    #[serde(flatten)]
    pub harvester_model: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunicationType {
    #[serde(flatten)]
    pub communication_type: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peripherals {
    #[serde(flatten)]
    pub peripherals: PeripheralsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Removed {
    #[serde(flatten)]
    pub removed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorResource {
    #[serde(flatten)]
    pub sub_contractor_resource: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manufacturer {
    #[serde(flatten)]
    pub manufacturer: BdtMachineManufacturerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutOfServiceStartDate {
    #[serde(flatten)]
    pub out_of_service_start_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnWeight {
    #[serde(flatten)]
    pub own_weight: BdtPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingWeight {
    #[serde(flatten)]
    pub working_weight: BdtWorkingWeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductivityUnit {
    #[serde(flatten)]
    pub productivity_unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemName {
    #[serde(flatten)]
    pub external_system_name: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(flatten)]
    pub model: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderScaleModel {
    #[serde(flatten)]
    pub loader_scale_model: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRating {
    #[serde(flatten)]
    pub load_rating: WctLoadRatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorVerified {
    #[serde(flatten)]
    pub sub_contractor_verified: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Productivity {
    #[serde(flatten)]
    pub productivity: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemInUse {
    #[serde(flatten)]
    pub external_system_in_use: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasuringDeviceVersion {
    #[serde(flatten)]
    pub measuring_device_version: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutOfServiceEndDate {
    #[serde(flatten)]
    pub out_of_service_end_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentMonth {
    #[serde(flatten)]
    pub deployment_month: BdtString5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelYear {
    #[serde(flatten)]
    pub model_year: BdtYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralCode {
    #[serde(flatten)]
    pub peripheral_code: BdtPeripheralCodeType,
}

