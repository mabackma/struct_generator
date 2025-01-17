use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataSpecialFeatures {
    #[serde(flatten)]
    pub control_data_special_features: ControlDataSpecialFeaturesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: SamplePlotSizeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialFeaturesControl {
    #[serde(flatten)]
    pub special_features_control: ControlDataSpecialFeatureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerInvolvement {
    #[serde(flatten)]
    pub owner_involvement: CoVirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringEvaluations {
    #[serde(flatten)]
    pub self_monitoring_evaluations: SelfMonitoringEvaluationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlForestUseDeclaration {
    #[serde(flatten)]
    pub control_forest_use_declaration: ControlForestUseDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Objects {
    #[serde(flatten)]
    pub objects: ForestCentreForestDataUpdateObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlreadyPaidCompensation {
    #[serde(flatten)]
    pub already_paid_compensation: CoDecimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlStandArea {
    #[serde(flatten)]
    pub control_stand_area: CoAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlledOperationType {
    #[serde(flatten)]
    pub controlled_operation_type: CoCostTypeNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataRegeneration {
    #[serde(flatten)]
    pub control_data_regeneration: RegenerationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathMeanWidth {
    #[serde(flatten)]
    pub vehicle_path_mean_width: Decimal5_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectProtectionOperation {
    #[serde(flatten)]
    pub object_protection_operation: ObjectProtectionOperationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReasonDescription {
    #[serde(flatten)]
    pub reason_description: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityData {
    #[serde(flatten)]
    pub accessibility_data: AccessibilityDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatLocation {
    #[serde(flatten)]
    pub habitat_location: CoHabitatLocationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    #[serde(flatten)]
    pub actor: ActorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlNo {
    #[serde(flatten)]
    pub control_no: CoString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectKeys {
    #[serde(flatten)]
    pub object_keys: ObjectKeysType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringDate {
    #[serde(flatten)]
    pub self_monitoring_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlObjectBasicData {
    #[serde(flatten)]
    pub control_object_basic_data: ControlObjectBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringBasicData {
    #[serde(flatten)]
    pub self_monitoring_basic_data: SelfMonitoringBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchMeanWidth {
    #[serde(flatten)]
    pub ditch_mean_width: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherEvaluation {
    #[serde(flatten)]
    pub other_evaluation: CoEvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalTreeSpecies {
    #[serde(flatten)]
    pub goal_tree_species: CoTreeSpeciesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeIntervalForMeasuringSamplePlot {
    #[serde(flatten)]
    pub time_interval_for_measuring_sample_plot: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    #[serde(flatten)]
    pub object: ForestCentreForestDataUpdateObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationCode {
    #[serde(flatten)]
    pub evaluation_code: CoEvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatSurviving {
    #[serde(flatten)]
    pub habitat_surviving: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringObjectData {
    #[serde(flatten)]
    pub self_monitoring_object_data: SelfMonitoringObjectDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub terrain_damage_outside_stand_evaluation: CoEvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: Xsinteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageSourceCode {
    #[serde(flatten)]
    pub damage_source_code: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalStemCount {
    #[serde(flatten)]
    pub goal_stem_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockingWIthSeedlings {
    #[serde(flatten)]
    pub stocking_w_ith_seedlings: Xsinteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepairPlantingCosts {
    #[serde(flatten)]
    pub repair_planting_costs: CoDecimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlOverallEvaluationData {
    #[serde(flatten)]
    pub control_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Compensation {
    #[serde(flatten)]
    pub compensation: CoDecimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class1DamageCount {
    #[serde(flatten)]
    pub class1_damage_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Damage {
    #[serde(flatten)]
    pub damage: DamageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingSignControlClassifier {
    #[serde(flatten)]
    pub harvesting_sign_control_classifier: CoEvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Work {
    #[serde(flatten)]
    pub work: WorkType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NaturalCropStemCount {
    #[serde(flatten)]
    pub natural_crop_stem_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObjectId {
    #[serde(flatten)]
    pub child_object_id: CoIdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UseCase {
    #[serde(flatten)]
    pub use_case: CoForestDataUpdateUseCaseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlBasicData {
    #[serde(flatten)]
    pub control_basic_data: ControlBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectionMethod {
    #[serde(flatten)]
    pub inspection_method: CoInspectionMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParcelId {
    #[serde(flatten)]
    pub parcel_id: CoIdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectReferenceType {
    #[serde(flatten)]
    pub object_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotBasicData {
    #[serde(flatten)]
    pub sample_plot_basic_data: SamplePlotBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonObjectData {
    #[serde(flatten)]
    pub common_object_data: CommonObjectDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatType {
    #[serde(flatten)]
    pub habitat_type: CoHabitatTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaAndMapEvaluation {
    #[serde(flatten)]
    pub area_and_map_evaluation: CoEvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoosePercentage {
    #[serde(flatten)]
    pub moose_percentage: CoPercentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingAccessibility {
    #[serde(flatten)]
    pub harvesting_accessibility: CoHarvestingAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notices {
    #[serde(flatten)]
    pub notices: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actors {
    #[serde(flatten)]
    pub actors: ActorsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonPersonificationId {
    #[serde(flatten)]
    pub non_personification_id: CoString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceOrOtherCompensation {
    #[serde(flatten)]
    pub insurance_or_other_compensation: CoDecimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlEvaluations {
    #[serde(flatten)]
    pub control_evaluations: ControlEvaluationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivationMaterial {
    #[serde(flatten)]
    pub cultivation_material: TwoDigitPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlAdditionalInformation {
    #[serde(flatten)]
    pub control_additional_information: ControlAdditionalInformationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonObjectDataReference {
    #[serde(flatten)]
    pub common_object_data_reference: CoReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmallPeelDamage {
    #[serde(flatten)]
    pub small_peel_damage: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeedForTreatment {
    #[serde(flatten)]
    pub need_for_treatment: CoVirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilPreparationSpotsAreEnough {
    #[serde(flatten)]
    pub soil_preparation_spots_are_enough: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingBy {
    #[serde(flatten)]
    pub cutting_by: CoVirtaCuttingByMachineType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetSelection {
    #[serde(flatten)]
    pub target_selection: CoTargetSelectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionData {
    #[serde(flatten)]
    pub restriction_data: StRestrictionDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectProtectionOperations {
    #[serde(flatten)]
    pub object_protection_operations: ObjectProtectionOperationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringEvaluation {
    #[serde(flatten)]
    pub self_monitoring_evaluation: SelfMonitoringEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlStandBasicData {
    #[serde(flatten)]
    pub control_stand_basic_data: ControlStandBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreclearingEvaluation {
    #[serde(flatten)]
    pub preclearing_evaluation: CoPreclearingEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObject {
    #[serde(flatten)]
    pub child_object: ChildObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamagedSeedlingCount {
    #[serde(flatten)]
    pub damaged_seedling_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDamagePercentage {
    #[serde(flatten)]
    pub stem_damage_percentage: CoPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectNumber {
    #[serde(flatten)]
    pub object_number: ObjectNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataMooseDamageData {
    #[serde(flatten)]
    pub control_data_moose_damage_data: MooseDamageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantingWorkQuality {
    #[serde(flatten)]
    pub planting_work_quality: Xsinteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObjectTypeSpecifier {
    #[serde(flatten)]
    pub child_object_type_specifier: ObjectTypeSpecifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkSafetyRiskDescription {
    #[serde(flatten)]
    pub work_safety_risk_description: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlReferenceType {
    #[serde(flatten)]
    pub control_reference_type: CoForestCentreMessageReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DitchMeanDepth {
    #[serde(flatten)]
    pub ditch_mean_depth: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssociationInvolvement {
    #[serde(flatten)]
    pub association_involvement: CoVirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletingNaturalCropStemCount {
    #[serde(flatten)]
    pub completing_natural_crop_stem_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageSourceDescription {
    #[serde(flatten)]
    pub damage_source_description: CoString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationDescription {
    #[serde(flatten)]
    pub operation_description: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationDeliveringEvaluation {
    #[serde(flatten)]
    pub declaration_delivering_evaluation: CoEvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalCompensations {
    #[serde(flatten)]
    pub total_compensations: CoDecimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegenerationEnsuring {
    #[serde(flatten)]
    pub regeneration_ensuring: ThreeDigitPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerOfAttorneyDate {
    #[serde(flatten)]
    pub power_of_attorney_date: FccPowerOfAttorneyDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationText {
    #[serde(flatten)]
    pub evaluation_text: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorderingWithWaterAreaOrStream {
    #[serde(flatten)]
    pub bordering_with_water_area_or_stream: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootDamagePercentage {
    #[serde(flatten)]
    pub root_damage_percentage: CoPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringObjectProtectionOperationsData {
    #[serde(flatten)]
    pub self_monitoring_object_protection_operations_data: SelfMonitoringObjectProtectionOperationsDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeDamageOutsideStandEvaluation {
    #[serde(flatten)]
    pub tree_damage_outside_stand_evaluation: CoEvaluationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectReference {
    #[serde(flatten)]
    pub object_reference: CoReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppliedLength {
    #[serde(flatten)]
    pub applied_length: Decimal6_2Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataSwampForestManagement {
    #[serde(flatten)]
    pub control_data_swamp_forest_management: ControlDataSwampForestManagementType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDate {
    #[serde(flatten)]
    pub control_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LargePeelDamage {
    #[serde(flatten)]
    pub large_peel_damage: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotDamagedSeedlingCount {
    #[serde(flatten)]
    pub not_damaged_seedling_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivatedDeadStemCount {
    #[serde(flatten)]
    pub cultivated_dead_stem_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectGeometry {
    #[serde(flatten)]
    pub object_geometry: ObjectGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UseCases {
    #[serde(flatten)]
    pub use_cases: ForestDataUpdateUseCasesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreinformDate {
    #[serde(flatten)]
    pub preinform_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub real_amount_of_soil_preparation_spot: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedWorkingTimeConsumption {
    #[serde(flatten)]
    pub estimated_working_time_consumption: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectOverallEvaluationData {
    #[serde(flatten)]
    pub object_overall_evaluation_data: ControlOverallEvaluationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class3DamageCount {
    #[serde(flatten)]
    pub class3_damage_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObjectType {
    #[serde(flatten)]
    pub child_object_type: CoObjectTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroundManipulationMethod {
    #[serde(flatten)]
    pub ground_manipulation_method: ThreeDigitPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class2DamageCount {
    #[serde(flatten)]
    pub class2_damage_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExceptionalPermitForHandling {
    #[serde(flatten)]
    pub exceptional_permit_for_handling: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlledOperationDescription {
    #[serde(flatten)]
    pub controlled_operation_description: CoString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: CoDecimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataForestRoadConstruction {
    #[serde(flatten)]
    pub control_data_forest_road_construction: ControlDataForestRoadConstructionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivatedCropStemCount {
    #[serde(flatten)]
    pub cultivated_crop_stem_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathSubsidenceLength {
    #[serde(flatten)]
    pub vehicle_path_subsidence_length: Decimal3_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageData {
    #[serde(flatten)]
    pub damage_data: DamageDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationCategory {
    #[serde(flatten)]
    pub evaluation_category: CoEvaluationSubjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectNo {
    #[serde(flatten)]
    pub project_no: ProjectNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationCost {
    #[serde(flatten)]
    pub evaluation_cost: CoDecimal7And2PositiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictionBasedOnStoniness {
    #[serde(flatten)]
    pub restriction_based_on_stoniness: CoRestrictionBasedOnStoninessType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartNorthingCoordinate {
    #[serde(flatten)]
    pub part_northing_coordinate: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlEvaluation {
    #[serde(flatten)]
    pub control_evaluation: ControlEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InTerrain {
    #[serde(flatten)]
    pub in_terrain: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class4DamageCount {
    #[serde(flatten)]
    pub class4_damage_count: CoStemCountType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreinformDetails {
    #[serde(flatten)]
    pub preinform_details: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainDamage {
    #[serde(flatten)]
    pub main_damage: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathMeanDistance {
    #[serde(flatten)]
    pub vehicle_path_mean_distance: Decimal5_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalAmountOfSoilPreparationSpot {
    #[serde(flatten)]
    pub goal_amount_of_soil_preparation_spot: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainReason {
    #[serde(flatten)]
    pub main_reason: CoYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationCode {
    #[serde(flatten)]
    pub operation_code: CoObjectProtectionOperationCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathSubsidencePercentage {
    #[serde(flatten)]
    pub vehicle_path_subsidence_percentage: Decimal3_1Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkSafetyRisks {
    #[serde(flatten)]
    pub work_safety_risks: WorkSafetyRisksType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeStandBasedData {
    #[serde(flatten)]
    pub tree_stand_based_data: StTreeStandBasedDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportAccessibility {
    #[serde(flatten)]
    pub transport_accessibility: CoTransportAccessibilityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkCompletionDate {
    #[serde(flatten)]
    pub work_completion_date: CoDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingSignData {
    #[serde(flatten)]
    pub harvesting_sign_data: HarvestingSignDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlReference {
    #[serde(flatten)]
    pub control_reference: CoReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthOfDitchDiggedDuringSoilPreparation {
    #[serde(flatten)]
    pub length_of_ditch_digged_during_soil_preparation: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Works {
    #[serde(flatten)]
    pub works: WorksType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationDescription {
    #[serde(flatten)]
    pub evaluation_description: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comments {
    #[serde(flatten)]
    pub comments: CoString1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSizeTreeReduction {
    #[serde(flatten)]
    pub sample_plot_size_tree_reduction: SamplePlotSizeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlDataSpecialFeature {
    #[serde(flatten)]
    pub control_data_special_feature: ControlDataSpecialFeatureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectBasicData {
    #[serde(flatten)]
    pub object_basic_data: ObjectBasicDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingSeason {
    #[serde(flatten)]
    pub harvesting_season: CoVirtaHarvestingSeasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompartmentId {
    #[serde(flatten)]
    pub compartment_id: CoIdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilData {
    #[serde(flatten)]
    pub soil_data: StBaseSoilDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringStandArea {
    #[serde(flatten)]
    pub self_monitoring_stand_area: CoAreaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildObjects {
    #[serde(flatten)]
    pub child_objects: ChildObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReasonCode {
    #[serde(flatten)]
    pub reason_code: Xsinteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringType {
    #[serde(flatten)]
    pub self_monitoring_type: CoSelfMonitoringTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: CoString2000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartEastingCoordinate {
    #[serde(flatten)]
    pub part_easting_coordinate: Xsstring,
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
pub struct RealEstateType {
    #[serde(flatten)]
    pub base: ReBaseRealEstateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlForestUseDeclarationType {
    #[serde(rename = "CuttingRealizationPractice", skip_serializing_if = "Option::is_none")]
    pub cutting_realization_practice: Option<CuttingTypeType>,
    #[serde(rename = "HarvestingSignControlClassifier", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_control_classifier: Option<EvaluationCodeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeSpecifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlEvaluationsType {
    #[serde(rename = "ControlEvaluation")]
    pub control_evaluation: Vec<ControlEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildObjectType {
    #[serde(rename = "ChildObjectType")]
    pub child_object_type: CoObjectTypeType,
    #[serde(rename = "ChildObjectTypeSpecifier", skip_serializing_if = "Option::is_none")]
    pub child_object_type_specifier: Option<ObjectTypeSpecifierType>,
    #[serde(rename = "ChildObjectId")]
    pub child_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageType {
    #[serde(rename = "MainDamage", skip_serializing_if = "Option::is_none")]
    pub main_damage: Option<YesNoType>,
    #[serde(rename = "DamageSourceCode", skip_serializing_if = "Option::is_none")]
    pub damage_source_code: Option<String>,
    #[serde(rename = "DamageSourceDescription", skip_serializing_if = "Option::is_none")]
    pub damage_source_description: Option<String100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealEstatesType {
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<BaseRealEstateType2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSwampForestManagementType {
    #[serde(rename = "DitchMeanDepth")]
    pub ditch_mean_depth: String,
    #[serde(rename = "DitchMeanWidth")]
    pub ditch_mean_width: String,
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
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<OpOperations>,
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
    pub working_site_final_audit_silviculture: Option<WorkingSiteFinalAuditSilvicultureSelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<WorkingSiteQualityControlSilvicultureSelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<WorkingSiteFinalAuditSoilConditioningSelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<WorkingSiteQualityControlSoilConditioningSelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
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
    pub stocking_w_ith_seedlings: i32,
    #[serde(rename = "GroundManipulationMethod")]
    pub ground_manipulation_method: ThreeDigitPositiveIntegerType,
    #[serde(rename = "RegenerationEnsuring", skip_serializing_if = "Option::is_none")]
    pub regeneration_ensuring: Option<ThreeDigitPositiveIntegerType>,
    #[serde(rename = "CultivationMaterial")]
    pub cultivation_material: TwoDigitPositiveIntegerType,
    #[serde(rename = "PlantingWorkQuality")]
    pub planting_work_quality: i32,
    #[serde(rename = "SoilModificationEstimate")]
    pub soil_modification_estimate: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal6_2Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoDigitPositiveIntegerType {
    pub base: i32,
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
    pub fcc_forest_use_declaration_number: Option<FccForestUseDeclarationNumber>,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub fcc_financing_act_number: Option<FccFinancingActNumber>,
    #[serde(rename = "CustomerReference", skip_serializing_if = "Option::is_none")]
    pub fcc_customer_reference: Option<FccCustomerReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlAdditionalInformationType {
    #[serde(rename = "PreinformDate", skip_serializing_if = "Option::is_none")]
    pub preinform_date: Option<CoDateType>,
    #[serde(rename = "PreinformDetails", skip_serializing_if = "Option::is_none")]
    pub preinform_details: Option<CoString1000Type>,
    #[serde(rename = "InTerrain", skip_serializing_if = "Option::is_none")]
    pub in_terrain: Option<CoYesNoType>,
    #[serde(rename = "Comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<CoString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "UpdatePreviousMessage")]
    pub update_previous_message: YesNoType,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: CoForestDataUpdateUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreSelfMonitoringObjectsType,
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
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<OpOperations>,
    #[serde(rename = "Works", skip_serializing_if = "Option::is_none")]
    pub works: Option<WorksType>,
    #[serde(rename = "WorkingSiteFinalAuditPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_plant_management: Option<WorkingSiteFinalAuditPlantManagementSelfMonitoringWorkingSiteFinalAuditPlantManagementType>,
    #[serde(rename = "WorkingSiteQualityControlPlantManagement", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_plant_management: Option<WorkingSiteQualityControlPlantManagementSelfMonitoringWorkingSiteQualityControlPlantManagementType>,
    #[serde(rename = "WorkingSiteFinalAuditSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_silviculture: Option<WorkingSiteFinalAuditSilvicultureSelfMonitoringWorkingSiteFinalAuditSilvicultureType>,
    #[serde(rename = "WorkingSiteQualityControlSilviculture", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_silviculture: Option<WorkingSiteQualityControlSilvicultureSelfMonitoringWorkingSiteQualityControlSilvicultureType>,
    #[serde(rename = "WorkingSiteFinalAuditSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_soil_conditioning: Option<WorkingSiteFinalAuditSoilConditioningSelfMonitoringWorkingSiteFinalAuditSoilConditioningType>,
    #[serde(rename = "WorkingSiteQualityControlSoilConditioning", skip_serializing_if = "Option::is_none")]
    pub working_site_quality_control_soil_conditioning: Option<WorkingSiteQualityControlSoilConditioningSelfMonitoringWorkingSiteQualityControlSoilConditioningType>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<SelfMonitoringImageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometryType {
    #[serde(rename = "@collectingMethod")]
    pub collecting_method: CollectingMethodType,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<CoAreaType>,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<CoAreaType>,
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<Decimal1FractionDigitType>,
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: GdtAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDataGroup {
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<Actors>,
    #[serde(rename = "ControlOverallEvaluationData", skip_serializing_if = "Option::is_none")]
    pub control_overall_evaluation_data: Option<ControlOverallEvaluationData>,
    #[serde(rename = "MapSymbol:MapSymbol", skip_serializing_if = "Option::is_none")]
    pub map_symbol:_map_symbol: Option<MapSymbol:MapSymbol>,
    #[serde(rename = "ControlBasicData", skip_serializing_if = "Option::is_none")]
    pub control_basic_data: Option<ControlBasicData>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<SoilData>,
    #[serde(rename = "ControlStandBasicData", skip_serializing_if = "Option::is_none")]
    pub control_stand_basic_data: Option<ControlStandBasicData>,
    #[serde(rename = "ControlDataRegeneration", skip_serializing_if = "Option::is_none")]
    pub control_data_regeneration: Option<ControlDataRegeneration>,
    #[serde(rename = "ControlDataWaterSystemProtection", skip_serializing_if = "Option::is_none")]
    pub control_data_water_system_protection: Option<ControlDataWaterSystemProtection>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StandBasicData>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<TreeStandBasedData>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Documents>,
    #[serde(rename = "workingSiteFinalAuditRoadMaking:WorkingSiteFinalAuditRoadMaking", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_road_making:_working_site_final_audit_road_making: Option<workingSiteFinalAuditRoadMaking:WorkingSiteFinalAuditRoadMaking>,
    #[serde(rename = "st:SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st:_special_features: Option<st:SpecialFeatures>,
    #[serde(rename = "SamplePlotBasicData", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basic_data: Option<SamplePlotBasicData>,
    #[serde(rename = "op:Operations", skip_serializing_if = "Option::is_none")]
    pub op:_operations: Option<op:Operations>,
    #[serde(rename = "SelfMonitoringObjectProtectionOperationsData", skip_serializing_if = "Option::is_none")]
    pub self_monitoring_object_protection_operations_data: Option<SelfMonitoringObjectProtectionOperationsData>,
    #[serde(rename = "ControlObjectBasicData", skip_serializing_if = "Option::is_none")]
    pub control_object_basic_data: Option<ControlObjectBasicData>,
    #[serde(rename = "workingSiteFinalAuditDraining:WorkingSiteFinalAuditDraining", skip_serializing_if = "Option::is_none")]
    pub working_site_final_audit_draining:_working_site_final_audit_draining: Option<workingSiteFinalAuditDraining:WorkingSiteFinalAuditDraining>,
    #[serde(rename = "HarvestingSignData", skip_serializing_if = "Option::is_none")]
    pub harvesting_sign_data: Option<HarvestingSignData>,
    #[serde(rename = "ChildObjects", skip_serializing_if = "Option::is_none")]
    pub child_objects: Option<ChildObjects>,
    #[serde(rename = "ControlDataMooseDamageData", skip_serializing_if = "Option::is_none")]
    pub control_data_moose_damage_data: Option<ControlDataMooseDamageData>,
    #[serde(rename = "ts:TreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts:_tree_stand_data: Option<ts:TreeStandData>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<RestrictionData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateUseCasesType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<Vec<ForestDataUpdateUseCaseType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorTypeSpecifierType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataForestRoadConstructionType {
    #[serde(rename = "AppliedLength")]
    pub applied_length: Vec<Decimal6_2Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataTreeStandDataDateType {
    #[serde(flatten)]
    pub base: TsTreeStandDataDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FourDigitPositiveIntegerType {
    pub base: i32,
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
    pub object_data_group: Option<ObjectDataGroup>,
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
pub struct ChildObjectsType {
    #[serde(rename = "ChildObject")]
    pub child_object: Vec<ChildObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectKeysType {
    #[serde(rename = "StandKeyGroup1")]
    pub stand_key_group1: StandKeyGroup1,
    #[serde(rename = "StandKeyGroup2")]
    pub stand_key_group2: StandKeyGroup2,
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
pub struct SelfMonitoringObjectProtectionOperationsDataType {
    #[serde(rename = "BorderingWithWaterAreaOrStream", skip_serializing_if = "Option::is_none")]
    pub bordering_with_water_area_or_stream: Option<YesNoType>,
    #[serde(rename = "LengthOfDitchDiggedDuringSoilPreparation", skip_serializing_if = "Option::is_none")]
    pub length_of_ditch_digged_during_soil_preparation: Option<PositiveInteger6digitsType>,
    #[serde(rename = "ObjectProtectionOperations", skip_serializing_if = "Option::is_none")]
    pub object_protection_operations: Option<ObjectProtectionOperationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonObjectDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: CoUseCaseType,
    #[serde(rename = "Objects")]
    pub objects: ObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorType {
    #[serde(rename = "@actorType")]
    pub actor_type: ActorTypeType,
    #[serde(rename = "@actorTypeSpecifier")]
    pub actor_type_specifier: ActorTypeSpecifierType,
    #[serde(flatten)]
    pub base: CiContactInformationType,
    #[serde(rename = "ControlAdditionalInformation", skip_serializing_if = "Option::is_none")]
    pub control_additional_information: Option<ControlAdditionalInformationType>,
    #[serde(rename = "PowerOfAttorney", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney: Option<FccPowerOfAttorneyType>,
    #[serde(rename = "PowerOfAttorneyDate", skip_serializing_if = "Option::is_none")]
    pub power_of_attorney_date: Option<FccPowerOfAttorneyDateType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSizeType {
    #[serde(flatten)]
    pub base: CoDecimal3And2PositiveType,
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
pub struct MooseDamageDataType {
    #[serde(rename = "EvaluationCost")]
    pub evaluation_cost: Decimal7And2PositiveType,
    #[serde(rename = "PartEastingCoordinate")]
    pub part_easting_coordinate: String,
    #[serde(rename = "PartNorthingCoordinate")]
    pub part_northing_coordinate: String,
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
pub struct ObjectProtectionOperationsType {
    #[serde(rename = "ObjectProtectionOperation")]
    pub object_protection_operation: Vec<ObjectProtectionOperationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreSelfMonitoringObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreSelfMonitoringObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreForestDataUpdateObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreForestDataUpdateObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorsType {
    #[serde(rename = "Actor")]
    pub actor: Vec<ActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decimal3_1Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreeDigitPositiveIntegerType {
    pub base: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNoType {
    #[serde(flatten)]
    pub base: CoReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBasicDataType {
    #[serde(rename = "ObjectNumber", skip_serializing_if = "Option::is_none")]
    pub object_number: Option<ObjectNumberType>,
    #[serde(rename = "ObjectReference", skip_serializing_if = "Option::is_none")]
    pub object_reference: Option<ReferenceType>,
    #[serde(rename = "NonPersonificationId", skip_serializing_if = "Option::is_none")]
    pub non_personification_id: Option<CoString100Type>,
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
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<OpOperations>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<StSpecialFeatures>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<FccDocumentsType>,
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
pub struct Decimal5_1Type {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageDataType {
    #[serde(rename = "Damage")]
    pub damage: Vec<DamageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorksType {
    #[serde(rename = "Work")]
    pub work: Vec<WorkType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDataSpecialFeaturesType {
    #[serde(rename = "ControlDataSpecialFeature")]
    pub control_data_special_feature: Vec<ControlDataSpecialFeatureType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkSafetyRisksType {
    #[serde(rename = "WorkSafetyRiskDescription")]
    pub work_safety_risk_description: Vec<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringEvaluationsType {
    #[serde(rename = "SelfMonitoringEvaluation")]
    pub self_monitoring_evaluation: Vec<SelfMonitoringEvaluationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNumberType {
    #[serde(flatten)]
    pub base: CoPositiveIntegerType,
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
pub struct ObjectProtectionOperationType {
    #[serde(rename = "OperationCode", skip_serializing_if = "Option::is_none")]
    pub operation_code: Option<ObjectProtectionOperationCodeType>,
    #[serde(rename = "OperationDescription", skip_serializing_if = "Option::is_none")]
    pub operation_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeStandDataType {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDate2Type>,
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
pub struct ControlBasicDataType {
    #[serde(rename = "UseCase", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<CoUseCaseType>,
    #[serde(rename = "ControlNo", skip_serializing_if = "Option::is_none")]
    pub control_no: Option<String100Type>,
    #[serde(rename = "ForestUseDeclarationNumber", skip_serializing_if = "Option::is_none")]
    pub forest_use_declaration_number: Option<ForestUseDeclarationNumberType>,
    #[serde(rename = "FinancingActNumber", skip_serializing_if = "Option::is_none")]
    pub financing_act_number: Option<FinancingActNumberType>,
    #[serde(rename = "ControlReferenceType", skip_serializing_if = "Option::is_none")]
    pub control_reference_type: Option<CoForestCentreMessageReferenceType>,
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
pub struct ObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ObjectType>,
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
pub struct ControlEvaluationType {
    #[serde(rename = "EvaluationCategory", skip_serializing_if = "Option::is_none")]
    pub evaluation_category: Option<EvaluationSubjectType>,
    #[serde(rename = "EvaluationCode", skip_serializing_if = "Option::is_none")]
    pub evaluation_code: Option<i32>,
    #[serde(rename = "EvaluationDescription", skip_serializing_if = "Option::is_none")]
    pub evaluation_description: Option<String1000Type>,
    #[serde(rename = "MainReason", skip_serializing_if = "Option::is_none")]
    pub main_reason: Option<YesNoType>,
    #[serde(rename = "ReasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<i32>,
    #[serde(rename = "ReasonDescription", skip_serializing_if = "Option::is_none")]
    pub reason_description: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdateType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CommonObjectDataReference")]
    pub common_object_data_reference: CoReferenceType,
    #[serde(rename = "Objects")]
    pub objects: ForestCentreForestDataUpdateObjectsType,
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
    pub stand_basic_data: Option<StBaseCompactStandBasicDataType>,
    #[serde(rename = "AccessibilityData", skip_serializing_if = "Option::is_none")]
    pub accessibility_data: Option<AccessibilityDataType>,
    #[serde(rename = "SoilData", skip_serializing_if = "Option::is_none")]
    pub soil_data: Option<StBaseSoilDataType>,
    #[serde(rename = "TreeStandBasedData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_based_data: Option<StTreeStandBasedDataType>,
    #[serde(rename = "RestrictionData", skip_serializing_if = "Option::is_none")]
    pub restriction_data: Option<StRestrictionDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<TreeStandDataType>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<OpOperations>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<StSpecialFeatures>,
    #[serde(rename = "Documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<FccDocumentsType>,
}

