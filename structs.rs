
pub struct FinancingActApplicationStandType {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: IdStringType,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: StandNumber,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtension>,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: StandReference,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Area,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<MainGroup>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroup>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClass>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<MeanHeight>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<DiameterClass>,
    #[serde(rename = "CostTypeAndCompletedWorkApplication", skip_serializing_if = "Option::is_none")]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplication,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<StandExtraInfo>,
    #[serde(rename = "polygonProperty", skip_serializing_if = "Option::is_none")]
    pub polygon_property: polygonProperty,
    #[serde(rename = "LocationEstate", skip_serializing_if = "Option::is_none")]
    pub location_estate: LocationEstate,
}

pub struct ApplicationActorsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "ApplicationActor", skip_serializing_if = "Option::is_none")]
    pub application_actor: Vec<ApplicationActorType>,
}

pub struct AuthorizationOfAttorneyType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
    pub authorization: YesNoType,
    #[serde(rename = "Date", skip_serializing_if = "Option::is_none")]
    pub date: DateType,
    #[serde(rename = "AttorneyReceivesPayment", skip_serializing_if = "Option::is_none")]
    pub attorney_receives_payment: YesNoType,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: String2000Type,
    #[serde(rename = "PowerOfAttorneyDocument", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_document: Option<base64Binary>,
}

pub struct FinancingActApplicationGeometriesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActApplicationGeometry", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_geometry: Vec<FinancingActApplicationGeometry>,
}

pub struct FinancingActApplicationStandsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActApplicationStand", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_stand: Vec<FinancingActApplicationStand>,
}

pub struct FinancingActCompletionStandsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActCompletionStand", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_stand: Vec<FinancingActCompletionStand>,
}

pub struct SubsidyApplierReferenceType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "SubsidyApplierId", skip_serializing_if = "Option::is_none")]
    pub subsidy_applier_id: SubsidyApplierId,
    #[serde(rename = "MainApplier", skip_serializing_if = "Option::is_none")]
    pub main_applier: YesNoType,
    #[serde(rename = "ShareOfOwnerShip", skip_serializing_if = "Option::is_none")]
    pub share_of_owner_ship: Option<PercentType>,
    #[serde(rename = "OwnerShipType", skip_serializing_if = "Option::is_none")]
    pub owner_ship_type: OwnerShipTypeType,
}

pub struct DocumentsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Document", skip_serializing_if = "Option::is_none")]
    pub document: Vec<Document>,
}

pub struct WorkingRepresentativesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "WorkingRepresentative", skip_serializing_if = "Option::is_none")]
    pub working_representative: Vec<WorkingRepresentative>,
}

pub struct SubsidyApplierReferenceListType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "SubsidyApplierReference", skip_serializing_if = "Option::is_none")]
    pub subsidy_applier_reference: Vec<SubsidyApplierReference>,
}

pub struct CostTypeAndCompletedWorkCompletionType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostType", skip_serializing_if = "Option::is_none")]
    pub cost_type: CostType,
    #[serde(rename = "CompletedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub completed_work_amount: CompletedWorkAmount,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPrice>,
    #[serde(rename = "SubsidyAmount", skip_serializing_if = "Option::is_none")]
    pub subsidy_amount: SubsidyAmount,
}

pub struct CostTypeAndCompletedWorkCompletionRealEstateType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "RealEstateId", skip_serializing_if = "Option::is_none")]
    pub real_estate_id: RealEstateId,
    #[serde(rename = "CostType", skip_serializing_if = "Option::is_none")]
    pub cost_type: CostType,
    #[serde(rename = "CompletedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub completed_work_amount: Option<CompletedWorkAmount>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPrice>,
    #[serde(rename = "PaymentReference", skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<PaymentReference>,
    #[serde(rename = "SubsidyAmount", skip_serializing_if = "Option::is_none")]
    pub subsidy_amount: SubsidyAmount,
}

pub struct PayeesAndRealEstatesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "PayeeAndRealEstate", skip_serializing_if = "Option::is_none")]
    pub payee_and_real_estate: Vec<PayeeAndRealEstate>,
}

pub struct WorkingRepresentativeType {
}

pub struct CostTypeType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostTypeNumber", skip_serializing_if = "Option::is_none")]
    pub cost_type_number: CostTypeNumber,
}

pub struct AttorneyType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "PowerOfAttorney", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney: PowerOfAttorney,
    #[serde(rename = "RightToSpecifyBankAccountsOfPaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub right_to_specify_bank_accounts_of_payment_transactions: RightToSpecifyBankAccountsOfPaymentTransactions,
}

pub struct PayeeType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,
}

pub struct ApplicationActorType {
}

