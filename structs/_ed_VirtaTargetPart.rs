#[derive(Debug, Serialize, Deserialize)]
pub struct CultivatedCropStemCount {
    #[serde(flatten)]
    pub cultivated_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilImprovementEvaluation {
    #[serde(flatten)]
    pub soil_improvement_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationYear {
    #[serde(flatten)]
    pub operation_year: CoYearType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatSurviving {
    #[serde(flatten)]
    pub habitat_surviving: VirtaHabitatSurvivingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review2 {
    #[serde(flatten)]
    pub review2: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionMethod {
    #[serde(flatten)]
    pub inspection_method: VirtaInspectionMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classification {
    #[serde(flatten)]
    pub classification: CoPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootRotControlEvaluation {
    #[serde(flatten)]
    pub root_rot_control_evaluation: VirtaRootRotControlEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchPulp {
    #[serde(flatten)]
    pub birch_pulp: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingSeason {
    #[serde(flatten)]
    pub harvesting_season: VirtaHarvestingSeasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockingWithSeedlings {
    #[serde(flatten)]
    pub stocking_with_seedlings: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundManipulationMethod {
    #[serde(flatten)]
    pub ground_manipulation_method: VirtaGroundManipulationMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compensation {
    #[serde(flatten)]
    pub compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathSubsidencePercentage {
    #[serde(flatten)]
    pub vehicle_path_subsidence_percentage: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PineLog {
    #[serde(flatten)]
    pub pine_log: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationMaterial {
    #[serde(flatten)]
    pub cultivation_material: VirtaCultivationMaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNumber {
    #[serde(flatten)]
    pub part_number: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason {
    #[serde(flatten)]
    pub reason: VirtaReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathDistance {
    #[serde(flatten)]
    pub vehicle_path_distance: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTimberValue {
    #[serde(flatten)]
    pub other_timber_value: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamageCount {
    #[serde(flatten)]
    pub stem_damage_count: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BirchLog {
    #[serde(flatten)]
    pub birch_log: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    #[serde(flatten)]
    pub suggestion: VirtaSuggestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedValueSurplus {
    #[serde(flatten)]
    pub expected_value_surplus: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartEastingCoordinate {
    #[serde(flatten)]
    pub part_easting_coordinate: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SprucePulp {
    #[serde(flatten)]
    pub spruce_pulp: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestingClassifiation {
    #[serde(flatten)]
    pub harvesting_classifiation: VirtaHarvestingClassificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SproutForestControlEvaluation {
    #[serde(flatten)]
    pub sprout_forest_control_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagedSeedlingCount {
    #[serde(flatten)]
    pub damaged_seedling_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallPeelDamage {
    #[serde(flatten)]
    pub small_peel_damage: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedPlantEvaluation {
    #[serde(flatten)]
    pub seed_plant_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalEstimation {
    #[serde(flatten)]
    pub total_estimation: VirtaTotalEstimationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PulpM3sum {
    #[serde(flatten)]
    pub pulp_m3sum: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendedDensity {
    #[serde(flatten)]
    pub recommended_density: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubStemCount {
    #[serde(flatten)]
    pub stub_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearingEstimation {
    #[serde(flatten)]
    pub clearing_estimation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class1damageCount {
    #[serde(flatten)]
    pub class1damage_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionalPermitForHandling {
    #[serde(flatten)]
    pub exceptional_permit_for_handling: VirtaExceptionalPermitForHandlingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceLog {
    #[serde(flatten)]
    pub spruce_log: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairPlantingCosts {
    #[serde(flatten)]
    pub repair_planting_costs: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathWidth {
    #[serde(flatten)]
    pub vehicle_path_width: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatCode {
    #[serde(flatten)]
    pub habitat_code: VirtaHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestroyedCuttingValue {
    #[serde(flatten)]
    pub destroyed_cutting_value: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NaturalCropStemCount {
    #[serde(flatten)]
    pub natural_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeedForTreatment {
    #[serde(flatten)]
    pub need_for_treatment: CoVirtaYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingByMachine {
    #[serde(flatten)]
    pub cutting_by_machine: VirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletingNaturalCropStemCount {
    #[serde(flatten)]
    pub completing_natural_crop_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class4damageCount {
    #[serde(flatten)]
    pub class4damage_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantEvaluation {
    #[serde(flatten)]
    pub plant_evaluation: VirtaPlantEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDamagedCount {
    #[serde(flatten)]
    pub not_damaged_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status3 {
    #[serde(flatten)]
    pub status3: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedValueCoefficient {
    #[serde(flatten)]
    pub expected_value_coefficient: CoPositiveDecimalMax1IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationEnsuring {
    #[serde(flatten)]
    pub regeneration_ensuring: OpSilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDate {
    #[serde(flatten)]
    pub operation_date: CoDateMmDdYyyyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingValue {
    #[serde(flatten)]
    pub cutting_value: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class3damageCount {
    #[serde(flatten)]
    pub class3damage_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlreadyPaidCompensation {
    #[serde(flatten)]
    pub already_paid_compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    #[serde(flatten)]
    pub review: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class2damageCount {
    #[serde(flatten)]
    pub class2damage_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LargePeelDamage {
    #[serde(flatten)]
    pub large_peel_damage: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase2youngCropCount {
    #[serde(flatten)]
    pub phase2young_crop_count: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartNorthingCoordinate {
    #[serde(flatten)]
    pub part_northing_coordinate: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyTimberValue {
    #[serde(flatten)]
    pub energy_timber_value: CoPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogM3sum {
    #[serde(flatten)]
    pub log_m3sum: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandQuality {
    #[serde(flatten)]
    pub stand_quality: VirtaStandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrainageState {
    #[serde(flatten)]
    pub drainage_state: CoDrainageStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrassControlEvaluation {
    #[serde(flatten)]
    pub grass_control_evaluation: SpVirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerationType {
    #[serde(flatten)]
    pub regeneration_type: VirtaRegenerationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryId {
    #[serde(flatten)]
    pub geometry_id: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuttingMethod {
    #[serde(flatten)]
    pub cutting_method: OpCuttingTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartsDetectedArea {
    #[serde(flatten)]
    pub parts_detected_area: CoPositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainGroup {
    #[serde(flatten)]
    pub main_group: CoMainGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadStemCount {
    #[serde(flatten)]
    pub dead_stem_count: CoPositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinePulp {
    #[serde(flatten)]
    pub pine_pulp: CoPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryStatus {
    #[serde(flatten)]
    pub geometry_status: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartStatus {
    #[serde(flatten)]
    pub target_part_status: VirtaTargetPartStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPart {
    #[serde(flatten)]
    pub target_part: TargetPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamageCount {
    #[serde(flatten)]
    pub root_damage_count: CoPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubMeanDiameter {
    #[serde(flatten)]
    pub stub_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason2 {
    #[serde(flatten)]
    pub reason2: VirtaReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCompensation {
    #[serde(flatten)]
    pub total_compensation: CoPositiveDecimalMax5IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageSource {
    #[serde(flatten)]
    pub damage_source: CoFeatureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitatType {
    #[serde(flatten)]
    pub habitat_type: VirtaHabitatTypeType,
}

