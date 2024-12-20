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
    #[serde(rename = "RadiusReduction", skip_serializing_if = "Option::is_none")]
    pub radius_reduction: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
    pub storey: Option<BdtStoreyType>,
    #[serde(rename = "ReductionCount", skip_serializing_if = "Option::is_none")]
    pub reduction_count: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "ReductionAvgDiameter", skip_serializing_if = "Option::is_none")]
    pub reduction_avg_diameter: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "ReductionStumpHeight", skip_serializing_if = "Option::is_none")]
    pub reduction_stump_height: Option<BdtFinalAuditAnswerType>,
    #[serde(rename = "RemainingTrees", skip_serializing_if = "Option::is_none")]
    pub remaining_trees: Option<BdtFinalAuditAnswerType>,
    #[serde(rename = "Damages", skip_serializing_if = "Option::is_none")]
    pub damages: Option<BdtFinalAuditAnswerType>,
    #[serde(rename = "HarvestedTrees", skip_serializing_if = "Option::is_none")]
    pub harvested_trees: Option<BdtFinalAuditAnswerType>,
    #[serde(rename = "Trees", skip_serializing_if = "Option::is_none")]
    pub trees: Option<SamplePlotTreesType>,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<BdtString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "ReductionCountSummary", skip_serializing_if = "Option::is_none")]
    pub reduction_count_summary: Option<BdtPositiveInteger5digitsType>,
    #[serde(rename = "ReductionAvgDiameterSummary", skip_serializing_if = "Option::is_none")]
    pub reduction_avg_diameter_summary: Option<BdtDecimal2FractionDigitsType>,
    #[serde(rename = "StandAvgDiameterSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_diameter_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StandAvgHeightSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_height_summary: Option<BdtDecimal1FractionDigitType>,
    #[serde(rename = "StandAvgAgeSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_age_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StandAvgStemCountSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_stem_count_summary: Option<BdtPositiveInteger5digitsType>,
    #[serde(rename = "TreeSummaries", skip_serializing_if = "Option::is_none")]
    pub tree_summaries: Option<SamplePlotTreesSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaryType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: BdtTreeSpeciesType,
    #[serde(rename = "AgeSummary", skip_serializing_if = "Option::is_none")]
    pub age_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StemCountSummary")]
    pub stem_count_summary: BdtPositiveInteger5digitsType,
    #[serde(rename = "MeanHeightSummary")]
    pub mean_height_summary: BdtDecimal1FractionDigitType,
    #[serde(rename = "MeanDiameterSummary")]
    pub mean_diameter_summary: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesType {
    #[serde(rename = "Trees")]
    pub trees: Vec<TreeType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlPlantManagementBaseType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plot_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: BdtTreeSpeciesType,
    #[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
    pub age: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StemCount")]
    pub stem_count: BdtPositiveInteger2digitsType,
    #[serde(rename = "MeanHeight")]
    pub mean_height: BdtDecimal1FractionDigitType,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: BdtPositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTreesSummaryType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementBaseType {
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
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: BdtString20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlPlantManagementType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlPlantManagementBaseType,
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

