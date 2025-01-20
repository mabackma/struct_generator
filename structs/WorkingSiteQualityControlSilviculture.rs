use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct DumplingLevelingErrorCount {
    #[serde(flatten)]
    pub dumpling_leveling_error_count: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClodMoistureErrorCount {
    #[serde(flatten)]
    pub clod_moisture_error_count: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantLocationErrorCount {
    #[serde(flatten)]
    pub plant_location_error_count: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantPositionErrorCount {
    #[serde(flatten)]
    pub plant_position_error_count: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantCount {
    #[serde(flatten)]
    pub plant_count: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantSealingErrorCount {
    #[serde(flatten)]
    pub plant_sealing_error_count: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantedSeedlingCountInNotCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_not_cultivated_spots: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestSeedlingPlantingDepth {
    #[serde(flatten)]
    pub nearest_seedling_planting_depth: BdtPositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamagedPlantsErrorCount {
    #[serde(flatten)]
    pub damaged_plants_error_count: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestSeedlingDistance {
    #[serde(flatten)]
    pub nearest_seedling_distance: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantingDepthErrorCount {
    #[serde(flatten)]
    pub planting_depth_error_count: BdtPositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantCountSummary {
    #[serde(flatten)]
    pub plant_count_summary: BdtPositiveInteger4digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NearestSeedlingSoilCompressed {
    #[serde(flatten)]
    pub nearest_seedling_soil_compressed: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantedSeedlingCountInCultivatedSpots {
    #[serde(flatten)]
    pub planted_seedling_count_in_cultivated_spots: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityControlSilviculture {
    #[serde(flatten)]
    pub working_site_quality_control_silviculture: WorkingSiteQualityControlSilvicultureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSilvicultureBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilvicultureType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSilvicultureBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images")]
    pub images: BdtPositiveInteger2digitsType,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteIdType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: PositiveIntegerType,
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "MeasureDate")]
    pub measure_date: DateType,
    #[serde(rename = "Measurable")]
    pub measurable: YesNoType,
    #[serde(rename = "Measurer")]
    pub measurer: String50Type,
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String20Type>,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "GeometryReal")]
    pub geometry_real: PointGeometryType,
    #[serde(rename = "GeometryRequired", skip_serializing_if = "Option::is_none")]
    pub geometry_required: Option<PointGeometryType>,
    #[serde(rename = "Radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: YesNoType,
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
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerTypeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "PlantCountSummary", skip_serializing_if = "Option::is_none")]
    pub plant_count_summary: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSilvicultureBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerIdType {
    pub base: String,
}

