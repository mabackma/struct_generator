#[derive(Debug, Serialize, Deserialize)]
pub struct GoalTreeSpecies {
    #[serde(flatten)]
    pub goal_tree_species: CoTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NaturalCropStemCount {
    #[serde(flatten)]
    pub natural_crop_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathMeanWidth {
    #[serde(flatten)]
    pub vehicle_path_mean_width: Decimal5_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringType {
    #[serde(flatten)]
    pub self_monitoring_type: CoSelfMonitoringTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathSubsidencePercentage {
    #[serde(flatten)]
    pub vehicle_path_subsidence_percentage: Decimal3_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartEastingCoordinate {
    #[serde(flatten)]
    pub part_easting_coordinate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationDescription {
    #[serde(flatten)]
    pub evaluation_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationEnsuring {
    #[serde(flatten)]
    pub regeneration_ensuring: ThreeDigitPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathSubsidenceLength {
    #[serde(flatten)]
    pub vehicle_path_subsidence_length: Decimal3_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub terrain_damage_outside_stand_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageData {
    #[serde(flatten)]
    pub damage_data: DamageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockingWIthSeedlings {
    #[serde(flatten)]
    pub stocking_w_ith_seedlings: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairPlantingCosts {
    #[serde(flatten)]
    pub repair_planting_costs: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandBasedData {
    #[serde(flatten)]
    pub tree_stand_based_data: StTreeStandBasedDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSourceDescription {
    #[serde(flatten)]
    pub damage_source_description: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovalClassType {
    #[serde(flatten)]
    pub removal_class_type: RemovalClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeature {
    #[serde(flatten)]
    pub control_data_special_feature: ControlDataSpecialFeatureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainReason {
    #[serde(flatten)]
    pub main_reason: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallWoodRemovalClassType {
    #[serde(flatten)]
    pub small_wood_removal_class_type: SmallWoodRemovalClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthOfDitchDiggedDuringSoilPreparation {
    #[serde(flatten)]
    pub length_of_ditch_digged_during_soil_preparation: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Works {
    #[serde(flatten)]
    pub works: WorksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReferenceType {
    #[serde(flatten)]
    pub control_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilPreparationSpotsAreEnough {
    #[serde(flatten)]
    pub soil_preparation_spots_are_enough: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonDescription {
    #[serde(flatten)]
    pub reason_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathMeanDistance {
    #[serde(flatten)]
    pub vehicle_path_mean_distance: Decimal5_1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    #[serde(flatten)]
    pub actor: ActorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationInvolvement {
    #[serde(flatten)]
    pub association_involvement: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionData {
    #[serde(flatten)]
    pub restriction_data: StRestrictionDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub goal_amount_of_soil_preparation_spot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionBasedOnStoniness {
    #[serde(flatten)]
    pub restriction_based_on_stoniness: CoRestrictionBasedOnStoninessType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlledOperationDescription {
    #[serde(flatten)]
    pub controlled_operation_description: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingStemCountType {
    #[serde(flatten)]
    pub cutting_stem_count_type: CuttingStemCount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actors {
    #[serde(flatten)]
    pub actors: ActorsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDamagedSeedlingCount {
    #[serde(flatten)]
    pub not_damaged_seedling_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingAccessibility {
    #[serde(flatten)]
    pub harvesting_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationText {
    #[serde(flatten)]
    pub evaluation_text: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceOrOtherCompensation {
    #[serde(flatten)]
    pub insurance_or_other_compensation: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreclearingEvaluation {
    #[serde(flatten)]
    pub preclearing_evaluation: CoPreclearingEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensations {
    #[serde(flatten)]
    pub total_compensations: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNo {
    #[serde(flatten)]
    pub project_no: ProjectNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletingNaturalCropStemCount {
    #[serde(flatten)]
    pub completing_natural_crop_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagement {
    #[serde(flatten)]
    pub control_data_swamp_forest_management: ControlDataSwampForestManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataRegeneration {
    #[serde(flatten)]
    pub control_data_regeneration: RegenerationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationDeliveringEvaluation {
    #[serde(flatten)]
    pub declaration_delivering_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCase {
    #[serde(flatten)]
    pub use_case: CoForestDataUpdateUseCaseType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeIntervalForMeasuringSamplePlot {
    #[serde(flatten)]
    pub time_interval_for_measuring_sample_plot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedWorkingTimeConsumption {
    #[serde(flatten)]
    pub estimated_working_time_consumption: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectProtectionOperationsData {
    #[serde(flatten)]
    pub self_monitoring_object_protection_operations_data: SelfMonitoringObjectProtectionOperationsDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignData {
    #[serde(flatten)]
    pub harvesting_sign_data: HarvestingSignDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRiskDescription {
    #[serde(flatten)]
    pub work_safety_risk_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlledOperationType {
    #[serde(flatten)]
    pub controlled_operation_type: CoCostTypeNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCode {
    #[serde(flatten)]
    pub evaluation_code: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreinformDate {
    #[serde(flatten)]
    pub preinform_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMeanWidth {
    #[serde(flatten)]
    pub ditch_mean_width: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperations {
    #[serde(flatten)]
    pub object_protection_operations: ObjectProtectionOperationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBasicData {
    #[serde(flatten)]
    pub object_basic_data: ObjectBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivatedDeadStemCount {
    #[serde(flatten)]
    pub cultivated_dead_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreinformDetails {
    #[serde(flatten)]
    pub preinform_details: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSourceCode {
    #[serde(flatten)]
    pub damage_source_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProtectionOperation {
    #[serde(flatten)]
    pub object_protection_operation: ObjectProtectionOperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionalPermitForHandling {
    #[serde(flatten)]
    pub exceptional_permit_for_handling: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompartmentId {
    #[serde(flatten)]
    pub compartment_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlNo {
    #[serde(flatten)]
    pub control_no: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: CoString2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSizeTreeReduction {
    #[serde(flatten)]
    pub sample_plot_size_tree_reduction: SamplePlotSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingWorkQuality {
    #[serde(flatten)]
    pub planting_work_quality: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlOverallEvaluationData {
    #[serde(flatten)]
    pub control_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedForTreatment {
    #[serde(flatten)]
    pub need_for_treatment: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlBasicData {
    #[serde(flatten)]
    pub control_basic_data: ControlBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringDate {
    #[serde(flatten)]
    pub self_monitoring_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectOverallEvaluationData {
    #[serde(flatten)]
    pub object_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlreadyPaidCompensation {
    #[serde(flatten)]
    pub already_paid_compensation: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlReference {
    #[serde(flatten)]
    pub control_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityData {
    #[serde(flatten)]
    pub accessibility_data: AccessibilityDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationCode {
    #[serde(flatten)]
    pub operation_code: CoObjectProtectionOperationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaAndMapEvaluation {
    #[serde(flatten)]
    pub area_and_map_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagedSeedlingCount {
    #[serde(flatten)]
    pub damaged_seedling_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalStemCount {
    #[serde(flatten)]
    pub goal_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppliedLength {
    #[serde(flatten)]
    pub applied_length: Decimal6_2Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationMaterial {
    #[serde(flatten)]
    pub cultivation_material: TwoDigitPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class2DamageCount {
    #[serde(flatten)]
    pub class2_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParcelId {
    #[serde(flatten)]
    pub parcel_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoosePercentage {
    #[serde(flatten)]
    pub moose_percentage: CoPercentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainDamage {
    #[serde(flatten)]
    pub main_damage: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compensation {
    #[serde(flatten)]
    pub compensation: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCases {
    #[serde(flatten)]
    pub use_cases: ForestDataUpdateUseCasesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notices {
    #[serde(flatten)]
    pub notices: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReasonCode {
    #[serde(flatten)]
    pub reason_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlForestUseDeclaration {
    #[serde(flatten)]
    pub control_forest_use_declaration: ControlForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectReferenceType {
    #[serde(flatten)]
    pub object_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDate {
    #[serde(flatten)]
    pub control_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluations {
    #[serde(flatten)]
    pub self_monitoring_evaluations: SelfMonitoringEvaluationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeatures {
    #[serde(flatten)]
    pub control_data_special_features: ControlDataSpecialFeaturesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundManipulationMethod {
    #[serde(flatten)]
    pub ground_manipulation_method: ThreeDigitPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectData {
    #[serde(flatten)]
    pub common_object_data: CommonObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectBasicData {
    #[serde(flatten)]
    pub control_object_basic_data: ControlObjectBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataForestRoadConstruction {
    #[serde(flatten)]
    pub control_data_forest_road_construction: ControlDataForestRoadConstructionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilData {
    #[serde(flatten)]
    pub soil_data: StBaseSoilDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringBasicData {
    #[serde(flatten)]
    pub self_monitoring_basic_data: SelfMonitoringBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    #[serde(flatten)]
    pub work: WorkType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamagePercentage {
    #[serde(flatten)]
    pub stem_damage_percentage: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class1DamageCount {
    #[serde(flatten)]
    pub class1_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCategory {
    #[serde(flatten)]
    pub evaluation_category: CoEvaluationSubjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InTerrain {
    #[serde(flatten)]
    pub in_terrain: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivatedCropStemCount {
    #[serde(flatten)]
    pub cultivated_crop_stem_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingBy {
    #[serde(flatten)]
    pub cutting_by: CoVirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetSelection {
    #[serde(flatten)]
    pub target_selection: CoTargetSelectionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectTypeSpecifier {
    #[serde(flatten)]
    pub child_object_type_specifier: ObjectTypeSpecifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerOfAttorneyDate {
    #[serde(flatten)]
    pub power_of_attorney_date: FccPowerOfAttorneyDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFeaturesControl {
    #[serde(flatten)]
    pub special_features_control: ControlDataSpecialFeatureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatLocation {
    #[serde(flatten)]
    pub habitat_location: CoHabitatLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: SamplePlotSizeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringObjectData {
    #[serde(flatten)]
    pub self_monitoring_object_data: SelfMonitoringObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometry {
    #[serde(flatten)]
    pub object_geometry: ObjectGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightClassType {
    #[serde(flatten)]
    pub height_class_type: HeightClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorderingWithWaterAreaOrStream {
    #[serde(flatten)]
    pub bordering_with_water_area_or_stream: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluation {
    #[serde(flatten)]
    pub control_evaluation: ControlEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataMooseDamageData {
    #[serde(flatten)]
    pub control_data_moose_damage_data: MooseDamageDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandBasicData {
    #[serde(flatten)]
    pub control_stand_basic_data: ControlStandBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluations {
    #[serde(flatten)]
    pub control_evaluations: ControlEvaluationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerInvolvement {
    #[serde(flatten)]
    pub owner_involvement: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class4DamageCount {
    #[serde(flatten)]
    pub class4_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCode {
    #[serde(flatten)]
    pub habitat_code: CoExtendedHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDescription {
    #[serde(flatten)]
    pub operation_description: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringStandArea {
    #[serde(flatten)]
    pub self_monitoring_stand_area: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatSurviving {
    #[serde(flatten)]
    pub habitat_surviving: CoYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCompletionDate {
    #[serde(flatten)]
    pub work_completion_date: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNumber {
    #[serde(flatten)]
    pub object_number: ObjectNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDataReference {
    #[serde(flatten)]
    pub common_object_data_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    #[serde(flatten)]
    pub comments: CoString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectKeys {
    #[serde(flatten)]
    pub object_keys: ObjectKeysType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub real_amount_of_soil_preparation_spot: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObject {
    #[serde(flatten)]
    pub child_object: ChildObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCost {
    #[serde(flatten)]
    pub evaluation_cost: CoDecimal7And2PositiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherEvaluation {
    #[serde(flatten)]
    pub other_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportAccessibility {
    #[serde(flatten)]
    pub transport_accessibility: CoTransportAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjects {
    #[serde(flatten)]
    pub child_objects: ChildObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    #[serde(flatten)]
    pub object: ForestCentreForestDataUpdateObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallPeelDamage {
    #[serde(flatten)]
    pub small_peel_damage: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingRealizationPractice {
    #[serde(flatten)]
    pub cutting_realization_practice: CoCuttingTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonPersonificationId {
    #[serde(flatten)]
    pub non_personification_id: CoString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlStandArea {
    #[serde(flatten)]
    pub control_stand_area: CoAreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectReference {
    #[serde(flatten)]
    pub object_reference: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub tree_damage_outside_stand_evaluation: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamagePercentage {
    #[serde(flatten)]
    pub root_damage_percentage: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethod {
    #[serde(flatten)]
    pub inspection_method: CoInspectionMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Damage {
    #[serde(flatten)]
    pub damage: DamageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRisks {
    #[serde(flatten)]
    pub work_safety_risks: WorkSafetyRisksType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectId {
    #[serde(flatten)]
    pub child_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LargePeelDamage {
    #[serde(flatten)]
    pub large_peel_damage: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Objects {
    #[serde(flatten)]
    pub objects: ForestCentreForestDataUpdateObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatType {
    #[serde(flatten)]
    pub habitat_type: CoHabitatTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluation {
    #[serde(flatten)]
    pub self_monitoring_evaluation: SelfMonitoringEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMeanDepth {
    #[serde(flatten)]
    pub ditch_mean_depth: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSeason {
    #[serde(flatten)]
    pub harvesting_season: CoVirtaHarvestingSeasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class3DamageCount {
    #[serde(flatten)]
    pub class3_damage_count: CoStemCountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNorthingCoordinate {
    #[serde(flatten)]
    pub part_northing_coordinate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectType {
    #[serde(flatten)]
    pub child_object_type: CoObjectTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasicData {
    #[serde(flatten)]
    pub sample_plot_basic_data: SamplePlotBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSignControlClassifier {
    #[serde(flatten)]
    pub harvesting_sign_control_classifier: CoEvaluationCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlAdditionalInformation {
    #[serde(flatten)]
    pub control_additional_information: ControlAdditionalInformationType,
}

