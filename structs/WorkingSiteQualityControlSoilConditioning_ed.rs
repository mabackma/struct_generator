#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantSiteCountSummary {
    #[serde(flatten)]
    pub plant_site_count_summary: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub target: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultivationScope {
    #[serde(flatten)]
    pub cultivation_scope: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchesInAdditionToCultivation {
    #[serde(flatten)]
    pub ditches_in_addition_to_cultivation: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpsCount {
    #[serde(flatten)]
    pub scalps_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryRequired {
    #[serde(flatten)]
    pub geometry_required: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilType {
    #[serde(flatten)]
    pub soil_type: CompactSoilTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaries {
    #[serde(flatten)]
    pub sample_plot_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlantingSiteCount {
    #[serde(flatten)]
    pub planting_site_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulgeHeight {
    #[serde(flatten)]
    pub bulge_height: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilConditioningDepth {
    #[serde(flatten)]
    pub soil_conditioning_depth: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotLength {
    #[serde(flatten)]
    pub nearest_cultivated_spot_length: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RockySoil {
    #[serde(flatten)]
    pub rocky_soil: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalpedMoundsCount {
    #[serde(flatten)]
    pub scalped_mounds_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlSoilConditioning {
    #[serde(flatten)]
    pub working_site_quality_control_soil_conditioning: WorkingSiteQualityControlSoilConditioningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DitchMoundsCount {
    #[serde(flatten)]
    pub ditch_mounds_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkCode {
    #[serde(flatten)]
    pub work_code: WorkCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: MeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnoverMoundsCount {
    #[serde(flatten)]
    pub turnover_mounds_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotWidth {
    #[serde(flatten)]
    pub nearest_cultivated_spot_width: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MineralSoilLayer {
    #[serde(flatten)]
    pub mineral_soil_layer: FinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestCultivatedSpotHeight {
    #[serde(flatten)]
    pub nearest_cultivated_spot_height: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

