#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
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
pub struct UserId {
    #[serde(flatten)]
    pub user_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummary {
    #[serde(flatten)]
    pub tree_summary: TreeSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandNumber {
    #[serde(flatten)]
    pub stand_number: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaries {
    #[serde(flatten)]
    pub tree_summaries: SamplePlotTreesSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeSummary {
    #[serde(flatten)]
    pub age_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSummary {
    #[serde(flatten)]
    pub mean_diameter_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionAvgDiameterSummary {
    #[serde(flatten)]
    pub reduction_avg_diameter_summary: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingTrees {
    #[serde(flatten)]
    pub remaining_trees: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgAgeSummary {
    #[serde(flatten)]
    pub stand_avg_age_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: WctTaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSummary {
    #[serde(flatten)]
    pub stem_count_summary: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryRequired {
    #[serde(flatten)]
    pub geometry_required: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: BdtYesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionAvgDiameter {
    #[serde(flatten)]
    pub reduction_avg_diameter: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: GdtPointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgHeightSummary {
    #[serde(flatten)]
    pub stand_avg_height_summary: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSummary {
    #[serde(flatten)]
    pub mean_height_summary: BdtDecimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: BdtString1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionCount {
    #[serde(flatten)]
    pub reduction_count: BdtPositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: BdtMeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionStumpHeight {
    #[serde(flatten)]
    pub reduction_stump_height: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Damages {
    #[serde(flatten)]
    pub damages: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgStemCountSummary {
    #[serde(flatten)]
    pub stand_avg_stem_count_summary: BdtPositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestedTrees {
    #[serde(flatten)]
    pub harvested_trees: BdtFinalAuditAnswerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagement {
    #[serde(flatten)]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadiusReduction {
    #[serde(flatten)]
    pub radius_reduction: BdtDecimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDiameterSummary {
    #[serde(flatten)]
    pub stand_avg_diameter_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReductionCountSummary {
    #[serde(flatten)]
    pub reduction_count_summary: BdtPositiveInteger5digitsType,
}

