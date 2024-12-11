#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRegenerationType {
    #[serde(flatten)]
    pub base: VirtaRegenerationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTargetPartStatusType {
    #[serde(flatten)]
    pub base: VirtaTargetPartStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCultivationMaterialType {
    #[serde(flatten)]
    pub base: VirtaCultivationMaterialType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatCodeType {
    #[serde(flatten)]
    pub base: VirtaHabitatCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTotalEstimationType {
    #[serde(flatten)]
    pub base: VirtaTotalEstimationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReasonType {
    #[serde(flatten)]
    pub base: VirtaReasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaSuggestionType {
    #[serde(flatten)]
    pub base: VirtaSuggestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaExceptionalPermitForHandlingType {
    #[serde(flatten)]
    pub base: VirtaExceptionalPermitForHandlingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaGroundManipulationMethodType {
    #[serde(flatten)]
    pub base: VirtaGroundManipulationMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaPlantEvaluationType {
    #[serde(flatten)]
    pub base: VirtaPlantEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaInspectionMethodType {
    #[serde(flatten)]
    pub base: VirtaInspectionMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaTreeDecimalType {
    #[serde(flatten)]
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingClassificationType {
    #[serde(flatten)]
    pub base: VirtaHarvestingClassificationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatSurvivingType {
    #[serde(flatten)]
    pub base: VirtaHabitatSurvivingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaRootRotControlEvaluationType {
    #[serde(flatten)]
    pub base: VirtaRootRotControlEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPartType {
    #[serde(rename = "Status3", skip_serializing_if = "Option::is_none")]
    pub status3: Option<ChangeStateType>,
    #[serde(rename = "PartNumber")]
    pub part_number: String,
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
pub struct VirtaStandQualityType {
    #[serde(flatten)]
    pub base: VirtaStandQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaCuttingByMachineType {
    #[serde(flatten)]
    pub base: VirtaCuttingByMachineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaReviewType {
    #[serde(flatten)]
    pub base: VirtaReviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHarvestingSeasonType {
    #[serde(flatten)]
    pub base: VirtaHarvestingSeasonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaHabitatTypeType {
    #[serde(flatten)]
    pub base: VirtaHabitatTypeType,
}

