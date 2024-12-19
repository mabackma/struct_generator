#[derive(Debug, Serialize, Deserialize)]
pub struct OrdererResponsibilityDocumentsChecked {
    #[serde(flatten)]
    pub orderer_responsibility_documents_checked: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElevatorCertificate {
    #[serde(flatten)]
    pub elevator_certificate: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordResourceId {
    #[serde(flatten)]
    pub stanford_resource_id: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtinguisherVerificationDate {
    #[serde(flatten)]
    pub extinguisher_verification_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Removed {
    #[serde(flatten)]
    pub removed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveDate {
    #[serde(flatten)]
    pub remove_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvesterModel {
    #[serde(flatten)]
    pub harvester_model: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnWeight {
    #[serde(flatten)]
    pub own_weight: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentYear {
    #[serde(flatten)]
    pub deployment_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStartDate {
    #[serde(flatten)]
    pub service_start_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    #[serde(flatten)]
    pub resource: ResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceId {
    #[serde(flatten)]
    pub service_buyer_resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemName {
    #[serde(flatten)]
    pub external_system_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HavesterModelYear {
    #[serde(flatten)]
    pub havester_model_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralCode {
    #[serde(flatten)]
    pub peripheral_code: PeripheralCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentMonth {
    #[serde(flatten)]
    pub deployment_month: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorResource {
    #[serde(flatten)]
    pub sub_contractor_resource: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutOfServiceEndDate {
    #[serde(flatten)]
    pub out_of_service_end_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingWeight {
    #[serde(flatten)]
    pub working_weight: WorkingWeightType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderScaleModel {
    #[serde(flatten)]
    pub loader_scale_model: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peripherals {
    #[serde(flatten)]
    pub peripherals: PeripheralsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelYear {
    #[serde(flatten)]
    pub model_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasuringDeviceLastControl {
    #[serde(flatten)]
    pub measuring_device_last_control: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manufacturer {
    #[serde(flatten)]
    pub manufacturer: MachineManufacturerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorVerified {
    #[serde(flatten)]
    pub sub_contractor_verified: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderScaleModelYear {
    #[serde(flatten)]
    pub loader_scale_model_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunicationType {
    #[serde(flatten)]
    pub communication_type: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Productivity {
    #[serde(flatten)]
    pub productivity: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasuringDeviceVersion {
    #[serde(flatten)]
    pub measuring_device_version: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutOfServiceStartDate {
    #[serde(flatten)]
    pub out_of_service_start_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemInUse {
    #[serde(flatten)]
    pub external_system_in_use: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRating {
    #[serde(flatten)]
    pub load_rating: LoadRatingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductivityUnit {
    #[serde(flatten)]
    pub productivity_unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(flatten)]
    pub model: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceName {
    #[serde(flatten)]
    pub resource_name: String50Type,
}

