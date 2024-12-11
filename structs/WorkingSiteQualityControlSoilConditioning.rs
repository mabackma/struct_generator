#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioningBaseType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
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
pub struct SelfMonitoringWorkingSiteQualityControlSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSoilConditioningBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: PositiveInteger3digitsType,
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
    #[serde(rename = "Radius")]
    pub radius: Decimal2FractionDigitsType,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: YesNoType,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<PositiveIntegerType>,
    #[serde(rename = "WorkCode", skip_serializing_if = "Option::is_none")]
    pub work_code: Option<WorkCodeType>,
    #[serde(rename = "CultivationScope", skip_serializing_if = "Option::is_none")]
    pub cultivation_scope: Option<FinalAuditAnswerType>,
    #[serde(rename = "BulgeHeight", skip_serializing_if = "Option::is_none")]
    pub bulge_height: Option<FinalAuditAnswerType>,
    #[serde(rename = "MineralSoilLayer", skip_serializing_if = "Option::is_none")]
    pub mineral_soil_layer: Option<FinalAuditAnswerType>,
    #[serde(rename = "SoilConditioningDepth", skip_serializing_if = "Option::is_none")]
    pub soil_conditioning_depth: Option<FinalAuditAnswerType>,
    #[serde(rename = "PlantingSiteCount")]
    pub planting_site_count: PositiveInteger2digitsType,
    #[serde(rename = "ScalpedMoundsCount", skip_serializing_if = "Option::is_none")]
    pub scalped_mounds_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "DitchMoundsCount", skip_serializing_if = "Option::is_none")]
    pub ditch_mounds_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "TurnoverMoundsCount", skip_serializing_if = "Option::is_none")]
    pub turnover_mounds_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "ScalpsCount", skip_serializing_if = "Option::is_none")]
    pub scalps_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "NearestCultivatedSpotLength", skip_serializing_if = "Option::is_none")]
    pub nearest_cultivated_spot_length: Option<PositiveInteger3digitsType>,
    #[serde(rename = "NearestCultivatedSpotWidth", skip_serializing_if = "Option::is_none")]
    pub nearest_cultivated_spot_width: Option<PositiveInteger3digitsType>,
    #[serde(rename = "NearestCultivatedSpotHeight", skip_serializing_if = "Option::is_none")]
    pub nearest_cultivated_spot_height: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub soil_type: Option<CompactSoilTypeType>,
    #[serde(rename = "RockySoil", skip_serializing_if = "Option::is_none")]
    pub rocky_soil: Option<YesNoType>,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<String1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "PlantSiteCountSummary", skip_serializing_if = "Option::is_none")]
    pub plant_site_count_summary: Option<PositiveInteger4digitsType>,
    #[serde(rename = "DitchesInAdditionToCultivation", skip_serializing_if = "Option::is_none")]
    pub ditches_in_addition_to_cultivation: Option<PositiveInteger4digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioningType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlSoilConditioningBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

