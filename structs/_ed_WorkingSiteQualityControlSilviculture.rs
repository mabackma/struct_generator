#[derive(Debug, Serialize, Deserialize)]
pub struct PlantCount {
    #[serde(flatten)]
    pub plant_count: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingPlantingDepth {
    #[serde(flatten)]
    pub nearest_seedling_planting_depth: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub target: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilviculture {
    #[serde(flatten)]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantedSeedlingCountInNotCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_not_cultivated_spots: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantedSeedlingCountInCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_cultivated_spots: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingDistance {
    #[serde(flatten)]
    pub nearest_seedling_distance: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantLocationErrorCount {
    #[serde(flatten)]
    pub plant_location_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantCountSummary {
    #[serde(flatten)]
    pub plant_count_summary: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumplingLevelingErrorCount {
    #[serde(flatten)]
    pub dumpling_leveling_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingDepthErrorCount {
    #[serde(flatten)]
    pub planting_depth_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantSealingErrorCount {
    #[serde(flatten)]
    pub plant_sealing_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagedPlantsErrorCount {
    #[serde(flatten)]
    pub damaged_plants_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantPositionErrorCount {
    #[serde(flatten)]
    pub plant_position_error_count: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestSeedlingSoilCompressed {
    #[serde(flatten)]
    pub nearest_seedling_soil_compressed: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RockySoil {
    #[serde(flatten)]
    pub rocky_soil: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClodMoistureErrorCount {
    #[serde(flatten)]
    pub clod_moisture_error_count: BdtPositiveIntegerType,
}

