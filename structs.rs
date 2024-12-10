
pub struct FinancingActRealEstateType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "SubsidyApplierReferenceList")]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

pub struct PayeeAndRealEstateType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: String,
    #[serde(rename = "PayeeId")]
    pub payee_id: String,
    #[serde(rename = "ParticipationPercentage", skip_serializing_if = "Option::is_none")]
    pub participation_percentage: Option<String>,
}

pub struct ApplicationActorType {
}

pub struct FinancingActRealEstatesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActRealEstate")]
    pub financing_act_real_estate: Vec<String>,
}

pub struct FinancingActCompletionOtherSubjectsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostTypeAndCompletedWorkCompletionRealEstate")]
    pub cost_type_and_completed_work_completion_real_estate: String,
    #[serde(rename = "FinancingActCompletionGeometries")]
    pub financing_act_completion_geometries: String,
}

pub struct CostTypeAndCompletedWorkApplicationRealEstateType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: String,
    #[serde(rename = "CostType")]
    pub cost_type: String,
    #[serde(rename = "PlannedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub planned_work_amount: Option<String>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: String,
}

pub struct FinancingActCompletionGeometriesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActCompletionGeometry")]
    pub financing_act_completion_geometry: Vec<String>,
}

pub struct PayeesAndRealEstatesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "PayeeAndRealEstate")]
    pub payee_and_real_estate: Vec<String>,
}

pub struct FinancingActApplicationStandsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActApplicationStand")]
    pub financing_act_application_stand: Vec<String>,
}

pub struct PayeeType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
}

pub struct CostTypeAndCompletedWorkCompletionRealEstateType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: String,
    #[serde(rename = "CostType")]
    pub cost_type: String,
    #[serde(rename = "CompletedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub completed_work_amount: Option<String>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
    #[serde(rename = "PaymentReference", skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<String>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: String,
}

pub struct CompletionDeclarationActorsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CompletionDeclarationActor")]
    pub completion_declaration_actor: Vec<PayeeType>,
}

pub struct ApplicationActorsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "ApplicationActor")]
    pub application_actor: Vec<ApplicationActorType>,
}

pub struct SubsidyAppliersType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "SubsidyApplier")]
    pub subsidy_applier: Vec<PayeeType>,
}

pub struct FinancingActApplicationOtherSubjectsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostTypeAndCompletedWorkApplicationRealEstate")]
    pub cost_type_and_completed_work_application_real_estate: String,
    #[serde(rename = "FinancingActApplicationGeometries")]
    pub financing_act_application_geometries: String,
}

pub struct OwnerShipTypeType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "OwnerShipTypeCode")]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String500Type>,
}

pub struct FinancingActApplicationGeometriesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActApplicationGeometry")]
    pub financing_act_application_geometry: Vec<String>,
}

pub struct AttorneyType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: String,
    #[serde(rename = "RightToSpecifyBankAccountsOfPaymentTransactions")]
    pub right_to_specify_bank_accounts_of_payment_transactions: String,
}

pub struct ForestCentreDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<String>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<String>,
}

pub struct WorkingRepresentativeType {
}

pub struct CostTypeAndCompletedWorkCompletionType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostType")]
    pub cost_type: String,
    #[serde(rename = "CompletedWorkAmount")]
    pub completed_work_amount: String,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: String,
}

pub struct FinancingActCompletionStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "StandNumber")]
    pub stand_number: String,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<String>,
    #[serde(rename = "StandReference")]
    pub stand_reference: String,
    #[serde(rename = "Area")]
    pub area: String,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<String>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<String>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<String>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<String>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<String>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<String>,
    #[serde(rename = "HeightClass", skip_serializing_if = "Option::is_none")]
    pub height_class: Option<String>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<String>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<String>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<String>,
    #[serde(rename = "RemovalClass", skip_serializing_if = "Option::is_none")]
    pub removal_class: Option<String>,
    #[serde(rename = "CuttingStemCount", skip_serializing_if = "Option::is_none")]
    pub cutting_stem_count: Option<String>,
    #[serde(rename = "SmallWoodRemovalClass", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_class: Option<String>,
    #[serde(rename = "SmallWoodRemovalVolume", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_volume: Option<String>,
    #[serde(rename = "CostTypeAndCompletedWorkCompletion")]
    pub cost_type_and_completed_work_completion: String,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<String>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "LocationEstate")]
    pub location_estate: String,
}

pub struct FinancingActApplicationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "StandNumber")]
    pub stand_number: String,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<String>,
    #[serde(rename = "StandReference")]
    pub stand_reference: String,
    #[serde(rename = "Area")]
    pub area: String,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<String>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<String>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<String>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<String>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<String>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<String>,
    #[serde(rename = "CostTypeAndCompletedWorkApplication")]
    pub cost_type_and_completed_work_application: String,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<String>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "LocationEstate")]
    pub location_estate: String,
}

pub struct CostTypeType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostTypeNumber")]
    pub cost_type_number: String,
}

pub struct AuthorizationOfAttorneyType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Authorization")]
    pub authorization: YesNoType,
    #[serde(rename = "Date")]
    pub date: DateType,
    #[serde(rename = "AttorneyReceivesPayment")]
    pub attorney_receives_payment: YesNoType,
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: String2000Type,
    #[serde(rename = "PowerOfAttorneyDocument", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_document: Option<base64Binary>,
}

pub struct SubsidyApplierReferenceType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "SubsidyApplierId")]
    pub subsidy_applier_id: String,
    #[serde(rename = "MainApplier")]
    pub main_applier: YesNoType,
    #[serde(rename = "ShareOfOwnerShip", skip_serializing_if = "Option::is_none")]
    pub share_of_owner_ship: Option<PercentType>,
    #[serde(rename = "OwnerShipType")]
    pub owner_ship_type: OwnerShipTypeType,
}

pub struct WorkingRepresentativesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "WorkingRepresentative")]
    pub working_representative: Vec<String>,
}

pub struct CostTypeAndCompletedWorkApplicationType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostType")]
    pub cost_type: String,
    #[serde(rename = "PlannedWorkAmount")]
    pub planned_work_amount: String,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: String,
}

pub struct FinancingActCompletionStandsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActCompletionStand")]
    pub financing_act_completion_stand: Vec<String>,
}

pub struct FinancingActGeometryType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: String,
    #[serde(rename = "FeatureInfo")]
    pub feature_info: String,
    #[serde(rename = "PointAndLineGeometriesGroup")]
    pub point_and_line_geometries_group: String,
}

pub struct DocumentsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Document")]
    pub document: Vec<String>,
}

pub struct SubsidyApplierReferenceListType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "SubsidyApplierReference")]
    pub subsidy_applier_reference: Vec<String>,
}
