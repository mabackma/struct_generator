#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: WctTaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioning {
    #[serde(flatten)]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotHeight {
    #[serde(flatten)]
    pub nearest_cultivated_spot_height: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchesInAdditionToCultivation {
    #[serde(flatten)]
    pub ditches_in_addition_to_cultivation: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotLength {
    #[serde(flatten)]
    pub nearest_cultivated_spot_length: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulgeHeight {
    #[serde(flatten)]
    pub bulge_height: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationScope {
    #[serde(flatten)]
    pub cultivation_scope: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpedMoundsCount {
    #[serde(flatten)]
    pub scalped_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpsCount {
    #[serde(flatten)]
    pub scalps_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotWidth {
    #[serde(flatten)]
    pub nearest_cultivated_spot_width: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingSiteCount {
    #[serde(flatten)]
    pub planting_site_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MineralSoilLayer {
    #[serde(flatten)]
    pub mineral_soil_layer: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMoundsCount {
    #[serde(flatten)]
    pub ditch_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnoverMoundsCount {
    #[serde(flatten)]
    pub turnover_mounds_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaries {
    #[serde(flatten)]
    pub sample_plot_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: BdtDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantSiteCountSummary {
    #[serde(flatten)]
    pub plant_site_count_summary: BdtPositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub target: BdtPositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningDepth {
    #[serde(flatten)]
    pub soil_conditioning_depth: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryRequired {
    #[serde(flatten)]
    pub geometry_required: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RockySoil {
    #[serde(flatten)]
    pub rocky_soil: BdtYesNoType,
}