pub struct FinancingActCompletionStandType {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: IdStringType,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: StandNumber,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtension>,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: StandReference,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Area,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<MainGroup>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroup>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClass>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<MainTreeSpecies>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCount>,
    #[serde(rename = "HeightClass", skip_serializing_if = "Option::is_none")]
    pub height_class: Option<HeightClass>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<MeanHeight>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<DiameterClass>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameter>,
    #[serde(rename = "RemovalClass", skip_serializing_if = "Option::is_none")]
    pub removal_class: Option<RemovalClass>,
    #[serde(rename = "CuttingStemCount", skip_serializing_if = "Option::is_none")]
    pub cutting_stem_count: Option<CuttingStemCount>,
    #[serde(rename = "SmallWoodRemovalClass", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_class: Option<SmallWoodRemovalClass>,
    #[serde(rename = "SmallWoodRemovalVolume", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_volume: Option<SmallWoodRemovalVolume>,
    #[serde(rename = "CostTypeAndCompletedWorkCompletion", skip_serializing_if = "Option::is_none")]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletion,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<StandExtraInfo>,
    #[serde(rename = "polygonProperty", skip_serializing_if = "Option::is_none")]
    pub polygon_property: polygonProperty,
    #[serde(rename = "LocationEstate", skip_serializing_if = "Option::is_none")]
    pub location_estate: LocationEstate,
}

pub struct ForestCentreDataType {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: IdStringType,
    #[serde(rename = "@schemaPackageVersion", skip_serializing_if = "Option::is_none")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion", skip_serializing_if = "Option::is_none")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate", skip_serializing_if = "Option::is_none")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "TimeStamp", skip_serializing_if = "Option::is_none")]
    pub time_stamp: TimeStamp,
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Message,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<RequestReference>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<MetadataText>,
}

pub struct FinancingActCompletionOtherSubjectsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostTypeAndCompletedWorkCompletionRealEstate", skip_serializing_if = "Option::is_none")]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstate,
    #[serde(rename = "FinancingActCompletionGeometries", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_geometries: FinancingActCompletionGeometries,
}

pub struct FinancingActApplicationOtherSubjectsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostTypeAndCompletedWorkApplicationRealEstate", skip_serializing_if = "Option::is_none")]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstate,
    #[serde(rename = "FinancingActApplicationGeometries", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_geometries: FinancingActApplicationGeometries,
}

pub struct PayeeAndRealEstateType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "RealEstateId", skip_serializing_if = "Option::is_none")]
    pub real_estate_id: RealEstateId,
    #[serde(rename = "PayeeId", skip_serializing_if = "Option::is_none")]
    pub payee_id: PayeeId,
    #[serde(rename = "ParticipationPercentage", skip_serializing_if = "Option::is_none")]
    pub participation_percentage: Option<ParticipationPercentage>,
}

pub struct OwnerShipTypeType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "OwnerShipTypeCode", skip_serializing_if = "Option::is_none")]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String500Type>,
}

pub struct FinancingActRealEstateType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "SubsidyApplierReferenceList", skip_serializing_if = "Option::is_none")]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

pub struct CompletionDeclarationActorsType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CompletionDeclarationActor", skip_serializing_if = "Option::is_none")]
    pub completion_declaration_actor: Vec<PayeeType>,
}

pub struct CostTypeAndCompletedWorkApplicationType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "CostType", skip_serializing_if = "Option::is_none")]
    pub cost_type: CostType,
    #[serde(rename = "PlannedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub planned_work_amount: PlannedWorkAmount,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPrice>,
    #[serde(rename = "SubsidyAmount", skip_serializing_if = "Option::is_none")]
    pub subsidy_amount: SubsidyAmount,
}

pub struct FinancingActRealEstatesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActRealEstate", skip_serializing_if = "Option::is_none")]
    pub financing_act_real_estate: Vec<FinancingActRealEstate>,
}

pub struct FinancingActCompletionGeometriesType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FinancingActCompletionGeometry", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_geometry: Vec<FinancingActCompletionGeometry>,
}

pub struct FinancingActGeometryType {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: IdStringType,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "FeatureCode", skip_serializing_if = "Option::is_none")]
    pub feature_code: FeatureCode,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: FeatureInfo,
    #[serde(rename = "PointAndLineGeometriesGroup", skip_serializing_if = "Option::is_none")]
    pub point_and_line_geometries_group: PointAndLineGeometriesGroup,
}

pub struct CostTypeAndCompletedWorkApplicationRealEstateType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "RealEstateId", skip_serializing_if = "Option::is_none")]
    pub real_estate_id: RealEstateId,
    #[serde(rename = "CostType", skip_serializing_if = "Option::is_none")]
    pub cost_type: CostType,
    #[serde(rename = "PlannedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub planned_work_amount: Option<PlannedWorkAmount>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPrice>,
    #[serde(rename = "SubsidyAmount", skip_serializing_if = "Option::is_none")]
    pub subsidy_amount: SubsidyAmount,
}

pub struct SubsidyAppliersType {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "SubsidyApplier", skip_serializing_if = "Option::is_none")]
    pub subsidy_applier: Vec<PayeeType>,
}
