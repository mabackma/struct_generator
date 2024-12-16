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
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "PlantCountSummary", skip_serializing_if = "Option::is_none")]
    pub plant_count_summary: Option<BdtPositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: BdtString20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
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
pub struct MeasurerType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerTypeType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocationType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerIdType {
    #[serde(flatten)]
    pub base: Xsstring,
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
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteIdType {
    #[serde(flatten)]
    pub base: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "@id")]
    pub id: BdtIdStringNotEmptyType,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: BdtPositiveIntegerType,
    #[serde(rename = "StandNumber")]
    pub stand_number: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "MeasureDate")]
    pub measure_date: BdtDateType,
    #[serde(rename = "Measurable")]
    pub measurable: BdtYesNoType,
    #[serde(rename = "Measurer")]
    pub measurer: BdtString50Type,
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<BdtString20Type>,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<WctTaxNumberType>,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "GeometryReal")]
    pub geometry_real: GdtPointGeometryType,
    #[serde(rename = "GeometryRequired", skip_serializing_if = "Option::is_none")]
    pub geometry_required: Option<GdtPointGeometryType>,
    #[serde(rename = "Radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: BdtYesNoType,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<BdtPositiveIntegerType>,
    #[serde(rename = "PlantLocationErrorCount", skip_serializing_if = "Option::is_none")]
    pub plant_location_error_count: Option<BdtPositiveIntegerType>,
    #[serde(rename = "PlantingDepthErrorCount", skip_serializing_if = "Option::is_none")]
    pub planting_depth_error_count: Option<BdtPositiveIntegerType>,
    #[serde(rename = "PlantSealingErrorCount", skip_serializing_if = "Option::is_none")]
    pub plant_sealing_error_count: Option<BdtPositiveIntegerType>,
    #[serde(rename = "ClodMoistureErrorCount", skip_serializing_if = "Option::is_none")]
    pub clod_moisture_error_count: Option<BdtPositiveIntegerType>,
    #[serde(rename = "PlantPositionErrorCount", skip_serializing_if = "Option::is_none")]
    pub plant_position_error_count: Option<BdtPositiveIntegerType>,
    #[serde(rename = "DumplingLevelingErrorCount", skip_serializing_if = "Option::is_none")]
    pub dumpling_leveling_error_count: Option<BdtPositiveIntegerType>,
    #[serde(rename = "DamagedPlantsErrorCount", skip_serializing_if = "Option::is_none")]
    pub damaged_plants_error_count: Option<BdtPositiveIntegerType>,
    #[serde(rename = "PlantCount")]
    pub plant_count: BdtPositiveInteger3digitsType,
    #[serde(rename = "PlantedSeedlingCountInCultivatedSpots", skip_serializing_if = "Option::is_none")]
    pub planted_seedling_count_in_cultivated_spots: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "PlantedSeedlingCountInNotCultivatedSpots", skip_serializing_if = "Option::is_none")]
    pub planted_seedling_count_in_not_cultivated_spots: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "NearestSeedlingPlantingDepth", skip_serializing_if = "Option::is_none")]
    pub nearest_seedling_planting_depth: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "NearestSeedlingDistance", skip_serializing_if = "Option::is_none")]
    pub nearest_seedling_distance: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "NearestSeedlingSoilCompressed", skip_serializing_if = "Option::is_none")]
    pub nearest_seedling_soil_compressed: Option<BdtYesNoType>,
    #[serde(rename = "RockySoil", skip_serializing_if = "Option::is_none")]
    pub rocky_soil: Option<BdtYesNoType>,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<BdtString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumberType {
    #[serde(flatten)]
    pub base: Xsstring,
}

