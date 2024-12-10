#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsType {
    #[serde(rename = "Product")]
    pub product: Vec<ProductType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluationsType {
    #[serde(rename = "SelfMonitoringEvaluation")]
    pub self_monitoring_evaluation: Vec<SelfMonitoringEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectsType {
    #[serde(rename = "ChildObject")]
    pub child_object: Vec<ChildObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(rename = "HopperNumber")]
    pub hopper_number: String20Type,
    #[serde(rename = "HopperType")]
    pub hopper_type: HopperTypeType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Geometry")]
    pub geometry: PointGeometryType,
    #[serde(rename = "HopperLocationFromGPS")]
    pub hopper_location_from_g_p_s: YesNoType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature2Type {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: FeatureTypeType,
    #[serde(rename = "FeatureDataGroup", skip_serializing_if = "Option::is_none")]
    pub feature_data_group: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "ObservationDate", skip_serializing_if = "Option::is_none")]
    pub observation_date: Option<DateType>,
    #[serde(rename = "UsingRight", skip_serializing_if = "Option::is_none")]
    pub using_right: Option<UsingRightType>,
    #[serde(rename = "FeatureSpecificAdditionalVariables", skip_serializing_if = "Option::is_none")]
    pub feature_specific_additional_variables: Option<FeatureSpecificAdditionalVariableType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature4Type {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<YesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: FeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub using_restrictions: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<DateType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: Decimal2FractionDigitsType,
    #[serde(rename = "MeanHeight")]
    pub mean_height: Decimal1FractionDigitType,
    #[serde(rename = "StemCount")]
    pub stem_count: PositiveIntegerType,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<PositiveIntegerType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<PositiveIntegerType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesType {
    #[serde(rename = "Role")]
    pub role: Vec<OrganizationRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferSilvicultureInfoType {
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimateType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "StartTime")]
    pub start_time: TimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferBusinessSenderType {
    #[serde(rename = "@role")]
    pub role: CallForOfferBusinessSenderRoleType,
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDataType {
    #[serde(rename = "TrainingId")]
    pub training_id: PositiveInteger4digitsType,
    #[serde(rename = "TrainingFreeText", skip_serializing_if = "Option::is_none")]
    pub training_free_text: Option<String50Type>,
    #[serde(rename = "TrainingDate")]
    pub training_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteAccountingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContarctorId")]
    pub contarctor_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "AccountingDate")]
    pub accounting_date: TimeStampType,
    #[serde(rename = "FinalAccounting")]
    pub final_accounting: YesNoType,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "DeclarationStandReference")]
    pub declaration_stand_reference: ReferenceType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "CuttingPurpose")]
    pub cutting_purpose: CuttingPurposeType,
    #[serde(rename = "CuttingRealizationPractice")]
    pub cutting_realization_practice: CuttingRealizationPracticeType,
    #[serde(rename = "ForestDamageQualifier", skip_serializing_if = "Option::is_none")]
    pub forest_damage_qualifier: Option<ForestDamageQualifierType>,
    #[serde(rename = "HabitatCode")]
    pub habitat_code: HabitatCodeType,
    #[serde(rename = "HabitatOperations", skip_serializing_if = "Option::is_none")]
    pub habitat_operations: Option<HabitatOperationsType>,
    #[serde(rename = "OtherHabitatCode", skip_serializing_if = "Option::is_none")]
    pub other_habitat_code: Option<OtherHabitatCodeType>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<ExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "DeclarationMainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub declaration_main_tree_species: Option<TreeSpeciesConciseType>,
    #[serde(rename = "DeclarationDevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub declaration_development_class: Option<DeclarationDevelopmentClassType>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<AgeType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "DeclarationRegenerationCommitment", skip_serializing_if = "Option::is_none")]
    pub declaration_regeneration_commitment: Option<RegenerationCommitmentType>,
    #[serde(rename = "DeclarationRegenerationOperation", skip_serializing_if = "Option::is_none")]
    pub declaration_regeneration_operation: Option<DeclarationRegenerationOperationType>,
    #[serde(rename = "DeclarationSoilPreparationOperation", skip_serializing_if = "Option::is_none")]
    pub declaration_soil_preparation_operation: Option<DeclarationSoilPreparationOperationType>,
    #[serde(rename = "DeclarationOtherOperations", skip_serializing_if = "Option::is_none")]
    pub declaration_other_operations: Option<DeclarationOtherOperationsType>,
    #[serde(rename = "DeclarationStandTextInformation", skip_serializing_if = "Option::is_none")]
    pub declaration_stand_text_information: Option<String2000Type>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDataType {
    #[serde(rename = "EvaluationCost")]
    pub evaluation_cost: Decimal7And2PositiveType,
    #[serde(rename = "PartEastingCoordinate")]
    pub part_easting_coordinate: string,
    #[serde(rename = "PartNorthingCoordinate")]
    pub part_northing_coordinate: string,
    #[serde(rename = "InsuranceOrOtherCompensation")]
    pub insurance_or_other_compensation: Decimal7And2PositiveType,
    #[serde(rename = "TotalCompensation")]
    pub total_compensation: Decimal7And2PositiveType,
    #[serde(rename = "Compensation")]
    pub compensation: Decimal7And2PositiveType,
    #[serde(rename = "AlreadyPaidCompensation")]
    pub already_paid_compensation: Decimal7And2PositiveType,
    #[serde(rename = "TotalCompensations")]
    pub total_compensations: Decimal7And2PositiveType,
    #[serde(rename = "OwnerInvolvement")]
    pub owner_involvement: VirtaYesNoType,
    #[serde(rename = "AssociationInvolvement")]
    pub association_involvement: VirtaYesNoType,
    #[serde(rename = "MoosePercentage")]
    pub moose_percentage: PercentType,
    #[serde(rename = "Class1DamageCount")]
    pub class1_damage_count: StemCountType,
    #[serde(rename = "Class2DamageCount")]
    pub class2_damage_count: StemCountType,
    #[serde(rename = "Class3DamageCount")]
    pub class3_damage_count: StemCountType,
    #[serde(rename = "Class4DamageCount")]
    pub class4_damage_count: StemCountType,
    #[serde(rename = "DamagedSeedlingCount")]
    pub damaged_seedling_count: StemCountType,
    #[serde(rename = "NotDamagedSeedlingCount")]
    pub not_damaged_seedling_count: StemCountType,
    #[serde(rename = "LargePeelDamage")]
    pub large_peel_damage: StemCountType,
    #[serde(rename = "SmallPeelDamage")]
    pub small_peel_damage: StemCountType,
    #[serde(rename = "NeedForTreatment")]
    pub need_for_treatment: VirtaYesNoType,
    #[serde(rename = "RepairPlantingCosts")]
    pub repair_planting_costs: Decimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlForestUseDeclarationType {
    #[serde(rename = "CuttingRealizationPractice", skip_serializing_if = "Option::is_none")]
    pub cutting_realization_practice: Option<CuttingTypeType>,
    #[serde(rename = "HarvestingSignControlClassifier", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_control_classifier: Option<EvaluationCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "MooseDamageDeclarationReference")]
    pub moose_damage_declaration_reference: ReferenceType,
    #[serde(rename = "MooseDamageDate")]
    pub moose_damage_date: DateType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String2000Type>,
    #[serde(rename = "CompensationApplicant")]
    pub compensation_applicant: ContactInformationBankAccountType,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<String>,
    #[serde(rename = "MooseDamageDeclarationRealEstates")]
    pub moose_damage_declaration_real_estates: MooseDamageDeclarationRealEstatesType,
    #[serde(rename = "CompensationByInsurance")]
    pub compensation_by_insurance: CompensationByInsuranceType,
    #[serde(rename = "CompensationByLegislation")]
    pub compensation_by_legislation: CompensationByLegislationType,
    #[serde(rename = "PreviousMooseDamages", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damages: Option<PreviousMooseDamagesType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedCompletionDataType {
    #[serde(rename = "OperationStatus", skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<ControlDataOperationStatusType>,
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreeCuttingType {
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<PositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature3Type {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluationsType {
    #[serde(rename = "ControlEvaluation")]
    pub control_evaluation: Vec<ControlEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrata2Type {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratum2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisType {
    #[serde(rename = "FeeBase")]
    pub fee_base: Vec<FeeBasisDataType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<Vec<FeeBasisDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingSustainableForestryType {
    #[serde(rename = "BlockIsFSFBlock", skip_serializing_if = "Option::is_none")]
    pub block_is_f_s_f_block: Option<YesNoType>,
    #[serde(rename = "FSFInformation", skip_serializing_if = "Option::is_none")]
    pub f_s_f_information: Option<Vec<FSFInformationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsOfAccessibilityAnalysisType {
    #[serde(rename = "ResultOfAccessibilityAnalysis")]
    pub result_of_accessibility_analysis: Vec<ResultOfAccessibilityAnalysisType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectsType {
    #[serde(rename = "GeometryObject")]
    pub geometry_object: Vec<GeometryObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeListItemType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "TreeNumber", skip_serializing_if = "Option::is_none")]
    pub tree_number: Option<integer>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<DiameterType>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<MeanHeightType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "TreeIdentifiers", skip_serializing_if = "Option::is_none")]
    pub tree_identifiers: Option<TreeIdentifiersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDateType {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: DateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: InventoryMethodType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tree_strata: Option<String>,
    #[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dead_tree_strata: Option<String>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tree_stand_summary: Option<String>,
    #[serde(rename = "TreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub tree_species_summary: Option<TreeSpeciesSummaryType>,
    #[serde(rename = "StemDistribution", skip_serializing_if = "Option::is_none")]
    pub stem_distribution: Option<String>,
    #[serde(rename = "StemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub stem_distribution_strata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationGeometriesType {
    #[serde(rename = "FinancingActApplicationGeometry")]
    pub financing_act_application_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBaseDynamicType {
    #[serde(rename = "Attributes")]
    pub attributes: AttributesType,
    #[serde(rename = "Audition")]
    pub audition: AuditionType,
    #[serde(rename = "AuditionResources", skip_serializing_if = "Option::is_none")]
    pub audition_resources: Option<AuditionResourcesType>,
    #[serde(rename = "Questions")]
    pub questions: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyAppliersType {
    #[serde(rename = "SubsidyApplier")]
    pub subsidy_applier: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeebaseListItemType {
    #[serde(rename = "Id")]
    pub id: PositiveIntegerType,
    #[serde(rename = "FeeValue")]
    pub fee_value: String10Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationsType {
    #[serde(rename = "Operation")]
    pub operation: Vec<OperationDefType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMessageLanguageType {
    #[serde(rename = "@LanguageCode")]
    pub language_code: LanguageCodeType,
    #[serde(rename = "StatusMessage")]
    pub status_message: String1000Type,
    #[serde(flatten)]
    pub base: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesType {
    #[serde(rename = "Resource")]
    pub resource: Vec<ResourceDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionType {
    #[serde(rename = "PaymentTransactionCategory")]
    pub payment_transaction_category: MoneyTransactionCategoryType,
    #[serde(rename = "PaymentTransactionType")]
    pub payment_transaction_type: MoneyTransactionTypeType,
    #[serde(rename = "Value")]
    pub value: MoneyType,
    #[serde(rename = "PaymentTransactionAsUnitGroup")]
    pub payment_transaction_as_unit_group: String,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "PaymentTransactionDescription", skip_serializing_if = "Option::is_none")]
    pub payment_transaction_description: Option<PaymentTransactionDescriptionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerShipTypeType {
    #[serde(rename = "OwnerShipTypeCode")]
    pub owner_ship_type_code: OwnerShipTypeCodeType,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "Payees")]
    pub payees: PayeesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreClearingInformationType {
    #[serde(rename = "ResponsibleOfPreClearing", skip_serializing_if = "Option::is_none")]
    pub responsible_of_pre_clearing: Option<ResponsibleOfPreClearingType>,
    #[serde(rename = "PreClearingExecutionTime", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_execution_time: Option<FSFValidityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandSilvicultureInfoType {
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<HarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<BasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<StemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType1 {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationToSendWsoInformation {
    #[serde(rename = "ServiceNameOfAPI")]
    pub service_name_of_a_p_i: ServiceNameofAPIType,
    #[serde(rename = "AuthorizedToSend")]
    pub authorized_to_send: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionDataType {
    #[serde(rename = "DataInformation")]
    pub data_information: DataInformationType,
    #[serde(rename = "Inspection")]
    pub inspection: Vec<InspectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: String,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: String,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<String>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerType {
    #[serde(rename = "PersonId", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String20Type>,
    #[serde(rename = "OrganisationId", skip_serializing_if = "Option::is_none")]
    pub organisation_id: Option<String20Type>,
    #[serde(rename = "PersonRole", skip_serializing_if = "Option::is_none")]
    pub person_role: Option<String50Type>,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String20Type,
    #[serde(rename = "EmailAddress")]
    pub email_address: String50Type,
    #[serde(rename = "LanguageCode")]
    pub language_code: LanguageCodeType,
    #[serde(rename = "SendWorkingAloneNotification", skip_serializing_if = "Option::is_none")]
    pub send_working_alone_notification: Option<YesNoType>,
    #[serde(rename = "SendNotifications", skip_serializing_if = "Option::is_none")]
    pub send_notifications: Option<YesNoType>,
    #[serde(rename = "NotificationContactPerson", skip_serializing_if = "Option::is_none")]
    pub notification_contact_person: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "ResourceIdMJ", skip_serializing_if = "Option::is_none")]
    pub resource_id_m_j: Option<String20Type>,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: String20Type,
    #[serde(rename = "ForestOwner", skip_serializing_if = "Option::is_none")]
    pub forest_owner: Option<String100Type>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment")]
    pub assortment: String50Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "DisQualificationPercentageTotal")]
    pub dis_qualification_percentage_total: Decimal2FractionDigitsType,
    #[serde(rename = "CuttingAccuracy")]
    pub cutting_accuracy: Decimal2FractionDigitsType,
    #[serde(rename = "DisQualificationSign")]
    pub dis_qualification_sign: String5Type,
    #[serde(rename = "CuttingAccuracySign")]
    pub cutting_accuracy_sign: String5Type,
    #[serde(rename = "Document", skip_serializing_if = "Option::is_none")]
    pub document: Option<base64Binary>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<base64Binary>,
    #[serde(rename = "DisQualificationReasons")]
    pub dis_qualification_reasons: DisQualificationReasonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesType {
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub simple_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFInformationType {
    #[serde(rename = "FSFNumber", skip_serializing_if = "Option::is_none")]
    pub f_s_f_number: Option<String50Type>,
    #[serde(rename = "FSFValidity", skip_serializing_if = "Option::is_none")]
    pub f_s_f_validity: Option<FSFValidityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassesType {
    #[serde(rename = "AssortmentClass")]
    pub assortment_class: Vec<AssortmentClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@operationId")]
    pub operation_id: string,
    #[serde(rename = "@standId")]
    pub stand_id: string,
    #[serde(rename = "@productId")]
    pub product_id: string,
    #[serde(rename = "ProductKeyGroup")]
    pub product_key_group: String,
    #[serde(rename = "ProductName")]
    pub product_name: String500Type,
    #[serde(rename = "Quantity")]
    pub quantity: Decimal2FractionDigitsType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: WideUnitType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "TotalPrice", skip_serializing_if = "Option::is_none")]
    pub total_price: Option<TotalPriceType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "Consumption", skip_serializing_if = "Option::is_none")]
    pub consumption: Option<ConsumptionType>,
    #[serde(rename = "ConsumptionUnit", skip_serializing_if = "Option::is_none")]
    pub consumption_unit: Option<ConsumptionUnitType>,
    #[serde(rename = "PlannedResource", skip_serializing_if = "Option::is_none")]
    pub planned_resource: Option<PlannedResourceType>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType2,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaryType {
    #[serde(rename = "Storey")]
    pub storey: StoreyType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "AgeSummary", skip_serializing_if = "Option::is_none")]
    pub age_summary: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StemCountSummary")]
    pub stem_count_summary: PositiveInteger4digitsType,
    #[serde(rename = "BasalAreaSummary", skip_serializing_if = "Option::is_none")]
    pub basal_area_summary: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightSummary")]
    pub mean_height_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
    #[serde(rename = "MeanDiameterSummary")]
    pub mean_diameter_summary: PositiveInteger3digitsType,
    #[serde(rename = "VolumeSummary", skip_serializing_if = "Option::is_none")]
    pub volume_summary: Option<PositiveInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDynamicType {
    #[serde(flatten)]
    pub base: WorkingSiteFinalAuditBaseDynamicType,
    #[serde(rename = "FinalAuditSpareTrees", skip_serializing_if = "Option::is_none")]
    pub final_audit_spare_trees: Option<FinalAuditSpareTreesByCategoryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilvicultureBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "MapSymbols")]
    pub map_symbols: MapSymbolsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupsType {
    #[serde(rename = "SpareTreeGroup")]
    pub spare_tree_group: SpareTreeGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesType {
    #[serde(rename = "AssortmentVolume")]
    pub assortment_volume: Vec<AssortmentVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDateType {
    #[serde(rename = "@type")]
    pub r#type: DatePrecipionType,
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandsType {
    #[serde(rename = "FinancingActApplicationStand")]
    pub financing_act_application_stand: Vec<FinancingActApplicationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioningType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType2 {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsListType {
    #[serde(rename = "AuditQuestion")]
    pub audit_question: AuditQuestionType,
    #[serde(rename = "Question")]
    pub question: AuditQuestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperativeDataType {
    #[serde(rename = "OperationTreeSpeciesSummary", skip_serializing_if = "Option::is_none")]
    pub operation_tree_species_summary: Option<OperationTreeSpeciesSummaryType>,
    #[serde(rename = "GrowingTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub growing_tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "MaterialAmount", skip_serializing_if = "Option::is_none")]
    pub material_amount: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "MaterialAmountUnit", skip_serializing_if = "Option::is_none")]
    pub material_amount_unit: Option<MaterialUnitType>,
    #[serde(rename = "TargetStemCount", skip_serializing_if = "Option::is_none")]
    pub target_stem_count: Option<StemCountType>,
    #[serde(rename = "TargetBasalArea", skip_serializing_if = "Option::is_none")]
    pub target_basal_area: Option<BasalAreaType>,
    #[serde(rename = "TargetAmount", skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<AmountType>,
    #[serde(rename = "TargetAmountUnit", skip_serializing_if = "Option::is_none")]
    pub target_amount_unit: Option<ExtendedWideUnitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttorneyType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: PowerOfAttorneyType,
    #[serde(rename = "RightToSpecifyBankAccountsOfPaymentTransactions")]
    pub right_to_specify_bank_accounts_of_payment_transactions: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: StemTypeType,
    #[serde(rename = "AssortmentCode", skip_serializing_if = "Option::is_none")]
    pub assortment_code: Option<AssortmentCodeType>,
    #[serde(rename = "AssortmentName", skip_serializing_if = "Option::is_none")]
    pub assortment_name: Option<AssortmentNameType>,
    #[serde(rename = "AssortmentPercent")]
    pub assortment_percent: AssortmentPercentType,
    #[serde(rename = "AssortmentVolume")]
    pub assortment_volume: AssortmentVolumeType,
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<ERPIdType>,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Name")]
    pub name: String50Type,
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "Quality")]
    pub quality: String5Type,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<WorkCodeUnitType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "MeasurementMethod")]
    pub measurement_method: MeasurementMethodType,
    #[serde(rename = "MeasurementPlace")]
    pub measurement_place: MeasurementPlaceType,
    #[serde(rename = "DiameterMin", skip_serializing_if = "Option::is_none")]
    pub diameter_min: Option<PositiveIntegerType>,
    #[serde(rename = "DiameterMax", skip_serializing_if = "Option::is_none")]
    pub diameter_max: Option<PositiveIntegerType>,
    #[serde(rename = "HeightMin", skip_serializing_if = "Option::is_none")]
    pub height_min: Option<Decimal1FractionDigitType>,
    #[serde(rename = "HeightMax", skip_serializing_if = "Option::is_none")]
    pub height_max: Option<Decimal1FractionDigitType>,
    #[serde(rename = "CanModify", skip_serializing_if = "Option::is_none")]
    pub can_modify: Option<YesNoType>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<String200Type>,
    #[serde(rename = "DeliveryUserId", skip_serializing_if = "Option::is_none")]
    pub delivery_user_id: Option<String50Type>,
    #[serde(rename = "AssortmentID", skip_serializing_if = "Option::is_none")]
    pub assortment_i_d: Option<ERPIdType>,
    #[serde(flatten)]
    pub base: AssortmentAllElementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataForestRoadConstructionType {
    #[serde(rename = "AppliedLength")]
    pub applied_length: Vec<Decimal6_2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "@dimension")]
    pub dimension: decimal,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "pointProperty")]
    pub point_property: String,
    #[serde(rename = "Id")]
    pub id: string,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "Point")]
    pub point: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestRealizationDataType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: String2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: string,
    #[serde(rename = "GeometryObjects")]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByInsuranceType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: YesNoType,
    #[serde(rename = "InsuranceCompany", skip_serializing_if = "Option::is_none")]
    pub insurance_company: Option<String500Type>,
    #[serde(rename = "InsuranceNumber", skip_serializing_if = "Option::is_none")]
    pub insurance_number: Option<String100Type>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<Decimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: RealEstatesWithOwnersInformationType2,
    #[serde(rename = "DeclarationPolygons")]
    pub declaration_polygons: DeclarationPolygonsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnerType {
    #[serde(rename = "Owner")]
    pub owner: ContactInformationType,
    #[serde(rename = "OwnerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub owner_representative_person: Option<ContactInformationType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalType {
    #[serde(rename = "Mean")]
    pub mean: MeanType,
    #[serde(rename = "Variance")]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataSetType {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: Vec<ForestPropertyDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformationType {
    #[serde(rename = "UserId")]
    pub user_id: String20Type,
    #[serde(rename = "Contractors")]
    pub contractors: ContractorsType,
    #[serde(rename = "OwnerContractorId")]
    pub owner_contractor_id: String20Type,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address")]
    pub address: String50Type,
    #[serde(rename = "PostalCode")]
    pub postal_code: String10Type,
    #[serde(rename = "PostOffice")]
    pub post_office: String50Type,
    #[serde(rename = "BirthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<DateType>,
    #[serde(rename = "Telephone")]
    pub telephone: String20Type,
    #[serde(rename = "ICEName", skip_serializing_if = "Option::is_none")]
    pub i_c_e_name: Option<String100Type>,
    #[serde(rename = "ICETelephone", skip_serializing_if = "Option::is_none")]
    pub i_c_e_telephone: Option<String20Type>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String100Type>,
    #[serde(rename = "UserRoles")]
    pub user_roles: UserRolesType,
    #[serde(rename = "UserName")]
    pub user_name: String100Type,
    #[serde(rename = "AdditionalName", skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String20Type>,
    #[serde(rename = "Trainings")]
    pub trainings: TrainingsType,
    #[serde(rename = "Employment")]
    pub employment: EmploymentDataType,
    #[serde(rename = "Machines", skip_serializing_if = "Option::is_none")]
    pub machines: Option<MachinesType>,
    #[serde(rename = "Languages")]
    pub languages: LanguagesType,
    #[serde(rename = "NationalityCode")]
    pub nationality_code: String5Type,
    #[serde(rename = "NationalityFreeText", skip_serializing_if = "Option::is_none")]
    pub nationality_free_text: Option<String50Type>,
    #[serde(rename = "WorkCodeGroups", skip_serializing_if = "Option::is_none")]
    pub work_code_groups: Option<WorkCodeGroupsType>,
    #[serde(rename = "Location")]
    pub location: PointGeometryType,
    #[serde(rename = "E101")]
    pub e101: YesNoType,
    #[serde(rename = "A1")]
    pub a1: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessagesType {
    #[serde(rename = "CommonMessage", skip_serializing_if = "Option::is_none")]
    pub common_message: Option<Vec<CommonMessageDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeReductionType {
    #[serde(rename = "StumpStemCount", skip_serializing_if = "Option::is_none")]
    pub stump_stem_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "StumpMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stump_mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesType {
    #[serde(rename = "Envelope")]
    pub envelope: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringImageType {
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceTypeType>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<ImageCategoryType>,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<ImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<FeatureCodeType>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<PointGeometryType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "Photographer", skip_serializing_if = "Option::is_none")]
    pub photographer: Option<String50Type>,
    #[serde(rename = "ImageDate")]
    pub image_date: TimeStampType,
    #[serde(rename = "Filename")]
    pub filename: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreasType {
    #[serde(rename = "ProcessingArea")]
    pub processing_area: Vec<ProcessingAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorType {
    #[serde(rename = "ActorId", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<IdStringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestObjectDataObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantityType {
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: ExtendedQuantityUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
    #[serde(rename = "Objects")]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeatureType {
    #[serde(rename = "HabitatCode", skip_serializing_if = "Option::is_none")]
    pub habitat_code: Option<ExtendedHabitatCodeType>,
    #[serde(rename = "HabitatType", skip_serializing_if = "Option::is_none")]
    pub habitat_type: Option<HabitatTypeType>,
    #[serde(rename = "HabitatSurviving", skip_serializing_if = "Option::is_none")]
    pub habitat_surviving: Option<YesNoType>,
    #[serde(rename = "ExceptionalPermitForHandling", skip_serializing_if = "Option::is_none")]
    pub exceptional_permit_for_handling: Option<YesNoType>,
    #[serde(rename = "HabitatLocation", skip_serializing_if = "Option::is_none")]
    pub habitat_location: Option<HabitatLocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMessageDataType {
    #[serde(rename = "CommonMessageId", skip_serializing_if = "Option::is_none")]
    pub common_message_id: Option<CommonMessageType>,
    #[serde(rename = "CommonMessageFreeText", skip_serializing_if = "Option::is_none")]
    pub common_message_free_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometryType {
    #[serde(rename = "MultiPolygon")]
    pub multi_polygon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionReceiversType {
    #[serde(rename = "DecisionReceiver")]
    pub decision_receiver: Vec<ContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectsType {
    #[serde(rename = "ParentObject")]
    pub parent_object: Vec<ParentObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStrataType {
    #[serde(rename = "DeadTreeStratum")]
    pub dead_tree_stratum: Vec<DeadTreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryType {
    #[serde(rename = "DeliveryUserId")]
    pub delivery_user_id: String50Type,
    #[serde(rename = "DeliveryInfo")]
    pub delivery_info: String50Type,
    #[serde(rename = "DeliveryName")]
    pub delivery_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "OfferWorkingSiteDetails")]
    pub offer_working_site_details: Vec<OfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedResourceType {
    #[serde(rename = "@Id")]
    pub id: string,
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "PlannedResource")]
    pub planned_resource: PlannedResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDataType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: String2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: UseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMeasurementsType {
    #[serde(rename = "LogDiameter")]
    pub log_diameter: PositiveInteger3digitsType,
    #[serde(rename = "ControlLogDiameter")]
    pub control_log_diameter: PositiveInteger3digitsType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulkType {
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "Bulk")]
    pub bulk: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeibullType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwnersType {
    #[serde(rename = "ForestOwner")]
    pub forest_owner: Vec<ForestOwnerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummariesType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlSoilConditioningType {
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkApplicationRealEstate")]
    pub cost_type_and_completed_work_application_real_estate: CostTypeAndCompletedWorkApplicationRealEstateType,
    #[serde(rename = "FinancingActApplicationGeometries")]
    pub financing_act_application_geometries: FinancingActApplicationGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(rename = "ParentObjectType")]
    pub parent_object_type: ObjectTypeType,
    #[serde(rename = "ParentObjectId")]
    pub parent_object_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluationType {
    #[serde(rename = "@assessorActorId")]
    pub assessor_actor_id: IdStringType,
    #[serde(rename = "@assessObjectActorId")]
    pub assess_object_actor_id: IdStringType,
    #[serde(rename = "EvaluationCode", skip_serializing_if = "Option::is_none")]
    pub evaluation_code: Option<EvaluationCodeType>,
    #[serde(rename = "EvaluationText", skip_serializing_if = "Option::is_none")]
    pub evaluation_text: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedMachineType {
    #[serde(rename = "MachineCode", skip_serializing_if = "Option::is_none")]
    pub machine_code: Option<MachineCodeType>,
    #[serde(rename = "MachineDescription", skip_serializing_if = "Option::is_none")]
    pub machine_description: Option<String500Type>,
    #[serde(rename = "MachineAccessoryCode", skip_serializing_if = "Option::is_none")]
    pub machine_accessory_code: Option<MachineAccessoryCodeType>,
    #[serde(rename = "MachineAccessoryDescription", skip_serializing_if = "Option::is_none")]
    pub machine_accessory_description: Option<String500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisDataType {
    #[serde(rename = "FeeId")]
    pub fee_id: String10Type,
    #[serde(rename = "FeeListId", skip_serializing_if = "Option::is_none")]
    pub fee_list_id: Option<PositiveIntegerType>,
    #[serde(rename = "FeeYesNo", skip_serializing_if = "Option::is_none")]
    pub fee_yes_no: Option<YesNoType>,
    #[serde(rename = "FeeValue", skip_serializing_if = "Option::is_none")]
    pub fee_value: Option<String10Type>,
    #[serde(rename = "FeeAssortment", skip_serializing_if = "Option::is_none")]
    pub fee_assortment: Option<String50Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "NeedToCheck", skip_serializing_if = "Option::is_none")]
    pub need_to_check: Option<YesNoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<StandsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
    #[serde(rename = "FeeBaseList", skip_serializing_if = "Option::is_none")]
    pub fee_base_list: Option<FeeBaseListType>,
    #[serde(rename = "FeeType")]
    pub fee_type: FeeBasisValueType,
    #[serde(rename = "FeeText")]
    pub fee_text: String50Type,
    #[serde(rename = "FeeUnit", skip_serializing_if = "Option::is_none")]
    pub fee_unit: Option<String10Type>,
    #[serde(rename = "InfoTextMandatory")]
    pub info_text_mandatory: YesNoType,
    #[serde(rename = "DefaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String50Type>,
    #[serde(rename = "ValueList", skip_serializing_if = "Option::is_none")]
    pub value_list: Option<ValueListType>,
    #[serde(rename = "FeeBasisList", skip_serializing_if = "Option::is_none")]
    pub fee_basis_list: Option<FeeBasisListType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotType {
    #[serde(rename = "WoodLotInformationGroup")]
    pub wood_lot_information_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StanfordFileType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsType {
    #[serde(rename = "Payment")]
    pub payment: Vec<ForestCentrePaymentDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativeType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingStratumType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "SeedlingBeginningCode", skip_serializing_if = "Option::is_none")]
    pub seedling_beginning_code: Option<SeedlingOriginType>,
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<PositiveInteger6digitsType>,
    #[serde(rename = "AmountInSpot", skip_serializing_if = "Option::is_none")]
    pub amount_in_spot: Option<PositiveInteger6digitsType>,
    #[serde(rename = "AmountOutsideSpot", skip_serializing_if = "Option::is_none")]
    pub amount_outside_spot: Option<PositiveInteger6digitsType>,
    #[serde(rename = "AmountUnit", skip_serializing_if = "Option::is_none")]
    pub amount_unit: Option<AmountUnitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometryType {
    #[serde(rename = "@collectingMethod")]
    pub collecting_method: CollectingMethodType,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<AreaType>,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<Decimal1FractionDigitType>,
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadType {
    #[serde(rename = "Acknowledge")]
    pub acknowledge: AcknowledgeType,
    #[serde(rename = "CompanyInformation")]
    pub company_information: CompanyInformationType,
    #[serde(rename = "Contract")]
    pub contract: ContractType,
    #[serde(rename = "ExternalFile")]
    pub external_file: ExternalFileType,
    #[serde(rename = "ForwardingEstimate")]
    pub forwarding_estimate: ForwardingEstimateType,
    #[serde(rename = "ForwardingNotification")]
    pub forwarding_notification: ForwardingNotificationType,
    #[serde(rename = "HarvestingOrder")]
    pub harvesting_order: HarvestingOrderType,
    #[serde(rename = "Image")]
    pub image: ImageType,
    #[serde(rename = "MapSymbol")]
    pub map_symbol: MapSymbolType,
    #[serde(rename = "OrderConfirmation")]
    pub order_confirmation: OrderConfirmationType,
    #[serde(rename = "ProductInstruction")]
    pub product_instruction: String,
    #[serde(rename = "QualityAttachment")]
    pub quality_attachment: QualityAttachmentType,
    #[serde(rename = "Resource")]
    pub resource: ResourceType,
    #[serde(rename = "ResourceSchedule")]
    pub resource_schedule: ResourceScheduleType,
    #[serde(rename = "SilvicultureOrder")]
    pub silviculture_order: SilvicultureOrderType,
    #[serde(rename = "ServiceBuyerResourceLocations")]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsType,
    #[serde(rename = "SmsOperatorStatus")]
    pub sms_operator_status: SmsOperatorStatusType,
    #[serde(rename = "StanfordFile")]
    pub stanford_file: StanfordFileType,
    #[serde(rename = "UserInformation")]
    pub user_information: UserInformationType,
    #[serde(rename = "WeekCalendar")]
    pub week_calendar: WeekCalendarType,
    #[serde(rename = "WorkingSiteAccounting")]
    pub working_site_accounting: WorkingSiteAccountingType,
    #[serde(rename = "WorkingSiteEndNotification")]
    pub working_site_end_notification: WorkingSiteEndNotificationType,
    #[serde(rename = "WorkingSiteFeeBasis")]
    pub working_site_fee_basis: WorkingSiteFeeBasisType,
    #[serde(rename = "WorkingSiteFinalAuditBioMassForwarding")]
    pub working_site_final_audit_bio_mass_forwarding: WorkingSiteFinalAuditBioMassForwardingType,
    #[serde(rename = "WorkingSiteFinalAuditDraining")]
    pub working_site_final_audit_draining: WorkingSiteFinalAuditDrainingType,
    #[serde(rename = "WorkingSiteFinalAuditFertilization")]
    pub working_site_final_audit_fertilization: WorkingSiteFinalAuditFertilizationType,
    #[serde(rename = "WorkingSiteFinalAuditHarvesting")]
    pub working_site_final_audit_harvesting: WorkingSiteFinalAuditHarvestingType,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement")]
    pub working_site_final_audit_plant_management: WorkingSiteFinalAuditPlantManagementType,
    #[serde(rename = "WorkingSiteFinalAuditRoadMaking")]
    pub working_site_final_audit_road_making: WorkingSiteFinalAuditRoadMakingType,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture")]
    pub working_site_final_audit_silviculture: WorkingSiteFinalAuditSilvicultureType,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning")]
    pub working_site_final_audit_soil_conditioning: WorkingSiteFinalAuditSoilConditioningType,
    #[serde(rename = "WorkingSiteFinalAuditStumpForwarding")]
    pub working_site_final_audit_stump_forwarding: WorkingSiteFinalAuditStumpForwardingType,
    #[serde(rename = "WorkingSiteFinalAuditStumpLifting")]
    pub working_site_final_audit_stump_lifting: WorkingSiteFinalAuditStumpLiftingType,
    #[serde(rename = "WorkingSiteFinalAuditDynamic")]
    pub working_site_final_audit_dynamic: WorkingSiteFinalAuditDynamicType,
    #[serde(rename = "WorkingSiteForwardedProduction")]
    pub working_site_forwarded_production: WorkingSiteForwardedProductionType,
    #[serde(rename = "WorkingSiteForwardingQualityControl")]
    pub working_site_forwarding_quality_control: WorkingSiteForwardingQualityControlType,
    #[serde(rename = "WorkingSiteHarvestedProduction")]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionType,
    #[serde(rename = "WorkingSiteHarvestingQualityControl")]
    pub working_site_harvesting_quality_control: WorkingSiteHarvestingQualityControlType,
    #[serde(rename = "WorkingSiteHarvestingQualityControlManual")]
    pub working_site_harvesting_quality_control_manual: WorkingSiteHarvestingQualityControlManualType,
    #[serde(rename = "WorkingSiteOperational")]
    pub working_site_operational: WorkingSiteOperationalType,
    #[serde(rename = "WorkingSiteOperationalUpdate")]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
    #[serde(rename = "WorkingSiteQualityControlCutting")]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingType,
    #[serde(rename = "WorkingSiteQualityControlFertilization")]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement")]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementType,
    #[serde(rename = "WorkingSiteQualityControlSilviculture")]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureType,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning")]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningType,
    #[serde(rename = "WorkingSiteQualityNotification")]
    pub working_site_quality_notification: WorkingSiteQualityNotificationType,
    #[serde(rename = "WorkingSiteStatus")]
    pub working_site_status: WorkingSiteStatusType,
    #[serde(rename = "WorkingSiteTravelNotification")]
    pub working_site_travel_notification: WorkingSiteTravelNotificationType,
    #[serde(rename = "WorkingSiteWorkLoad")]
    pub working_site_work_load: WorkingSiteWorkLoadType,
    #[serde(rename = "WorkingSiteWorkTime")]
    pub working_site_work_time: WorkingSiteWorkTimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataAndSubsidyType {
    #[serde(rename = "FinancingActCompletionStands", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Value")]
    pub value: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactStandBasicDataType {
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<YearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<YearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StandInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<DataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreesType {
    #[serde(rename = "TreeListItem", skip_serializing_if = "Option::is_none")]
    pub tree_list_item: Option<Vec<TreeListItemType>>,
    #[serde(rename = "Tree")]
    pub tree: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationsType {
    #[serde(rename = "ForestUseDeclarationReference")]
    pub forest_use_declaration_reference: Vec<ForestUseDeclarationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsedPricingMethodsType {
    #[serde(rename = "UsedPricingMethod")]
    pub used_pricing_method: Vec<UsedPricingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "LocationTimestamp")]
    pub location_timestamp: TimeStampType,
    #[serde(rename = "Location")]
    pub location: PointGeometryType,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: int,
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: DateType,
    #[serde(rename = "EndDate")]
    pub end_date: DateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<PositiveInteger2digitsType>,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "WorkingSiteName")]
    pub working_site_name: WorkingSiteNameType,
    #[serde(rename = "IncludeInOffer")]
    pub include_in_offer: IncludeInOfferType,
    #[serde(rename = "SellerRepresentativePerson")]
    pub seller_representative_person: SellerRepresentativePersonType,
    #[serde(rename = "Sellers")]
    pub sellers: SellersType,
    #[serde(rename = "VATInfo")]
    pub v_a_t_info: VATInfoType,
    #[serde(rename = "RealEstate")]
    pub real_estate: RealEstateType,
    #[serde(rename = "RealEstates")]
    pub real_estates: RealEstatesType,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<WorkingSiteTextType>,
    #[serde(rename = "RoadUsingRight", skip_serializing_if = "Option::is_none")]
    pub road_using_right: Option<RoadUsingRightType>,
    #[serde(rename = "WorkingSitePlan", skip_serializing_if = "Option::is_none")]
    pub working_site_plan: Option<WorkingSitePlanType>,
    #[serde(rename = "CallForOfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_wood_trade_info: Option<CallForOfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<String>,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub certification_systems: Option<String>,
    #[serde(rename = "WorkingSiteGeometries", skip_serializing_if = "Option::is_none")]
    pub working_site_geometries: Option<WorkingSiteGeometriesType>,
    #[serde(rename = "CallForOfferWorkingSiteSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_working_site_silviculture_info: Option<CallForOfferWorkingSiteSilvicultureInfoType>,
    #[serde(rename = "IncludedInOffer")]
    pub included_in_offer: IncludedInOfferType,
    #[serde(rename = "PurchaserRepresentativePerson")]
    pub purchaser_representative_person: PurchaserRepresentativePersonType,
    #[serde(rename = "OfferWorkingSiteWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_wood_trade_info: Option<OfferWorkingSiteWoodTradeInfoType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_info: Option<OfferWorkingSiteSilvicultureInfoType>,
    #[serde(rename = "OfferWorkingSitePaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_payment_transactions: Option<OfferWorkingSitePaymentTransactionsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorsType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlersType {
    #[serde(rename = "DecisionHandler")]
    pub decision_handler: Vec<DecisionHandlerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreReplyType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: TimeStampType,
    #[serde(rename = "Acceptance")]
    pub acceptance: AcceptanceType,
    #[serde(rename = "ReplyCode")]
    pub reply_code: ReplyCodeType,
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference: Option<ReferenceType>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<DateType>,
    #[serde(rename = "RegistrationId", skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<ReferenceType>,
    #[serde(rename = "ErrorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<ErrorMessagesType>,
    #[serde(rename = "AdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSilvicultureType {
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<WorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<DateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlManualType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "InfoText")]
    pub info_text: String200Type,
    #[serde(rename = "Measurements")]
    pub measurements: MeasurementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeAndRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringNotEmptyType,
    #[serde(rename = "PayeeId")]
    pub payee_id: IdStringNotEmptyType,
    #[serde(rename = "ParticipationPercentage", skip_serializing_if = "Option::is_none")]
    pub participation_percentage: Option<PercentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document4MBType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: FileBinaryRestrictedSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBaseHarvestingType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "HarvesterId", skip_serializing_if = "Option::is_none")]
    pub harvester_id: Option<String20Type>,
    #[serde(rename = "ForwarderId", skip_serializing_if = "Option::is_none")]
    pub forwarder_id: Option<String20Type>,
    #[serde(rename = "PurchaseContractId", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: IdentifierTypeType,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: IdentifierValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationsType {
    #[serde(rename = "ObjectProtectionOperation")]
    pub object_protection_operation: Vec<ObjectProtectionOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreForestDataUpdateObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreForestDataUpdateObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesAndRealEstatesType {
    #[serde(rename = "PayeeAndRealEstate")]
    pub payee_and_real_estate: Vec<PayeeAndRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "CaseNumber")]
    pub case_number: FinancingActNumberType,
    #[serde(rename = "CaseDate")]
    pub case_date: DateType,
    #[serde(rename = "DecisionNumber")]
    pub decision_number: String100Type,
    #[serde(rename = "DecisionType")]
    pub decision_type: DecisionTypeType,
    #[serde(rename = "DecisionDate")]
    pub decision_date: DateType,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_reference: Option<String>,
    #[serde(rename = "CompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub completion_declaration_reference: Option<String>,
    #[serde(rename = "ForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_reference: Option<String>,
    #[serde(rename = "Acceptance")]
    pub acceptance: AcceptanceType,
    #[serde(rename = "CaseActions")]
    pub case_actions: CaseActionsType,
    #[serde(rename = "OriginalSender")]
    pub original_sender: ContactInformationType,
    #[serde(rename = "DecisionReceivers")]
    pub decision_receivers: DecisionReceiversType,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<ContactInformationType>,
    #[serde(rename = "RealEstates")]
    pub real_estates: RealEstatesType,
    #[serde(rename = "DecisionGeometries", skip_serializing_if = "Option::is_none")]
    pub decision_geometries: Option<DecisionGeometriesType>,
    #[serde(rename = "Works")]
    pub works: WorksType,
    #[serde(rename = "SubsidyArgument", skip_serializing_if = "Option::is_none")]
    pub subsidy_argument: Option<SubsidyArgumentType>,
    #[serde(rename = "FinancingActData", skip_serializing_if = "Option::is_none")]
    pub financing_act_data: Option<FinancingActDataType>,
    #[serde(rename = "Justifications")]
    pub justifications: JustificationsType,
    #[serde(rename = "RectificationDemand")]
    pub rectification_demand: String5000Type,
    #[serde(rename = "DecisionHandlers")]
    pub decision_handlers: DecisionHandlersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesType {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<LocatedSpecialFeature3Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonsType {
    #[serde(rename = "Reason")]
    pub reason: Vec<ReasonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSitesType {
    #[serde(rename = "WorkingSiteKey")]
    pub working_site_key: Vec<WorkingSiteKeyType>,
    #[serde(rename = "CallForOfferWorkingSiteDetails")]
    pub call_for_offer_working_site_details: Vec<CallForOfferWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatOperationsType {
    #[serde(rename = "HabitatOperation")]
    pub habitat_operation: Vec<HabitatOperationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentsType {
    #[serde(rename = "Document")]
    pub document: Vec<Document4MBType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionDataType {
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagementType {
    #[serde(rename = "DitchMeanDepth")]
    pub ditch_mean_depth: string,
    #[serde(rename = "DitchMeanWidth")]
    pub ditch_mean_width: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInformationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OrganizationName")]
    pub organization_name: String100Type,
    #[serde(rename = "RepresentativePerson")]
    pub representative_person: ContactInformationType,
    #[serde(rename = "ContactPersonInFinland")]
    pub contact_person_in_finland: ContactInformationType,
    #[serde(rename = "CompanyType")]
    pub company_type: CompanyTypeType,
    #[serde(rename = "QualitySystems", skip_serializing_if = "Option::is_none")]
    pub quality_systems: Option<QualitySystemsType>,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub certification_systems: Option<CertificationSystemsType>,
    #[serde(rename = "TradeRegistration", skip_serializing_if = "Option::is_none")]
    pub trade_registration: Option<DateType>,
    #[serde(rename = "TaxDebt")]
    pub tax_debt: DateType,
    #[serde(rename = "EmployeePensionCertificate", skip_serializing_if = "Option::is_none")]
    pub employee_pension_certificate: Option<DateType>,
    #[serde(rename = "EmployeeHealthCare")]
    pub employee_health_care: YesNoType,
    #[serde(rename = "EmployeeHealthCareInfo", skip_serializing_if = "Option::is_none")]
    pub employee_health_care_info: Option<String100Type>,
    #[serde(rename = "CollectiveAgreements")]
    pub collective_agreements: CollectiveAgreementsType,
    #[serde(rename = "ServiceTypes")]
    pub service_types: ServiceTypesType,
    #[serde(rename = "LiabilityInsurance", skip_serializing_if = "Option::is_none")]
    pub liability_insurance: Option<DateType>,
    #[serde(rename = "LegalAccidentInsurance", skip_serializing_if = "Option::is_none")]
    pub legal_accident_insurance: Option<DateType>,
    #[serde(rename = "SubContractorWrittenAgreement")]
    pub sub_contractor_written_agreement: YesNoType,
    #[serde(rename = "EmployeeWrittenAgreement")]
    pub employee_written_agreement: YesNoType,
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: YesNoType,
    #[serde(rename = "PreDebtCollectionRegister")]
    pub pre_debt_collection_register: YesNoType,
    #[serde(rename = "EmployerRegister")]
    pub employer_register: EmployerRegisterType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String50Type>,
    #[serde(rename = "BankCode", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String20Type>,
    #[serde(rename = "IsActive")]
    pub is_active: YesNoType,
    #[serde(rename = "SubContractors", skip_serializing_if = "Option::is_none")]
    pub sub_contractors: Option<SubContractorsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "Status4", skip_serializing_if = "Option::is_none")]
    pub status4: Option<ChangeStateType>,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: string,
    #[serde(rename = "SamplePlotEastingCoordinate", skip_serializing_if = "Option::is_none")]
    pub sample_plot_easting_coordinate: Option<string>,
    #[serde(rename = "SamplePlotNorthingCoordinate", skip_serializing_if = "Option::is_none")]
    pub sample_plot_northing_coordinate: Option<string>,
    #[serde(rename = "SamplePlotSize", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size: Option<integer>,
    #[serde(rename = "SamplePlotMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub sample_plot_mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotStubDiameter", skip_serializing_if = "Option::is_none")]
    pub sample_plot_stub_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotMeanHeight", skip_serializing_if = "Option::is_none")]
    pub sample_plot_mean_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotDominantHeight", skip_serializing_if = "Option::is_none")]
    pub sample_plot_dominant_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotBasalArea", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basal_area: Option<integer>,
    #[serde(rename = "SoilModificationEstimate", skip_serializing_if = "Option::is_none")]
    pub soil_modification_estimate: Option<VirtaEvaluationType>,
    #[serde(rename = "SamplePlotTrackDistance", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_distance: Option<integer>,
    #[serde(rename = "SamplePlotTrackWidth", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_width: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotTrackDepth", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_depth: Option<integer>,
    #[serde(rename = "SecondStoreyTrees", skip_serializing_if = "Option::is_none")]
    pub second_storey_trees: Option<integer>,
    #[serde(rename = "Trees", skip_serializing_if = "Option::is_none")]
    pub trees: Option<TreesType>,
    #[serde(rename = "TestAreaNumber")]
    pub test_area_number: PositiveIntegerType,
    #[serde(rename = "Geometry")]
    pub geometry: PointGeometryType,
    #[serde(rename = "StandId")]
    pub stand_id: String20Type,
    #[serde(rename = "MeasureDate")]
    pub measure_date: TimeStampType,
    #[serde(rename = "Measurer", skip_serializing_if = "Option::is_none")]
    pub measurer: Option<String50Type>,
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String20Type>,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: YesNoType,
    #[serde(rename = "District")]
    pub district: ThinningDistrictType,
    #[serde(rename = "Radius")]
    pub radius: Decimal2FractionDigitsType,
    #[serde(rename = "ManipulationMethod", skip_serializing_if = "Option::is_none")]
    pub manipulation_method: Option<WorkCodeQualifierType2>,
    #[serde(rename = "ForestType", skip_serializing_if = "Option::is_none")]
    pub forest_type: Option<FertilityClassType>,
    #[serde(rename = "StemCountPine", skip_serializing_if = "Option::is_none")]
    pub stem_count_pine: Option<PositiveInteger2digitsType>,
    #[serde(rename = "StemCountSpruce", skip_serializing_if = "Option::is_none")]
    pub stem_count_spruce: Option<PositiveInteger2digitsType>,
    #[serde(rename = "StemCountHardWood", skip_serializing_if = "Option::is_none")]
    pub stem_count_hard_wood: Option<PositiveInteger2digitsType>,
    #[serde(rename = "StemCountSilverBirch", skip_serializing_if = "Option::is_none")]
    pub stem_count_silver_birch: Option<PositiveInteger2digitsType>,
    #[serde(rename = "MeanHeightPine", skip_serializing_if = "Option::is_none")]
    pub mean_height_pine: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightSpruce", skip_serializing_if = "Option::is_none")]
    pub mean_height_spruce: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightHardWood", skip_serializing_if = "Option::is_none")]
    pub mean_height_hard_wood: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightSilverBirch", skip_serializing_if = "Option::is_none")]
    pub mean_height_silver_birch: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaPine", skip_serializing_if = "Option::is_none")]
    pub basal_area_pine: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaSpruce", skip_serializing_if = "Option::is_none")]
    pub basal_area_spruce: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaHardWood", skip_serializing_if = "Option::is_none")]
    pub basal_area_hard_wood: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaSilverBirch", skip_serializing_if = "Option::is_none")]
    pub basal_area_silver_birch: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Storey")]
    pub storey: StoreyType,
    #[serde(rename = "DiameterMax", skip_serializing_if = "Option::is_none")]
    pub diameter_max: Option<PositiveInteger3digitsType>,
    #[serde(rename = "DiameterMin", skip_serializing_if = "Option::is_none")]
    pub diameter_min: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameterPine", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_pine: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameterSpruce", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_spruce: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameterHardWood", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_hard_wood: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameterSilverBirch", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_silver_birch: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: PositiveInteger3digitsType,
    #[serde(rename = "StemCountTarget")]
    pub stem_count_target: PositiveInteger2digitsType,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "ThinningTooExcessive")]
    pub thinning_too_excessive: YesNoType,
    #[serde(rename = "ThinningTooExcessiveCount", skip_serializing_if = "Option::is_none")]
    pub thinning_too_excessive_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "ClearCutting")]
    pub clear_cutting: YesNoType,
    #[serde(rename = "OtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub other_tree_species: Option<OtherTreeSpeciesType>,
    #[serde(rename = "StemDamages", skip_serializing_if = "Option::is_none")]
    pub stem_damages: Option<PositiveInteger3digitsType>,
    #[serde(rename = "RootDamages", skip_serializing_if = "Option::is_none")]
    pub root_damages: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<String1000Type>,
    #[serde(rename = "CorrectHeightStumps", skip_serializing_if = "Option::is_none")]
    pub correct_height_stumps: Option<PositiveInteger3digitsType>,
    #[serde(rename = "TooHeightStumps", skip_serializing_if = "Option::is_none")]
    pub too_height_stumps: Option<PositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathTooDeep", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_too_deep: Option<PositiveInteger2digitsType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "Measurable")]
    pub measurable: YesNoType,
    #[serde(rename = "MeasurerId")]
    pub measurer_id: String20Type,
    #[serde(rename = "MeasurerName")]
    pub measurer_name: String50Type,
    #[serde(rename = "GeometryReal")]
    pub geometry_real: PointGeometryType,
    #[serde(rename = "PilotName", skip_serializing_if = "Option::is_none")]
    pub pilot_name: Option<String50Type>,
    #[serde(rename = "FertileType", skip_serializing_if = "Option::is_none")]
    pub fertile_type: Option<MaterialCodeType>,
    #[serde(rename = "MeanVolume", skip_serializing_if = "Option::is_none")]
    pub mean_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Hoppers")]
    pub hoppers: HoppersType,
    #[serde(rename = "GeometryRequired", skip_serializing_if = "Option::is_none")]
    pub geometry_required: Option<PointGeometryType>,
    #[serde(rename = "RadiusReduction", skip_serializing_if = "Option::is_none")]
    pub radius_reduction: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "ReductionCount", skip_serializing_if = "Option::is_none")]
    pub reduction_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "ReductionAvgDiameter", skip_serializing_if = "Option::is_none")]
    pub reduction_avg_diameter: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "ReductionStumpHeight", skip_serializing_if = "Option::is_none")]
    pub reduction_stump_height: Option<FinalAuditAnswerType>,
    #[serde(rename = "RemainingTrees", skip_serializing_if = "Option::is_none")]
    pub remaining_trees: Option<FinalAuditAnswerType>,
    #[serde(rename = "Damages", skip_serializing_if = "Option::is_none")]
    pub damages: Option<FinalAuditAnswerType>,
    #[serde(rename = "HarvestedTrees", skip_serializing_if = "Option::is_none")]
    pub harvested_trees: Option<FinalAuditAnswerType>,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<PositiveIntegerType>,
    #[serde(rename = "PlantLocationErrorCount", skip_serializing_if = "Option::is_none")]
    pub plant_location_error_count: Option<PositiveIntegerType>,
    #[serde(rename = "PlantingDepthErrorCount", skip_serializing_if = "Option::is_none")]
    pub planting_depth_error_count: Option<PositiveIntegerType>,
    #[serde(rename = "PlantSealingErrorCount", skip_serializing_if = "Option::is_none")]
    pub plant_sealing_error_count: Option<PositiveIntegerType>,
    #[serde(rename = "ClodMoistureErrorCount", skip_serializing_if = "Option::is_none")]
    pub clod_moisture_error_count: Option<PositiveIntegerType>,
    #[serde(rename = "PlantPositionErrorCount", skip_serializing_if = "Option::is_none")]
    pub plant_position_error_count: Option<PositiveIntegerType>,
    #[serde(rename = "DumplingLevelingErrorCount", skip_serializing_if = "Option::is_none")]
    pub dumpling_leveling_error_count: Option<PositiveIntegerType>,
    #[serde(rename = "DamagedPlantsErrorCount", skip_serializing_if = "Option::is_none")]
    pub damaged_plants_error_count: Option<PositiveIntegerType>,
    #[serde(rename = "PlantCount")]
    pub plant_count: PositiveInteger3digitsType,
    #[serde(rename = "PlantedSeedlingCountInCultivatedSpots", skip_serializing_if = "Option::is_none")]
    pub planted_seedling_count_in_cultivated_spots: Option<PositiveInteger3digitsType>,
    #[serde(rename = "PlantedSeedlingCountInNotCultivatedSpots", skip_serializing_if = "Option::is_none")]
    pub planted_seedling_count_in_not_cultivated_spots: Option<PositiveInteger3digitsType>,
    #[serde(rename = "NearestSeedlingPlantingDepth", skip_serializing_if = "Option::is_none")]
    pub nearest_seedling_planting_depth: Option<PositiveInteger2digitsType>,
    #[serde(rename = "NearestSeedlingDistance", skip_serializing_if = "Option::is_none")]
    pub nearest_seedling_distance: Option<PositiveInteger3digitsType>,
    #[serde(rename = "NearestSeedlingSoilCompressed", skip_serializing_if = "Option::is_none")]
    pub nearest_seedling_soil_compressed: Option<YesNoType>,
    #[serde(rename = "RockySoil", skip_serializing_if = "Option::is_none")]
    pub rocky_soil: Option<YesNoType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "CultivationScope", skip_serializing_if = "Option::is_none")]
    pub cultivation_scope: Option<FinalAuditAnswerType>,
    #[serde(rename = "BulgeHeight", skip_serializing_if = "Option::is_none")]
    pub bulge_height: Option<FinalAuditAnswerType>,
    #[serde(rename = "MineralSoilLayer", skip_serializing_if = "Option::is_none")]
    pub mineral_soil_layer: Option<FinalAuditAnswerType>,
    #[serde(rename = "SoilConditioningDepth", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_depth: Option<FinalAuditAnswerType>,
    #[serde(rename = "PlantingSiteCount")]
    pub planting_site_count: PositiveInteger2digitsType,
    #[serde(rename = "ScalpedMoundsCount", skip_serializing_if = "Option::is_none")]
    pub scalped_mounds_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "DitchMoundsCount", skip_serializing_if = "Option::is_none")]
    pub ditch_mounds_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "TurnoverMoundsCount", skip_serializing_if = "Option::is_none")]
    pub turnover_mounds_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "ScalpsCount", skip_serializing_if = "Option::is_none")]
    pub scalps_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "NearestCultivatedSpotLength", skip_serializing_if = "Option::is_none")]
    pub nearest_cultivated_spot_length: Option<PositiveInteger3digitsType>,
    #[serde(rename = "NearestCultivatedSpotWidth", skip_serializing_if = "Option::is_none")]
    pub nearest_cultivated_spot_width: Option<PositiveInteger3digitsType>,
    #[serde(rename = "NearestCultivatedSpotHeight", skip_serializing_if = "Option::is_none")]
    pub nearest_cultivated_spot_height: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CompactSoilTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifierType {
    #[serde(rename = "Type")]
    pub r#type: integer,
    #[serde(rename = "Value")]
    pub value: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalUpdateType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ImageCount")]
    pub image_count: PositiveInteger2digitsType,
    #[serde(rename = "LoadNumber", skip_serializing_if = "Option::is_none")]
    pub load_number: Option<String20Type>,
    #[serde(rename = "LoadPaymentReference", skip_serializing_if = "Option::is_none")]
    pub load_payment_reference: Option<String50Type>,
    #[serde(rename = "WorkingSitePlanningStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_status: Option<WorkingSitePlanningStatusType>,
    #[serde(rename = "WorkingSitePlanningOperation", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_operation: Option<WorkingSitePlanningOperationStatusType>,
    #[serde(rename = "WorkingSitePlanningInfo", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_info: Option<String3000Type>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "Storages", skip_serializing_if = "Option::is_none")]
    pub storages: Option<StoragesType>,
    #[serde(rename = "StoragesForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "StorageProposals", skip_serializing_if = "Option::is_none")]
    pub storage_proposals: Option<StoragesType>,
    #[serde(rename = "StoragesProposalForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_proposal_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "ProductUserIds", skip_serializing_if = "Option::is_none")]
    pub product_user_ids: Option<ProductUserIdsType>,
    #[serde(rename = "CanCultivateInAutumn", skip_serializing_if = "Option::is_none")]
    pub can_cultivate_in_autumn: Option<YesNoType>,
    #[serde(rename = "DelinationObjectOrderId", skip_serializing_if = "Option::is_none")]
    pub delination_object_order_id: Option<String200Type>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "WorkingsiteInfo", skip_serializing_if = "Option::is_none")]
    pub workingsite_info: Option<String3000Type>,
    #[serde(rename = "PurchaseContractExtraInfo", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_extra_info: Option<String3000Type>,
    #[serde(rename = "EnvironmentalObjectInfo", skip_serializing_if = "Option::is_none")]
    pub environmental_object_info: Option<String3000Type>,
    #[serde(rename = "WorkingSafetyInfo", skip_serializing_if = "Option::is_none")]
    pub working_safety_info: Option<String3000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationType {
    #[serde(rename = "@id")]
    pub id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateType>,
    #[serde(rename = "SilvicultureValidity", skip_serializing_if = "Option::is_none")]
    pub silviculture_validity: Option<SilvicultureValidityType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductsType>,
    #[serde(rename = "OfferWorkingSiteSilvicultureText", skip_serializing_if = "Option::is_none")]
    pub offer_working_site_silviculture_text: Option<string>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummaryType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "MeanAge")]
    pub mean_age: MeanAgeType,
    #[serde(rename = "BasalArea")]
    pub basal_area: BasalAreaType,
    #[serde(rename = "StemCount")]
    pub stem_count: StemCountType,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanStumpDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_stump_diameter: Option<DiameterType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: MeanHeightType,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume")]
    pub volume: VolumeType,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth")]
    pub volume_growth: VolumeGrowthType,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<DevelopmentClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<TreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartsType {
    #[serde(rename = "TargetPart")]
    pub target_part: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditPlantManagementType {
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<WorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<DateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceTaxType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType2 {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "UnseparetedParcelTypeChar", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_type_char: Option<UnseparetedParcelTypeCharType>,
    #[serde(rename = "UnseparetedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_number: Option<UnseparetedParcelNumberType>,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: RealEstateNameType,
    #[serde(rename = "LocationMunicipalityNumber")]
    pub location_municipality_number: MunicipalityNumberType,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<MunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolsType {
    #[serde(rename = "Symbol")]
    pub symbol: Vec<MapSymbolDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreeGroupType {
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<NotEmptyTreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilDataType {
    #[serde(rename = "SoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub soil_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometryType {
    #[serde(rename = "Id")]
    pub id: string,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "LineString")]
    pub line_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataDate2Type {
    #[serde(rename = "@type")]
    pub r#type: TreeStandDataMomentType,
    #[serde(rename = "@date")]
    pub date: DateType,
    #[serde(rename = "@inventoryMethod")]
    pub inventory_method: InventoryMethodType,
    #[serde(rename = "AlternativeIdentifier", skip_serializing_if = "Option::is_none")]
    pub alternative_identifier: Option<AlternativeIdentifierType>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tree_strata: Option<TreeStrata2Type>,
    #[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dead_tree_strata: Option<String>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tree_stand_summary: Option<TreeStandSummary2Type>,
    #[serde(rename = "Trees", skip_serializing_if = "Option::is_none")]
    pub trees: Option<String>,
    #[serde(rename = "StemDistribution", skip_serializing_if = "Option::is_none")]
    pub stem_distribution: Option<String>,
    #[serde(rename = "StemDistributionStrata", skip_serializing_if = "Option::is_none")]
    pub stem_distribution_strata: Option<String>,
    #[serde(rename = "OperationTreeReduction", skip_serializing_if = "Option::is_none")]
    pub operation_tree_reduction: Option<OperationTreeReductionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "ControlBasicData", skip_serializing_if = "Option::is_none")]
    pub control_basic_data: Option<ControlBasicDataType>,
    #[serde(rename = "ControlForestUseDeclaration", skip_serializing_if = "Option::is_none")]
    pub control_forest_use_declaration: Option<ControlForestUseDeclarationType>,
    #[serde(rename = "ControlObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub control_object_basic_data: Option<ControlObjectBasicDataType>,
    #[serde(rename = "ControlStandBasicData", skip_serializing_if = "Option::is_none")]
    pub control_stand_basic_data: Option<ControlStandBasicDataType>,
    #[serde(rename = "SamplePlotBasicData", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basic_data: Option<SamplePlotBasicDataType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<BaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "Damages", skip_serializing_if = "Option::is_none")]
    pub damages: Option<DamageDataType>,
    #[serde(rename = "ControlDataSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub control_data_special_features: Option<ControlDataSpecialFeaturesType>,
    #[serde(rename = "HarvestingSignData", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_data: Option<HarvestingSignDataType>,
    #[serde(rename = "ObjectOverallEvaluationData", skip_serializing_if = "Option::is_none")]
    pub object_overall_evaluation_data: Option<ControlOverallEvaluationDataType>,
    #[serde(rename = "ControlEvaluations", skip_serializing_if = "Option::is_none")]
    pub control_evaluations: Option<ControlEvaluationsType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<SelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<SelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<SelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<SelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDecisionDataType {
    #[serde(flatten)]
    pub base: ForestCentreDataType,
    #[serde(rename = "ForestCentreDecision")]
    pub forest_centre_decision: ForestCentreDecisionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandId")]
    pub stand_id: String20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBasisListType {
    #[serde(rename = "FeeBasisListItem")]
    pub fee_basis_list_item: Vec<FeebasisListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOffersType {
    #[serde(rename = "RelatedCallForOffer")]
    pub related_call_for_offer: Vec<RelatedCallForOfferType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: String20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: PositiveIntegerType,
    #[serde(rename = "IsForestHaulageDistanceContinued")]
    pub is_forest_haulage_distance_continued: YesNoType,
    #[serde(rename = "StorageFinished")]
    pub storage_finished: YesNoType,
    #[serde(rename = "StorageFinishedDate", skip_serializing_if = "Option::is_none")]
    pub storage_finished_date: Option<TimeStampType>,
    #[serde(rename = "WorkingSiteFinished")]
    pub working_site_finished: YesNoType,
    #[serde(rename = "WorkingSiteFinishedDate", skip_serializing_if = "Option::is_none")]
    pub working_site_finished_date: Option<TimeStampType>,
    #[serde(rename = "NotificationDate")]
    pub notification_date: TimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
    #[serde(rename = "AssortmentsChanges", skip_serializing_if = "Option::is_none")]
    pub assortments_changes: Option<AssortmentsChangesType>,
    #[serde(rename = "CommonMessages", skip_serializing_if = "Option::is_none")]
    pub common_messages: Option<CommonMessagesType>,
    #[serde(rename = "ScaleFactors", skip_serializing_if = "Option::is_none")]
    pub scale_factors: Option<ScaleFactorsType>,
    #[serde(rename = "ClientApplicationId", skip_serializing_if = "Option::is_none")]
    pub client_application_id: Option<ClientApplicationIdType>,
    #[serde(rename = "LoadRange", skip_serializing_if = "Option::is_none")]
    pub load_range: Option<LoadRangeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceType {
    #[serde(rename = "SubsidyApplierId")]
    pub subsidy_applier_id: IdStringNotEmptyType,
    #[serde(rename = "MainApplier")]
    pub main_applier: YesNoType,
    #[serde(rename = "ShareOfOwnerShip", skip_serializing_if = "Option::is_none")]
    pub share_of_owner_ship: Option<PercentType>,
    #[serde(rename = "OwnerShipType")]
    pub owner_ship_type: OwnerShipTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRisksType {
    #[serde(rename = "WorkSafetyRiskDescription")]
    pub work_safety_risk_description: Vec<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationBankAccountType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "BankAccount")]
    pub bank_account: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEstatesType {
    #[serde(rename = "LocationEstate")]
    pub location_estate: Vec<LocationEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpecificAdditionalVariableType {
    #[serde(rename = "ForestDepotAccessibility")]
    pub forest_depot_accessibility: ForestDepotAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditHarvestingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "HarvesterId", skip_serializing_if = "Option::is_none")]
    pub harvester_id: Option<String20Type>,
    #[serde(rename = "ForwarderId", skip_serializing_if = "Option::is_none")]
    pub forwarder_id: Option<String20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonsType {
    #[serde(rename = "DisQualificationReason", skip_serializing_if = "Option::is_none")]
    pub dis_qualification_reason: Option<Vec<DisQualificationReasonDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JustificationsType {
    #[serde(rename = "Justification")]
    pub justification: Vec<String5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlDataType {
    #[serde(flatten)]
    pub base: ForestCentreDataType,
    #[serde(rename = "ControlObjectData")]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamagesType {
    #[serde(rename = "PreviousMooseDamage")]
    pub previous_moose_damage: Vec<PreviousMooseDamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocationsType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceLocations", skip_serializing_if = "Option::is_none")]
    pub resource_locations: Option<ResourceLocationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingsType {
    #[serde(rename = "Cutting")]
    pub cutting: Vec<CuttingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferType {
    #[serde(rename = "@callForOfferId")]
    pub call_for_offer_id: string,
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@versionNo")]
    pub version_no: int,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "OfferBusinessSender")]
    pub offer_business_sender: OfferBusinessSenderType,
    #[serde(rename = "CallForOfferBusinessSender", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_business_sender: Option<CallForOfferBusinessSenderType>,
    #[serde(rename = "TechnicalContactPerson", skip_serializing_if = "Option::is_none")]
    pub technical_contact_person: Option<TechnicalContactPersonType>,
    #[serde(rename = "OfferDate")]
    pub offer_date: OfferDateType,
    #[serde(rename = "OfferExpirationDate")]
    pub offer_expiration_date: OfferExpirationDateType,
    #[serde(rename = "OfferText", skip_serializing_if = "Option::is_none")]
    pub offer_text: Option<OfferTextType>,
    #[serde(rename = "CallForOfferText", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_text: Option<CallForOfferTextType>,
    #[serde(rename = "OfferWorkingSites")]
    pub offer_working_sites: String,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<CallForOfferWoodTradeInfoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesWithOwnersInformationType2 {
    #[serde(rename = "RealEstateOwners")]
    pub real_estate_owners: RealEstateOwnersType,
    #[serde(rename = "RealEstates")]
    pub real_estates: BaseRealEstatesType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensationByLegislationType {
    #[serde(rename = "ReceivedCompensation")]
    pub received_compensation: YesNoType,
    #[serde(rename = "CompensationDescription", skip_serializing_if = "Option::is_none")]
    pub compensation_description: Option<string>,
    #[serde(rename = "CompensationAmount", skip_serializing_if = "Option::is_none")]
    pub compensation_amount: Option<Decimal7And2PositiveType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumesType {
    #[serde(rename = "StemTypeVolume")]
    pub stem_type_volume: Vec<StemTypeVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstatesType {
    #[serde(rename = "FinancingActRealEstate")]
    pub financing_act_real_estate: Vec<FinancingActRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandInfoType {
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(flatten)]
    pub base: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitesType {
    #[serde(rename = "WorkingSite", skip_serializing_if = "Option::is_none")]
    pub working_site: Option<Vec<WorkingSiteType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetType {
    #[serde(rename = "Status2")]
    pub status2: ChangeStateType,
    #[serde(rename = "TargetId")]
    pub target_id: string,
    #[serde(rename = "TargetNumber")]
    pub target_number: PositiveDecimalMax5IntegralPartMax1FractionalPartType,
    #[serde(rename = "BasePartNumber")]
    pub base_part_number: VirtaPartNumberType,
    #[serde(rename = "EstablishedPartNumber")]
    pub established_part_number: VirtaPartNumberType,
    #[serde(rename = "TargetAnnouncedAmount")]
    pub target_announced_amount: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
    #[serde(rename = "HabitatAdvertisement")]
    pub habitat_advertisement: VirtaHabitatAdvertisementType,
    #[serde(rename = "TargetExtraInfo")]
    pub target_extra_info: VirtaExtraInfoType,
    #[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
    pub polygon: Option<String>,
    #[serde(rename = "TargetParts", skip_serializing_if = "Option::is_none")]
    pub target_parts: Option<TargetPartsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDataType {
    #[serde(rename = "CompletionDate")]
    pub completion_date: CompletionDateType,
    #[serde(rename = "CompletionYear")]
    pub completion_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferredContactingMethodsType {
    #[serde(rename = "PreferredContactingMethod")]
    pub preferred_contacting_method: Vec<PreferredContactingMethodType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageDataType {
    #[serde(rename = "Damage")]
    pub damage: Vec<DamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDefType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@mainType")]
    pub main_type: MainTypeType,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "OperationType")]
    pub operation_type: OperationTypeType,
    #[serde(rename = "ProposalData", skip_serializing_if = "Option::is_none")]
    pub proposal_data: Option<ProposalDataType>,
    #[serde(rename = "CompletionData", skip_serializing_if = "Option::is_none")]
    pub completion_data: Option<ExtendedCompletionDataType>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatusType>,
    #[serde(rename = "DifficultyClass", skip_serializing_if = "Option::is_none")]
    pub difficulty_class: Option<DifficultyClassType>,
    #[serde(rename = "OperativeData", skip_serializing_if = "Option::is_none")]
    pub operative_data: Option<OperativeDataType>,
    #[serde(rename = "OperationInfo", skip_serializing_if = "Option::is_none")]
    pub operation_info: Option<OperationInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "Specifications", skip_serializing_if = "Option::is_none")]
    pub specifications: Option<SpecificationsType>,
    #[serde(rename = "Cutting")]
    pub cutting: CuttingType,
    #[serde(rename = "Silviculture")]
    pub silviculture: SilvicultureType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "OperationStatus")]
    pub operation_status: OperationStatusType,
    #[serde(rename = "ActingDate")]
    pub acting_date: ActingDateType,
    #[serde(rename = "ResponsibleActor", skip_serializing_if = "Option::is_none")]
    pub responsible_actor: Option<ResponsibleActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitialLoadType {
    #[serde(rename = "PartitialLoadId")]
    pub partitial_load_id: nonNegativeInteger,
    #[serde(rename = "Assortment")]
    pub assortment: String50Type,
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "LoadVolume", skip_serializing_if = "Option::is_none")]
    pub load_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "LoadGreenMass")]
    pub load_green_mass: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "StemCountOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub stem_count_other_tree_species: Option<PositiveInteger2digitsType>,
    #[serde(rename = "MeanHeightOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub mean_height_other_tree_species: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub basal_area_other_tree_species: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanDiameterOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_other_tree_species: Option<PositiveInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidiesType {
    #[serde(rename = "Subsidy")]
    pub subsidy: Vec<SubsidyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatusType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WorkingSiteNumberType>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "Status")]
    pub status: WorkingSiteStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FSFValidityType {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratum2Type {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<StratumNumberType>,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<BasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<MeanHeightType>,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<SeedlingOriginType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<Decimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<ERPIdType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Volume")]
    pub volume: PositiveInteger4digitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<WorkCodeUnitType>,
    #[serde(rename = "Loads")]
    pub loads: PositiveInteger3digitsType,
    #[serde(rename = "Day")]
    pub day: DateType,
    #[serde(rename = "SentStorageVolume")]
    pub sent_storage_volume: Decimal3FractionDigitsType,
    #[serde(rename = "SentWorkingSiteVolume")]
    pub sent_working_site_volume: Decimal3FractionDigitsType,
    #[serde(rename = "HarvesterVolume", skip_serializing_if = "Option::is_none")]
    pub harvester_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "ForestHaulageDistanceContinued", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance_continued: Option<PositiveIntegerType>,
    #[serde(rename = "Covered")]
    pub covered: YesNoType,
    #[serde(rename = "HasAssortmentChanges")]
    pub has_assortment_changes: YesNoType,
    #[serde(rename = "ScaleTransformation")]
    pub scale_transformation: YesNoType,
    #[serde(rename = "Finished")]
    pub finished: YesNoType,
    #[serde(rename = "DeliveryUserId", skip_serializing_if = "Option::is_none")]
    pub delivery_user_id: Option<String50Type>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String50Type>,
    #[serde(rename = "CodeGroup")]
    pub code_group: AssortmentGroupType,
    #[serde(rename = "BeginDate")]
    pub begin_date: DateType,
    #[serde(rename = "EndDate")]
    pub end_date: DateType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "PlannedVolume")]
    pub planned_volume: Decimal3FractionDigitsType,
    #[serde(rename = "HarvestedVolume")]
    pub harvested_volume: Decimal3FractionDigitsType,
    #[serde(rename = "HarvestedVolumeAccounted")]
    pub harvested_volume_accounted: Decimal3FractionDigitsType,
    #[serde(rename = "ForwardedVolume")]
    pub forwarded_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ForwardedVolumeAccounted")]
    pub forwarded_volume_accounted: Decimal3FractionDigitsType,
    #[serde(rename = "ForestHaulageDistance")]
    pub forest_haulage_distance: PositiveInteger4digitsType,
    #[serde(rename = "VolumeLeft")]
    pub volume_left: Decimal6TotalDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartOfProjectType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AreaNo")]
    pub area_no: String,
    #[serde(rename = "FinancingActWorkCode")]
    pub financing_act_work_code: String,
    #[serde(rename = "PlanAndSubsidy")]
    pub plan_and_subsidy: PlanAndSubsidyType,
    #[serde(rename = "PayeesAndRealEstates")]
    pub payees_and_real_estates: String,
    #[serde(rename = "CompletionDataAndSubsidy")]
    pub completion_data_and_subsidy: CompletionDataAndSubsidyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateDataType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: BaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandBasicDataType {
    #[serde(rename = "ControlStandArea", skip_serializing_if = "Option::is_none")]
    pub control_stand_area: Option<AreaType>,
    #[serde(rename = "InspectionMethod", skip_serializing_if = "Option::is_none")]
    pub inspection_method: Option<InspectionMethodType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "SpecificationCode")]
    pub specification_code: SpecificationCodeType,
    #[serde(rename = "SpecificationText", skip_serializing_if = "Option::is_none")]
    pub specification_text: Option<String2000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeInfoType {
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Productivity")]
    pub productivity: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
    #[serde(rename = "ProductivityUnit")]
    pub productivity_unit: WorkCodeUnitType,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "Amount")]
    pub amount: Decimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String100Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: String20Type,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "ControlReferenceMass")]
    pub control_reference_mass: Decimal1FractionDigitType,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
    #[serde(rename = "ScaleData")]
    pub scale_data: Vec<ScaleDataType>,
    #[serde(rename = "Calibration", skip_serializing_if = "Option::is_none")]
    pub calibration: Option<Vec<CalibrationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationType {
    #[serde(rename = "OperationCode", skip_serializing_if = "Option::is_none")]
    pub operation_code: Option<ObjectProtectionOperationCodeType>,
    #[serde(rename = "OperationDescription", skip_serializing_if = "Option::is_none")]
    pub operation_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptancesType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "BusinessAcceptance", skip_serializing_if = "Option::is_none")]
    pub business_acceptance: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceItemType {
    #[serde(rename = "AverageStemVolume")]
    pub average_stem_volume: Decimal3FractionDigitsType,
    #[serde(rename = "UnitPrice")]
    pub unit_price: UnitPriceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeDataType {
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "AmountPlanned")]
    pub amount_planned: Decimal3FractionDigitsType,
    #[serde(rename = "AmountNotified")]
    pub amount_notified: Decimal3FractionDigitsType,
    #[serde(rename = "AmountAccounted")]
    pub amount_accounted: Decimal3FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
    #[serde(rename = "AmountLeft")]
    pub amount_left: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: StemTypeType,
    #[serde(rename = "Quantity")]
    pub quantity: QuantityType,
    #[serde(rename = "QuantityUnit")]
    pub quantity_unit: ExtendedQuantityUnitType,
    #[serde(rename = "AssortmentInfo")]
    pub assortment_info: AssortmentInfoType,
    #[serde(flatten)]
    pub base: AssortmentCompactElementsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "TreeNumber")]
    pub tree_number: TreeNumberType,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<StratumNumberType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "Count")]
    pub count: CountType,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<DiameterType>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<HeightType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "Status5")]
    pub status5: ChangeStateType,
    #[serde(rename = "TreeDiameter", skip_serializing_if = "Option::is_none")]
    pub tree_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeHeight", skip_serializing_if = "Option::is_none")]
    pub tree_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "TreeCount", skip_serializing_if = "Option::is_none")]
    pub tree_count: Option<StemCountType>,
    #[serde(rename = "WorkQuality", skip_serializing_if = "Option::is_none")]
    pub work_quality: Option<VirtaWorkQualityType>,
    #[serde(rename = "DamageClass", skip_serializing_if = "Option::is_none")]
    pub damage_class: Option<VirtaDamageClassType>,
    #[serde(rename = "StemCount")]
    pub stem_count: PositiveInteger2digitsType,
    #[serde(rename = "MeanHeight")]
    pub mean_height: Decimal1FractionDigitType,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesForestHaulageDistancesType {
    #[serde(rename = "StorageForestHaulageDistance")]
    pub storage_forest_haulage_distance: Vec<StorageForestHaulageDistanceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "PolygonGeometryGroup")]
    pub polygon_geometry_group: String,
    #[serde(rename = "Id")]
    pub id: string,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "Polygon")]
    pub polygon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "SelfMonitoringBasicData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_basic_data: Option<SelfMonitoringBasicDataType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "Works", skip_serializing_if = "Option::is_none")]
    pub works: Option<WorksType>,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_plant_management: Option<SelfMonitoringWorkingSiteFinalAuditPlantManagementType>,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_plant_management: Option<SelfMonitoringWorkingSiteQualityControlPlantManagementType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<SelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<SelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<SelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<SelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<SelfMonitoringImageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistributionType {
    #[serde(rename = "CumulativePoint")]
    pub cumulative_point: Vec<CumulativePointType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "DocumentClass")]
    pub document_class: DocumentClassType,
    #[serde(rename = "DocumentDescription", skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescriptionType>,
    #[serde(rename = "DocumentDate", skip_serializing_if = "Option::is_none")]
    pub document_date: Option<DateType>,
    #[serde(rename = "DocumentFileName")]
    pub document_file_name: DocumentFileNameType,
    #[serde(rename = "FileType")]
    pub file_type: FileTypeType,
    #[serde(rename = "FileBinary")]
    pub file_binary: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubContractorsType {
    #[serde(rename = "SubContractor", skip_serializing_if = "Option::is_none")]
    pub sub_contractor: Option<Vec<String20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "CompletedWorkAmount")]
    pub completed_work_amount: AmountType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureType {
    #[serde(rename = "CuttingRelated", skip_serializing_if = "Option::is_none")]
    pub cutting_related: Option<CuttingRelatedType>,
    #[serde(rename = "Cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<CostType>,
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
pub struct LocatedSpecialFeature3Type {
    #[serde(flatten)]
    pub base: BasicFeature3Type,
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub simple_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadTreeStratumType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "DeadTreeType")]
    pub dead_tree_type: DeadTreeTypeType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceType {
    #[serde(rename = "@message")]
    pub message: MessageTypeType,
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: string,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "BusinessMessageTimeStamp")]
    pub business_message_time_stamp: BusinessMessageTimeStampType,
    #[serde(rename = "BusinessAcceptanceActor")]
    pub business_acceptance_actor: BusinessAcceptanceActorType,
    #[serde(rename = "BusinessAcceptanceStatus")]
    pub business_acceptance_status: BusinessAcceptanceStatusType,
    #[serde(rename = "BusinessAcceptanceId", skip_serializing_if = "Option::is_none")]
    pub business_acceptance_id: Option<BusinessAcceptanceIdType>,
    #[serde(rename = "AdditionalInformation")]
    pub additional_information: AdditionalInformationType,
    #[serde(rename = "BusinessAcceptanceDate")]
    pub business_acceptance_date: BusinessAcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeType {
    #[serde(rename = "ReplyTo")]
    pub reply_to: String50Type,
    #[serde(rename = "StatusCode")]
    pub status_code: PositiveInteger3digitsType,
    #[serde(rename = "StatusMessage")]
    pub status_message: String1000Type,
    #[serde(rename = "OriginalMessageType")]
    pub original_message_type: String50Type,
    #[serde(rename = "StatusMessages", skip_serializing_if = "Option::is_none")]
    pub status_messages: Option<StatusMessageLanguageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionsType {
    #[serde(rename = "DiameterSection")]
    pub diameter_section: Vec<SectionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestStatisticsDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "CompanyID")]
    pub company_i_d: CompanyIDType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "RoundWoodSalesRows", skip_serializing_if = "Option::is_none")]
    pub round_wood_sales_rows: Option<String>,
    #[serde(rename = "OperationRows", skip_serializing_if = "Option::is_none")]
    pub operation_rows: Option<OperationRowsType>,
    #[serde(rename = "LoggingsRows", skip_serializing_if = "Option::is_none")]
    pub loggings_rows: Option<LoggingsRowsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListType {
    #[serde(rename = "ValueListItem")]
    pub value_list_item: Vec<ValueListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationOtherOperationsType {
    #[serde(rename = "DeclarationOtherOperation")]
    pub declaration_other_operation: Vec<DeclarationOtherOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeBulksType {
    #[serde(rename = "StemTypeBulk", skip_serializing_if = "Option::is_none")]
    pub stem_type_bulk: Option<Vec<StemTypeBulkType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeebasisListItemType {
    #[serde(rename = "Id")]
    pub id: PositiveIntegerType,
    #[serde(rename = "FeeText")]
    pub fee_text: String50Type,
    #[serde(rename = "FeeUnit")]
    pub fee_unit: String10Type,
    #[serde(rename = "DefaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String50Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTravelNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "TravelStartTime")]
    pub travel_start_time: TimeStampType,
    #[serde(rename = "TravelEndTime")]
    pub travel_end_time: TimeStampType,
    #[serde(rename = "Vehicle")]
    pub vehicle: VehicleType,
    #[serde(rename = "Kilometers")]
    pub kilometers: PositiveInteger4digitsType,
    #[serde(rename = "Route")]
    pub route: String200Type,
    #[serde(rename = "KilometersWithTrailer", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_trailer: Option<PositiveInteger4digitsType>,
    #[serde(rename = "KilometersWithCaravan", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_caravan: Option<PositiveInteger4digitsType>,
    #[serde(rename = "KilometersWithBreakHouse", skip_serializing_if = "Option::is_none")]
    pub kilometers_with_break_house: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson1", skip_serializing_if = "Option::is_none")]
    pub extra_person1: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson2", skip_serializing_if = "Option::is_none")]
    pub extra_person2: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson3", skip_serializing_if = "Option::is_none")]
    pub extra_person3: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPerson4", skip_serializing_if = "Option::is_none")]
    pub extra_person4: Option<PositiveInteger4digitsType>,
    #[serde(rename = "ExtraPersonText", skip_serializing_if = "Option::is_none")]
    pub extra_person_text: Option<String200Type>,
    #[serde(rename = "SittingMoneyKilometers", skip_serializing_if = "Option::is_none")]
    pub sitting_money_kilometers: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMemberType {
    #[serde(rename = "ResourceId")]
    pub resource_id: ShortERPIdType,
    #[serde(rename = "UserId")]
    pub user_id: ShortERPIdType,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignDataType {
    #[serde(rename = "RootDamagePercentage", skip_serializing_if = "Option::is_none")]
    pub root_damage_percentage: Option<PositiveIntegerType>,
    #[serde(rename = "StemDamagePercentage", skip_serializing_if = "Option::is_none")]
    pub stem_damage_percentage: Option<PositiveIntegerType>,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathMeanDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_mean_distance: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathMeanWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_mean_width: Option<Decimal5_1Type>,
    #[serde(rename = "VehiclePathSubsidenceLength", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_length: Option<Decimal3_1Type>,
    #[serde(rename = "VehiclePathSubsidencePercentage", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_percentage: Option<Decimal3_1Type>,
    #[serde(rename = "CuttingBy", skip_serializing_if = "Option::is_none")]
    pub cutting_by: Option<VirtaCuttingByMachineType>,
    #[serde(rename = "HarvestingSeason", skip_serializing_if = "Option::is_none")]
    pub harvesting_season: Option<VirtaHarvestingSeasonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkCompletionRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringNotEmptyType,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "CompletedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub completed_work_amount: Option<AmountType>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "PaymentReference", skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<PaymentsReferenceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellersType {
    #[serde(rename = "@subsidyPossibility")]
    pub subsidy_possibility: YesNoNotKnownType,
    #[serde(rename = "@sellerGroup")]
    pub seller_group: SellerGroupType,
    #[serde(rename = "Seller")]
    pub seller: Vec<SellerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicFeature1Type {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@infoProviderRole")]
    pub info_provider_role: InfoProviderRoleType,
    #[serde(rename = "@infoProviderOrganizationName")]
    pub info_provider_organization_name: OrganizationNameType,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub main_feature: Option<YesNoType>,
    #[serde(rename = "FeatureType")]
    pub feature_type: FeatureTypeType,
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub using_restrictions: Option<String>,
    #[serde(rename = "Validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<ValidityType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<FeatureInfoType>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<FeatureAdditionalInfoType>,
    #[serde(rename = "InventoryDate", skip_serializing_if = "Option::is_none")]
    pub inventory_date: Option<DateType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDataType {
    #[serde(rename = "MeasurementId")]
    pub measurement_id: PositiveIntegerType,
    #[serde(rename = "Measurer")]
    pub measurer: String50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "SelectionType")]
    pub selection_type: String10Type,
    #[serde(rename = "Temperature")]
    pub temperature: integer,
    #[serde(rename = "ProductKey")]
    pub product_key: ERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: PositiveInteger5digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: PositiveInteger5digitsType,
    #[serde(rename = "LogCount")]
    pub log_count: PositiveInteger2digitsType,
    #[serde(rename = "ControlLogCount")]
    pub control_log_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasicDataType {
    #[serde(rename = "@timeStamp")]
    pub time_stamp: TimeStampType,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<TwoDigitPositiveIntegerType>,
    #[serde(rename = "SamplePlotSize", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size: Option<SamplePlotSizeType>,
    #[serde(rename = "SamplePlotSizeTreeReduction", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size_tree_reduction: Option<SamplePlotSizeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@realEstateid")]
    pub real_estateid: string,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<ReferenceType>,
    #[serde(rename = "StandId")]
    pub stand_id: String20Type,
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
    #[serde(rename = "RegisterUnitId", skip_serializing_if = "Option::is_none")]
    pub register_unit_id: Option<String20Type>,
    #[serde(rename = "ParcelNo", skip_serializing_if = "Option::is_none")]
    pub parcel_no: Option<PositiveInteger6digitsType>,
    #[serde(rename = "ParcelLabel", skip_serializing_if = "Option::is_none")]
    pub parcel_label: Option<String100Type>,
    #[serde(rename = "ForestPlanStandId", skip_serializing_if = "Option::is_none")]
    pub forest_plan_stand_id: Option<String10Type>,
    #[serde(rename = "DitchOrRoadPlanId", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_id: Option<String10Type>,
    #[serde(rename = "DitchOrRoadPlanName", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_name: Option<String100Type>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "StumpTreatment", skip_serializing_if = "Option::is_none")]
    pub stump_treatment: Option<YesNoType>,
    #[serde(rename = "BiomassCollection", skip_serializing_if = "Option::is_none")]
    pub biomass_collection: Option<YesNoType>,
    #[serde(rename = "StumpRaising", skip_serializing_if = "Option::is_none")]
    pub stump_raising: Option<YesNoType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<WideDevelopmentClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<YesNoType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveIntegerType>,
    #[serde(rename = "LabelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<PointGeometryType>,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<String1000Type>,
    #[serde(rename = "ForestUseDeclarationStandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_stand_extra_info: Option<String2000Type>,
    #[serde(rename = "ForestUseDeclarationStandFellingPurpose", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_stand_felling_purpose: Option<CuttingPurposeType>,
    #[serde(rename = "StandTreesCuttingVolume", skip_serializing_if = "Option::is_none")]
    pub stand_trees_cutting_volume: Option<PositiveInteger4digitsType>,
    #[serde(rename = "GrowingTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub growing_tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "TargetDensityGroup", skip_serializing_if = "Option::is_none")]
    pub target_density_group: Option<String>,
    #[serde(rename = "WorkCodes")]
    pub work_codes: WorkCodesType,
    #[serde(rename = "Materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<MaterialsType>,
    #[serde(rename = "StandTreesCutting", skip_serializing_if = "Option::is_none")]
    pub stand_trees_cutting: Option<StandTreesCuttingType>,
    #[serde(rename = "StandTreesCurrent", skip_serializing_if = "Option::is_none")]
    pub stand_trees_current: Option<StandTreesType>,
    #[serde(rename = "StandTreesStratumLeaving", skip_serializing_if = "Option::is_none")]
    pub stand_trees_stratum_leaving: Option<StandTreesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicDataType,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<String>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
    #[serde(rename = "StandWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub stand_wood_trade_info: Option<StandWoodTradeInfoType>,
    #[serde(rename = "StandSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub stand_silviculture_info: Option<StandSilvicultureInfoType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionType {
    #[serde(rename = "FinalAuditType", skip_serializing_if = "Option::is_none")]
    pub final_audit_type: Option<FinalAuditTypeType>,
    #[serde(rename = "FinalAuditerType")]
    pub final_auditer_type: FinalAuditerTypeType,
    #[serde(rename = "FinalAuditer")]
    pub final_auditer: String50Type,
    #[serde(rename = "FinalAuditDate")]
    pub final_audit_date: TimeStampType,
    #[serde(rename = "FinalAuditRequired")]
    pub final_audit_required: YesNoType,
    #[serde(rename = "FinalAuditerId")]
    pub final_auditer_id: String20Type,
    #[serde(rename = "FinalAuditerName")]
    pub final_auditer_name: String50Type,
    #[serde(rename = "FinalAuditIdentifier")]
    pub final_audit_identifier: String50Type,
    #[serde(rename = "FinalAuditIdentifierVersion")]
    pub final_audit_identifier_version: String10Type,
    #[serde(rename = "FinalAuditName", skip_serializing_if = "Option::is_none")]
    pub final_audit_name: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMatrixType {
    #[serde(rename = "PriceItem")]
    pub price_item: Vec<PriceItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteFinalAuditSoilConditioningType {
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<DateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAcceptanceActorType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandType1 {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@realEstateId")]
    pub real_estate_id: string,
    #[serde(rename = "@parcelId")]
    pub parcel_id: string,
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicDataWithGeometryType,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<String>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "PlannedOperationChains", skip_serializing_if = "Option::is_none")]
    pub planned_operation_chains: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentDataType {
    #[serde(rename = "Startdate")]
    pub startdate: DateType,
    #[serde(rename = "Enddate", skip_serializing_if = "Option::is_none")]
    pub enddate: Option<DateType>,
    #[serde(rename = "WorkingContract")]
    pub working_contract: YesNoType,
    #[serde(rename = "Active")]
    pub active: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometriesType {
    #[serde(rename = "PolygonGeometry", skip_serializing_if = "Option::is_none")]
    pub polygon_geometry: Option<Vec<String>>,
    #[serde(rename = "LineGeometry", skip_serializing_if = "Option::is_none")]
    pub line_geometry: Option<Vec<String>>,
    #[serde(rename = "PointGeometry", skip_serializing_if = "Option::is_none")]
    pub point_geometry: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourceType {
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceTypeType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalContactPersonType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDataType {
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: SequenceNumberType,
    #[serde(rename = "PaymentType")]
    pub payment_type: PaymentTypeType,
    #[serde(rename = "Value")]
    pub value: ValueType,
    #[serde(rename = "VAT")]
    pub vat: VATType,
    #[serde(rename = "AdvanceTax")]
    pub advance_tax: AdvanceTaxType,
    #[serde(rename = "ForestFundPayment")]
    pub forest_fund_payment: ForestFundPaymentType,
    #[serde(rename = "TotalValue")]
    pub total_value: TotalValueType,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "DueDate")]
    pub due_date: DueDateType,
    #[serde(rename = "PaymentPermissionDate", skip_serializing_if = "Option::is_none")]
    pub payment_permission_date: Option<PaymentPermissionDateType>,
    #[serde(rename = "PaymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<PaymentDateType>,
    #[serde(rename = "Payee")]
    pub payee: PayeeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteDetailsType {
    #[serde(flatten)]
    pub base: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkGrouMembersType {
    #[serde(rename = "WorkGrouMember")]
    pub work_grou_member: Vec<WorkGrouMemberType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartType {
    #[serde(rename = "Status3", skip_serializing_if = "Option::is_none")]
    pub status3: Option<ChangeStateType>,
    #[serde(rename = "PartNumber")]
    pub part_number: string,
    #[serde(rename = "PartsDetectedArea", skip_serializing_if = "Option::is_none")]
    pub parts_detected_area: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "CuttingMethod", skip_serializing_if = "Option::is_none")]
    pub cutting_method: Option<CuttingTypeType>,
    #[serde(rename = "RegenerationType", skip_serializing_if = "Option::is_none")]
    pub regeneration_type: Option<VirtaRegenerationType>,
    #[serde(rename = "TargetPartStatus", skip_serializing_if = "Option::is_none")]
    pub target_part_status: Option<VirtaTargetPartStatusType>,
    #[serde(rename = "OperationDate", skip_serializing_if = "Option::is_none")]
    pub operation_date: Option<DateMmDdYyyyType>,
    #[serde(rename = "OperationYear", skip_serializing_if = "Option::is_none")]
    pub operation_year: Option<YearType>,
    #[serde(rename = "Classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<PositiveIntegerType>,
    #[serde(rename = "Review", skip_serializing_if = "Option::is_none")]
    pub review: Option<VirtaReviewType>,
    #[serde(rename = "Reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<VirtaReasonType>,
    #[serde(rename = "Review2", skip_serializing_if = "Option::is_none")]
    pub review2: Option<VirtaReviewType>,
    #[serde(rename = "Reason2", skip_serializing_if = "Option::is_none")]
    pub reason2: Option<VirtaReasonType>,
    #[serde(rename = "InspectionMethod", skip_serializing_if = "Option::is_none")]
    pub inspection_method: Option<VirtaInspectionMethodType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "SeedStemCount", skip_serializing_if = "Option::is_none")]
    pub seed_stem_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "StubStemCount", skip_serializing_if = "Option::is_none")]
    pub stub_stem_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "CultivatedCropStemCount", skip_serializing_if = "Option::is_none")]
    pub cultivated_crop_stem_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "NaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub natural_crop_stem_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "CompletingNaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub completing_natural_crop_stem_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "DeadStemCount", skip_serializing_if = "Option::is_none")]
    pub dead_stem_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<decimal>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<PositiveInteger6digitsType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<PositiveInteger3digitsType>,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<DevelopmentClassType>,
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<VirtaStandQualityType>,
    #[serde(rename = "HabitatCode", skip_serializing_if = "Option::is_none")]
    pub habitat_code: Option<VirtaHabitatCodeType>,
    #[serde(rename = "HabitatType", skip_serializing_if = "Option::is_none")]
    pub habitat_type: Option<VirtaHabitatTypeType>,
    #[serde(rename = "HabitatSurviving", skip_serializing_if = "Option::is_none")]
    pub habitat_surviving: Option<VirtaHabitatSurvivingType>,
    #[serde(rename = "ExceptionalPermitForHandling", skip_serializing_if = "Option::is_none")]
    pub exceptional_permit_for_handling: Option<VirtaExceptionalPermitForHandlingType>,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<MainGroupType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageStateType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "PineDecimal", skip_serializing_if = "Option::is_none")]
    pub pine_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "SpruceDecimal", skip_serializing_if = "Option::is_none")]
    pub spruce_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "BirchDecimal", skip_serializing_if = "Option::is_none")]
    pub birch_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "StockingWithSeedlings", skip_serializing_if = "Option::is_none")]
    pub stocking_with_seedlings: Option<VirtaYesNoType>,
    #[serde(rename = "GroundManipulationMethod", skip_serializing_if = "Option::is_none")]
    pub ground_manipulation_method: Option<VirtaGroundManipulationMethodType>,
    #[serde(rename = "SoilImprovementEvaluation", skip_serializing_if = "Option::is_none")]
    pub soil_improvement_evaluation: Option<VirtaEvaluationType>,
    #[serde(rename = "ClearingEstimation", skip_serializing_if = "Option::is_none")]
    pub clearing_estimation: Option<VirtaEvaluationType>,
    #[serde(rename = "DamageSource", skip_serializing_if = "Option::is_none")]
    pub damage_source: Option<FeatureTypeType>,
    #[serde(rename = "RegenerationEnsuring", skip_serializing_if = "Option::is_none")]
    pub regeneration_ensuring: Option<SilvicultureTypeType>,
    #[serde(rename = "CultivationMaterial", skip_serializing_if = "Option::is_none")]
    pub cultivation_material: Option<VirtaCultivationMaterialType>,
    #[serde(rename = "PlantEvaluation", skip_serializing_if = "Option::is_none")]
    pub plant_evaluation: Option<VirtaPlantEvaluationType>,
    #[serde(rename = "GrassControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub grass_control_evaluation: Option<VirtaEvaluationType>,
    #[serde(rename = "SproutForestControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub sprout_forest_control_evaluation: Option<VirtaEvaluationType>,
    #[serde(rename = "SeedPlantEvaluation", skip_serializing_if = "Option::is_none")]
    pub seed_plant_evaluation: Option<VirtaEvaluationType>,
    #[serde(rename = "RootRotControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub root_rot_control_evaluation: Option<VirtaRootRotControlEvaluationType>,
    #[serde(rename = "HarvestingClassifiation", skip_serializing_if = "Option::is_none")]
    pub harvesting_classifiation: Option<VirtaHarvestingClassificationType>,
    #[serde(rename = "RootDamageCount", skip_serializing_if = "Option::is_none")]
    pub root_damage_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StemDamageCount", skip_serializing_if = "Option::is_none")]
    pub stem_damage_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<PositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "VehiclePathSubsidencePercentage", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_percentage: Option<PositiveInteger3digitsType>,
    #[serde(rename = "TotalEstimation", skip_serializing_if = "Option::is_none")]
    pub total_estimation: Option<VirtaTotalEstimationType>,
    #[serde(rename = "CuttingByMachine", skip_serializing_if = "Option::is_none")]
    pub cutting_by_machine: Option<VirtaCuttingByMachineType>,
    #[serde(rename = "HarvestingSeason", skip_serializing_if = "Option::is_none")]
    pub harvesting_season: Option<VirtaHarvestingSeasonType>,
    #[serde(rename = "PartEastingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_easting_coordinate: Option<string>,
    #[serde(rename = "PartNorthingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_northing_coordinate: Option<string>,
    #[serde(rename = "NotDamagedCount", skip_serializing_if = "Option::is_none")]
    pub not_damaged_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "Class1damageCount", skip_serializing_if = "Option::is_none")]
    pub class1damage_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "Class2damageCount", skip_serializing_if = "Option::is_none")]
    pub class2damage_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "Class3damageCount", skip_serializing_if = "Option::is_none")]
    pub class3damage_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "Class4damageCount", skip_serializing_if = "Option::is_none")]
    pub class4damage_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "SmallPeelDamage", skip_serializing_if = "Option::is_none")]
    pub small_peel_damage: Option<PositiveInteger6digitsType>,
    #[serde(rename = "LargePeelDamage", skip_serializing_if = "Option::is_none")]
    pub large_peel_damage: Option<PositiveInteger6digitsType>,
    #[serde(rename = "DamagedSeedlingCount", skip_serializing_if = "Option::is_none")]
    pub damaged_seedling_count: Option<PositiveInteger6digitsType>,
    #[serde(rename = "PineLog", skip_serializing_if = "Option::is_none")]
    pub pine_log: Option<PositiveInteger4digitsType>,
    #[serde(rename = "SpruceLog", skip_serializing_if = "Option::is_none")]
    pub spruce_log: Option<PositiveInteger4digitsType>,
    #[serde(rename = "BirchLog", skip_serializing_if = "Option::is_none")]
    pub birch_log: Option<PositiveInteger4digitsType>,
    #[serde(rename = "PinePulp", skip_serializing_if = "Option::is_none")]
    pub pine_pulp: Option<PositiveInteger4digitsType>,
    #[serde(rename = "LogM3sum", skip_serializing_if = "Option::is_none")]
    pub log_m3sum: Option<PositiveInteger4digitsType>,
    #[serde(rename = "PulpM3sum", skip_serializing_if = "Option::is_none")]
    pub pulp_m3sum: Option<PositiveInteger4digitsType>,
    #[serde(rename = "SprucePulp", skip_serializing_if = "Option::is_none")]
    pub spruce_pulp: Option<PositiveInteger4digitsType>,
    #[serde(rename = "BirchPulp", skip_serializing_if = "Option::is_none")]
    pub birch_pulp: Option<PositiveInteger4digitsType>,
    #[serde(rename = "OtherTimberValue", skip_serializing_if = "Option::is_none")]
    pub other_timber_value: Option<PositiveInteger5digitsType>,
    #[serde(rename = "EnergyTimberValue", skip_serializing_if = "Option::is_none")]
    pub energy_timber_value: Option<PositiveInteger5digitsType>,
    #[serde(rename = "CuttingValue", skip_serializing_if = "Option::is_none")]
    pub cutting_value: Option<PositiveInteger5digitsType>,
    #[serde(rename = "DestroyedCuttingValue", skip_serializing_if = "Option::is_none")]
    pub destroyed_cutting_value: Option<PositiveInteger5digitsType>,
    #[serde(rename = "ExpectedValueCoefficient", skip_serializing_if = "Option::is_none")]
    pub expected_value_coefficient: Option<PositiveDecimalMax1IntegralPartMax2FractionalPartType>,
    #[serde(rename = "ExpectedValueSurplus", skip_serializing_if = "Option::is_none")]
    pub expected_value_surplus: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "Compensation", skip_serializing_if = "Option::is_none")]
    pub compensation: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "AlreadyPaidCompensation", skip_serializing_if = "Option::is_none")]
    pub already_paid_compensation: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TotalCompensation", skip_serializing_if = "Option::is_none")]
    pub total_compensation: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "NeedForTreatment", skip_serializing_if = "Option::is_none")]
    pub need_for_treatment: Option<VirtaYesNoType>,
    #[serde(rename = "Suggestion", skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<VirtaSuggestionType>,
    #[serde(rename = "Phase2youngCropCount", skip_serializing_if = "Option::is_none")]
    pub phase2young_crop_count: Option<PositiveInteger5digitsType>,
    #[serde(rename = "RecommendedDensity", skip_serializing_if = "Option::is_none")]
    pub recommended_density: Option<PositiveInteger5digitsType>,
    #[serde(rename = "RepairPlantingCosts", skip_serializing_if = "Option::is_none")]
    pub repair_planting_costs: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
    #[serde(rename = "GeometryStatus", skip_serializing_if = "Option::is_none")]
    pub geometry_status: Option<string>,
    #[serde(rename = "GeometryId", skip_serializing_if = "Option::is_none")]
    pub geometry_id: Option<string>,
    #[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
    pub polygon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeesType {
    #[serde(rename = "Payee")]
    pub payee: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySystemsType {
    #[serde(rename = "QualitySystem", skip_serializing_if = "Option::is_none")]
    pub quality_system: Option<Vec<QualitySystemType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesType {
    #[serde(rename = "Storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<StorageType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandSummary2Type {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
    pub mean_age: Option<MeanAgeType>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<BasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanStumpDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_stump_diameter: Option<DiameterType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<MeanHeightType>,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<DevelopmentClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<TreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilvicultureBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<WorkCodeQualifierType1>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<DateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeEnvelopeType {
    #[serde(flatten)]
    pub base: EnvelopeBaseType,
    #[serde(rename = "Message")]
    pub message: WorkingSiteTradeMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygonType {
    #[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
    pub point_property: Option<String>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "FellingRightDuration")]
    pub felling_right_duration: FellingRightDurationType,
    #[serde(rename = "FellingRightValidityDate")]
    pub felling_right_validity_date: FellingRightValidityDateType,
    #[serde(rename = "AssortmentClasses")]
    pub assortment_classes: String,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<string>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisQualificationReasonDataType {
    #[serde(rename = "DisQualificationReason")]
    pub dis_qualification_reason: String10Type,
    #[serde(rename = "DisQualificationReasonText")]
    pub dis_qualification_reason_text: String200Type,
    #[serde(rename = "DisQualificationPercentage")]
    pub dis_qualification_percentage: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature1Type {
    #[serde(flatten)]
    pub base: BasicFeature4Type,
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingAreaType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "ProcessingAreaNumber")]
    pub processing_area_number: ProcessingAreaNumberType,
    #[serde(rename = "ProcessingAreaReference")]
    pub processing_area_reference: ReferenceType,
    #[serde(rename = "DeclarationStands")]
    pub declaration_stands: DeclarationStandsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDataType {
    #[serde(rename = "StemId")]
    pub stem_id: PositiveIntegerType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
    pub stem_type: Option<HarvestingStemTypeType>,
    #[serde(rename = "SelectionType")]
    pub selection_type: ControlStemSelectionType,
    #[serde(rename = "RandomControlStemRejectedReason", skip_serializing_if = "Option::is_none")]
    pub random_control_stem_rejected_reason: Option<String100Type>,
    #[serde(rename = "StemCoordinates", skip_serializing_if = "Option::is_none")]
    pub stem_coordinates: Option<PointGeometryType>,
    #[serde(rename = "Log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<LogDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectDataType {
    #[serde(rename = "SelfMonitoringStandArea", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_stand_area: Option<AreaType>,
    #[serde(rename = "GoalTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub goal_tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "GoalStemCount", skip_serializing_if = "Option::is_none")]
    pub goal_stem_count: Option<StemCountType>,
    #[serde(rename = "GoalAmountOfSoilPreparationSpot", skip_serializing_if = "Option::is_none")]
    pub goal_amount_of_soil_preparation_spot: Option<PositiveInteger5digitsType>,
    #[serde(rename = "RealAmountOfSoilPreparationSpot", skip_serializing_if = "Option::is_none")]
    pub real_amount_of_soil_preparation_spot: Option<PositiveInteger5digitsType>,
    #[serde(rename = "EstimatedWorkingTimeConsumption", skip_serializing_if = "Option::is_none")]
    pub estimated_working_time_consumption: Option<PositiveInteger5digitsType>,
    #[serde(rename = "TimeIntervalForMeasuringSamplePlot", skip_serializing_if = "Option::is_none")]
    pub time_interval_for_measuring_sample_plot: Option<PositiveInteger5digitsType>,
    #[serde(rename = "Notices", skip_serializing_if = "Option::is_none")]
    pub notices: Option<String1000Type>,
    #[serde(rename = "WorkSafetyRisks", skip_serializing_if = "Option::is_none")]
    pub work_safety_risks: Option<WorkSafetyRisksType>,
    #[serde(rename = "SoilPreparationSpotsAreEnough", skip_serializing_if = "Option::is_none")]
    pub soil_preparation_spots_are_enough: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFileType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<ERPIdType>,
    #[serde(rename = "ProductUserId", skip_serializing_if = "Option::is_none")]
    pub product_user_id: Option<String50Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "FileFormat")]
    pub file_format: String5Type,
    #[serde(rename = "Label")]
    pub label: String100Type,
    #[serde(rename = "DocumentClass", skip_serializing_if = "Option::is_none")]
    pub document_class: Option<DocumentClassType>,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentAllElementsType {
    #[serde(rename = "AssortmentIdentifierGroup")]
    pub assortment_identifier_group: String,
    #[serde(rename = "PricingMethod", skip_serializing_if = "Option::is_none")]
    pub pricing_method: Option<UsedPricingMethodType>,
    #[serde(rename = "QuantityGroup")]
    pub quantity_group: String,
    #[serde(rename = "PricesAndCurrencyGroup", skip_serializing_if = "Option::is_none")]
    pub prices_and_currency_group: Option<String>,
    #[serde(rename = "DimensionRequirementsGroup", skip_serializing_if = "Option::is_none")]
    pub dimension_requirements_group: Option<String>,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "GradeCode", skip_serializing_if = "Option::is_none")]
    pub grade_code: Option<GradeCodeType>,
    #[serde(rename = "WoodLots", skip_serializing_if = "Option::is_none")]
    pub wood_lots: Option<WoodLotsType>,
    #[serde(rename = "MeasurementMethod", skip_serializing_if = "Option::is_none")]
    pub measurement_method: Option<MeasurementMethodType>,
    #[serde(rename = "AssortmentInfo", skip_serializing_if = "Option::is_none")]
    pub assortment_info: Option<AssortmentInfoType>,
    #[serde(rename = "PriceMatrix", skip_serializing_if = "Option::is_none")]
    pub price_matrix: Option<PriceMatrixType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesType {
    #[serde(rename = "OtherTreeSpecies")]
    pub other_tree_species: Vec<OtherTreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleActorType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousMooseDamageType {
    #[serde(rename = "PreviousMooseDamageEvaluationDate", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_date: Option<DateType>,
    #[serde(rename = "PreviousMooseDamageEvaluationMunicipality", skip_serializing_if = "Option::is_none")]
    pub previous_moose_damage_evaluation_municipality: Option<MunicipalityNameType>,
    #[serde(rename = "PreviousSameAreaMooseDamageCompensationYear", skip_serializing_if = "Option::is_none")]
    pub previous_same_area_moose_damage_compensation_year: Option<YearType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementsType {
    #[serde(rename = "Measurement")]
    pub measurement: Vec<MeasurementDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaliperType {
    #[serde(rename = "CaliperId", skip_serializing_if = "Option::is_none")]
    pub caliper_id: Option<String200Type>,
    #[serde(rename = "CaliperApplication", skip_serializing_if = "Option::is_none")]
    pub caliper_application: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveriesType {
    #[serde(rename = "Delivery")]
    pub delivery: Vec<DeliveryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: StatisticsPurchaseModeCodeType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "StatisticsAssortmentCompactClasses")]
    pub statistics_assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditRoadMakingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSitesType {
    #[serde(rename = "ContractWorkingSiteDetails")]
    pub contract_working_site_details: Vec<ContractWorkingSiteDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "PowerOfAttorney")]
    pub power_of_attorney: PowerOfAttorneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseCompactStandBasicDataType {
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<StandNumberType>,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<YearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<YearType>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "StandBasicDataDate", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data_date: Option<StandBasicDataDateType>,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StandInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<DataSourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoungForestManagementDataType {
    #[serde(rename = "HeightClass")]
    pub height_class: String,
    #[serde(rename = "RemovalClass")]
    pub removal_class: String,
    #[serde(rename = "CuttingStemCount")]
    pub cutting_stem_count: String,
    #[serde(rename = "SmallWoodRemovalClass")]
    pub small_wood_removal_class: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureOrderType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<String20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "OrderId")]
    pub order_id: String20Type,
    #[serde(rename = "WorkCodes")]
    pub work_codes: WorkCodesType,
    #[serde(rename = "BeginDate")]
    pub begin_date: DateType,
    #[serde(rename = "EndDate")]
    pub end_date: DateType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<AttachmentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSoilConditioningBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<DateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "Description")]
    pub description: String2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "Objects")]
    pub objects: ForestObjectDataObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionType {
    #[serde(rename = "ActionType")]
    pub action_type: string,
    #[serde(rename = "ActionDescription", skip_serializing_if = "Option::is_none")]
    pub action_description: Option<String1000Type>,
    #[serde(rename = "ActionDate")]
    pub action_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonsType {
    #[serde(rename = "DeclarationPolygon")]
    pub declaration_polygon: Vec<DeclarationPolygonType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstatesType {
    #[serde(rename = "BaseRealEstate")]
    pub base_real_estate: Vec<BaseRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuestionType {
    #[serde(rename = "QuestionId")]
    pub question_id: FinalAuditQuestionType,
    #[serde(rename = "QuestionAnswer")]
    pub question_answer: FinalAuditAnswerType,
    #[serde(rename = "QuestionAsText", skip_serializing_if = "Option::is_none")]
    pub question_as_text: Option<String200Type>,
    #[serde(rename = "QuestionAnswerAsText", skip_serializing_if = "Option::is_none")]
    pub question_answer_as_text: Option<String50Type>,
    #[serde(rename = "QuestionAnswerAdditionalText", skip_serializing_if = "Option::is_none")]
    pub question_answer_additional_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeBaseType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "StatisticsOperation")]
    pub statistics_operation: StatisticsOperationType,
    #[serde(rename = "ForestOwnerGroup")]
    pub forest_owner_group: ForestOwnerGroupType,
    #[serde(rename = "ReportedStatisticsOperationType")]
    pub reported_statistics_operation_type: ReportedStatisticsOperationTypeType,
    #[serde(rename = "StatisticsQuantities")]
    pub statistics_quantities: StatisticsQuantitiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationsToSendWsoInformationType {
    #[serde(rename = "AuthorizationToSendWsoInformation")]
    pub authorization_to_send_wso_information: Vec<AuthorizationToSendWsoInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationActorsType {
    #[serde(rename = "ApplicationActor")]
    pub application_actor: Vec<ApplicationActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeBaseListType {
    #[serde(rename = "FeeBaseListItem")]
    pub fee_base_list_item: Vec<FeebaseListItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorDataType {
    #[serde(rename = "ScaleAssortmentType")]
    pub scale_assortment_type: ScaleAssortmentType,
    #[serde(rename = "LoadCount")]
    pub load_count: PositiveIntegerType,
    #[serde(rename = "Area")]
    pub area: AreaCodeType,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "CodeName")]
    pub code_name: String50Type,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<ScaleFactorTreeSpeciesType>,
    #[serde(rename = "Month", skip_serializing_if = "Option::is_none")]
    pub month: Option<MonthType>,
    #[serde(rename = "HarvestingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub harvesting_finished_date: Option<TimeStampType>,
    #[serde(rename = "SemiDry", skip_serializing_if = "Option::is_none")]
    pub semi_dry: Option<YesNoType>,
    #[serde(rename = "SnowOrIce", skip_serializing_if = "Option::is_none")]
    pub snow_or_ice: Option<YesNoType>,
    #[serde(rename = "WeightClass", skip_serializing_if = "Option::is_none")]
    pub weight_class: Option<PositiveInteger1digitsType>,
    #[serde(rename = "Humidity", skip_serializing_if = "Option::is_none")]
    pub humidity: Option<Decimal1FractionDigitType>,
    #[serde(rename = "HumidityMeasured", skip_serializing_if = "Option::is_none")]
    pub humidity_measured: Option<YesNoType>,
    #[serde(rename = "ForestHaulageFinishedDate", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_finished_date: Option<TimeStampType>,
    #[serde(rename = "CleanlinessClass", skip_serializing_if = "Option::is_none")]
    pub cleanliness_class: Option<CleanlinessClassType>,
    #[serde(rename = "Weight")]
    pub weight: Integer7digitsType,
    #[serde(rename = "Density")]
    pub density: Decimal3FractionDigitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "MeltedWater", skip_serializing_if = "Option::is_none")]
    pub melted_water: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: ReferenceType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<ExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<HeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<DiameterClassType>,
    #[serde(rename = "CostTypeAndCompletedWorkApplication")]
    pub cost_type_and_completed_work_application: CostTypeAndCompletedWorkApplicationType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<String2000Type>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousBlockInfoType {
    #[serde(rename = "PreviousBlock", skip_serializing_if = "Option::is_none")]
    pub previous_block: Option<Vec<String20Type>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferBusinessSenderType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesCuttingType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeCuttingType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassType {
    #[serde(rename = "AssortmentClassCode")]
    pub assortment_class_code: AssortmentClassCodeType,
    #[serde(rename = "AssortmentsCompact")]
    pub assortments_compact: AssortmentsCompactType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "BankAccount", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccountType>,
    #[serde(rename = "Payments")]
    pub payments: PaymentsType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationsType {
    #[serde(rename = "Specification")]
    pub specification: Vec<SpecificationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureType {
    #[serde(rename = "FeatureDataGroup")]
    pub feature_data_group: String,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<String>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilviculturalOperationsType {
    #[serde(rename = "SilviculturalOperation")]
    pub silvicultural_operation: Vec<SilviculturalOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksType {
    #[serde(rename = "Work")]
    pub work: Vec<WorkType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingsRowsType {
    #[serde(rename = "LoggingsRow")]
    pub loggings_row: Vec<LoggingsRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentCompactClassesType {
    #[serde(rename = "AssortmentCompactClass")]
    pub assortment_compact_class: Vec<AssortmentCompactClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataType {
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
    pub complete_state: Option<CompleteStateType>,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<IdentifiersType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "SoilDataGroup")]
    pub soil_data_group: String,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub ditching_year: Option<YearType>,
    #[serde(rename = "ThinningYear", skip_serializing_if = "Option::is_none")]
    pub thinning_year: Option<YearType>,
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: String,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "RestrictionsMainGroup")]
    pub restrictions_main_group: String,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: StandBasicDataDateType,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub stand_info: Option<StandInfoType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub growth_place_data_source: Option<DataSourceType>,
    #[serde(rename = "MainGroup")]
    pub main_group: MainGroupType,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<DrainageStateType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<DevelopmentClassType>,
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub stand_quality: Option<StandQualityType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<MainTreeSpeciesType>,
    #[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction: Option<CuttingRestrictionType>,
    #[serde(rename = "CuttingRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub cutting_restriction_ends: Option<DateType>,
    #[serde(rename = "SilvicultureRestriction", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction: Option<SilvicultureRestrictionType>,
    #[serde(rename = "SilvicultureRestrictionEnds", skip_serializing_if = "Option::is_none")]
    pub silviculture_restriction_ends: Option<DateType>,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaDecreaseType>,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectProtectionOperationsDataType {
    #[serde(rename = "BorderingWithWaterAreaOrStream", skip_serializing_if = "Option::is_none")]
    pub bordering_with_water_area_or_stream: Option<YesNoType>,
    #[serde(rename = "LengthOfDitchDiggedDuringSoilPreparation", skip_serializing_if = "Option::is_none")]
    pub length_of_ditch_digged_during_soil_preparation: Option<PositiveInteger6digitsType>,
    #[serde(rename = "ObjectProtectionOperations", skip_serializing_if = "Option::is_none")]
    pub object_protection_operations: Option<ObjectProtectionOperationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodesType {
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<Vec<WorkCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealizationType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorType {
    #[serde(rename = "@actorType")]
    pub actor_type: ActorTypeType,
    #[serde(rename = "@actorTypeSpecifier")]
    pub actor_type_specifier: ActorTypeSpecifierType,
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "ControlAdditionalInformation", skip_serializing_if = "Option::is_none")]
    pub control_additional_information: Option<ControlAdditionalInformationType>,
    #[serde(rename = "PowerOfAttorney", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney: Option<PowerOfAttorneyType>,
    #[serde(rename = "PowerOfAttorneyDate", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_date: Option<PowerOfAttorneyDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringBasicDataType {
    #[serde(rename = "ProjectNo", skip_serializing_if = "Option::is_none")]
    pub project_no: Option<ProjectNoType>,
    #[serde(rename = "SelfMonitoringType", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_type: Option<SelfMonitoringTypeType>,
    #[serde(rename = "SelfMonitoringDate", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_date: Option<DateType>,
    #[serde(rename = "ForestUseDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_number: Option<String>,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub financing_act_number: Option<String>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeGroupsType {
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: Vec<WorkCodeGroupType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAttachmentType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<String50Type>,
    #[serde(rename = "ModificationDate")]
    pub modification_date: DateType,
    #[serde(rename = "Version")]
    pub version: String10Type,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "Filename")]
    pub filename: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekCalendarType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleFactorsType {
    #[serde(rename = "ScaleFactor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<Vec<ScaleFactorDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GammaType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: TimeStampType,
    #[serde(rename = "Message")]
    pub message: MessageType,
    #[serde(rename = "SenderEmail", skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<EmailAddressType>,
    #[serde(rename = "ForestUseDeclaration")]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestingQualityControlType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "OperatorId")]
    pub operator_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String100Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
    #[serde(rename = "Stem")]
    pub stem: Vec<StemDataType>,
    #[serde(rename = "LengthCalibration", skip_serializing_if = "Option::is_none")]
    pub length_calibration: Option<Vec<LengthCalibrationType>>,
    #[serde(rename = "DiameterCalibration", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration: Option<Vec<DiameterCalibrationType>>,
    #[serde(rename = "Caliper", skip_serializing_if = "Option::is_none")]
    pub caliper: Option<CaliperType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceScheduleType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointType {
    #[serde(rename = "Diameter")]
    pub diameter: DiameterType,
    #[serde(rename = "CumulativeMass")]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlPlantManagementType {
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreventionFungusOfTheGenusDataType {
    #[serde(rename = "PreventionCompleted", skip_serializing_if = "Option::is_none")]
    pub prevention_completed: Option<YesNoType>,
    #[serde(rename = "PreventionSubstance", skip_serializing_if = "Option::is_none")]
    pub prevention_substance: Option<PreventionSubstanceType>,
    #[serde(rename = "PreventionSubstanceProductName", skip_serializing_if = "Option::is_none")]
    pub prevention_substance_product_name: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlBasicDataType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<UseCaseType>,
    #[serde(rename = "ControlNo", skip_serializing_if = "Option::is_none")]
    pub control_no: Option<String100Type>,
    #[serde(rename = "ForestUseDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_number: Option<ForestUseDeclarationNumberType>,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub financing_act_number: Option<FinancingActNumberType>,
    #[serde(rename = "ControlReferenceType", skip_serializing_if = "Option::is_none")]
    pub control_reference_type: Option<ForestCentreMessageReferenceType>,
    #[serde(rename = "ControlReference", skip_serializing_if = "Option::is_none")]
    pub control_reference: Option<ReferenceType>,
    #[serde(rename = "ControlDate", skip_serializing_if = "Option::is_none")]
    pub control_date: Option<DateType>,
    #[serde(rename = "TargetSelection", skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<TargetSelectionType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionDetailsType {
    #[serde(rename = "SilvicultureRestrictionGroup")]
    pub silviculture_restriction_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountType {
    #[serde(rename = "IBAN")]
    pub iban: IBANType,
    #[serde(rename = "BIC")]
    pub bic: BICType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: PositiveInteger3digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "FertilizerVolumeOrdered")]
    pub fertilizer_volume_ordered: PositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasured")]
    pub fertilizer_volume_measured: PositiveIntegerType,
    #[serde(rename = "FertilizerVolumeMeasuredText", skip_serializing_if = "Option::is_none")]
    pub fertilizer_volume_measured_text: Option<String200Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseSoilDataType {
    #[serde(rename = "BaseSoilDataGroup", skip_serializing_if = "Option::is_none")]
    pub base_soil_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageType {
    #[serde(rename = "MainDamage", skip_serializing_if = "Option::is_none")]
    pub main_damage: Option<YesNoType>,
    #[serde(rename = "DamageSourceCode", skip_serializing_if = "Option::is_none")]
    pub damage_source_code: Option<string>,
    #[serde(rename = "DamageSourceDescription", skip_serializing_if = "Option::is_none")]
    pub damage_source_description: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditionResourcesType {
    #[serde(rename = "AuditionResource", skip_serializing_if = "Option::is_none")]
    pub audition_resource: Option<AuditionResourceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationDataType {
    #[serde(rename = "ObjectRealization")]
    pub object_realization: Vec<ObjectRealizationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesType {
    #[serde(rename = "UserRole")]
    pub user_role: Vec<UserRoleType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialType {
    #[serde(rename = "MaterialId")]
    pub material_id: String20Type,
    #[serde(rename = "MaterialCode")]
    pub material_code: MaterialCodeType,
    #[serde(rename = "CommercialName", skip_serializing_if = "Option::is_none")]
    pub commercial_name: Option<String100Type>,
    #[serde(rename = "Amount")]
    pub amount: Decimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: MaterialUnitType,
    #[serde(rename = "Supplier", skip_serializing_if = "Option::is_none")]
    pub supplier: Option<String50Type>,
    #[serde(rename = "MaterialProducer", skip_serializing_if = "Option::is_none")]
    pub material_producer: Option<String50Type>,
    #[serde(rename = "Diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "GrainSize", skip_serializing_if = "Option::is_none")]
    pub grain_size: Option<PositiveInteger3digitsType>,
    #[serde(rename = "DeliveryNumber", skip_serializing_if = "Option::is_none")]
    pub delivery_number: Option<String20Type>,
    #[serde(rename = "Delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<YesNoType>,
    #[serde(rename = "DeliveryDate", skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<DateType>,
    #[serde(rename = "PackagingDate", skip_serializing_if = "Option::is_none")]
    pub packaging_date: Option<DateType>,
    #[serde(rename = "FreezingDate", skip_serializing_if = "Option::is_none")]
    pub freezing_date: Option<DateType>,
    #[serde(rename = "MeltingDate", skip_serializing_if = "Option::is_none")]
    pub melting_date: Option<DateType>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String100Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<ERPIdType>,
    #[serde(rename = "MaterialDeliveryType", skip_serializing_if = "Option::is_none")]
    pub material_delivery_type: Option<MaterialDeliveryTypeType>,
    #[serde(rename = "RegionOfOrigin", skip_serializing_if = "Option::is_none")]
    pub region_of_origin: Option<String10Type>,
    #[serde(rename = "DegreeDays", skip_serializing_if = "Option::is_none")]
    pub degree_days: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Altitude", skip_serializing_if = "Option::is_none")]
    pub altitude: Option<PositiveInteger4digitsType>,
    #[serde(rename = "MaterialVolume")]
    pub material_volume: Decimal2FractionDigitsType,
    #[serde(rename = "MaterialUnit")]
    pub material_unit: MaterialUnitType,
    #[serde(rename = "MaterialSupplier", skip_serializing_if = "Option::is_none")]
    pub material_supplier: Option<String50Type>,
    #[serde(rename = "MaterialShipment", skip_serializing_if = "Option::is_none")]
    pub material_shipment: Option<String20Type>,
    #[serde(rename = "UnfreezingDate", skip_serializing_if = "Option::is_none")]
    pub unfreezing_date: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayType {
    #[serde(rename = "CalendarDay")]
    pub calendar_day: DateType,
    #[serde(rename = "Hours")]
    pub hours: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(rename = "TransmissionId")]
    pub transmission_id: String,
    #[serde(rename = "BusinessSender")]
    pub business_sender: String,
    #[serde(rename = "BusinessReceiver", skip_serializing_if = "Option::is_none")]
    pub business_receiver: Option<String>,
    #[serde(rename = "TransmissionSender", skip_serializing_if = "Option::is_none")]
    pub transmission_sender: Option<String>,
    #[serde(rename = "TransmissionReceiver", skip_serializing_if = "Option::is_none")]
    pub transmission_receiver: Option<String>,
    #[serde(rename = "System", skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(rename = "TransmissionTime")]
    pub transmission_time: dateTime,
    #[serde(rename = "Action")]
    pub action: ActionType,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<PriorityType>,
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "EntityTag", skip_serializing_if = "Option::is_none")]
    pub entity_tag: Option<EntityTagType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreSelfMonitoringObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationStandsType {
    #[serde(rename = "DeclarationStand")]
    pub declaration_stand: Vec<DeclarationStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStrataType {
    #[serde(rename = "StemDistributionStratum")]
    pub stem_distribution_stratum: Vec<StemDistributionStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditDrainingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionType {
    #[serde(rename = "Tree")]
    pub tree: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardedProductionType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String50Type,
    #[serde(rename = "StartDate")]
    pub start_date: TimeStampType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<TimeStampType>,
    #[serde(rename = "SendDate")]
    pub send_date: TimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "FileName")]
    pub file_name: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
    #[serde(rename = "LoadCount")]
    pub load_count: nonNegativeInteger,
    #[serde(rename = "Load")]
    pub load: Vec<LoadType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogDataType {
    #[serde(rename = "LogKey")]
    pub log_key: String10Type,
    #[serde(rename = "ProductKey")]
    pub product_key: ERPIdType,
    #[serde(rename = "LogVolume")]
    pub log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ControlLogVolume")]
    pub control_log_volume: Decimal3FractionDigitsType,
    #[serde(rename = "LogLength")]
    pub log_length: PositiveInteger4digitsType,
    #[serde(rename = "ControlLogLength")]
    pub control_log_length: PositiveInteger4digitsType,
    #[serde(rename = "CalibrationUseLog", skip_serializing_if = "Option::is_none")]
    pub calibration_use_log: Option<YesNoType>,
    #[serde(rename = "LogDiameterClass", skip_serializing_if = "Option::is_none")]
    pub log_diameter_class: Option<PositiveInteger3digitsType>,
    #[serde(rename = "LogLengthClass", skip_serializing_if = "Option::is_none")]
    pub log_length_class: Option<PositiveInteger4digitsType>,
    #[serde(rename = "LogMeasurements")]
    pub log_measurements: Vec<LogMeasurementsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumeType {
    #[serde(rename = "ProductUserId")]
    pub product_user_id: String100Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment")]
    pub assortment: String50Type,
    #[serde(rename = "AssortmentName")]
    pub assortment_name: String50Type,
    #[serde(rename = "Quality")]
    pub quality: String5Type,
    #[serde(rename = "LengthClass")]
    pub length_class: PositiveInteger4digitsType,
    #[serde(rename = "DiameterClass")]
    pub diameter_class: PositiveInteger4digitsType,
    #[serde(rename = "Count")]
    pub count: PositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "DiameterCalibrationDate")]
    pub diameter_calibration_date: TimeStampType,
    #[serde(rename = "DiameterCalibrationReason")]
    pub diameter_calibration_reason: String200Type,
    #[serde(rename = "DiameterCalibrationDescription")]
    pub diameter_calibration_description: String200Type,
    #[serde(rename = "DiameterCalibrationAdjustment")]
    pub diameter_calibration_adjustment: Integer3digitsType,
    #[serde(rename = "DiameterCalibrationAdjustmentButtLog", skip_serializing_if = "Option::is_none")]
    pub diameter_calibration_adjustment_butt_log: Option<Integer3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIdsType {
    #[serde(rename = "@DeliveryUserId")]
    pub delivery_user_id: String50Type,
    #[serde(rename = "@Status")]
    pub status: AssortmentStatusType,
    #[serde(rename = "ProductUserId")]
    pub product_user_id: String100Type,
    #[serde(rename = "ModificationDate")]
    pub modification_date: TimeStampType,
    #[serde(flatten)]
    pub base: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkLoadType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
    pub stand_number: Option<String20Type>,
    #[serde(rename = "WorkLoadId")]
    pub work_load_id: unsignedLong,
    #[serde(rename = "Accepted")]
    pub accepted: YesNoType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "UserId")]
    pub user_id: String20Type,
    #[serde(rename = "LoadNumber", skip_serializing_if = "Option::is_none")]
    pub load_number: Option<String20Type>,
    #[serde(rename = "LoadPaymentReference", skip_serializing_if = "Option::is_none")]
    pub load_payment_reference: Option<String50Type>,
    #[serde(rename = "ForestSystemPaymentReference", skip_serializing_if = "Option::is_none")]
    pub forest_system_payment_reference: Option<String50Type>,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "WorkCodeQualifier1", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier1: Option<Vec<WorkCodeQualifierType1>>,
    #[serde(rename = "WorkCodeQualifier2", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier2: Option<Vec<WorkCodeQualifierType2>>,
    #[serde(rename = "WorkCodeQualifier3", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier3: Option<Vec<WorkCodeQualifierType3>>,
    #[serde(rename = "WorkCodeQualifier4", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier4: Option<Vec<WorkCodeQualifierType4>>,
    #[serde(rename = "WorkCodeQualifier5", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier5: Option<Vec<WorkCodeQualifierType5>>,
    #[serde(rename = "WorkLoad1")]
    pub work_load1: Decimal2FractionDigitsType,
    #[serde(rename = "WorkLoad1Unit")]
    pub work_load1_unit: WorkCodeUnitType,
    #[serde(rename = "TransportDistance", skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TransportDistanceUnit", skip_serializing_if = "Option::is_none")]
    pub transport_distance_unit: Option<DistanceUnitType>,
    #[serde(rename = "WorkLoad2", skip_serializing_if = "Option::is_none")]
    pub work_load2: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkLoad2Unit", skip_serializing_if = "Option::is_none")]
    pub work_load2_unit: Option<WorkCodeUnitType>,
    #[serde(rename = "WorkLoadInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_invoiced: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkLoadUnitInvoiced", skip_serializing_if = "Option::is_none")]
    pub work_load_unit_invoiced: Option<WorkCodeUnitType>,
    #[serde(rename = "Materials", skip_serializing_if = "Option::is_none")]
    pub materials: Option<MaterialsType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<FeeBasisType>,
    #[serde(rename = "StandFinished", skip_serializing_if = "Option::is_none")]
    pub stand_finished: Option<YesNoType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeStampType>,
    #[serde(rename = "EndTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeStampType>,
    #[serde(rename = "SavingTime", skip_serializing_if = "Option::is_none")]
    pub saving_time: Option<TimeStampType>,
    #[serde(rename = "WorkPointCount", skip_serializing_if = "Option::is_none")]
    pub work_point_count: Option<PositiveInteger5digitsType>,
    #[serde(rename = "WorkGrouMembers", skip_serializing_if = "Option::is_none")]
    pub work_grou_members: Option<WorkGrouMembersType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType,
    #[serde(rename = "SubsidyZone")]
    pub subsidy_zone: ForestActAreaType,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<StandsType>,
    #[serde(rename = "RealEstateOwners", skip_serializing_if = "Option::is_none")]
    pub real_estate_owners: Option<RealEstateOwnersType>,
    #[serde(rename = "Parcels", skip_serializing_if = "Option::is_none")]
    pub parcels: Option<String>,
    #[serde(rename = "RealEstatesGroup")]
    pub real_estates_group: Vec<String>,
    #[serde(rename = "CertificationSystems", skip_serializing_if = "Option::is_none")]
    pub certification_systems: Option<CertificationSystemsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: TimeStampType,
    #[serde(rename = "Message")]
    pub message: ForestCentreDataMessageType,
    #[serde(rename = "RequestReference", skip_serializing_if = "Option::is_none")]
    pub request_reference: Option<ReferenceType>,
    #[serde(rename = "MetadataText", skip_serializing_if = "Option::is_none")]
    pub metadata_text: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionActorsType {
    #[serde(rename = "CompletionActor")]
    pub completion_actor: Vec<CompletionActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTreesType {
    #[serde(rename = "StandTree", skip_serializing_if = "Option::is_none")]
    pub stand_tree: Option<Vec<StandTreeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionContactInformationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FirstName")]
    pub first_name: FirstNameType,
    #[serde(rename = "LastName")]
    pub last_name: LastNameType,
    #[serde(rename = "PersonOrganizationName")]
    pub person_organization_name: PersonOrganizationNameType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementCertificateType {
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@versionNo")]
    pub version_no: VersionNoType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: SequenceNumberType,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: MeasurementDateType,
    #[serde(rename = "InsertDate")]
    pub insert_date: InsertDateType,
    #[serde(rename = "MeasurementCertificateType")]
    pub measurement_certificate_type: MeasurementCertificateTypeType,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<TextType>,
    #[serde(rename = "Value")]
    pub value: ValueType,
    #[serde(rename = "VAT")]
    pub vat: VATType,
    #[serde(rename = "TotalValue")]
    pub total_value: TotalValueType,
    #[serde(rename = "PaidValue", skip_serializing_if = "Option::is_none")]
    pub paid_value: Option<PaidValueType>,
    #[serde(rename = "Currency")]
    pub currency: CurrencyType,
    #[serde(rename = "AssortmentClasses")]
    pub assortment_classes: String,
    #[serde(rename = "PaymentTransactions", skip_serializing_if = "Option::is_none")]
    pub payment_transactions: Option<PaymentTransactionsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateOwnersType {
    #[serde(rename = "RealEstateOwner")]
    pub real_estate_owner: Vec<ContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@type")]
    pub r#type: ObjectTypeType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ParentObjects", skip_serializing_if = "Option::is_none")]
    pub parent_objects: Option<ParentObjectsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StandBasicDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<String>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureRestrictionsType {
    #[serde(rename = "SilvicultureRestrictionDetails")]
    pub silviculture_restriction_details: Vec<SilvicultureRestrictionDetailsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadType {
    #[serde(rename = "LoadNumber")]
    pub load_number: nonNegativeInteger,
    #[serde(rename = "ForwardingDistance")]
    pub forwarding_distance: nonNegativeInteger,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: TimeStampType,
    #[serde(rename = "PartitialLoad")]
    pub partitial_load: Vec<PartitialLoadType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCodeType {
    #[serde(rename = "WorkingCode")]
    pub working_code: WorkCodeType,
    #[serde(rename = "Amount")]
    pub amount: Decimal2FractionDigitsType,
    #[serde(rename = "Unit")]
    pub unit: WorkCodeUnitType,
    #[serde(rename = "DifficultyClass", skip_serializing_if = "Option::is_none")]
    pub difficulty_class: Option<DifficultyClassType>,
    #[serde(rename = "Attribute1", skip_serializing_if = "Option::is_none")]
    pub attribute1: Option<Vec<WorkCodeQualifierType1>>,
    #[serde(rename = "Attribute2", skip_serializing_if = "Option::is_none")]
    pub attribute2: Option<Vec<WorkCodeQualifierType2>>,
    #[serde(rename = "Attribute3", skip_serializing_if = "Option::is_none")]
    pub attribute3: Option<Vec<WorkCodeQualifierType3>>,
    #[serde(rename = "Attribute4", skip_serializing_if = "Option::is_none")]
    pub attribute4: Option<Vec<WorkCodeQualifierType4>>,
    #[serde(rename = "Attribute5", skip_serializing_if = "Option::is_none")]
    pub attribute5: Option<Vec<WorkCodeQualifierType5>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeIdentifiersType {
    #[serde(rename = "TreeIdentifier", skip_serializing_if = "Option::is_none")]
    pub tree_identifier: Option<Vec<TreeIdentifierType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<WorkCodeQualifierType1>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger2digitsType,
    #[serde(rename = "SamplePlotCountRequired")]
    pub sample_plot_count_required: PositiveInteger2digitsType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Bulk", skip_serializing_if = "Option::is_none")]
    pub bulk: Option<PositiveInteger4digitsType>,
    #[serde(rename = "Quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityOfTreeSpeciesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformationType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@language")]
    pub language: ISO639char2LanguageType,
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String50Type>,
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String10Type>,
    #[serde(rename = "PostOffice", skip_serializing_if = "Option::is_none")]
    pub post_office: Option<String50Type>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String20Type>,
    #[serde(rename = "EmailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String50Type>,
    #[serde(rename = "LanguageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String100Type>,
    #[serde(rename = "IdentifierGroup", skip_serializing_if = "Option::is_none")]
    pub identifier_group: Option<String>,
    #[serde(rename = "NameAndOrganizationGroup")]
    pub name_and_organization_group: String,
    #[serde(rename = "PostAddressGroup", skip_serializing_if = "Option::is_none")]
    pub post_address_group: Option<String>,
    #[serde(rename = "PhoneAndTelefaxGroup")]
    pub phone_and_telefax_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlAdditionalInformationType {
    #[serde(rename = "PreinformDate", skip_serializing_if = "Option::is_none")]
    pub preinform_date: Option<DateType>,
    #[serde(rename = "PreinformDetails", skip_serializing_if = "Option::is_none")]
    pub preinform_details: Option<String1000Type>,
    #[serde(rename = "InTerrain", skip_serializing_if = "Option::is_none")]
    pub in_terrain: Option<YesNoType>,
    #[serde(rename = "Comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringDataType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: YesNoType,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: ReferenceType,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: String2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: ForestDataUpdateUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreSelfMonitoringObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "Explanation")]
    pub explanation: ExplanationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestFundPaymentType {
    #[serde(flatten)]
    pub base: MoneyAndPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstateOwnerType {
    #[serde(rename = "NameAndOrganizationGroup")]
    pub name_and_organization_group: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCasesType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<Vec<ForestDataUpdateUseCaseType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataTreeStandDataDateType {
    #[serde(flatten)]
    pub base: TreeStandDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionType {
    #[serde(rename = "Organisation1", skip_serializing_if = "Option::is_none")]
    pub organisation1: Option<String20Type>,
    #[serde(rename = "Organisation2", skip_serializing_if = "Option::is_none")]
    pub organisation2: Option<String20Type>,
    #[serde(rename = "Organisation3", skip_serializing_if = "Option::is_none")]
    pub organisation3: Option<String20Type>,
    #[serde(rename = "Organisation4", skip_serializing_if = "Option::is_none")]
    pub organisation4: Option<String20Type>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "Description")]
    pub description: String100Type,
    #[serde(rename = "Code")]
    pub code: String20Type,
    #[serde(rename = "MinDiameter")]
    pub min_diameter: PositiveIntegerType,
    #[serde(rename = "MinLength")]
    pub min_length: PositiveIntegerType,
    #[serde(rename = "UserCode")]
    pub user_code: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetaType {
    #[serde(rename = "ShapeAlfa")]
    pub shape_alfa: ShapeAlfaType,
    #[serde(rename = "ShapeBeta")]
    pub shape_beta: ShapeBetaType,
    #[serde(rename = "Minimum")]
    pub minimum: MinimumType,
    #[serde(rename = "Maximum")]
    pub maximum: MaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseActionsType {
    #[serde(rename = "CaseAction")]
    pub case_action: Vec<CaseActionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlCuttingType {
    #[serde(rename = "QualityControlDate", skip_serializing_if = "Option::is_none")]
    pub quality_control_date: Option<DateType>,
    #[serde(rename = "SamplePlotsSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plots_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "Category")]
    pub category: ImageCategoryType,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<ImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<FeatureCodeType>,
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<ERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<String20Type>,
    #[serde(rename = "Position")]
    pub position: PointGeometryType,
    #[serde(rename = "InfoText")]
    pub info_text: String200Type,
    #[serde(rename = "Photographer")]
    pub photographer: String50Type,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<PositiveInteger3digitsType>,
    #[serde(rename = "ImageDate")]
    pub image_date: TimeStampType,
    #[serde(rename = "Filename")]
    pub filename: String100Type,
    #[serde(rename = "Bytes")]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringType,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<ReferenceType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<IdStringNotEmptyType>,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType2,
    #[serde(rename = "CostTypeDescription")]
    pub cost_type_description: CostTypeDescriptionType,
    #[serde(rename = "SubsidyPercent", skip_serializing_if = "Option::is_none")]
    pub subsidy_percent: Option<PercentType>,
    #[serde(rename = "ApplicationAmount", skip_serializing_if = "Option::is_none")]
    pub application_amount: Option<Decimal7And2Type>,
    #[serde(rename = "ApplicationAmountUnit", skip_serializing_if = "Option::is_none")]
    pub application_amount_unit: Option<ForestCentreUnitType>,
    #[serde(rename = "ApplicationUnitPrice", skip_serializing_if = "Option::is_none")]
    pub application_unit_price: Option<MoneyType>,
    #[serde(rename = "ApplicationTotalSubsidy", skip_serializing_if = "Option::is_none")]
    pub application_total_subsidy: Option<MoneyType>,
    #[serde(rename = "DecidedAmount", skip_serializing_if = "Option::is_none")]
    pub decided_amount: Option<DecidedAmountType>,
    #[serde(rename = "DecidedAmountUnit", skip_serializing_if = "Option::is_none")]
    pub decided_amount_unit: Option<DecidedAmountUnitType>,
    #[serde(rename = "DecidedUnitPrice", skip_serializing_if = "Option::is_none")]
    pub decided_unit_price: Option<DecidedUnitPriceType>,
    #[serde(rename = "DecidedTotalSubsidy")]
    pub decided_total_subsidy: DecidedTotalSubsidyType,
    #[serde(rename = "Reasons", skip_serializing_if = "Option::is_none")]
    pub reasons: Option<ReasonsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringFinalAuditHarvestingType {
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits", skip_serializing_if = "Option::is_none")]
    pub audits: Option<AuditsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandsType {
    #[serde(rename = "Stand")]
    pub stand: Vec<StandType>,
    #[serde(rename = "StandNumber")]
    pub stand_number: Vec<String20Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyArgumentType {
    #[serde(rename = "SubsidyArgumentText")]
    pub subsidy_argument_text: Vec<String5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "DBH")]
    pub dbh: PositiveInteger3digitsType,
    #[serde(rename = "Count")]
    pub count: PositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectType {
    #[serde(rename = "ChildObjectType")]
    pub child_object_type: ObjectTypeType,
    #[serde(rename = "ChildObjectTypeSpecifier", skip_serializing_if = "Option::is_none")]
    pub child_object_type_specifier: Option<ObjectTypeSpecifierType>,
    #[serde(rename = "ChildObjectId")]
    pub child_object_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierReferenceListType {
    #[serde(rename = "SubsidyApplierReference")]
    pub subsidy_applier_reference: Vec<SubsidyApplierReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ParcelNumber")]
    pub parcel_number: ParcelNumberType,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionsType {
    #[serde(rename = "UsingRestriction")]
    pub using_restriction: Vec<UsingRestrictionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionDeclarationActorsType {
    #[serde(rename = "CompletionDeclarationActor")]
    pub completion_declaration_actor: Vec<PayeeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteKeyType {
    #[serde(rename = "@id")]
    pub id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTextsType {
    #[serde(rename = "PaymentText")]
    pub payment_text: Vec<String5000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ObjectDataGroup", skip_serializing_if = "Option::is_none")]
    pub object_data_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectKeys", skip_serializing_if = "Option::is_none")]
    pub object_keys: Option<ObjectKeysType>,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "UseCases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<ForestDataUpdateUseCasesType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<BaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<RestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlBaseCuttingType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "QualityControlDate", skip_serializing_if = "Option::is_none")]
    pub quality_control_date: Option<DateType>,
    #[serde(rename = "SamplePlotsSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plots_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "PurchaseContractNumber")]
    pub purchase_contract_number: String20Type,
    #[serde(rename = "PurchaseContractExtraInfo", skip_serializing_if = "Option::is_none")]
    pub purchase_contract_extra_info: Option<String3000Type>,
    #[serde(rename = "WorkingSiteNumber")]
    pub working_site_number: WorkingSiteNumberType,
    #[serde(rename = "WorkingSiteName")]
    pub working_site_name: String100Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ContractId")]
    pub contract_id: String20Type,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "CustomerType", skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<String50Type>,
    #[serde(rename = "SilvicultureContractNumber", skip_serializing_if = "Option::is_none")]
    pub silviculture_contract_number: Option<String20Type>,
    #[serde(rename = "Organisation1", skip_serializing_if = "Option::is_none")]
    pub organisation1: Option<String20Type>,
    #[serde(rename = "Organisation2", skip_serializing_if = "Option::is_none")]
    pub organisation2: Option<String20Type>,
    #[serde(rename = "Organisation3", skip_serializing_if = "Option::is_none")]
    pub organisation3: Option<String20Type>,
    #[serde(rename = "Organisation4", skip_serializing_if = "Option::is_none")]
    pub organisation4: Option<String20Type>,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "ServiceBuyerContactInformation")]
    pub service_buyer_contact_information: ServiceBuyerContactInformationType,
    #[serde(rename = "CustomerRepresentativePerson", skip_serializing_if = "Option::is_none")]
    pub customer_representative_person: Option<ContactInformationType>,
    #[serde(rename = "ForestOwners")]
    pub forest_owners: ForestOwnersType,
    #[serde(rename = "AuthorizationsToSendWsoInformation", skip_serializing_if = "Option::is_none")]
    pub authorizations_to_send_wso_information: Option<AuthorizationsToSendWsoInformationType>,
    #[serde(rename = "TransportCompany", skip_serializing_if = "Option::is_none")]
    pub transport_company: Option<ContactInformationType>,
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "Accessibility")]
    pub accessibility: AccessibilityType,
    #[serde(rename = "TerrainClass", skip_serializing_if = "Option::is_none")]
    pub terrain_class: Option<TerrainClassType>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
    #[serde(rename = "ResultsOfAccessibilityAnalysis", skip_serializing_if = "Option::is_none")]
    pub results_of_accessibility_analysis: Option<ResultsOfAccessibilityAnalysisType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<PositiveInteger5digitsType>,
    #[serde(rename = "TerrainPlanningDone", skip_serializing_if = "Option::is_none")]
    pub terrain_planning_done: Option<YesNoType>,
    #[serde(rename = "PreClearingInformation", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_information: Option<PreClearingInformationType>,
    #[serde(rename = "WorkingSitePlanningStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_status: Option<WorkingSitePlanningStatusType>,
    #[serde(rename = "WorkingSitePlanningOperation", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_operation: Option<WorkingSitePlanningOperationStatusType>,
    #[serde(rename = "WorkingSitePlanningInfo", skip_serializing_if = "Option::is_none")]
    pub working_site_planning_info: Option<String3000Type>,
    #[serde(rename = "WorkingSitePlannedForHarvestingDate", skip_serializing_if = "Option::is_none")]
    pub working_site_planned_for_harvesting_date: Option<DateType>,
    #[serde(rename = "WorkingSiteStatus", skip_serializing_if = "Option::is_none")]
    pub working_site_status: Option<WorkingSiteStatusType>,
    #[serde(rename = "ReadyToDo")]
    pub ready_to_do: YesNoType,
    #[serde(rename = "SellersLogs", skip_serializing_if = "Option::is_none")]
    pub sellers_logs: Option<String200Type>,
    #[serde(rename = "SellersLogsInfo", skip_serializing_if = "Option::is_none")]
    pub sellers_logs_info: Option<String1000Type>,
    #[serde(rename = "WorkingsiteInfo", skip_serializing_if = "Option::is_none")]
    pub workingsite_info: Option<String3000Type>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<WorkingSitePriorityType>,
    #[serde(rename = "OperationTimeStart", skip_serializing_if = "Option::is_none")]
    pub operation_time_start: Option<DateType>,
    #[serde(rename = "OperationTimeEnd", skip_serializing_if = "Option::is_none")]
    pub operation_time_end: Option<DateType>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "ForestUseDeclaration", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration: Option<ForestUseDeclarationType>,
    #[serde(rename = "OperationRestriction", skip_serializing_if = "Option::is_none")]
    pub operation_restriction: Option<OperationRestrictionType>,
    #[serde(rename = "ContractValidDate", skip_serializing_if = "Option::is_none")]
    pub contract_valid_date: Option<DateType>,
    #[serde(rename = "MaterialAreaId", skip_serializing_if = "Option::is_none")]
    pub material_area_id: Option<String20Type>,
    #[serde(rename = "QualityAttachments", skip_serializing_if = "Option::is_none")]
    pub quality_attachments: Option<String100Type>,
    #[serde(rename = "TransportArea", skip_serializing_if = "Option::is_none")]
    pub transport_area: Option<String10Type>,
    #[serde(rename = "HasSupport", skip_serializing_if = "Option::is_none")]
    pub has_support: Option<YesNoType>,
    #[serde(rename = "FinancingSustainableForestry", skip_serializing_if = "Option::is_none")]
    pub financing_sustainable_forestry: Option<FinancingSustainableForestryType>,
    #[serde(rename = "PaymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<DateType>,
    #[serde(rename = "PreNotificationAllowed", skip_serializing_if = "Option::is_none")]
    pub pre_notification_allowed: Option<YesNoType>,
    #[serde(rename = "BeginNotificationAllowed", skip_serializing_if = "Option::is_none")]
    pub begin_notification_allowed: Option<YesNoType>,
    #[serde(rename = "SendNotificationsAlways", skip_serializing_if = "Option::is_none")]
    pub send_notifications_always: Option<YesNoType>,
    #[serde(rename = "LargeSummaryReportRequired", skip_serializing_if = "Option::is_none")]
    pub large_summary_report_required: Option<YesNoType>,
    #[serde(rename = "TestAreaMethod", skip_serializing_if = "Option::is_none")]
    pub test_area_method: Option<SamplePlotType>,
    #[serde(rename = "TestAreaRequired", skip_serializing_if = "Option::is_none")]
    pub test_area_required: Option<YesNoType>,
    #[serde(rename = "IslandWorkingSite", skip_serializing_if = "Option::is_none")]
    pub island_working_site: Option<YesNoType>,
    #[serde(rename = "StormWorkingSite", skip_serializing_if = "Option::is_none")]
    pub storm_working_site: Option<YesNoType>,
    #[serde(rename = "SodWorkingSite", skip_serializing_if = "Option::is_none")]
    pub sod_working_site: Option<YesNoType>,
    #[serde(rename = "CanCultivateInAutumn", skip_serializing_if = "Option::is_none")]
    pub can_cultivate_in_autumn: Option<YesNoType>,
    #[serde(rename = "MixedForestRegenarationMethods", skip_serializing_if = "Option::is_none")]
    pub mixed_forest_regenaration_methods: Option<YesNoType>,
    #[serde(rename = "IsValueForceWorkingSite", skip_serializing_if = "Option::is_none")]
    pub is_value_force_working_site: Option<YesNoType>,
    #[serde(rename = "ForestCertification")]
    pub forest_certification: Vec<CertificationSystemType>,
    #[serde(rename = "CertificationHandlingInstructions", skip_serializing_if = "Option::is_none")]
    pub certification_handling_instructions: Option<String3000Type>,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<SpareTreesByCategoryType>,
    #[serde(rename = "DryingClass", skip_serializing_if = "Option::is_none")]
    pub drying_class: Option<DryingClassType>,
    #[serde(rename = "HumidityPercentage", skip_serializing_if = "Option::is_none")]
    pub humidity_percentage: Option<Decimal1FractionDigitType>,
    #[serde(rename = "CuttingControlRequired", skip_serializing_if = "Option::is_none")]
    pub cutting_control_required: Option<YesNoType>,
    #[serde(rename = "CuttingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub cutting_finished_date: Option<DateType>,
    #[serde(rename = "StumpLiftingFinishedDate", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_finished_date: Option<DateType>,
    #[serde(rename = "BiomassFinishedDate", skip_serializing_if = "Option::is_none")]
    pub biomass_finished_date: Option<DateType>,
    #[serde(rename = "AssortmentIncrementAllowed", skip_serializing_if = "Option::is_none")]
    pub assortment_increment_allowed: Option<YesNoType>,
    #[serde(rename = "EnvironmentalObjectInfo", skip_serializing_if = "Option::is_none")]
    pub environmental_object_info: Option<String3000Type>,
    #[serde(rename = "WorkingSafetyInfo", skip_serializing_if = "Option::is_none")]
    pub working_safety_info: Option<String3000Type>,
    #[serde(rename = "AccessRightsInfo", skip_serializing_if = "Option::is_none")]
    pub access_rights_info: Option<String3000Type>,
    #[serde(rename = "MainWorkCode")]
    pub main_work_code: MainWorkCodeType,
    #[serde(rename = "ProductionFileSendFrequency", skip_serializing_if = "Option::is_none")]
    pub production_file_send_frequency: Option<PositiveIntegerType>,
    #[serde(rename = "ForestSystemPaymentReference", skip_serializing_if = "Option::is_none")]
    pub forest_system_payment_reference: Option<String50Type>,
    #[serde(rename = "PricingMethod", skip_serializing_if = "Option::is_none")]
    pub pricing_method: Option<PricingMethodType>,
    #[serde(rename = "ContinuousCoverForestry", skip_serializing_if = "Option::is_none")]
    pub continuous_cover_forestry: Option<YesNoType>,
    #[serde(rename = "PreviousBlockState", skip_serializing_if = "Option::is_none")]
    pub previous_block_state: Option<PreviousBlockStatusType>,
    #[serde(rename = "PreviousBlocks", skip_serializing_if = "Option::is_none")]
    pub previous_blocks: Option<PreviousBlockInfoType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "Stands")]
    pub stands: StandsType,
    #[serde(rename = "Storages", skip_serializing_if = "Option::is_none")]
    pub storages: Option<StoragesType>,
    #[serde(rename = "StoragesForestHaulageDistances", skip_serializing_if = "Option::is_none")]
    pub storages_forest_haulage_distances: Option<StoragesForestHaulageDistancesType>,
    #[serde(rename = "FeeBasis", skip_serializing_if = "Option::is_none")]
    pub fee_basis: Option<FeeBasisType>,
    #[serde(rename = "StemTypeBulks", skip_serializing_if = "Option::is_none")]
    pub stem_type_bulks: Option<StemTypeBulksType>,
    #[serde(rename = "TreeSpeciesAttributes", skip_serializing_if = "Option::is_none")]
    pub tree_species_attributes: Option<TreeSpeciesAttributesType>,
    #[serde(rename = "ProductUserIds", skip_serializing_if = "Option::is_none")]
    pub product_user_ids: Option<ProductUserIdsType>,
    #[serde(rename = "DiameterSections", skip_serializing_if = "Option::is_none")]
    pub diameter_sections: Option<DiameterSectionsType>,
    #[serde(rename = "OtherRemarks", skip_serializing_if = "Option::is_none")]
    pub other_remarks: Option<String3000Type>,
    #[serde(rename = "Deliveries", skip_serializing_if = "Option::is_none")]
    pub deliveries: Option<DeliveriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFeeBasisType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "FeeBasis")]
    pub fee_basis: FeeBasisType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeType {
    #[serde(rename = "CostTypeNumber")]
    pub cost_type_number: CostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygon2Type {
    #[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
    pub point_property: Option<String>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreaType {
    #[serde(rename = "Geometry")]
    pub geometry: PolygonOrMultiPolygon2Type,
    #[serde(rename = "Supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: decimal,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "pointProperty")]
    pub point_property: String,
    #[serde(rename = "lineStringProperty")]
    pub line_string_property: String,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsRealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<PaymentsRealEstateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingAreasType {
    #[serde(rename = "WorkingArea", skip_serializing_if = "Option::is_none")]
    pub working_area: Option<Vec<WorkingAreaType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsCompactType {
    #[serde(rename = "AssortmentCompact")]
    pub assortment_compact: Vec<AssortmentCompactType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditBioMassForwardingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: ReferenceType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreForestDataUpdateObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActDataType {
    #[serde(rename = "WorksDueDate")]
    pub works_due_date: DateType,
    #[serde(rename = "CompletionDeclarationDeliveryDueDate")]
    pub completion_declaration_delivery_due_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRowsType {
    #[serde(rename = "OperationRow")]
    pub operation_row: Vec<OperationRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonType {
    #[serde(rename = "ReasonCode")]
    pub reason_code: String10Type,
    #[serde(rename = "ReasonDescription")]
    pub reason_description: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlOverallEvaluationDataType {
    #[serde(rename = "RestrictionBasedOnStoniness", skip_serializing_if = "Option::is_none")]
    pub restriction_based_on_stoniness: Option<RestrictionBasedOnStoninessType>,
    #[serde(rename = "PreclearingEvaluation", skip_serializing_if = "Option::is_none")]
    pub preclearing_evaluation: Option<PreclearingEvaluationType>,
    #[serde(rename = "DeclarationDeliveringEvaluation", skip_serializing_if = "Option::is_none")]
    pub declaration_delivering_evaluation: Option<EvaluationCodeType>,
    #[serde(rename = "AreaAndMapEvaluation", skip_serializing_if = "Option::is_none")]
    pub area_and_map_evaluation: Option<EvaluationCodeType>,
    #[serde(rename = "OtherEvaluation", skip_serializing_if = "Option::is_none")]
    pub other_evaluation: Option<EvaluationCodeType>,
    #[serde(rename = "TreeDamageOutsideStandEvaluation", skip_serializing_if = "Option::is_none")]
    pub tree_damage_outside_stand_evaluation: Option<EvaluationCodeType>,
    #[serde(rename = "TerrainDamageOutsideStandEvaluation", skip_serializing_if = "Option::is_none")]
    pub terrain_damage_outside_stand_evaluation: Option<EvaluationCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingRepresentativesType {
    #[serde(rename = "WorkingRepresentative")]
    pub working_representative: Vec<WorkingRepresentativeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplicationType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: String,
    #[serde(rename = "FinancingActApplicationReference")]
    pub financing_act_application_reference: String,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub financing_act_number: Option<String>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: OtherPublicSubstituteType,
    #[serde(rename = "FinancingActApplicationTextInformation", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_text_information: Option<String>,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Sender")]
    pub sender: String,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<String>,
    #[serde(rename = "SentDate")]
    pub sent_date: String,
    #[serde(rename = "ElectronicNotification")]
    pub electronic_notification: String,
    #[serde(rename = "FinancingType")]
    pub financing_type: FinancingActFinancingType,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: MunicipalityNumberType,
    #[serde(rename = "EstimatedStartDate")]
    pub estimated_start_date: String,
    #[serde(rename = "EstimatedEndDate")]
    pub estimated_end_date: String,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: String,
    #[serde(rename = "FinancingActWorkGroup")]
    pub financing_act_work_group: String,
    #[serde(rename = "CopOperationProject")]
    pub cop_operation_project: String,
    #[serde(rename = "FinancingActRealEstates")]
    pub financing_act_real_estates: String,
    #[serde(rename = "ApplicationActors")]
    pub application_actors: String,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsidyApplierBaseContactAndEstateInfoType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<PaymentsRealEstatesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedlingDataType {
    #[serde(rename = "SeedlingStratum")]
    pub seedling_stratum: Vec<SeedlingStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowsType {
    #[serde(rename = "RoundWoodSalesRow")]
    pub round_wood_sales_row: Vec<RoundWoodSalesRowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCuttingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "QualityControlDate")]
    pub quality_control_date: DateType,
    #[serde(rename = "SamplePlotsSummaries")]
    pub sample_plots_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentsDataType {
    #[serde(flatten)]
    pub base: ForestCentreDataType,
    #[serde(rename = "ForestCentrePayments")]
    pub forest_centre_payments: ForestCentrePaymentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedDataType {
    #[serde(rename = "TreeStandBasedDataGroup")]
    pub tree_stand_based_data_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesRowType {
    #[serde(rename = "AreaType")]
    pub area_type: AreaTypeType,
    #[serde(rename = "AreaCode")]
    pub area_code: AreaCodeType,
    #[serde(rename = "PurchaseModeCode")]
    pub purchase_mode_code: PurchaseModeType,
    #[serde(rename = "AssortmentCompactClasses")]
    pub assortment_compact_classes: AssortmentCompactClassesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsOperatorStatusType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String20Type,
    #[serde(rename = "NotificationType")]
    pub notification_type: NotificationTypeType,
    #[serde(rename = "RecipientType")]
    pub recipient_type: RecipientTypeType,
    #[serde(rename = "SenderUserId")]
    pub sender_user_id: String20Type,
    #[serde(rename = "SendTimestamp")]
    pub send_timestamp: TimeStampType,
    #[serde(rename = "StatusTimestamp")]
    pub status_timestamp: TimeStampType,
    #[serde(rename = "StatusCode")]
    pub status_code: StatusCodeType,
    #[serde(rename = "OriginalMessage")]
    pub original_message: String1000Type,
    #[serde(rename = "StatusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetsType {
    #[serde(rename = "Target")]
    pub target: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultOfAccessibilityAnalysisType {
    #[serde(rename = "Accessibility")]
    pub accessibility: AccessibilityType,
    #[serde(rename = "Percentage")]
    pub percentage: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "RelatedCallForOffers", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offers: Option<RelatedCallForOffersType>,
    #[serde(rename = "AdditionalCode", skip_serializing_if = "Option::is_none")]
    pub additional_code: Option<AdditionalCodeType>,
    #[serde(rename = "CallForOfferBusinessSender")]
    pub call_for_offer_business_sender: CallForOfferBusinessSenderType,
    #[serde(rename = "Sellers", skip_serializing_if = "Option::is_none")]
    pub sellers: Option<SellersType>,
    #[serde(rename = "TechnicalContactPerson", skip_serializing_if = "Option::is_none")]
    pub technical_contact_person: Option<TechnicalContactPersonType>,
    #[serde(rename = "CallForOfferDate")]
    pub call_for_offer_date: CallForOfferDateType,
    #[serde(rename = "OfferExpirationDate")]
    pub offer_expiration_date: OfferExpirationDateType,
    #[serde(rename = "CallForOfferText", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_text: Option<CallForOfferTextType>,
    #[serde(rename = "CallForOfferWoodTradeInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_wood_trade_info: Option<CallForOfferWoodTradeInfoType>,
    #[serde(rename = "CallForOfferSilvicultureInfo", skip_serializing_if = "Option::is_none")]
    pub call_for_offer_silviculture_info: Option<CallForOfferSilvicultureInfoType>,
    #[serde(rename = "CallForOfferWorkingSites")]
    pub call_for_offer_working_sites: String,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneryWorkPermissionType {
    #[serde(rename = "SceneryWorkPermissionNeeded")]
    pub scenery_work_permission_needed: SceneryWorkPermissionNeededType,
    #[serde(rename = "SceneryWorkPermissionAcceptance", skip_serializing_if = "Option::is_none")]
    pub scenery_work_permission_acceptance: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractType {
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: int,
    #[serde(rename = "@id")]
    pub id: string,
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
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "ContractBeginningDate")]
    pub contract_beginning_date: ContractBeginningDateType,
    #[serde(rename = "ContractEndingDate")]
    pub contract_ending_date: ContractEndingDateType,
    #[serde(rename = "ContractText", skip_serializing_if = "Option::is_none")]
    pub contract_text: Option<String1500Type>,
    #[serde(rename = "ContractWorkingSites", skip_serializing_if = "Option::is_none")]
    pub contract_working_sites: Option<ContractWorkingSitesType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkType {
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "MaterialCode", skip_serializing_if = "Option::is_none")]
    pub material_code: Option<MaterialCodeType>,
    #[serde(rename = "WorkCompletionDate", skip_serializing_if = "Option::is_none")]
    pub work_completion_date: Option<DateType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: ForestCentreWorkCodeGroupType,
    #[serde(rename = "Subsidies")]
    pub subsidies: SubsidiesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VATInfoType {
    #[serde(rename = "VATStatus")]
    pub v_a_t_status: VATStatusType,
    #[serde(rename = "VATRegistrationDate", skip_serializing_if = "Option::is_none")]
    pub v_a_t_registration_date: Option<VATRegistrationDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingPlannerLiabilityType {
    #[serde(flatten)]
    pub base: YesNoNotKnownType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestedProductionType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<ERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: String100Type,
    #[serde(rename = "StartDate")]
    pub start_date: TimeStampType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<TimeStampType>,
    #[serde(rename = "SendDate")]
    pub send_date: TimeStampType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: String100Type,
    #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String100Type>,
    #[serde(rename = "Bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<base64Binary>,
    #[serde(rename = "StemTypeVolumes")]
    pub stem_type_volumes: StemTypeVolumesType,
    #[serde(rename = "AssortmentVolumes")]
    pub assortment_volumes: AssortmentVolumesType,
    #[serde(rename = "AssortmentVolumesUnclassified", skip_serializing_if = "Option::is_none")]
    pub assortment_volumes_unclassified: Option<AssortmentVolumesUnclassifiedType>,
    #[serde(rename = "AssortmentMatrixVolumes")]
    pub assortment_matrix_volumes: AssortmentMatrixVolumesType,
    #[serde(rename = "ProductUserIds")]
    pub product_user_ids: Vec<ProductUserIdsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBasicDataType {
    #[serde(rename = "ObjectNumber", skip_serializing_if = "Option::is_none")]
    pub object_number: Option<ObjectNumberType>,
    #[serde(rename = "ObjectReference", skip_serializing_if = "Option::is_none")]
    pub object_reference: Option<ReferenceType>,
    #[serde(rename = "NonPersonificationId", skip_serializing_if = "Option::is_none")]
    pub non_personification_id: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedActorsType {
    #[serde(rename = "InformedActor")]
    pub informed_actor: Vec<ContactInformationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingsType {
    #[serde(rename = "Training", skip_serializing_if = "Option::is_none")]
    pub training: Option<Vec<TrainingDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingOrderType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ServiceBuyerArea")]
    pub service_buyer_area: String20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "OrderId")]
    pub order_id: String20Type,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<AttachmentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationDataType {
    #[serde(rename = "CultivatedCropStemCount")]
    pub cultivated_crop_stem_count: StemCountType,
    #[serde(rename = "NaturalCropStemCount")]
    pub natural_crop_stem_count: StemCountType,
    #[serde(rename = "CompletingNaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub completing_natural_crop_stem_count: Option<StemCountType>,
    #[serde(rename = "CultivatedDeadStemCount", skip_serializing_if = "Option::is_none")]
    pub cultivated_dead_stem_count: Option<StemCountType>,
    #[serde(rename = "StockingWIthSeedlings")]
    pub stocking_w_ith_seedlings: integer,
    #[serde(rename = "GroundManipulationMethod")]
    pub ground_manipulation_method: ThreeDigitPositiveIntegerType,
    #[serde(rename = "RegenerationEnsuring", skip_serializing_if = "Option::is_none")]
    pub regeneration_ensuring: Option<ThreeDigitPositiveIntegerType>,
    #[serde(rename = "CultivationMaterial")]
    pub cultivation_material: TwoDigitPositiveIntegerType,
    #[serde(rename = "PlantingWorkQuality")]
    pub planting_work_quality: integer,
    #[serde(rename = "SoilModificationEstimate")]
    pub soil_modification_estimate: integer,
    #[serde(rename = "SeedlingLotIdentifier", skip_serializing_if = "Option::is_none")]
    pub seedling_lot_identifier: Option<String100Type>,
    #[serde(rename = "SeedlingConditionAndQuality", skip_serializing_if = "Option::is_none")]
    pub seedling_condition_and_quality: Option<SeedlingConditionAndQualityType>,
    #[serde(rename = "SeedlingConditionAndQualityDescription", skip_serializing_if = "Option::is_none")]
    pub seedling_condition_and_quality_description: Option<String1000Type>,
    #[serde(rename = "AmountSeedlingsToPlant", skip_serializing_if = "Option::is_none")]
    pub amount_seedlings_to_plant: Option<PositiveInteger5digitsType>,
    #[serde(rename = "DateSeedlingsExitFromTreeNursery", skip_serializing_if = "Option::is_none")]
    pub date_seedlings_exit_from_tree_nursery: Option<DateType>,
    #[serde(rename = "DateSeedlingsToWorkingSite", skip_serializing_if = "Option::is_none")]
    pub date_seedlings_to_working_site: Option<DateType>,
    #[serde(rename = "DateSeedlingsPlanted", skip_serializing_if = "Option::is_none")]
    pub date_seedlings_planted: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationType {
    #[serde(rename = "CalibrationDate")]
    pub calibration_date: TimeStampType,
    #[serde(rename = "CalibrationAdjustment")]
    pub calibration_adjustment: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsChangesType {
    #[serde(rename = "AssortmentsChange", skip_serializing_if = "Option::is_none")]
    pub assortments_change: Option<Vec<AssortmentChangeDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessagesType {
    #[serde(rename = "ErrorMessageData")]
    pub error_message_data: Vec<ErrorMessageDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderConfirmationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ServiceBuyerArea", skip_serializing_if = "Option::is_none")]
    pub service_buyer_area: Option<String20Type>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OperationalRegion", skip_serializing_if = "Option::is_none")]
    pub operational_region: Option<String50Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "OrderId")]
    pub order_id: String20Type,
    #[serde(rename = "OrderStatus")]
    pub order_status: OrderStatusType,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActGeometryType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCodeType,
    #[serde(rename = "FeatureInfo")]
    pub feature_info: String500Type,
    #[serde(rename = "PointAndLineGeometriesGroup")]
    pub point_and_line_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometriesType {
    #[serde(rename = "DecisionGeometry")]
    pub decision_geometry: Vec<DecisionGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalDataType {
    #[serde(rename = "ProposalType")]
    pub proposal_type: ProposalTypeType,
    #[serde(rename = "PlanningYearAndOperationUrgencyGroup")]
    pub planning_year_and_operation_urgency_group: String,
    #[serde(rename = "TimeBetweenProposalYearsGroup")]
    pub time_between_proposal_years_group: String,
    #[serde(rename = "ProposalAndOriginalYearGroup")]
    pub proposal_and_original_year_group: String,
    #[serde(rename = "ProposalDate")]
    pub proposal_date: ProposalDateType,
    #[serde(rename = "ProposalAreaGroup", skip_serializing_if = "Option::is_none")]
    pub proposal_area_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesSummaryType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteTradeMessageType {
    #[serde(rename = "CallForOffer")]
    pub call_for_offer: String,
    #[serde(rename = "CFOWorkingSite")]
    pub c_f_o_working_site: String,
    #[serde(rename = "BusinessAcceptance")]
    pub business_acceptance: String,
    #[serde(rename = "Payment")]
    pub payment: String,
    #[serde(rename = "MeasurementCertificate")]
    pub measurement_certificate: String,
    #[serde(rename = "ForestUseDeclarationReferences")]
    pub forest_use_declaration_references: String,
    #[serde(rename = "Contract")]
    pub contract: String,
    #[serde(rename = "Offer")]
    pub offer: String,
    #[serde(rename = "OWorkingSite")]
    pub o_working_site: String,
    #[serde(rename = "Operations")]
    pub operations: String,
    #[serde(rename = "ContactRequest")]
    pub contact_request: String,
    #[serde(rename = "MapSymbol")]
    pub map_symbol: String,
    #[serde(rename = "Acknowledge")]
    pub acknowledge: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesType {
    #[serde(rename = "Trees")]
    pub trees: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTransactionsType {
    #[serde(rename = "PaymentTransaction")]
    pub payment_transaction: PaymentTransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "UpdatePreviousDeclaration")]
    pub update_previous_declaration: YesNoType,
    #[serde(rename = "DeclarationReference")]
    pub declaration_reference: ReferenceType,
    #[serde(rename = "DeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub declaration_text_information: Option<String2000Type>,
    #[serde(rename = "SpecialPermission")]
    pub special_permission: YesNoType,
    #[serde(rename = "CuttingRightsOwner", skip_serializing_if = "Option::is_none")]
    pub cutting_rights_owner: Option<ContactInformationType>,
    #[serde(rename = "CuttingsRightsOwnerRepresentative", skip_serializing_if = "Option::is_none")]
    pub cuttings_rights_owner_representative: Option<ContactInformationType>,
    #[serde(rename = "Sender")]
    pub sender: SenderType,
    #[serde(rename = "DeclarationRealEstates")]
    pub declaration_real_estates: DeclarationRealEstatesType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<SpecialFeaturesType>,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "AcceptanceDate")]
    pub acceptance_date: AcceptanceDateType,
    #[serde(rename = "AdditionalText", skip_serializing_if = "Option::is_none")]
    pub additional_text: Option<AdditionalTextType>,
    #[serde(rename = "OriginalXmlFile", skip_serializing_if = "Option::is_none")]
    pub original_xml_file: Option<base64Binary>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
    #[serde(rename = "ForestUseDeclarationNotNeeded", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_not_needed: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralsType {
    #[serde(rename = "PeripheralCode", skip_serializing_if = "Option::is_none")]
    pub peripheral_code: Option<Vec<PeripheralCodeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMultiPolygonGeometryType {
    #[serde(rename = "MultiPolygon")]
    pub multi_polygon: String,
    #[serde(rename = "MultiSurface")]
    pub multi_surface: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioningType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundWoodSalesDataType {
    #[serde(rename = "CompanyID")]
    pub company_i_d: CompanyIDType,
    #[serde(rename = "StartDate")]
    pub start_date: StartDateType,
    #[serde(rename = "EndDate")]
    pub end_date: EndDateType,
    #[serde(rename = "RoundWoodSalesRows")]
    pub round_wood_sales_rows: RoundWoodSalesRowsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SawinghoursDataType {
    #[serde(rename = "Hours")]
    pub hours: PositiveInteger2digitsType,
    #[serde(rename = "Minutes")]
    pub minutes: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDistributionStratumType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
    pub stratum_number: Option<StratumNumberType>,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<AgeType>,
    #[serde(rename = "BasalArea")]
    pub basal_area: BasalAreaType,
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteWorkTimeType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "StartTime")]
    pub start_time: TimeStampType,
    #[serde(rename = "EndTime")]
    pub end_time: TimeStampType,
    #[serde(rename = "SavingTime")]
    pub saving_time: TimeStampType,
    #[serde(rename = "Sawinghours", skip_serializing_if = "Option::is_none")]
    pub sawinghours: Option<SawinghoursDataType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteEndNotificationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContactorId")]
    pub contactor_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "Interrupted")]
    pub interrupted: YesNoType,
    #[serde(rename = "FulfilledArea", skip_serializing_if = "Option::is_none")]
    pub fulfilled_area: Option<FulfilledAreasType>,
    #[serde(rename = "UnfulfilledArea", skip_serializing_if = "Option::is_none")]
    pub unfulfilled_area: Option<PolygonOrMultiPolygon2Type>,
    #[serde(rename = "Infotext", skip_serializing_if = "Option::is_none")]
    pub infotext: Option<String1000Type>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "WorkCodes", skip_serializing_if = "Option::is_none")]
    pub work_codes: Option<WorkCodesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlSilvicultureType {
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionStandsType {
    #[serde(rename = "FinancingActCompletionStand")]
    pub financing_act_completion_stand: Vec<FinancingActCompletionStandType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclarationType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: String,
    #[serde(rename = "CompletionDeclarationReference")]
    pub completion_declaration_reference: String,
    #[serde(rename = "FinancingActNumber")]
    pub financing_act_number: String,
    #[serde(rename = "FinancingActApplicationReference", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_reference: Option<String>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[serde(rename = "BankReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub bank_reference_number: Option<BankReferenceNumberType>,
    #[serde(rename = "FinancingActProjectCompleted")]
    pub financing_act_project_completed: YesNoType,
    #[serde(rename = "ExtraFinancingApplication")]
    pub extra_financing_application: YesNoType,
    #[serde(rename = "OtherPublicSubstitute")]
    pub other_public_substitute: OtherPublicSubstituteType,
    #[serde(rename = "FinancingActCompletionDeclarationTextInformation", skip_serializing_if = "Option::is_none")]
    pub financing_act_completion_declaration_text_information: Option<String>,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "ElectronicNotification")]
    pub electronic_notification: String,
    #[serde(rename = "Sender")]
    pub sender: String,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<String>,
    #[serde(rename = "SentDate")]
    pub sent_date: String,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "FinancingActRealEstates")]
    pub financing_act_real_estates: String,
    #[serde(rename = "CompletionDeclarationActors")]
    pub completion_declaration_actors: String,
    #[serde(rename = "WorkingRepresentatives", skip_serializing_if = "Option::is_none")]
    pub working_representatives: Option<String>,
    #[serde(rename = "PartsOfProject")]
    pub parts_of_project: PartsOfProjectType,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStrataType {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratumType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WoodLotsType {
    #[serde(rename = "Woodlot")]
    pub woodlot: Vec<WoodLotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorsType {
    #[serde(rename = "Actor")]
    pub actor: Vec<ActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthCalibrationType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "LengthCalibrationDate")]
    pub length_calibration_date: TimeStampType,
    #[serde(rename = "LengthCalibrationReason")]
    pub length_calibration_reason: String200Type,
    #[serde(rename = "LengthCalibrationDescription")]
    pub length_calibration_description: String200Type,
    #[serde(rename = "LengthCalibrationAdjustment")]
    pub length_calibration_adjustment: Integer3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferWorkingSitePaymentTransactionsType {
    #[serde(rename = "PaymentTransactions")]
    pub payment_transactions: Vec<PaymentTransactionsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationsType {
    #[serde(rename = "ResourceLocation")]
    pub resource_location: Vec<ResourceLocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistanceType {
    #[serde(rename = "StorageForestHaulageDistanceGroup")]
    pub storage_forest_haulage_distance_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectiveAgreementsType {
    #[serde(rename = "CollectiveAgreement")]
    pub collective_agreement: Vec<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRightType {
    #[serde(rename = "UsingRightExists")]
    pub using_right_exists: YesNoNotKnownType,
    #[serde(rename = "UsingRightExaminedDate")]
    pub using_right_examined_date: DateType,
    #[serde(rename = "UsingRightCompensation")]
    pub using_right_compensation: YesNoNotKnownType,
    #[serde(rename = "UsingRightCompensationAmount", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_amount: Option<Decimal7And2Type>,
    #[serde(rename = "UsingRightCompensationDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_compensation_description: Option<String1500Type>,
    #[serde(rename = "UsingRightCompensationResponsible")]
    pub using_right_compensation_responsible: UsingRightResponsibleType,
    #[serde(rename = "UsingRightDescription", skip_serializing_if = "Option::is_none")]
    pub using_right_description: Option<String1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessageDataType {
    #[serde(rename = "ReferenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<ForestCentreMessageReferenceType>,
    #[serde(rename = "Reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceType>,
    #[serde(rename = "KeyElementNS", skip_serializing_if = "Option::is_none")]
    pub key_element_n_s: Option<String500Type>,
    #[serde(rename = "KeyElementName", skip_serializing_if = "Option::is_none")]
    pub key_element_name: Option<String200Type>,
    #[serde(rename = "KeyElementId", skip_serializing_if = "Option::is_none")]
    pub key_element_id: Option<IdStringNotEmptyType>,
    #[serde(rename = "KeyInfoAsText", skip_serializing_if = "Option::is_none")]
    pub key_info_as_text: Option<String2000Type>,
    #[serde(rename = "ErrorCode")]
    pub error_code: String25Type,
    #[serde(rename = "ErrorMessage")]
    pub error_message: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilvicultureType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerRegisterType {
    #[serde(rename = "Registered")]
    pub registered: YesNoType,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationRestrictionType {
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String1000Type>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateType>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectKeysType {
    #[serde(rename = "StandKeyGroup1")]
    pub stand_key_group1: String,
    #[serde(rename = "StandKeyGroup2")]
    pub stand_key_group2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsOfProjectType {
    #[serde(rename = "PartOfProject")]
    pub part_of_project: Vec<PartOfProjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: NotEmptyTreeSpeciesType,
    #[serde(rename = "AverageStemVolume", skip_serializing_if = "Option::is_none")]
    pub average_stem_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "SawlogPercent", skip_serializing_if = "Option::is_none")]
    pub sawlog_percent: Option<SawLogPercentType>,
    #[serde(rename = "TotalSawlogVolume", skip_serializing_if = "Option::is_none")]
    pub total_sawlog_volume: Option<SawLogVolumeType>,
    #[serde(rename = "TotalPulpwoodVolume", skip_serializing_if = "Option::is_none")]
    pub total_pulpwood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "TotalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<VolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicDataWithGeometryType {
    #[serde(flatten)]
    pub base: StandBasicDataType,
    #[serde(rename = "Area")]
    pub area: AreaCodeType,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaType>,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionGeometryType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "ObjectType")]
    pub object_type: DecisionGeometryObjectType,
    #[serde(rename = "StandReference", skip_serializing_if = "Option::is_none")]
    pub stand_reference: Option<ReferenceType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<IdStringNotEmptyType>,
    #[serde(rename = "PointLineAndPolygonGeometriesGroup")]
    pub point_line_and_polygon_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanType {
    #[serde(rename = "WorkingSitePlanDate")]
    pub working_site_plan_date: WorkingSitePlanDateType,
    #[serde(rename = "WorkingSitePlanner")]
    pub working_site_planner: WorkingSitePlannerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSBType {
    #[serde(rename = "ShapeGamma")]
    pub shape_gamma: ShapeGammaType,
    #[serde(rename = "ShapeDelta")]
    pub shape_delta: ShapeDeltaType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
    #[serde(rename = "Location")]
    pub location: LocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadRangeType {
    #[serde(rename = "StartLoadNumber")]
    pub start_load_number: PositiveInteger4digitsType,
    #[serde(rename = "EndLoadNumber")]
    pub end_load_number: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleDataType {
    #[serde(rename = "ScaledMass")]
    pub scaled_mass: Decimal1FractionDigitType,
    #[serde(rename = "Orientation")]
    pub orientation: OrientationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesUnclassifiedType {
    #[serde(rename = "AssortmentVolumeUnclassified")]
    pub assortment_volume_unclassified: Vec<AssortmentVolumeUnclassifiedType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoppersType {
    #[serde(rename = "Hopper", skip_serializing_if = "Option::is_none")]
    pub hopper: Option<Vec<HopperType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "BusinessId")]
    pub business_id: string,
    #[serde(rename = "OrganizationName")]
    pub organization_name: string,
    #[serde(rename = "Roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RolesType>>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServicesType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionGeometriesType {
    #[serde(rename = "FinancingActCompletionGeometry")]
    pub financing_act_completion_geometry: Vec<FinancingActGeometryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationPolygonType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "DeclarationPolygonReference")]
    pub declaration_polygon_reference: ReferenceType,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "LocationEstates")]
    pub location_estates: LocationEstatesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionType {
    #[serde(rename = "InternalInspectionId")]
    pub internal_inspection_id: string,
    #[serde(rename = "VirtaId")]
    pub virta_id: VirtaIdType,
    #[serde(rename = "InspectionType", skip_serializing_if = "Option::is_none")]
    pub inspection_type: Option<VirtaInspectionTypeType>,
    #[serde(rename = "SaveIncomplete", skip_serializing_if = "Option::is_none")]
    pub save_incomplete: Option<VirtaSaveIncompleteType>,
    #[serde(rename = "Status1", skip_serializing_if = "Option::is_none")]
    pub status1: Option<ChangeStateType>,
    #[serde(rename = "AnnouncementId", skip_serializing_if = "Option::is_none")]
    pub announcement_id: Option<AnnouncementIdType>,
    #[serde(rename = "KemeraId", skip_serializing_if = "Option::is_none")]
    pub kemera_id: Option<VirtaIdType>,
    #[serde(rename = "EstateOwner", skip_serializing_if = "Option::is_none")]
    pub estate_owner: Option<string>,
    #[serde(rename = "MunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub municipality_number: Option<KuntaKoodiTyyppi>,
    #[serde(rename = "GroupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<string>,
    #[serde(rename = "UnitNumber", skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<string>,
    #[serde(rename = "UnseparatedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel_number: Option<string>,
    #[serde(rename = "UnseparatedParcel", skip_serializing_if = "Option::is_none")]
    pub unseparated_parcel: Option<string>,
    #[serde(rename = "KemeraMunicipalityId", skip_serializing_if = "Option::is_none")]
    pub kemera_municipality_id: Option<string>,
    #[serde(rename = "ArrivalDate", skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<DateMmDdYyyyType>,
    #[serde(rename = "TargetSelection", skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<VirtaTargetSelectionType>,
    #[serde(rename = "Population", skip_serializing_if = "Option::is_none")]
    pub population: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MastoInspection", skip_serializing_if = "Option::is_none")]
    pub masto_inspection: Option<VirtaMastoInspectionType>,
    #[serde(rename = "WorkType", skip_serializing_if = "Option::is_none")]
    pub work_type: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<VirtaPhaseType>,
    #[serde(rename = "ProjectStatus", skip_serializing_if = "Option::is_none")]
    pub project_status: Option<VirtaProjectStatusType>,
    #[serde(rename = "AnnouncedArea", skip_serializing_if = "Option::is_none")]
    pub announced_area: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "AnnouncedLength", skip_serializing_if = "Option::is_none")]
    pub announced_length: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "Law", skip_serializing_if = "Option::is_none")]
    pub law: Option<VirtaLawType>,
    #[serde(rename = "EarliestInspectionDate", skip_serializing_if = "Option::is_none")]
    pub earliest_inspection_date: Option<DateMmDdYyyyType>,
    #[serde(rename = "Advertiser", skip_serializing_if = "Option::is_none")]
    pub advertiser: Option<VirtaAdvertiserType>,
    #[serde(rename = "Operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<VirtaAdvertiserType>,
    #[serde(rename = "AdvertisementDating", skip_serializing_if = "Option::is_none")]
    pub advertisement_dating: Option<VirtaAdvertisementDatingType>,
    #[serde(rename = "AreaAndMapEvaluation", skip_serializing_if = "Option::is_none")]
    pub area_and_map_evaluation: Option<VirtaEvaluationType>,
    #[serde(rename = "OtherEvaluation", skip_serializing_if = "Option::is_none")]
    pub other_evaluation: Option<VirtaEvaluationType>,
    #[serde(rename = "TreeDamageOutsideStand", skip_serializing_if = "Option::is_none")]
    pub tree_damage_outside_stand: Option<VirtaEvaluationType>,
    #[serde(rename = "TerrainDamageOutsideStand", skip_serializing_if = "Option::is_none")]
    pub terrain_damage_outside_stand: Option<VirtaEvaluationType>,
    #[serde(rename = "InspectionDate", skip_serializing_if = "Option::is_none")]
    pub inspection_date: Option<DateMmDdYyyyType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<VirtaExtraInfoType>,
    #[serde(rename = "HarvestExtraInfo", skip_serializing_if = "Option::is_none")]
    pub harvest_extra_info: Option<VirtaExtraInfoType>,
    #[serde(rename = "SumTableArea", skip_serializing_if = "Option::is_none")]
    pub sum_table_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "StubPriceArea", skip_serializing_if = "Option::is_none")]
    pub stub_price_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "UnitCostArea", skip_serializing_if = "Option::is_none")]
    pub unit_cost_area: Option<VirtaSumTableAreaType>,
    #[serde(rename = "EvaluationCost", skip_serializing_if = "Option::is_none")]
    pub evaluation_cost: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "InsuranceOrOtherCompensation", skip_serializing_if = "Option::is_none")]
    pub insurance_or_other_compensation: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TotalCompensation", skip_serializing_if = "Option::is_none")]
    pub total_compensation: Option<PositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "OwnerInvolvement", skip_serializing_if = "Option::is_none")]
    pub owner_involvement: Option<VirtaYesNoType>,
    #[serde(rename = "AssociationInvolvement", skip_serializing_if = "Option::is_none")]
    pub association_involvement: Option<VirtaYesNoType>,
    #[serde(rename = "OwnerSampleAreaApproval", skip_serializing_if = "Option::is_none")]
    pub owner_sample_area_approval: Option<VirtaApprovalType>,
    #[serde(rename = "OwnerActionApproval", skip_serializing_if = "Option::is_none")]
    pub owner_action_approval: Option<VirtaApprovalType>,
    #[serde(rename = "MoosePercentage", skip_serializing_if = "Option::is_none")]
    pub moose_percentage: Option<PercentType>,
    #[serde(rename = "AssociationEvaluationApproval", skip_serializing_if = "Option::is_none")]
    pub association_evaluation_approval: Option<VirtaApprovalType>,
    #[serde(rename = "Targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<TargetsType>,
    #[serde(rename = "HelpGeometries", skip_serializing_if = "Option::is_none")]
    pub help_geometries: Option<HelpGeometriesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainsType {
    #[serde(rename = "PlannedOperationChain")]
    pub planned_operation_chain: Vec<PlannedOperationChainType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedOperationChainType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "AlternativeIdentifier")]
    pub alternative_identifier: AlternativeIdentifierType,
    #[serde(rename = "AlternativeName", skip_serializing_if = "Option::is_none")]
    pub alternative_name: Option<AlternativeNameType>,
    #[serde(rename = "Operations")]
    pub operations: OperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<SpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioningBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCallForOfferType {
    #[serde(rename = "RelatedCallForOfferId")]
    pub related_call_for_offer_id: string,
    #[serde(rename = "RelatedCallForOfferDescription", skip_serializing_if = "Option::is_none")]
    pub related_call_for_offer_description: Option<String1500Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteWoodTradeInfoType {
    #[serde(rename = "ForestUseDeclarationResponsible")]
    pub forest_use_declaration_responsible: ForestUseDeclarationResponsibleType,
    #[serde(rename = "ForestUseDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_reference: Option<String>,
    #[serde(rename = "FellingRightValidityDate", skip_serializing_if = "Option::is_none")]
    pub felling_right_validity_date: Option<FellingRightValidityDateType>,
    #[serde(rename = "SupplyPoint", skip_serializing_if = "Option::is_none")]
    pub supply_point: Option<SupplyPointType>,
    #[serde(rename = "AssortmentClasses", skip_serializing_if = "Option::is_none")]
    pub assortment_classes: Option<String>,
    #[serde(rename = "Cuttings", skip_serializing_if = "Option::is_none")]
    pub cuttings: Option<CuttingsType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandWoodTradeInfoType {
    #[serde(rename = "CuttingAreaPreclearingNeed", skip_serializing_if = "Option::is_none")]
    pub cutting_area_preclearing_need: Option<CuttingAreaPreclearingNeedType>,
    #[serde(rename = "BearingCapacityClass", skip_serializing_if = "Option::is_none")]
    pub bearing_capacity_class: Option<BearingCapacityClassType>,
    #[serde(rename = "ForestHaulageDistance", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_distance: Option<ForestHaulageDistanceType>,
    #[serde(rename = "StoutTimberClassifier", skip_serializing_if = "Option::is_none")]
    pub stout_timber_classifier: Option<StoutTimberClassifierType>,
    #[serde(rename = "LoggingAccessibility", skip_serializing_if = "Option::is_none")]
    pub logging_accessibility: Option<HarvestingAccessibilityType>,
    #[serde(rename = "ForestHaulageAccessibility", skip_serializing_if = "Option::is_none")]
    pub forest_haulage_accessibility: Option<HarvestingAccessibilityType>,
    #[serde(rename = "PlannedBasalArea", skip_serializing_if = "Option::is_none")]
    pub planned_basal_area: Option<BasalAreaType>,
    #[serde(rename = "PlannedStemCount", skip_serializing_if = "Option::is_none")]
    pub planned_stem_count: Option<StemCountType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaserRepresentativePersonType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionHandlerType {
    #[serde(rename = "@role")]
    pub role: String100Type,
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentChangeDataType {
    #[serde(rename = "OldDestinationStorage")]
    pub old_destination_storage: String20Type,
    #[serde(rename = "OldCode")]
    pub old_code: String50Type,
    #[serde(rename = "NewDestinationStorage")]
    pub new_destination_storage: String20Type,
    #[serde(rename = "NewCode")]
    pub new_code: String50Type,
    #[serde(rename = "ChangeVolume")]
    pub change_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerRepresentativePersonType {
    #[serde(flatten)]
    pub base: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesSummaryType {
    #[serde(rename = "TreeSpeciesData")]
    pub tree_species_data: Vec<TreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationsType {
    #[serde(rename = "Organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Vec<OrganizationType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlannerType {
    #[serde(flatten)]
    pub base: ContactInformationType,
    #[serde(rename = "CuttingPlannerLiability", skip_serializing_if = "Option::is_none")]
    pub cutting_planner_liability: Option<CuttingPlannerLiabilityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSymbolDataType {
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<ERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<String20Type>,
    #[serde(rename = "MapSymbolName", skip_serializing_if = "Option::is_none")]
    pub map_symbol_name: Option<String20Type>,
    #[serde(rename = "FeatureType", skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<FeatureTypeType>,
    #[serde(rename = "FeatureCode")]
    pub feature_code: FeatureCodeType,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub feature_additional_code: Option<FeatureAdditionalCodeType>,
    #[serde(rename = "FeatureInfo", skip_serializing_if = "Option::is_none")]
    pub feature_info: Option<String1000Type>,
    #[serde(rename = "FeatureAdditionalInfo", skip_serializing_if = "Option::is_none")]
    pub feature_additional_info: Option<String1000Type>,
    #[serde(rename = "Geometry")]
    pub geometry: AlternativeGeometries2Type,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<BufferDistanceType>,
    #[serde(rename = "CanModify")]
    pub can_modify: YesNoType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "DitchType", skip_serializing_if = "Option::is_none")]
    pub ditch_type: Option<DitchTypeType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<PositiveInteger5digitsType>,
    #[serde(rename = "Depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<PositiveInteger3digitsType>,
    #[serde(rename = "MaterialCode", skip_serializing_if = "Option::is_none")]
    pub material_code: Option<MaterialCodeType>,
    #[serde(rename = "MaterialInfoText", skip_serializing_if = "Option::is_none")]
    pub material_info_text: Option<String1000Type>,
    #[serde(rename = "DitchOrRoadPlanName", skip_serializing_if = "Option::is_none")]
    pub ditch_or_road_plan_name: Option<String100Type>,
    #[serde(rename = "SpareGroupOfTrees", skip_serializing_if = "Option::is_none")]
    pub spare_group_of_trees: Option<SpareTreesByCategoryType>,
    #[serde(rename = "UsingRestrictions", skip_serializing_if = "Option::is_none")]
    pub using_restrictions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditsType {
    #[serde(rename = "hasEnvironmentalObjects")]
    pub has_environmental_objects: YesNoType,
    #[serde(rename = "hasEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub has_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalObjectsNoticed", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed: Option<YesNoType>,
    #[serde(rename = "EnvironmentalObjectsNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_objects_noticed_text: Option<String200Type>,
    #[serde(rename = "hasFoundNewEnvironmentalObjects")]
    pub has_found_new_environmental_objects: YesNoType,
    #[serde(rename = "NewEnvironmentalObjectsText", skip_serializing_if = "Option::is_none")]
    pub new_environmental_objects_text: Option<String200Type>,
    #[serde(rename = "LimitsToWaterSystem", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system: Option<YesNoType>,
    #[serde(rename = "LimitsToWaterSystemText", skip_serializing_if = "Option::is_none")]
    pub limits_to_water_system_text: Option<String200Type>,
    #[serde(rename = "WaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub water_system_protection: Option<YesNoType>,
    #[serde(rename = "WaterSystemProtectionText", skip_serializing_if = "Option::is_none")]
    pub water_system_protection_text: Option<String200Type>,
    #[serde(rename = "WorkingSafetyNoticed", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed: Option<YesNoType>,
    #[serde(rename = "WorkingSafetyNoticedText", skip_serializing_if = "Option::is_none")]
    pub working_safety_noticed_text: Option<String200Type>,
    #[serde(rename = "WorkingInstructionsSufficient", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient: Option<YesNoType>,
    #[serde(rename = "WorkingInstructionsSufficientText", skip_serializing_if = "Option::is_none")]
    pub working_instructions_sufficient_text: Option<String200Type>,
    #[serde(rename = "Storage")]
    pub storage: YesNoType,
    #[serde(rename = "StorageText", skip_serializing_if = "Option::is_none")]
    pub storage_text: Option<String200Type>,
    #[serde(rename = "RoadDamages", skip_serializing_if = "Option::is_none")]
    pub road_damages: Option<YesNoType>,
    #[serde(rename = "RoadDamagesText", skip_serializing_if = "Option::is_none")]
    pub road_damages_text: Option<String200Type>,
    #[serde(rename = "TreeDamages", skip_serializing_if = "Option::is_none")]
    pub tree_damages: Option<YesNoType>,
    #[serde(rename = "TreeDamagesText", skip_serializing_if = "Option::is_none")]
    pub tree_damages_text: Option<String200Type>,
    #[serde(rename = "VehiclePathPressures", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_pressures: Option<YesNoType>,
    #[serde(rename = "VehiclePathPressuresText", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_pressures_text: Option<String200Type>,
    #[serde(rename = "BioMassQuality")]
    pub bio_mass_quality: YesNoType,
    #[serde(rename = "BioMassQualityText", skip_serializing_if = "Option::is_none")]
    pub bio_mass_quality_text: Option<String200Type>,
    #[serde(rename = "ForestEnergySuitable", skip_serializing_if = "Option::is_none")]
    pub forest_energy_suitable: Option<YesNoType>,
    #[serde(rename = "ForestEnergySuitableText", skip_serializing_if = "Option::is_none")]
    pub forest_energy_suitable_text: Option<String200Type>,
    #[serde(rename = "RemainingBiomass", skip_serializing_if = "Option::is_none")]
    pub remaining_biomass: Option<YesNoType>,
    #[serde(rename = "RemainingBiomassText", skip_serializing_if = "Option::is_none")]
    pub remaining_biomass_text: Option<String200Type>,
    #[serde(rename = "SedimentationBasinAndDamDimensioningIsFollowed", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_and_dam_dimensioning_is_followed: Option<YesNoNotKnownType>,
    #[serde(rename = "SedimentationBasinAndDamDimensioningIsFollowedText", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_and_dam_dimensioning_is_followed_text: Option<String200Type>,
    #[serde(rename = "SedimentationBasinSizeShapeAndFunctionality")]
    pub sedimentation_basin_size_shape_and_functionality: WorkingQualityType,
    #[serde(rename = "SedimentationBasinSizeShapeAndFunctionalityText", skip_serializing_if = "Option::is_none")]
    pub sedimentation_basin_size_shape_and_functionality_text: Option<String200Type>,
    #[serde(rename = "SuspensionHoleDensitySizeAndLocation")]
    pub suspension_hole_density_size_and_location: WorkingQualityType,
    #[serde(rename = "SuspensionHoleDensitySizeAndLocationText", skip_serializing_if = "Option::is_none")]
    pub suspension_hole_density_size_and_location_text: Option<String200Type>,
    #[serde(rename = "CleaningBreaksDone")]
    pub cleaning_breaks_done: YesNoType,
    #[serde(rename = "CleaningBreaksDoneText", skip_serializing_if = "Option::is_none")]
    pub cleaning_breaks_done_text: Option<String200Type>,
    #[serde(rename = "DitchCleaningBreakLocation")]
    pub ditch_cleaning_break_location: WorkingQualityType,
    #[serde(rename = "DitchCleaningBreakLocationText", skip_serializing_if = "Option::is_none")]
    pub ditch_cleaning_break_location_text: Option<String200Type>,
    #[serde(rename = "SurfaceRunoffFieldGroundIsUnbroken")]
    pub surface_runoff_field_ground_is_unbroken: YesNoNotKnownType,
    #[serde(rename = "SurfaceRunoffFieldGroundIsUnbrokenText", skip_serializing_if = "Option::is_none")]
    pub surface_runoff_field_ground_is_unbroken_text: Option<String200Type>,
    #[serde(rename = "DitchDepthWidthAndDrainageEffect")]
    pub ditch_depth_width_and_drainage_effect: WorkingQualityType,
    #[serde(rename = "DitchDepthWidthAndDrainageEffectText", skip_serializing_if = "Option::is_none")]
    pub ditch_depth_width_and_drainage_effect_text: Option<String200Type>,
    #[serde(rename = "SomeDitchesNotDiggedAsMentionedInOrder")]
    pub some_ditches_not_digged_as_mentioned_in_order: YesNoType,
    #[serde(rename = "SomeDitchesNotDiggedAsMentionedInOrderText", skip_serializing_if = "Option::is_none")]
    pub some_ditches_not_digged_as_mentioned_in_order_text: Option<String200Type>,
    #[serde(rename = "ExcavationSoilLocation")]
    pub excavation_soil_location: WorkingQualityType,
    #[serde(rename = "ExcavationSoilLocationText", skip_serializing_if = "Option::is_none")]
    pub excavation_soil_location_text: Option<String200Type>,
    #[serde(rename = "ExcavatorMovingAndTreeDamages")]
    pub excavator_moving_and_tree_damages: WorkingQualityType,
    #[serde(rename = "ExcavatorMovingAndTreeDamagesText", skip_serializing_if = "Option::is_none")]
    pub excavator_moving_and_tree_damages_text: Option<String200Type>,
    #[serde(rename = "NewEnvironmentalObjects")]
    pub new_environmental_objects: YesNoType,
    #[serde(rename = "EnvironmentalObjectNoticed")]
    pub environmental_object_noticed: YesNoType,
    #[serde(rename = "EnvironmentalObjectNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_object_noticed_text: Option<String200Type>,
    #[serde(rename = "FertilizationTotalAudit")]
    pub fertilization_total_audit: WorkingQualityType,
    #[serde(rename = "FertilizationTotalAuditText", skip_serializing_if = "Option::is_none")]
    pub fertilization_total_audit_text: Option<String200Type>,
    #[serde(rename = "TreeOrGroundDamages")]
    pub tree_or_ground_damages: YesNoType,
    #[serde(rename = "TreeOrGroundDamagesText", skip_serializing_if = "Option::is_none")]
    pub tree_or_ground_damages_text: Option<String200Type>,
    #[serde(rename = "EnvironmentCleanlinessNoticed")]
    pub environment_cleanliness_noticed: YesNoType,
    #[serde(rename = "EnvironmentCleanlinessNoticedText", skip_serializing_if = "Option::is_none")]
    pub environment_cleanliness_noticed_text: Option<String200Type>,
    #[serde(rename = "DrainStorageAsInstructed")]
    pub drain_storage_as_instructed: YesNoType,
    #[serde(rename = "DrainStorageAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub drain_storage_as_instructed_text: Option<String200Type>,
    #[serde(rename = "AirdromeAsInstructed", skip_serializing_if = "Option::is_none")]
    pub airdrome_as_instructed: Option<YesNoType>,
    #[serde(rename = "AirdromeAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub airdrome_as_instructed_text: Option<String200Type>,
    #[serde(rename = "WaterEconomySystemNoticed")]
    pub water_economy_system_noticed: YesNoType,
    #[serde(rename = "WaterEconomySystemNoticedText", skip_serializing_if = "Option::is_none")]
    pub water_economy_system_noticed_text: Option<String200Type>,
    #[serde(rename = "SaveTreesLeft")]
    pub save_trees_left: YesNoType,
    #[serde(rename = "SaveTreesLeftText", skip_serializing_if = "Option::is_none")]
    pub save_trees_left_text: Option<String200Type>,
    #[serde(rename = "StumpTreatment", skip_serializing_if = "Option::is_none")]
    pub stump_treatment: Option<YesNoNotNeededType>,
    #[serde(rename = "StumpTreatmentText", skip_serializing_if = "Option::is_none")]
    pub stump_treatment_text: Option<String200Type>,
    #[serde(rename = "HighStumps", skip_serializing_if = "Option::is_none")]
    pub high_stumps: Option<YesNoType>,
    #[serde(rename = "HighStumpsText", skip_serializing_if = "Option::is_none")]
    pub high_stumps_text: Option<String200Type>,
    #[serde(rename = "PreClearing", skip_serializing_if = "Option::is_none")]
    pub pre_clearing: Option<YesNoNotNeededType>,
    #[serde(rename = "PreClearingText", skip_serializing_if = "Option::is_none")]
    pub pre_clearing_text: Option<String200Type>,
    #[serde(rename = "SeparateSpareTrees", skip_serializing_if = "Option::is_none")]
    pub separate_spare_trees: Option<SpareTreesByCategoryType>,
    #[serde(rename = "SpareTreesFromMapSymbols", skip_serializing_if = "Option::is_none")]
    pub spare_trees_from_map_symbols: Option<SpareTreesByCategoryType>,
    #[serde(rename = "PlantManagementQuality")]
    pub plant_management_quality: WorkingQualityType,
    #[serde(rename = "PlantManagementQualityText", skip_serializing_if = "Option::is_none")]
    pub plant_management_quality_text: Option<String200Type>,
    #[serde(rename = "RoadStructure", skip_serializing_if = "Option::is_none")]
    pub road_structure: Option<WorkingQualityType>,
    #[serde(rename = "RoadStructureText", skip_serializing_if = "Option::is_none")]
    pub road_structure_text: Option<String200Type>,
    #[serde(rename = "RoadStructureShape", skip_serializing_if = "Option::is_none")]
    pub road_structure_shape: Option<WorkingQualityType>,
    #[serde(rename = "RoadStructureShapeText", skip_serializing_if = "Option::is_none")]
    pub road_structure_shape_text: Option<String200Type>,
    #[serde(rename = "RoadStructureDrainage", skip_serializing_if = "Option::is_none")]
    pub road_structure_drainage: Option<WorkingQualityType>,
    #[serde(rename = "RoadStructureDrainageText", skip_serializing_if = "Option::is_none")]
    pub road_structure_drainage_text: Option<String200Type>,
    #[serde(rename = "PassingPlace", skip_serializing_if = "Option::is_none")]
    pub passing_place: Option<WorkingQualityType>,
    #[serde(rename = "PassingPlaceText", skip_serializing_if = "Option::is_none")]
    pub passing_place_text: Option<String200Type>,
    #[serde(rename = "TurningPlace", skip_serializing_if = "Option::is_none")]
    pub turning_place: Option<WorkingQualityType>,
    #[serde(rename = "TurningPlaceText", skip_serializing_if = "Option::is_none")]
    pub turning_place_text: Option<String200Type>,
    #[serde(rename = "PipeInstallation", skip_serializing_if = "Option::is_none")]
    pub pipe_installation: Option<WorkingQualityType>,
    #[serde(rename = "PipeInstallationText", skip_serializing_if = "Option::is_none")]
    pub pipe_installation_text: Option<String200Type>,
    #[serde(rename = "ErosionBlockingAction", skip_serializing_if = "Option::is_none")]
    pub erosion_blocking_action: Option<WorkingQualityType>,
    #[serde(rename = "ErosionBlockingActionText", skip_serializing_if = "Option::is_none")]
    pub erosion_blocking_action_text: Option<String200Type>,
    #[serde(rename = "WaterProtectionAction", skip_serializing_if = "Option::is_none")]
    pub water_protection_action: Option<WorkingQualityType>,
    #[serde(rename = "WaterProtectionActionText", skip_serializing_if = "Option::is_none")]
    pub water_protection_action_text: Option<String200Type>,
    #[serde(rename = "LandScaping", skip_serializing_if = "Option::is_none")]
    pub land_scaping: Option<WorkingQualityType>,
    #[serde(rename = "LandScapingText", skip_serializing_if = "Option::is_none")]
    pub land_scaping_text: Option<String200Type>,
    #[serde(rename = "FeedbackForPlanner", skip_serializing_if = "Option::is_none")]
    pub feedback_for_planner: Option<WorkingQualityType>,
    #[serde(rename = "FeedbackForPlannerText", skip_serializing_if = "Option::is_none")]
    pub feedback_for_planner_text: Option<String200Type>,
    #[serde(rename = "SilvicultureMethodCorrect")]
    pub silviculture_method_correct: YesNoType,
    #[serde(rename = "SilvicultureMethodCorrectText", skip_serializing_if = "Option::is_none")]
    pub silviculture_method_correct_text: Option<String200Type>,
    #[serde(rename = "SoilConditioningMethod", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method: Option<WorkCodeType>,
    #[serde(rename = "SoilConditioningMethodCorrect", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method_correct: Option<YesNoType>,
    #[serde(rename = "SoilConditioningMethodCorrectText", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_method_correct_text: Option<String200Type>,
    #[serde(rename = "PlantStorageManagement")]
    pub plant_storage_management: YesNoType,
    #[serde(rename = "PlantStorageManagementText", skip_serializing_if = "Option::is_none")]
    pub plant_storage_management_text: Option<String200Type>,
    #[serde(rename = "SoilConditioningQuality")]
    pub soil_conditioning_quality: WorkingQualityType,
    #[serde(rename = "SoilConditioningQualityText", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_quality_text: Option<String200Type>,
    #[serde(rename = "WaterEconomySystemsNoticed")]
    pub water_economy_systems_noticed: YesNoType,
    #[serde(rename = "WaterEconomySystemsNoticedText", skip_serializing_if = "Option::is_none")]
    pub water_economy_systems_noticed_text: Option<String200Type>,
    #[serde(rename = "EnvironmentalTidinessNoticed")]
    pub environmental_tidiness_noticed: YesNoType,
    #[serde(rename = "EnvironmentalTidinessNoticedText", skip_serializing_if = "Option::is_none")]
    pub environmental_tidiness_noticed_text: Option<String200Type>,
    #[serde(rename = "SilvicultureQuality")]
    pub silviculture_quality: WorkingQualityType,
    #[serde(rename = "SilvicultureQualityText", skip_serializing_if = "Option::is_none")]
    pub silviculture_quality_text: Option<String200Type>,
    #[serde(rename = "BioMassCollectionDone", skip_serializing_if = "Option::is_none")]
    pub bio_mass_collection_done: Option<YesNoType>,
    #[serde(rename = "BioMassCollectionDoneText", skip_serializing_if = "Option::is_none")]
    pub bio_mass_collection_done_text: Option<String200Type>,
    #[serde(rename = "ClearingDone")]
    pub clearing_done: YesNoType,
    #[serde(rename = "ClearingDoneText", skip_serializing_if = "Option::is_none")]
    pub clearing_done_text: Option<String200Type>,
    #[serde(rename = "StumpsLifted", skip_serializing_if = "Option::is_none")]
    pub stumps_lifted: Option<YesNoType>,
    #[serde(rename = "StumpsLiftedText", skip_serializing_if = "Option::is_none")]
    pub stumps_lifted_text: Option<String200Type>,
    #[serde(rename = "DitchingBreaks", skip_serializing_if = "Option::is_none")]
    pub ditching_breaks: Option<YesNoType>,
    #[serde(rename = "DitchingBreaksText", skip_serializing_if = "Option::is_none")]
    pub ditching_breaks_text: Option<String200Type>,
    #[serde(rename = "SettlingPits", skip_serializing_if = "Option::is_none")]
    pub settling_pits: Option<YesNoType>,
    #[serde(rename = "SettlingPitsText", skip_serializing_if = "Option::is_none")]
    pub settling_pits_text: Option<String200Type>,
    #[serde(rename = "SettlingBasins", skip_serializing_if = "Option::is_none")]
    pub settling_basins: Option<YesNoType>,
    #[serde(rename = "SettlingBasinsText", skip_serializing_if = "Option::is_none")]
    pub settling_basins_text: Option<String200Type>,
    #[serde(rename = "SurfaceDrainingAreas", skip_serializing_if = "Option::is_none")]
    pub surface_draining_areas: Option<YesNoType>,
    #[serde(rename = "SurfaceDrainingAreasText", skip_serializing_if = "Option::is_none")]
    pub surface_draining_areas_text: Option<String200Type>,
    #[serde(rename = "OtherConservationMethods", skip_serializing_if = "Option::is_none")]
    pub other_conservation_methods: Option<YesNoType>,
    #[serde(rename = "OtherConservationMethodsText", skip_serializing_if = "Option::is_none")]
    pub other_conservation_methods_text: Option<String200Type>,
    #[serde(rename = "NotificationsAndImprovements", skip_serializing_if = "Option::is_none")]
    pub notifications_and_improvements: Option<YesNoType>,
    #[serde(rename = "NotificationsAndImprovementsText", skip_serializing_if = "Option::is_none")]
    pub notifications_and_improvements_text: Option<String200Type>,
    #[serde(rename = "StumpCuttingAsInstructed", skip_serializing_if = "Option::is_none")]
    pub stump_cutting_as_instructed: Option<YesNoType>,
    #[serde(rename = "StumpCuttingAsInstructedText", skip_serializing_if = "Option::is_none")]
    pub stump_cutting_as_instructed_text: Option<String200Type>,
    #[serde(rename = "StumpTidiness")]
    pub stump_tidiness: YesNoType,
    #[serde(rename = "StumpTidinessText", skip_serializing_if = "Option::is_none")]
    pub stump_tidiness_text: Option<String200Type>,
    #[serde(rename = "StumpLiftingSuitable")]
    pub stump_lifting_suitable: YesNoType,
    #[serde(rename = "StumpLiftingSuitableText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_suitable_text: Option<String200Type>,
    #[serde(rename = "RemainingStumpCount", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count: Option<YesNoType>,
    #[serde(rename = "RemainingStumpCountText", skip_serializing_if = "Option::is_none")]
    pub remaining_stump_count_text: Option<String200Type>,
    #[serde(rename = "StumpLiftingQuality", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality: Option<WorkingQualityType>,
    #[serde(rename = "StumpLiftingQualityText", skip_serializing_if = "Option::is_none")]
    pub stump_lifting_quality_text: Option<String200Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfilledAreasType {
    #[serde(rename = "FulfilledArea")]
    pub fulfilled_area: Vec<FulfilledAreaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequestType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@source")]
    pub source: string,
    #[serde(rename = "ContactInformation")]
    pub contact_information: ContactInformationType,
    #[serde(rename = "ContactMunicipality")]
    pub contact_municipality: MunicipalityNumberType,
    #[serde(rename = "ContactLocationInformation")]
    pub contact_location_information: AlternativeGeometriesType,
    #[serde(rename = "RequestCode")]
    pub request_code: RequestCodeType,
    #[serde(rename = "RequestInfo", skip_serializing_if = "Option::is_none")]
    pub request_info: Option<String2000Type>,
    #[serde(rename = "PreferredContactingMethods", skip_serializing_if = "Option::is_none")]
    pub preferred_contacting_methods: Option<PreferredContactingMethodsType>,
    #[serde(rename = "CreateDate")]
    pub create_date: DateType,
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateType>,
    #[serde(rename = "ForestPropertyDataSet", skip_serializing_if = "Option::is_none")]
    pub forest_property_data_set: Option<ForestPropertyDataSetType>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentrePaymentDetailsType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "CaseNumber", skip_serializing_if = "Option::is_none")]
    pub case_number: Option<FinancingActNumberType>,
    #[serde(rename = "ForestCentreMessageReferenceType", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference_type: Option<String>,
    #[serde(rename = "ForestCentreMessageReference", skip_serializing_if = "Option::is_none")]
    pub forest_centre_message_reference: Option<String>,
    #[serde(rename = "CompletionDeclarationReference", skip_serializing_if = "Option::is_none")]
    pub completion_declaration_reference: Option<String>,
    #[serde(rename = "CompletionDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub completion_declaration_number: Option<String>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[serde(rename = "ArrivalDate")]
    pub arrival_date: String,
    #[serde(rename = "PaymentReference")]
    pub payment_reference: String,
    #[serde(rename = "PaymentDate")]
    pub payment_date: String,
    #[serde(rename = "BankAccount")]
    pub bank_account: String,
    #[serde(rename = "Attorney", skip_serializing_if = "Option::is_none")]
    pub attorney: Option<ContactInformationType>,
    #[serde(rename = "ContactPerson")]
    pub contact_person: String,
    #[serde(rename = "PaymentTexts", skip_serializing_if = "Option::is_none")]
    pub payment_texts: Option<PaymentTextsType>,
    #[serde(rename = "OverallTotalSubsidy")]
    pub overall_total_subsidy: String,
    #[serde(rename = "SubsidyAppliers")]
    pub subsidy_appliers: SubsidyAppliersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWoodTradeInfoType {
    #[serde(rename = "PurchaseMode")]
    pub purchase_mode: PurchaseModeType,
    #[serde(rename = "IncludePaymentPlan", skip_serializing_if = "Option::is_none")]
    pub include_payment_plan: Option<IncludePaymentPlanType>,
    #[serde(rename = "IncludeForestFundPayment")]
    pub include_forest_fund_payment: IncludeForestFundPaymentType,
    #[serde(rename = "UsedPricingMethods", skip_serializing_if = "Option::is_none")]
    pub used_pricing_methods: Option<UsedPricingMethodsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifiersType {
    #[serde(rename = "Identifier")]
    pub identifier: Vec<IdentifierType>,
    #[serde(flatten)]
    pub base: IdentifiersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeUnclassifiedType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "AssortmentName")]
    pub assortment_name: String50Type,
    #[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
    pub assortment: Option<String50Type>,
    #[serde(rename = "Quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<String5Type>,
    #[serde(rename = "ProductGroupName", skip_serializing_if = "Option::is_none")]
    pub product_group_name: Option<String50Type>,
    #[serde(rename = "Count")]
    pub count: PositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpareTreesType {
    #[serde(rename = "SpareTreeCategory")]
    pub spare_tree_category: SpareTreeCategoryType,
    #[serde(rename = "AmountOfSpareTrees")]
    pub amount_of_spare_trees: PositiveInteger5digitsType,
    #[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tree_species: Option<TreeSpeciesType>,
    #[serde(rename = "MeanDiameterOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_of_spare_trees: Option<DiameterType>,
    #[serde(rename = "MeanHeightOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub mean_height_of_spare_trees: Option<HeightType>,
    #[serde(rename = "VolumeOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub volume_of_spare_trees: Option<VolumeType>,
    #[serde(rename = "DiameterClassOfSpareTrees", skip_serializing_if = "Option::is_none")]
    pub diameter_class_of_spare_trees: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallForOfferWorkingSiteSilvicultureInfoType {
    #[serde(rename = "SilviculturalOperations", skip_serializing_if = "Option::is_none")]
    pub silvicultural_operations: Option<SilviculturalOperationsType>,
    #[serde(rename = "Products", skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "CuttingVolume", skip_serializing_if = "Option::is_none")]
    pub cutting_volume: Option<CuttingVolumeType>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub assortments: Option<AssortmentsType>,
    #[serde(rename = "CuttingIncome", skip_serializing_if = "Option::is_none")]
    pub cutting_income: Option<CuttingIncomeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBinaryRestrictedSizeType {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDate2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationRealEstateType {
    #[serde(rename = "RealEstateId")]
    pub real_estate_id: IdStringNotEmptyType,
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "PlannedWorkAmount", skip_serializing_if = "Option::is_none")]
    pub planned_work_amount: Option<AmountType>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceDataType {
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<String100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<String20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceTypeType,
    #[serde(rename = "WorkingSites")]
    pub working_sites: WorkingSitesType,
    #[serde(rename = "ResourceName")]
    pub resource_name: String50Type,
    #[serde(rename = "WorkingHoursBusinessDay")]
    pub working_hours_business_day: PositiveInteger2digitsType,
    #[serde(rename = "WorkingHoursSaturday")]
    pub working_hours_saturday: PositiveInteger2digitsType,
    #[serde(rename = "WorkingHoursSunday")]
    pub working_hours_sunday: PositiveInteger2digitsType,
    #[serde(rename = "Days")]
    pub days: DaysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStratumType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "StratumNumber")]
    pub stratum_number: StratumNumberType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "TreeClass", skip_serializing_if = "Option::is_none")]
    pub tree_class: Option<TreeClassType>,
    #[serde(rename = "Storey")]
    pub storey: StoreyType,
    #[serde(rename = "Age")]
    pub age: AgeType,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<BasalAreaType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<MeanDiameterType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: MeanHeightType,
    #[serde(rename = "StratumOrigin", skip_serializing_if = "Option::is_none")]
    pub stratum_origin: Option<SeedlingOriginType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeType>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub saw_log_percent: Option<SawLogPercentType>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub saw_log_volume: Option<SawLogVolumeType>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub pulp_wood_volume: Option<PulpWoodVolumeType>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub volume_growth: Option<VolumeGrowthType>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub leaf_biomass: Option<LeafBiomassType>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub branch_biomass: Option<BranchBiomassType>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub stem_biomass: Option<StemBiomassType>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub stump_biomass: Option<StumpBiomassType>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyType>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub value_growth_percent: Option<Decimal2FractionDigitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumesType {
    #[serde(rename = "AssortmentMatrixVolume")]
    pub assortment_matrix_volume: Vec<AssortmentMatrixVolumeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityDataType {
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityType>,
    #[serde(rename = "TransportAccessibility", skip_serializing_if = "Option::is_none")]
    pub transport_accessibility: Option<TransportAccessibilityType>,
    #[serde(rename = "HarvestingAccessibility", skip_serializing_if = "Option::is_none")]
    pub harvesting_accessibility: Option<HarvestingAccessibilityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTreeSpeciesSummaryType {
    #[serde(rename = "OperativeTreeSpeciesData")]
    pub operative_tree_species_data: Vec<TreeSpeciesDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsQuantitiesType {
    #[serde(rename = "StatisticsQuantity")]
    pub statistics_quantity: Vec<StatisticsQuantityType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeaturesType {
    #[serde(rename = "ControlDataSpecialFeature")]
    pub control_data_special_feature: Vec<ControlDataSpecialFeatureType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Type {
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectBasicDataType {
    #[serde(rename = "ObjectReferenceType", skip_serializing_if = "Option::is_none")]
    pub object_reference_type: Option<ForestCentreMessageReferenceType>,
    #[serde(rename = "ObjectReference", skip_serializing_if = "Option::is_none")]
    pub object_reference: Option<ReferenceType>,
    #[serde(rename = "ControlledOperationType", skip_serializing_if = "Option::is_none")]
    pub controlled_operation_type: Option<CostTypeNumberType>,
    #[serde(rename = "ControlledOperationDescription", skip_serializing_if = "Option::is_none")]
    pub controlled_operation_description: Option<String100Type>,
    #[serde(rename = "ObjectNumber", skip_serializing_if = "Option::is_none")]
    pub object_number: Option<ObjectNumberType>,
    #[serde(rename = "ExtraInfo", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificationSystemsType {
    #[serde(rename = "CertificationSystem", skip_serializing_if = "Option::is_none")]
    pub certification_system: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluationType {
    #[serde(rename = "EvaluationCategory", skip_serializing_if = "Option::is_none")]
    pub evaluation_category: Option<EvaluationSubjectType>,
    #[serde(rename = "EvaluationCode", skip_serializing_if = "Option::is_none")]
    pub evaluation_code: Option<integer>,
    #[serde(rename = "EvaluationDescription", skip_serializing_if = "Option::is_none")]
    pub evaluation_description: Option<String1000Type>,
    #[serde(rename = "MainReason", skip_serializing_if = "Option::is_none")]
    pub main_reason: Option<YesNoType>,
    #[serde(rename = "ReasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<integer>,
    #[serde(rename = "ReasonDescription", skip_serializing_if = "Option::is_none")]
    pub reason_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteGeometriesType {
    #[serde(rename = "WorkingSiteGeometry")]
    pub working_site_geometry: Vec<LocatedSpecialFeature2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeatureDataType {
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "SpecialFeatures")]
    pub special_features: SpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpLiftingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationRealEstatesType {
    #[serde(flatten)]
    pub base: RealEstatesWithOwnersInformationType2,
    #[serde(rename = "ProcessingAreas")]
    pub processing_areas: ProcessingAreasType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealEstateType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "RegisterUnitId")]
    pub register_unit_id: RegisterUnitIdType,
    #[serde(rename = "EstateRegisterUnitGroup")]
    pub estate_register_unit_group: String,
    #[serde(rename = "UnseparetedParcelTypeChar", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_type_char: Option<UnseparetedParcelTypeCharType>,
    #[serde(rename = "UnseparetedParcelNumber", skip_serializing_if = "Option::is_none")]
    pub unsepareted_parcel_number: Option<UnseparetedParcelNumberType>,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: RealEstateNameType,
    #[serde(rename = "LocationMunicipalityNumber", skip_serializing_if = "Option::is_none")]
    pub location_municipality_number: Option<MunicipalityNumberType>,
    #[serde(rename = "LocationMunicipalityName", skip_serializing_if = "Option::is_none")]
    pub location_municipality_name: Option<MunicipalityNameType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelsType {
    #[serde(rename = "Parcel")]
    pub parcel: Vec<ParcelType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachinesType {
    #[serde(rename = "Machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<Vec<MachineTypeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDataType {
    #[serde(rename = "Name")]
    pub name: String100Type,
    #[serde(rename = "Data")]
    pub data: hexBinary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditSilvicultureType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<WorkCodeQualifierType1>,
    #[serde(rename = "Area")]
    pub area: Decimal2FractionDigitsType,
    #[serde(rename = "WorkingTime")]
    pub working_time: DateType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "SamplePlotCount")]
    pub sample_plot_count: PositiveInteger3digitsType,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger3digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: StanfordTreeSpeciesType,
    #[serde(rename = "StemType")]
    pub stem_type: HarvestingStemTypeType,
    #[serde(rename = "ProductUserId")]
    pub product_user_id: String100Type,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Assortment")]
    pub assortment: String50Type,
    #[serde(rename = "Quality")]
    pub quality: String5Type,
    #[serde(rename = "Count")]
    pub count: PositiveInteger6digitsType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "RunningMeters")]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesType {
    #[serde(rename = "Service")]
    pub service: Vec<OrganizationServiceType>,
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
pub struct FinancingActCompletionStandType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "StandNumber")]
    pub stand_number: StandNumberType,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub stand_number_extension: Option<StandNumberExtensionType>,
    #[serde(rename = "StandReference")]
    pub stand_reference: ReferenceType,
    #[serde(rename = "Area")]
    pub area: AreaType,
    #[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
    pub main_group: Option<ExtendedMainGroupType>,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<SubGroupType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<FertilityClassType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<SoilTypeType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<TreeSpeciesConciseType>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub stem_count: Option<StemCountType>,
    #[serde(rename = "HeightClass", skip_serializing_if = "Option::is_none")]
    pub height_class: Option<HeightClassType>,
    #[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
    pub mean_height: Option<HeightType>,
    #[serde(rename = "DiameterClass", skip_serializing_if = "Option::is_none")]
    pub diameter_class: Option<DiameterClassType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<DiameterType>,
    #[serde(rename = "RemovalClass", skip_serializing_if = "Option::is_none")]
    pub removal_class: Option<RemovalClassType>,
    #[serde(rename = "CuttingStemCount", skip_serializing_if = "Option::is_none")]
    pub cutting_stem_count: Option<CuttingStemCountType>,
    #[serde(rename = "SmallWoodRemovalClass", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_class: Option<SmallWoodRemovalClassType>,
    #[serde(rename = "SmallWoodRemovalVolume", skip_serializing_if = "Option::is_none")]
    pub small_wood_removal_volume: Option<PositiveInteger4digitsType>,
    #[serde(rename = "CostTypeAndCompletedWorkCompletion")]
    pub cost_type_and_completed_work_completion: CostTypeAndCompletedWorkCompletionType,
    #[serde(rename = "StandExtraInfo", skip_serializing_if = "Option::is_none")]
    pub stand_extra_info: Option<String2000Type>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "LocationEstate")]
    pub location_estate: LocationEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditPlantManagementBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "WorkingSiteName", skip_serializing_if = "Option::is_none")]
    pub working_site_name: Option<String100Type>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "OrderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<ERPIdType>,
    #[serde(rename = "WorkCodeGroup", skip_serializing_if = "Option::is_none")]
    pub work_code_group: Option<WorkCodeGroupType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "WorkCodeQualifier", skip_serializing_if = "Option::is_none")]
    pub work_code_qualifier: Option<WorkCodeQualifierType1>,
    #[serde(rename = "ContractorId", skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<String20Type>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "WorkingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<DateType>,
    #[serde(rename = "Audit", skip_serializing_if = "Option::is_none")]
    pub audit: Option<AuditionType>,
    #[serde(rename = "SamplePlotCount", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotCountRequired", skip_serializing_if = "Option::is_none")]
    pub sample_plot_count_required: Option<PositiveInteger2digitsType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
    #[serde(rename = "AuditsList")]
    pub audits_list: AuditsListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypesType {
    #[serde(rename = "ServiceType")]
    pub service_type: Vec<ServiceTypeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagesType {
    #[serde(rename = "Language")]
    pub language: Vec<LanguageCode1Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpeciesAttributesType {
    #[serde(rename = "TreeSpeciesAttribute", skip_serializing_if = "Option::is_none")]
    pub tree_species_attribute: Option<Vec<TreeSpeciesAttributeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAndSubsidyType {
    #[serde(rename = "FinancingActApplicationStands", skip_serializing_if = "Option::is_none")]
    pub financing_act_application_stands: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueListItemType {
    #[serde(rename = "ListId")]
    pub list_id: PositiveIntegerType,
    #[serde(rename = "ListItem")]
    pub list_item: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteFinalAuditStumpForwardingType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "PurchaseContractId")]
    pub purchase_contract_id: String20Type,
    #[serde(rename = "WorkCodeGroup")]
    pub work_code_group: WorkCodeGroupType,
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "Audit")]
    pub audit: AuditionType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String1000Type>,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "Audits")]
    pub audits: AuditsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyAndPercentType {
    #[serde(rename = "AbsoluteQuantity")]
    pub absolute_quantity: AbsoluteQuantityType,
    #[serde(rename = "Percent")]
    pub percent: PercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreForestDataUpdateObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@objectType")]
    pub object_type: ObjectTypeType,
    #[serde(rename = "@objectTypeSpecifier")]
    pub object_type_specifier: ObjectTypeSpecifierType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjectsType>,
    #[serde(rename = "ObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub object_basic_data: Option<ObjectBasicDataType>,
    #[serde(rename = "UseCases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<ForestDataUpdateUseCasesType>,
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<ActorsType>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub real_estates: Option<RealEstatesType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<BaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<BaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<RestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<String>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreControlObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActRealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType2,
    #[serde(rename = "SubsidyApplierReferenceList")]
    pub subsidy_applier_reference_list: SubsidyApplierReferenceListType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionOtherSubjectsType {
    #[serde(rename = "CostTypeAndCompletedWorkCompletionRealEstate")]
    pub cost_type_and_completed_work_completion_real_estate: CostTypeAndCompletedWorkCompletionRealEstateType,
    #[serde(rename = "FinancingActCompletionGeometries")]
    pub financing_act_completion_geometries: FinancingActCompletionGeometriesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributesType {
    #[serde(rename = "Attribute")]
    pub attribute: AttributeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageType {
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "StorageDisplayId", skip_serializing_if = "Option::is_none")]
    pub storage_display_id: Option<String20Type>,
    #[serde(rename = "Geometry")]
    pub geometry: PointAndLineOrPolygonType,
    #[serde(rename = "GeometryModificationAllowed")]
    pub geometry_modification_allowed: YesNoType,
    #[serde(rename = "PlowingName", skip_serializing_if = "Option::is_none")]
    pub plowing_name: Option<String50Type>,
    #[serde(rename = "PlowingTelephone", skip_serializing_if = "Option::is_none")]
    pub plowing_telephone: Option<String20Type>,
    #[serde(rename = "PlowingEmail", skip_serializing_if = "Option::is_none")]
    pub plowing_email: Option<String50Type>,
    #[serde(rename = "PlowingArranged", skip_serializing_if = "Option::is_none")]
    pub plowing_arranged: Option<YesNoType>,
    #[serde(rename = "PlowingDate", skip_serializing_if = "Option::is_none")]
    pub plowing_date: Option<DateType>,
    #[serde(rename = "Accessibility")]
    pub accessibility: TransportAccessibilityType,
    #[serde(rename = "TurningPointClass", skip_serializing_if = "Option::is_none")]
    pub turning_point_class: Option<TurningPointClassType>,
    #[serde(rename = "StorageInfo", skip_serializing_if = "Option::is_none")]
    pub storage_info: Option<String500Type>,
    #[serde(rename = "DeliveryRestriction", skip_serializing_if = "Option::is_none")]
    pub delivery_restriction: Option<YesNoType>,
    #[serde(rename = "StorageName", skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String50Type>,
    #[serde(rename = "StorageAddress", skip_serializing_if = "Option::is_none")]
    pub storage_address: Option<String500Type>,
    #[serde(rename = "StorageDryingClass", skip_serializing_if = "Option::is_none")]
    pub storage_drying_class: Option<StorageDryingClassType>,
    #[serde(rename = "StorageLandOwner", skip_serializing_if = "Option::is_none")]
    pub storage_land_owner: Option<StorageLandOwnerType>,
    #[serde(rename = "StorageLandOwnerInformation", skip_serializing_if = "Option::is_none")]
    pub storage_land_owner_information: Option<ContactInformationType>,
    #[serde(rename = "StorageAdditionalRemarks", skip_serializing_if = "Option::is_none")]
    pub storage_additional_remarks: Option<String3000Type>,
    #[serde(rename = "StorageLinkedToWorkingSite", skip_serializing_if = "Option::is_none")]
    pub storage_linked_to_working_site: Option<YesNoType>,
    #[serde(rename = "StorageClass", skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<StorageDryingClassType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocatedSpecialFeature2Type {
    #[serde(flatten)]
    pub base: BasicFeature2Type,
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "StandAvgDiameterSummary")]
    pub stand_avg_diameter_summary: PositiveInteger3digitsType,
    #[serde(rename = "StandAvgHeightSummary")]
    pub stand_avg_height_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
    #[serde(rename = "StandAvgDominantHeight", skip_serializing_if = "Option::is_none")]
    pub stand_avg_dominant_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StandAvgAgeSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_age_summary: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StandAvgStemCountSummary")]
    pub stand_avg_stem_count_summary: PositiveInteger5digitsType,
    #[serde(rename = "StandBasalAreaSummary", skip_serializing_if = "Option::is_none")]
    pub stand_basal_area_summary: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StandVolumeSummary", skip_serializing_if = "Option::is_none")]
    pub stand_volume_summary: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StandStemDamagesPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_stem_damages_percentage: Option<PercentWithFraction1Type>,
    #[serde(rename = "StandRootDamagesPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_root_damages_percentage: Option<PercentWithFraction1Type>,
    #[serde(rename = "StandCorrectHeightStumpsPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_correct_height_stumps_percentage: Option<PercentWithFraction1Type>,
    #[serde(rename = "StandTooHeightStumpsPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_too_height_stumps_percentage: Option<PercentWithFraction1Type>,
    #[serde(rename = "StandVehiclePathTooDeepPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_vehicle_path_too_deep_percentage: Option<PercentWithFraction1Type>,
    #[serde(rename = "StandAvgVehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub stand_avg_vehicle_path_distance: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "StandVehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub stand_vehicle_path_width: Option<PositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TreeSummaries")]
    pub tree_summaries: TreeSummariesType,
    #[serde(rename = "AverageVolume")]
    pub average_volume: Decimal3FractionDigitsType,
    #[serde(rename = "ReductionCountSummary", skip_serializing_if = "Option::is_none")]
    pub reduction_count_summary: Option<PositiveInteger5digitsType>,
    #[serde(rename = "ReductionAvgDiameterSummary", skip_serializing_if = "Option::is_none")]
    pub reduction_avg_diameter_summary: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "PlantCountSummary", skip_serializing_if = "Option::is_none")]
    pub plant_count_summary: Option<PositiveInteger4digitsType>,
    #[serde(rename = "PlantSiteCountSummary", skip_serializing_if = "Option::is_none")]
    pub plant_site_count_summary: Option<PositiveInteger4digitsType>,
    #[serde(rename = "DitchesInAdditionToCultivation", skip_serializing_if = "Option::is_none")]
    pub ditches_in_addition_to_cultivation: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesByCategoryType {
    #[serde(rename = "SpareTrees")]
    pub spare_trees: Vec<FinalAuditSpareTreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaysType {
    #[serde(rename = "Day", skip_serializing_if = "Option::is_none")]
    pub day: Option<Vec<DayType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SilvicultureValidityType {
    #[serde(rename = "StartDate")]
    pub start_date: DateYYYY-MMOrYYYY-MM-DDType,
    #[serde(rename = "EndDate")]
    pub end_date: DateYYYY-MMOrYYYY-MM-DDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialsType {
    #[serde(rename = "Material", skip_serializing_if = "Option::is_none")]
    pub material: Option<Vec<MaterialType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
    #[serde(rename = "Message")]
    pub message: PayloadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentsType {
    #[serde(rename = "Attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentDataType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeAndCompletedWorkApplicationType {
    #[serde(rename = "CostType")]
    pub cost_type: CostTypeType,
    #[serde(rename = "PlannedWorkAmount")]
    pub planned_work_amount: AmountType,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<UnitPriceType>,
    #[serde(rename = "SubsidyAmount")]
    pub subsidy_amount: MoneyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInformationType {
    #[serde(rename = "DataName")]
    pub data_name: string,
    #[serde(rename = "DataId")]
    pub data_id: string,
    #[serde(rename = "InspectorName")]
    pub inspector_name: string,
    #[serde(rename = "Password")]
    pub password: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalAuditSpareTreesType {
    #[serde(flatten)]
    pub base: SpareTreesType,
    #[serde(rename = "RetentionTreeTarget", skip_serializing_if = "Option::is_none")]
    pub retention_tree_target: Option<PositiveInteger5digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWorkingSiteDetailsType {
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@parentVersionNo")]
    pub parent_version_no: int,
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "WorkingSiteText", skip_serializing_if = "Option::is_none")]
    pub working_site_text: Option<String1500Type>,
    #[serde(rename = "AssortmentClasses")]
    pub assortment_classes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsingRestrictionType {
    #[serde(rename = "RestrictionType")]
    pub restriction_type: String,
    #[serde(rename = "RestrictionCode")]
    pub restriction_code: String,
    #[serde(rename = "RestrictionStartDate", skip_serializing_if = "Option::is_none")]
    pub restriction_start_date: Option<StartDateType>,
    #[serde(rename = "RestrictionEndDate", skip_serializing_if = "Option::is_none")]
    pub restriction_end_date: Option<EndDateType>,
    #[serde(rename = "BufferDistance", skip_serializing_if = "Option::is_none")]
    pub buffer_distance: Option<BufferDistanceType>,
    #[serde(rename = "RestrictionDescription", skip_serializing_if = "Option::is_none")]
    pub restriction_description: Option<String1500Type>,
    #[serde(rename = "RestrictionOutOfObject", skip_serializing_if = "Option::is_none")]
    pub restriction_out_of_object: Option<YesNoType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationOfAttorneyType {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<String20Type>,
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceTypeType>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<ImageCategoryType>,
    #[serde(rename = "SubCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<ImageSubCategoryType>,
    #[serde(rename = "MapSymbolType", skip_serializing_if = "Option::is_none")]
    pub map_symbol_type: Option<FeatureCodeType>,
    #[serde(rename = "MapSymbolId", skip_serializing_if = "Option::is_none")]
    pub map_symbol_id: Option<ERPIdType>,
    #[serde(rename = "InsertedMapSymbolId", skip_serializing_if = "Option::is_none")]
    pub inserted_map_symbol_id: Option<String20Type>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<PointGeometryType>,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String200Type>,
    #[serde(rename = "Photographer", skip_serializing_if = "Option::is_none")]
    pub photographer: Option<String50Type>,
    #[serde(rename = "SamplePlotNumber", skip_serializing_if = "Option::is_none")]
    pub sample_plot_number: Option<PositiveInteger3digitsType>,
    #[serde(rename = "ImageDate", skip_serializing_if = "Option::is_none")]
    pub image_date: Option<TimeStampType>,
    #[serde(rename = "Filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String100Type>,
    #[serde(rename = "Bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<base64Binary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerContactInformationType {
    #[serde(rename = "ServiceBuyer")]
    pub service_buyer: Vec<ServiceBuyerType>,
}

