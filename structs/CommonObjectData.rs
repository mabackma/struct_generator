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
pub struct FourDigitPositiveIntegerType {
    #[serde(flatten)]
    pub base: integer,
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
pub struct WorkType {
    #[serde(rename = "WorkCode")]
    pub work_code: WorkCodeType,
    #[serde(rename = "MaterialCode", skip_serializing_if = "Option::is_none")]
    pub material_code: Option<MaterialCodeType>,
    #[serde(rename = "WorkCompletionDate", skip_serializing_if = "Option::is_none")]
    pub work_completion_date: Option<DateType>,
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
pub struct WorksType {
    #[serde(rename = "Work")]
    pub work: Vec<WorkType>,
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
pub struct ForestCentreForestDataUpdateObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreForestDataUpdateObjectType>,
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
pub struct SelfMonitoringEvaluationsType {
    #[serde(rename = "SelfMonitoringEvaluation")]
    pub self_monitoring_evaluation: Vec<SelfMonitoringEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNumberType {
    #[serde(flatten)]
    pub base: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreSelfMonitoringObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectsType {
    #[serde(rename = "ChildObject")]
    pub child_object: Vec<ChildObjectType>,
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
pub struct SamplePlotSizeType {
    #[serde(flatten)]
    pub base: Decimal3And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDate2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6_2Type {
    #[serde(flatten)]
    pub base: decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorsType {
    #[serde(rename = "Actor")]
    pub actor: Vec<ActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluationsType {
    #[serde(rename = "ControlEvaluation")]
    pub control_evaluation: Vec<ControlEvaluationType>,
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
pub struct TwoDigitPositiveIntegerType {
    #[serde(flatten)]
    pub base: integer,
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
pub struct ProjectNoType {
    #[serde(flatten)]
    pub base: ReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageDataType {
    #[serde(rename = "Damage")]
    pub damage: Vec<DamageType>,
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
pub struct Decimal5_1Type {
    #[serde(flatten)]
    pub base: decimal,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagementType {
    #[serde(rename = "DitchMeanDepth")]
    pub ditch_mean_depth: string,
    #[serde(rename = "DitchMeanWidth")]
    pub ditch_mean_width: string,
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
pub struct WorkSafetyRisksType {
    #[serde(rename = "WorkSafetyRiskDescription")]
    pub work_safety_risk_description: Vec<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataTreeStandDataDateType {
    #[serde(flatten)]
    pub base: TreeStandDataDateType,
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
pub struct Decimal3_1Type {
    #[serde(flatten)]
    pub base: decimal,
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
pub struct ControlDataForestRoadConstructionType {
    #[serde(rename = "AppliedLength")]
    pub applied_length: Vec<Decimal6_2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
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
pub struct ObjectTypeSpecifierType {
    #[serde(flatten)]
    pub base: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperationsType {
    #[serde(rename = "ObjectProtectionOperation")]
    pub object_protection_operation: Vec<ObjectProtectionOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeType {
    #[serde(flatten)]
    pub base: string,
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
pub struct ThreeDigitPositiveIntegerType {
    #[serde(flatten)]
    pub base: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstateType {
    #[serde(flatten)]
    pub base: BaseRealEstateType,
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
pub struct ControlDataSpecialFeaturesType {
    #[serde(rename = "ControlDataSpecialFeature")]
    pub control_data_special_feature: Vec<ControlDataSpecialFeatureType>,
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
pub struct ControlForestUseDeclarationType {
    #[serde(rename = "CuttingRealizationPractice", skip_serializing_if = "Option::is_none")]
    pub cutting_realization_practice: Option<CuttingTypeType>,
    #[serde(rename = "HarvestingSignControlClassifier", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_control_classifier: Option<EvaluationCodeType>,
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
pub struct ObjectKeysType {
    #[serde(rename = "StandKeyGroup1")]
    pub stand_key_group1: String,
    #[serde(rename = "StandKeyGroup2")]
    pub stand_key_group2: String,
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
pub struct ObjectProtectionOperationType {
    #[serde(rename = "OperationCode", skip_serializing_if = "Option::is_none")]
    pub operation_code: Option<ObjectProtectionOperationCodeType>,
    #[serde(rename = "OperationDescription", skip_serializing_if = "Option::is_none")]
    pub operation_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCasesType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<Vec<ForestDataUpdateUseCaseType>>,
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
pub struct ActorTypeSpecifierType {
    #[serde(flatten)]
    pub base: string,
}

