use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameterSummary {
    #[serde(flatten)]
    pub mean_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemainingTrees {
    #[serde(flatten)]
    pub remaining_trees: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSummaries {
    #[serde(flatten)]
    pub tree_summaries: SamplePlotTreesSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityControlPlantManagement {
    #[serde(flatten)]
    pub working_site_quality_control_plant_management: WorkingSiteQualityControlPlantManagementType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Damages {
    #[serde(flatten)]
    pub damages: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionAvgDiameter {
    #[serde(flatten)]
    pub reduction_avg_diameter: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgHeightSummary {
    #[serde(flatten)]
    pub stand_avg_height_summary: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RadiusReduction {
    #[serde(flatten)]
    pub radius_reduction: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionStumpHeight {
    #[serde(flatten)]
    pub reduction_stump_height: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionCount {
    #[serde(flatten)]
    pub reduction_count: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgeSummary {
    #[serde(flatten)]
    pub age_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSummaries {
    #[serde(flatten)]
    pub sample_plot_summaries: SamplePlotSummariesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: String1000Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: String20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgAgeSummary {
    #[serde(flatten)]
    pub stand_avg_age_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: MeasurerTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarvestedTrees {
    #[serde(flatten)]
    pub harvested_trees: FinalAuditAnswerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: String50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgDiameterSummary {
    #[serde(flatten)]
    pub stand_avg_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanHeightSummary {
    #[serde(flatten)]
    pub mean_height_summary: Decimal1FractionDigitType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionCountSummary {
    #[serde(flatten)]
    pub reduction_count_summary: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    #[serde(flatten)]
    pub images: PositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeSummary {
    #[serde(flatten)]
    pub tree_summary: TreeSummaryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemCountSummary {
    #[serde(flatten)]
    pub stem_count_summary: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: TreeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReductionAvgDiameterSummary {
    #[serde(flatten)]
    pub reduction_avg_diameter_summary: Decimal2FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: DateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryRequired {
    #[serde(flatten)]
    pub geometry_required: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: PositiveIntegerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandAvgStemCountSummary {
    #[serde(flatten)]
    pub stand_avg_stem_count_summary: PositiveInteger5digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: YesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlPlantManagementBaseType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "Images")]
    pub images: PositiveInteger2digitsType,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "ReductionCountSummary", skip_serializing_if = "Option::is_none")]
    pub reduction_count_summary: Option<PositiveInteger5digitsType>,
    #[serde(rename = "ReductionAvgDiameterSummary", skip_serializing_if = "Option::is_none")]
    pub reduction_avg_diameter_summary: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "StandAvgDiameterSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_diameter_summary: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StandAvgHeightSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_height_summary: Option<Decimal1FractionDigitType>,
    #[serde(rename = "StandAvgAgeSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_age_summary: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StandAvgStemCountSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_stem_count_summary: Option<PositiveInteger5digitsType>,
    #[serde(rename = "TreeSummaries", skip_serializing_if = "Option::is_none")]
    pub tree_summaries: Option<SamplePlotTreesSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlPlantManagementBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<PositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementBaseType {
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
pub struct SamplePlotTreesSummaryType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesType {
    #[serde(rename = "Trees")]
    pub trees: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StemCount")]
    pub stem_count: PositiveInteger2digitsType,
    #[serde(rename = "MeanHeight")]
    pub mean_height: Decimal1FractionDigitType,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaryType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: TreeSpeciesType,
    #[serde(rename = "AgeSummary", skip_serializing_if = "Option::is_none")]
    pub age_summary: Option<PositiveInteger3digitsType>,
    #[serde(rename = "StemCountSummary")]
    pub stem_count_summary: PositiveInteger5digitsType,
    #[serde(rename = "MeanHeightSummary")]
    pub mean_height_summary: Decimal1FractionDigitType,
    #[serde(rename = "MeanDiameterSummary")]
    pub mean_diameter_summary: PositiveInteger3digitsType,
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
    #[serde(rename = "RadiusReduction", skip_serializing_if = "Option::is_none")]
    pub radius_reduction: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<StoreyType>,
    #[serde(rename = "ReductionCount", skip_serializing_if = "Option::is_none")]
    pub reduction_count: Option<PositiveInteger2digitsType>,
    #[serde(rename = "ReductionAvgDiameter", skip_serializing_if = "Option::is_none")]
    pub reduction_avg_diameter: Option<Decimal2FractionDigitsType>,
    #[serde(rename = "ReductionStumpHeight", skip_serializing_if = "Option::is_none")]
    pub reduction_stump_height: Option<FinalAuditAnswerType>,
    #[serde(rename = "RemainingTrees", skip_serializing_if = "Option::is_none")]
    pub remaining_trees: Option<FinalAuditAnswerType>,
    #[serde(rename = "Damages", skip_serializing_if = "Option::is_none")]
    pub damages: Option<FinalAuditAnswerType>,
    #[serde(rename = "HarvestedTrees", skip_serializing_if = "Option::is_none")]
    pub harvested_trees: Option<FinalAuditAnswerType>,
    #[serde(rename = "Trees", skip_serializing_if = "Option::is_none")]
    pub trees: Option<SamplePlotTreesType>,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<String1000Type>,
}

