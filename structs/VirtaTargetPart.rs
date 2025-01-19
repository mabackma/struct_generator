use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    #[serde(flatten)]
    pub review: VirtaReviewType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrassControlEvaluation {
    #[serde(flatten)]
    pub grass_control_evaluation: SpVirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirchPulp {
    #[serde(flatten)]
    pub birch_pulp: CoPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingByMachine {
    #[serde(flatten)]
    pub cutting_by_machine: VirtaCuttingByMachineType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotDamagedCount {
    #[serde(flatten)]
    pub not_damaged_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmallPeelDamage {
    #[serde(flatten)]
    pub small_peel_damage: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExceptionalPermitForHandling {
    #[serde(flatten)]
    pub exceptional_permit_for_handling: VirtaExceptionalPermitForHandlingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LargePeelDamage {
    #[serde(flatten)]
    pub large_peel_damage: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class1damageCount {
    #[serde(flatten)]
    pub class1damage_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathSubsidencePercentage {
    #[serde(flatten)]
    pub vehicle_path_subsidence_percentage: CoPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootDamageCount {
    #[serde(flatten)]
    pub root_damage_count: CoPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantEvaluation {
    #[serde(flatten)]
    pub plant_evaluation: VirtaPlantEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SproutForestControlEvaluation {
    #[serde(flatten)]
    pub sprout_forest_control_evaluation: SpVirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetPartStatus {
    #[serde(flatten)]
    pub target_part_status: VirtaTargetPartStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepairPlantingCosts {
    #[serde(flatten)]
    pub repair_planting_costs: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherTimberValue {
    #[serde(flatten)]
    pub other_timber_value: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DestroyedCuttingValue {
    #[serde(flatten)]
    pub destroyed_cutting_value: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectionMethod {
    #[serde(flatten)]
    pub inspection_method: VirtaInspectionMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reason {
    #[serde(flatten)]
    pub reason: VirtaReasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Suggestion {
    #[serde(flatten)]
    pub suggestion: VirtaSuggestionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivatedCropStemCount {
    #[serde(flatten)]
    pub cultivated_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PulpM3sum {
    #[serde(flatten)]
    pub pulp_m3sum: CoPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegenerationType {
    #[serde(flatten)]
    pub regeneration_type: VirtaRegenerationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase2youngCropCount {
    #[serde(flatten)]
    pub phase2young_crop_count: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingMethod {
    #[serde(flatten)]
    pub cutting_method: OpCuttingTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationDate {
    #[serde(flatten)]
    pub operation_date: CoDateMmDdYyyyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingSeason {
    #[serde(flatten)]
    pub harvesting_season: VirtaHarvestingSeasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartNorthingCoordinate {
    #[serde(flatten)]
    pub part_northing_coordinate: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttingValue {
    #[serde(flatten)]
    pub cutting_value: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletingNaturalCropStemCount {
    #[serde(flatten)]
    pub completing_natural_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpectedValueCoefficient {
    #[serde(flatten)]
    pub expected_value_coefficient: CoPositiveDecimalMax1IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatType {
    #[serde(flatten)]
    pub habitat_type: VirtaHabitatTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirchLog {
    #[serde(flatten)]
    pub birch_log: CoPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetPart {
    #[serde(flatten)]
    pub target_part: TargetPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeadStemCount {
    #[serde(flatten)]
    pub dead_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootRotControlEvaluation {
    #[serde(flatten)]
    pub root_rot_control_evaluation: VirtaRootRotControlEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Compensation {
    #[serde(flatten)]
    pub compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlreadyPaidCompensation {
    #[serde(flatten)]
    pub already_paid_compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SprucePulp {
    #[serde(flatten)]
    pub spruce_pulp: CoPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatCode {
    #[serde(flatten)]
    pub habitat_code: VirtaHabitatCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockingWithSeedlings {
    #[serde(flatten)]
    pub stocking_with_seedlings: CoVirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NaturalCropStemCount {
    #[serde(flatten)]
    pub natural_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathWidth {
    #[serde(flatten)]
    pub vehicle_path_width: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpruceLog {
    #[serde(flatten)]
    pub spruce_log: CoPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestingClassifiation {
    #[serde(flatten)]
    pub harvesting_classifiation: VirtaHarvestingClassificationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reason2 {
    #[serde(flatten)]
    pub reason2: VirtaReasonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageSource {
    #[serde(flatten)]
    pub damage_source: CoFeatureTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamagedSeedlingCount {
    #[serde(flatten)]
    pub damaged_seedling_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review2 {
    #[serde(flatten)]
    pub review2: VirtaReviewType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinePulp {
    #[serde(flatten)]
    pub pine_pulp: CoPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status3 {
    #[serde(flatten)]
    pub status3: CoChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilImprovementEvaluation {
    #[serde(flatten)]
    pub soil_improvement_evaluation: SpVirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartNumber {
    #[serde(flatten)]
    pub part_number: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpectedValueSurplus {
    #[serde(flatten)]
    pub expected_value_surplus: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnergyTimberValue {
    #[serde(flatten)]
    pub energy_timber_value: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubMeanDiameter {
    #[serde(flatten)]
    pub stub_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classification {
    #[serde(flatten)]
    pub classification: CoPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsDetectedArea {
    #[serde(flatten)]
    pub parts_detected_area: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitatSurviving {
    #[serde(flatten)]
    pub habitat_surviving: VirtaHabitatSurvivingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedPlantEvaluation {
    #[serde(flatten)]
    pub seed_plant_evaluation: SpVirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryStatus {
    #[serde(flatten)]
    pub geometry_status: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendedDensity {
    #[serde(flatten)]
    pub recommended_density: CoPositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationYear {
    #[serde(flatten)]
    pub operation_year: CoYearType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearingEstimation {
    #[serde(flatten)]
    pub clearing_estimation: SpVirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalEstimation {
    #[serde(flatten)]
    pub total_estimation: VirtaTotalEstimationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VehiclePathDistance {
    #[serde(flatten)]
    pub vehicle_path_distance: CoPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegenerationEnsuring {
    #[serde(flatten)]
    pub regeneration_ensuring: OpSilvicultureTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PineLog {
    #[serde(flatten)]
    pub pine_log: CoPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroundManipulationMethod {
    #[serde(flatten)]
    pub ground_manipulation_method: VirtaGroundManipulationMethodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class3damageCount {
    #[serde(flatten)]
    pub class3damage_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class2damageCount {
    #[serde(flatten)]
    pub class2damage_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubStemCount {
    #[serde(flatten)]
    pub stub_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartEastingCoordinate {
    #[serde(flatten)]
    pub part_easting_coordinate: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeedForTreatment {
    #[serde(flatten)]
    pub need_for_treatment: CoVirtaYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class4damageCount {
    #[serde(flatten)]
    pub class4damage_count: CoPositiveInteger6digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemDamageCount {
    #[serde(flatten)]
    pub stem_damage_count: CoPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogM3sum {
    #[serde(flatten)]
    pub log_m3sum: CoPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CultivationMaterial {
    #[serde(flatten)]
    pub cultivation_material: VirtaCultivationMaterialType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryId {
    #[serde(flatten)]
    pub geometry_id: Xsstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTotalEstimationType {
    #[serde(flatten)]
    pub base: CoVirtaTotalEstimationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRegenerationType {
    #[serde(flatten)]
    pub base: CoVirtaRegenerationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReasonType {
    #[serde(flatten)]
    pub base: CoVirtaReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetPartStatusType {
    #[serde(flatten)]
    pub base: CoVirtaTargetPartStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartType {
    #[serde(rename = "Status3", skip_serializing_if = "Option::is_none")]
    pub status3: Option<CoChangeStateType>,
    #[serde(rename = "PartNumber")]
    pub part_number: String,
    #[serde(rename = "PartsDetectedArea", skip_serializing_if = "Option::is_none")]
    pub parts_detected_area: Option<CoPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "CuttingMethod", skip_serializing_if = "Option::is_none")]
    pub cutting_method: Option<OpCuttingTypeType>,
    #[serde(rename = "RegenerationType", skip_serializing_if = "Option::is_none")]
    pub regeneration_type: Option<VirtaRegenerationType>,
    #[serde(rename = "TargetPartStatus", skip_serializing_if = "Option::is_none")]
    pub target_part_status: Option<VirtaTargetPartStatusType>,
    #[serde(rename = "OperationDate", skip_serializing_if = "Option::is_none")]
    pub operation_date: Option<CoDateMmDdYyyyType>,
    #[serde(rename = "OperationYear", skip_serializing_if = "Option::is_none")]
    pub operation_year: Option<CoYearType>,
    #[serde(rename = "Classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<CoPositiveIntegerType>,
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
    pub stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "SeedStemCount", skip_serializing_if = "Option::is_none")]
    pub seed_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "StubStemCount", skip_serializing_if = "Option::is_none")]
    pub stub_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "CultivatedCropStemCount", skip_serializing_if = "Option::is_none")]
    pub cultivated_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "NaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub natural_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "CompletingNaturalCropStemCount", skip_serializing_if = "Option::is_none")]
    pub completing_natural_crop_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "DeadStemCount", skip_serializing_if = "Option::is_none")]
    pub dead_stem_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub basal_area: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StubMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub stub_mean_diameter: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "AverageHeight", skip_serializing_if = "Option::is_none")]
    pub average_height: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub development_class: Option<CoDevelopmentClassType>,
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
    pub main_group: Option<CoMainGroupType>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub drainage_state: Option<CoDrainageStateType>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub fertility_class: Option<CoFertilityClassType>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub main_tree_species: Option<CoTreeSpeciesType>,
    #[serde(rename = "PineDecimal", skip_serializing_if = "Option::is_none")]
    pub pine_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "SpruceDecimal", skip_serializing_if = "Option::is_none")]
    pub spruce_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "BirchDecimal", skip_serializing_if = "Option::is_none")]
    pub birch_decimal: Option<VirtaTreeDecimalType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CoSoilTypeType>,
    #[serde(rename = "StockingWithSeedlings", skip_serializing_if = "Option::is_none")]
    pub stocking_with_seedlings: Option<CoVirtaYesNoType>,
    #[serde(rename = "GroundManipulationMethod", skip_serializing_if = "Option::is_none")]
    pub ground_manipulation_method: Option<VirtaGroundManipulationMethodType>,
    #[serde(rename = "SoilImprovementEvaluation", skip_serializing_if = "Option::is_none")]
    pub soil_improvement_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "ClearingEstimation", skip_serializing_if = "Option::is_none")]
    pub clearing_estimation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "DamageSource", skip_serializing_if = "Option::is_none")]
    pub damage_source: Option<CoFeatureTypeType>,
    #[serde(rename = "RegenerationEnsuring", skip_serializing_if = "Option::is_none")]
    pub regeneration_ensuring: Option<OpSilvicultureTypeType>,
    #[serde(rename = "CultivationMaterial", skip_serializing_if = "Option::is_none")]
    pub cultivation_material: Option<VirtaCultivationMaterialType>,
    #[serde(rename = "PlantEvaluation", skip_serializing_if = "Option::is_none")]
    pub plant_evaluation: Option<VirtaPlantEvaluationType>,
    #[serde(rename = "GrassControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub grass_control_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "SproutForestControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub sprout_forest_control_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "SeedPlantEvaluation", skip_serializing_if = "Option::is_none")]
    pub seed_plant_evaluation: Option<SpVirtaEvaluationType>,
    #[serde(rename = "RootRotControlEvaluation", skip_serializing_if = "Option::is_none")]
    pub root_rot_control_evaluation: Option<VirtaRootRotControlEvaluationType>,
    #[serde(rename = "HarvestingClassifiation", skip_serializing_if = "Option::is_none")]
    pub harvesting_classifiation: Option<VirtaHarvestingClassificationType>,
    #[serde(rename = "RootDamageCount", skip_serializing_if = "Option::is_none")]
    pub root_damage_count: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "StemDamageCount", skip_serializing_if = "Option::is_none")]
    pub stem_damage_count: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<CoPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "VehiclePathSubsidencePercentage", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_subsidence_percentage: Option<CoPositiveInteger3digitsType>,
    #[serde(rename = "TotalEstimation", skip_serializing_if = "Option::is_none")]
    pub total_estimation: Option<VirtaTotalEstimationType>,
    #[serde(rename = "CuttingByMachine", skip_serializing_if = "Option::is_none")]
    pub cutting_by_machine: Option<VirtaCuttingByMachineType>,
    #[serde(rename = "HarvestingSeason", skip_serializing_if = "Option::is_none")]
    pub harvesting_season: Option<VirtaHarvestingSeasonType>,
    #[serde(rename = "PartEastingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_easting_coordinate: Option<String>,
    #[serde(rename = "PartNorthingCoordinate", skip_serializing_if = "Option::is_none")]
    pub part_northing_coordinate: Option<String>,
    #[serde(rename = "NotDamagedCount", skip_serializing_if = "Option::is_none")]
    pub not_damaged_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class1damageCount", skip_serializing_if = "Option::is_none")]
    pub class1damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class2damageCount", skip_serializing_if = "Option::is_none")]
    pub class2damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class3damageCount", skip_serializing_if = "Option::is_none")]
    pub class3damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "Class4damageCount", skip_serializing_if = "Option::is_none")]
    pub class4damage_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "SmallPeelDamage", skip_serializing_if = "Option::is_none")]
    pub small_peel_damage: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "LargePeelDamage", skip_serializing_if = "Option::is_none")]
    pub large_peel_damage: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "DamagedSeedlingCount", skip_serializing_if = "Option::is_none")]
    pub damaged_seedling_count: Option<CoPositiveInteger6digitsType>,
    #[serde(rename = "PineLog", skip_serializing_if = "Option::is_none")]
    pub pine_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "SpruceLog", skip_serializing_if = "Option::is_none")]
    pub spruce_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "BirchLog", skip_serializing_if = "Option::is_none")]
    pub birch_log: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "PinePulp", skip_serializing_if = "Option::is_none")]
    pub pine_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "LogM3sum", skip_serializing_if = "Option::is_none")]
    pub log_m3sum: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "PulpM3sum", skip_serializing_if = "Option::is_none")]
    pub pulp_m3sum: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "SprucePulp", skip_serializing_if = "Option::is_none")]
    pub spruce_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "BirchPulp", skip_serializing_if = "Option::is_none")]
    pub birch_pulp: Option<CoPositiveInteger4digitsType>,
    #[serde(rename = "OtherTimberValue", skip_serializing_if = "Option::is_none")]
    pub other_timber_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "EnergyTimberValue", skip_serializing_if = "Option::is_none")]
    pub energy_timber_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "CuttingValue", skip_serializing_if = "Option::is_none")]
    pub cutting_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "DestroyedCuttingValue", skip_serializing_if = "Option::is_none")]
    pub destroyed_cutting_value: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "ExpectedValueCoefficient", skip_serializing_if = "Option::is_none")]
    pub expected_value_coefficient: Option<CoPositiveDecimalMax1IntegralPartMax2FractionalPartType>,
    #[serde(rename = "ExpectedValueSurplus", skip_serializing_if = "Option::is_none")]
    pub expected_value_surplus: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "Compensation", skip_serializing_if = "Option::is_none")]
    pub compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "AlreadyPaidCompensation", skip_serializing_if = "Option::is_none")]
    pub already_paid_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TotalCompensation", skip_serializing_if = "Option::is_none")]
    pub total_compensation: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "NeedForTreatment", skip_serializing_if = "Option::is_none")]
    pub need_for_treatment: Option<CoVirtaYesNoType>,
    #[serde(rename = "Suggestion", skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<VirtaSuggestionType>,
    #[serde(rename = "Phase2youngCropCount", skip_serializing_if = "Option::is_none")]
    pub phase2young_crop_count: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "RecommendedDensity", skip_serializing_if = "Option::is_none")]
    pub recommended_density: Option<CoPositiveInteger5digitsType>,
    #[serde(rename = "RepairPlantingCosts", skip_serializing_if = "Option::is_none")]
    pub repair_planting_costs: Option<CoPositiveDecimalMax5IntegralPartMax2FractionalPartType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
    #[serde(rename = "GeometryStatus", skip_serializing_if = "Option::is_none")]
    pub geometry_status: Option<String>,
    #[serde(rename = "GeometryId", skip_serializing_if = "Option::is_none")]
    pub geometry_id: Option<String>,
    #[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
    pub gml_polygon: Option<GmlPolygon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatTypeType {
    #[serde(flatten)]
    pub base: CoVirtaHabitatTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCuttingByMachineType {
    #[serde(flatten)]
    pub base: CoVirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExceptionalPermitForHandlingType {
    #[serde(flatten)]
    pub base: CoVirtaExceptionalPermitForHandlingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCultivationMaterialType {
    #[serde(flatten)]
    pub base: CoVirtaCultivationMaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatSurvivingType {
    #[serde(flatten)]
    pub base: CoVirtaHabitatSurvivingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReviewType {
    #[serde(flatten)]
    pub base: CoVirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSuggestionType {
    #[serde(flatten)]
    pub base: CoVirtaSuggestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTreeDecimalType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPlantEvaluationType {
    #[serde(flatten)]
    pub base: CoVirtaPlantEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingSeasonType {
    #[serde(flatten)]
    pub base: CoVirtaHarvestingSeasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sp_sample_plot: Vec<SpSamplePlot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionMethodType {
    #[serde(flatten)]
    pub base: CoVirtaInspectionMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaStandQualityType {
    #[serde(flatten)]
    pub base: CoVirtaStandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatCodeType {
    #[serde(flatten)]
    pub base: CoVirtaHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingClassificationType {
    #[serde(flatten)]
    pub base: CoVirtaHarvestingClassificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaGroundManipulationMethodType {
    #[serde(flatten)]
    pub base: CoVirtaGroundManipulationMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRootRotControlEvaluationType {
    #[serde(flatten)]
    pub base: CoVirtaRootRotControlEvaluationType,
}

