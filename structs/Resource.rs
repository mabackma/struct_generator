#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeInfoType {
    #[serde(rename = "WorkCode")]
    pub work_code: BdtWorkCodeType,
    #[serde(rename = "Productivity")]
    pub productivity: BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType,
    #[serde(rename = "ProductivityUnit")]
    pub productivity_unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<BdtWorkCodeGroupType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceType {
    #[serde(rename = "Contractors")]
    pub contractors: WctContractorsType,
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<BdtString100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: BdtResourceTypeType,
    #[serde(rename = "ResourceName")]
    pub resource_name: BdtString50Type,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "WorkCodeGroups")]
    pub work_code_groups: WorkCodeGroupsType,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<BdtMachineManufacturerType>,
    #[serde(rename = "Model", skip_serializing_if = "Option::is_none")]
    pub model: Option<BdtString50Type>,
    #[serde(rename = "ModelYear", skip_serializing_if = "Option::is_none")]
    pub model_year: Option<BdtYearType>,
    #[serde(rename = "HarvesterModel", skip_serializing_if = "Option::is_none")]
    pub harvester_model: Option<BdtString50Type>,
    #[serde(rename = "HavesterModelYear", skip_serializing_if = "Option::is_none")]
    pub havester_model_year: Option<BdtYearType>,
    #[serde(rename = "DeploymentYear", skip_serializing_if = "Option::is_none")]
    pub deployment_year: Option<BdtYearType>,
    #[serde(rename = "DeploymentMonth", skip_serializing_if = "Option::is_none")]
    pub deployment_month: Option<BdtString5Type>,
    #[serde(rename = "OwnWeight", skip_serializing_if = "Option::is_none")]
    pub own_weight: Option<BdtPositiveInteger6digitsType>,
    #[serde(rename = "WorkingWeight", skip_serializing_if = "Option::is_none")]
    pub working_weight: Option<BdtWorkingWeightType>,
    #[serde(rename = "CommunicationType", skip_serializing_if = "Option::is_none")]
    pub communication_type: Option<BdtString100Type>,
    #[serde(rename = "MeasuringDeviceVersion", skip_serializing_if = "Option::is_none")]
    pub measuring_device_version: Option<BdtString50Type>,
    #[serde(rename = "MeasuringDeviceLastControl", skip_serializing_if = "Option::is_none")]
    pub measuring_device_last_control: Option<BdtDateType>,
    #[serde(rename = "LoaderScaleModel", skip_serializing_if = "Option::is_none")]
    pub loader_scale_model: Option<BdtString50Type>,
    #[serde(rename = "LoaderScaleModelYear", skip_serializing_if = "Option::is_none")]
    pub loader_scale_model_year: Option<BdtYearType>,
    #[serde(rename = "ServiceStartDate")]
    pub service_start_date: BdtDateType,
    #[serde(rename = "OutOfServiceStartDate", skip_serializing_if = "Option::is_none")]
    pub out_of_service_start_date: Option<BdtDateType>,
    #[serde(rename = "OutOfServiceEndDate", skip_serializing_if = "Option::is_none")]
    pub out_of_service_end_date: Option<BdtDateType>,
    #[serde(rename = "LoadRating", skip_serializing_if = "Option::is_none")]
    pub load_rating: Option<WctLoadRatingType>,
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<GdtPointGeometryType>,
    #[serde(rename = "ElevatorCertificate", skip_serializing_if = "Option::is_none")]
    pub elevator_certificate: Option<BdtYesNoType>,
    #[serde(rename = "ExtinguisherVerificationDate", skip_serializing_if = "Option::is_none")]
    pub extinguisher_verification_date: Option<BdtDateType>,
    #[serde(rename = "Telephone", skip_serializing_if = "Option::is_none")]
    pub telephone: Option<BdtString20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<BdtString50Type>,
    #[serde(rename = "SubContractorResource", skip_serializing_if = "Option::is_none")]
    pub sub_contractor_resource: Option<BdtYesNoType>,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: BdtString20Type,
    #[serde(rename = "SubContractorVerified", skip_serializing_if = "Option::is_none")]
    pub sub_contractor_verified: Option<BdtYesNoType>,
    #[serde(rename = "Removed", skip_serializing_if = "Option::is_none")]
    pub removed: Option<BdtYesNoType>,
    #[serde(rename = "RemoveDate", skip_serializing_if = "Option::is_none")]
    pub remove_date: Option<BdtDateType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString200Type>,
    #[serde(rename = "OrdererResponsibilityDocumentsChecked", skip_serializing_if = "Option::is_none")]
    pub orderer_responsibility_documents_checked: Option<BdtYesNoType>,
    #[serde(rename = "Peripherals", skip_serializing_if = "Option::is_none")]
    pub peripherals: Option<PeripheralsType>,
    #[serde(rename = "ExternalSystemInUse", skip_serializing_if = "Option::is_none")]
    pub external_system_in_use: Option<BdtYesNoType>,
    #[serde(rename = "ExternalSystemName", skip_serializing_if = "Option::is_none")]
    pub external_system_name: Option<BdtString50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralsType {
    #[serde(rename = "PeripheralCode", skip_serializing_if = "Option::is_none")]
    pub peripheral_code: Option<Vec<BdtPeripheralCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<BdtServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode")]
    pub work_code: Vec<WorkCodeInfoType>,
}

