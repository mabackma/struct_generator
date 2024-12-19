#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPart {
    #[serde(flatten)]
    pub target_part: TargetPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartStatus {
    #[serde(flatten)]
    pub target_part_status: VirtaTargetPartStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceLog {
    #[serde(flatten)]
    pub spruce_log: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDate {
    #[serde(flatten)]
    pub operation_date: DateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantEvaluation {
    #[serde(flatten)]
    pub plant_evaluation: VirtaPlantEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationYear {
    #[serde(flatten)]
    pub operation_year: YearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class2damageCount {
    #[serde(flatten)]
    pub class2damage_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockingWithSeedlings {
    #[serde(flatten)]
    pub stocking_with_seedlings: VirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    #[serde(flatten)]
    pub review: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classification {
    #[serde(flatten)]
    pub classification: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTimberValue {
    #[serde(flatten)]
    pub other_timber_value: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class1damageCount {
    #[serde(flatten)]
    pub class1damage_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingMethod {
    #[serde(flatten)]
    pub cutting_method: CuttingTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinePulp {
    #[serde(flatten)]
    pub pine_pulp: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason2 {
    #[serde(flatten)]
    pub reason2: VirtaReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestroyedCuttingValue {
    #[serde(flatten)]
    pub destroyed_cutting_value: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubStemCount {
    #[serde(flatten)]
    pub stub_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review2 {
    #[serde(flatten)]
    pub review2: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDamagedCount {
    #[serde(flatten)]
    pub not_damaged_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootRotControlEvaluation {
    #[serde(flatten)]
    pub root_rot_control_evaluation: VirtaRootRotControlEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status3 {
    #[serde(flatten)]
    pub status3: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrassControlEvaluation {
    #[serde(flatten)]
    pub grass_control_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpM3sum {
    #[serde(flatten)]
    pub pulp_m3sum: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SproutForestControlEvaluation {
    #[serde(flatten)]
    pub sprout_forest_control_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNumber {
    #[serde(flatten)]
    pub part_number: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchPulp {
    #[serde(flatten)]
    pub birch_pulp: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyTimberValue {
    #[serde(flatten)]
    pub energy_timber_value: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingEstimation {
    #[serde(flatten)]
    pub clearing_estimation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationType {
    #[serde(flatten)]
    pub regeneration_type: VirtaRegenerationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilImprovementEvaluation {
    #[serde(flatten)]
    pub soil_improvement_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class4damageCount {
    #[serde(flatten)]
    pub class4damage_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedValueSurplus {
    #[serde(flatten)]
    pub expected_value_surplus: PositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamageCount {
    #[serde(flatten)]
    pub stem_damage_count: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadStemCount {
    #[serde(flatten)]
    pub dead_stem_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    #[serde(flatten)]
    pub suggestion: VirtaSuggestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingByMachine {
    #[serde(flatten)]
    pub cutting_by_machine: VirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogM3sum {
    #[serde(flatten)]
    pub log_m3sum: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubMeanDiameter {
    #[serde(flatten)]
    pub stub_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryId {
    #[serde(flatten)]
    pub geometry_id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingValue {
    #[serde(flatten)]
    pub cutting_value: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingClassifiation {
    #[serde(flatten)]
    pub harvesting_classifiation: VirtaHarvestingClassificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedValueCoefficient {
    #[serde(flatten)]
    pub expected_value_coefficient: PositiveDecimalMax1IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SprucePulp {
    #[serde(flatten)]
    pub spruce_pulp: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchLog {
    #[serde(flatten)]
    pub birch_log: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedPlantEvaluation {
    #[serde(flatten)]
    pub seed_plant_evaluation: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase2youngCropCount {
    #[serde(flatten)]
    pub phase2young_crop_count: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryStatus {
    #[serde(flatten)]
    pub geometry_status: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSource {
    #[serde(flatten)]
    pub damage_source: FeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsDetectedArea {
    #[serde(flatten)]
    pub parts_detected_area: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamageCount {
    #[serde(flatten)]
    pub root_damage_count: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class3damageCount {
    #[serde(flatten)]
    pub class3damage_count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PineLog {
    #[serde(flatten)]
    pub pine_log: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalEstimation {
    #[serde(flatten)]
    pub total_estimation: VirtaTotalEstimationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendedDensity {
    #[serde(flatten)]
    pub recommended_density: PositiveInteger5digitsType,
}

