#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandId")]
    pub stand_id: BdtString20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "StandAvgDiameterSummary")]
    pub stand_avg_diameter_summary: BdtPositiveInteger3digitsType,
    #[serde(rename = "StandAvgHeightSummary")]
    pub stand_avg_height_summary: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
    #[serde(rename = "StandAvgDominantHeight", skip_serializing_if = "Option::is_none")]
    pub stand_avg_dominant_height: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StandAvgAgeSummary", skip_serializing_if = "Option::is_none")]
    pub stand_avg_age_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StandAvgStemCountSummary")]
    pub stand_avg_stem_count_summary: BdtPositiveInteger5digitsType,
    #[serde(rename = "StandBasalAreaSummary", skip_serializing_if = "Option::is_none")]
    pub stand_basal_area_summary: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "StandVolumeSummary", skip_serializing_if = "Option::is_none")]
    pub stand_volume_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StandStemDamagesPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_stem_damages_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandRootDamagesPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_root_damages_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandCorrectHeightStumpsPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_correct_height_stumps_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandTooHeightStumpsPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_too_height_stumps_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandVehiclePathTooDeepPercentage", skip_serializing_if = "Option::is_none")]
    pub stand_vehicle_path_too_deep_percentage: Option<BdtPercentWithFraction1Type>,
    #[serde(rename = "StandAvgVehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub stand_avg_vehicle_path_distance: Option<BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "StandVehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub stand_vehicle_path_width: Option<BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "TreeSummaries")]
    pub tree_summaries: TreeSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummariesType {
    #[serde(rename = "TreeSummary")]
    pub tree_summary: Vec<TreeSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesDataType {
    #[serde(rename = "TreeSpecies")]
    pub tree_species: BdtTreeSpeciesType,
    #[serde(rename = "StemCountOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub stem_count_other_tree_species: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "MeanHeightOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub mean_height_other_tree_species: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub basal_area_other_tree_species: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanDiameterOtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_other_tree_species: Option<BdtPositiveInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlBaseCuttingType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "QualityControlDate", skip_serializing_if = "Option::is_none")]
    pub quality_control_date: Option<BdtDateType>,
    #[serde(rename = "SamplePlotsSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plots_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "@id")]
    pub id: BdtIdStringNotEmptyType,
    #[serde(rename = "StandId")]
    pub stand_id: BdtString20Type,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: BdtPositiveInteger3digitsType,
    #[serde(rename = "MeasureDate")]
    pub measure_date: BdtTimeStampType,
    #[serde(rename = "Measurer", skip_serializing_if = "Option::is_none")]
    pub measurer: Option<BdtString50Type>,
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<BdtString20Type>,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<WctTaxNumberType>,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPointGeometryType,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: BdtYesNoType,
    #[serde(rename = "District")]
    pub district: BdtThinningDistrictType,
    #[serde(rename = "Radius")]
    pub radius: BdtDecimal2FractionDigitsType,
    #[serde(rename = "ManipulationMethod", skip_serializing_if = "Option::is_none")]
    pub manipulation_method: Option<BdtWorkCodeQualifierType2>,
    #[serde(rename = "ForestType", skip_serializing_if = "Option::is_none")]
    pub forest_type: Option<BdtFertilityClassType>,
    #[serde(rename = "StemCountPine", skip_serializing_if = "Option::is_none")]
    pub stem_count_pine: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "StemCountSpruce", skip_serializing_if = "Option::is_none")]
    pub stem_count_spruce: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "StemCountHardWood", skip_serializing_if = "Option::is_none")]
    pub stem_count_hard_wood: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "StemCountSilverBirch", skip_serializing_if = "Option::is_none")]
    pub stem_count_silver_birch: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "MeanHeightPine", skip_serializing_if = "Option::is_none")]
    pub mean_height_pine: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightSpruce", skip_serializing_if = "Option::is_none")]
    pub mean_height_spruce: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightHardWood", skip_serializing_if = "Option::is_none")]
    pub mean_height_hard_wood: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightSilverBirch", skip_serializing_if = "Option::is_none")]
    pub mean_height_silver_birch: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
    #[serde(rename = "DominantHeight", skip_serializing_if = "Option::is_none")]
    pub dominant_height: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaPine", skip_serializing_if = "Option::is_none")]
    pub basal_area_pine: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaSpruce", skip_serializing_if = "Option::is_none")]
    pub basal_area_spruce: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaHardWood", skip_serializing_if = "Option::is_none")]
    pub basal_area_hard_wood: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "BasalAreaSilverBirch", skip_serializing_if = "Option::is_none")]
    pub basal_area_silver_birch: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "Storey")]
    pub storey: BdtStoreyType,
    #[serde(rename = "DiameterMax", skip_serializing_if = "Option::is_none")]
    pub diameter_max: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "DiameterMin", skip_serializing_if = "Option::is_none")]
    pub diameter_min: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameterPine", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_pine: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameterSpruce", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_spruce: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameterHardWood", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_hard_wood: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameterSilverBirch", skip_serializing_if = "Option::is_none")]
    pub mean_diameter_silver_birch: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: BdtPositiveInteger3digitsType,
    #[serde(rename = "StemCountTarget")]
    pub stem_count_target: BdtPositiveInteger2digitsType,
    #[serde(rename = "VehiclePathDistance", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_distance: Option<BdtPositiveDecimalMax4IntegralPartMax2FractionalPartType>,
    #[serde(rename = "VehiclePathWidth", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_width: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "ThinningTooExcessive")]
    pub thinning_too_excessive: BdtYesNoType,
    #[serde(rename = "ThinningTooExcessiveCount", skip_serializing_if = "Option::is_none")]
    pub thinning_too_excessive_count: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "ClearCutting")]
    pub clear_cutting: BdtYesNoType,
    #[serde(rename = "OtherTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub other_tree_species: Option<OtherTreeSpeciesType>,
    #[serde(rename = "StemDamages", skip_serializing_if = "Option::is_none")]
    pub stem_damages: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "RootDamages", skip_serializing_if = "Option::is_none")]
    pub root_damages: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<BdtString1000Type>,
    #[serde(rename = "CorrectHeightStumps", skip_serializing_if = "Option::is_none")]
    pub correct_height_stumps: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "TooHeightStumps", skip_serializing_if = "Option::is_none")]
    pub too_height_stumps: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "VehiclePathTooDeep", skip_serializing_if = "Option::is_none")]
    pub vehicle_path_too_deep: Option<BdtPositiveInteger2digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaryType {
    #[serde(rename = "Storey")]
    pub storey: BdtStoreyType,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: BdtTreeSpeciesType,
    #[serde(rename = "AgeSummary", skip_serializing_if = "Option::is_none")]
    pub age_summary: Option<BdtPositiveInteger3digitsType>,
    #[serde(rename = "StemCountSummary")]
    pub stem_count_summary: BdtPositiveInteger4digitsType,
    #[serde(rename = "BasalAreaSummary", skip_serializing_if = "Option::is_none")]
    pub basal_area_summary: Option<BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "MeanHeightSummary")]
    pub mean_height_summary: BdtPositiveDecimalMax2IntegralPartMax1FractionalPartType,
    #[serde(rename = "MeanDiameterSummary")]
    pub mean_diameter_summary: BdtPositiveInteger3digitsType,
    #[serde(rename = "VolumeSummary", skip_serializing_if = "Option::is_none")]
    pub volume_summary: Option<BdtPositiveInteger3digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCuttingType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlBaseCuttingType,
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "QualityControlDate")]
    pub quality_control_date: BdtDateType,
    #[serde(rename = "SamplePlotsSummaries")]
    pub sample_plots_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringWorkingSiteQualityControlCuttingType {
    #[serde(flatten)]
    pub base: WorkingSiteQualityControlBaseCuttingType,
    #[serde(rename = "QualityControlDate", skip_serializing_if = "Option::is_none")]
    pub quality_control_date: Option<BdtDateType>,
    #[serde(rename = "SamplePlotsSummaries", skip_serializing_if = "Option::is_none")]
    pub sample_plots_summaries: Option<SamplePlotSummariesType>,
    #[serde(rename = "SamplePlots", skip_serializing_if = "Option::is_none")]
    pub sample_plots: Option<SamplePlotsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpeciesType {
    #[serde(rename = "OtherTreeSpecies")]
    pub other_tree_species: Vec<OtherTreeSpeciesDataType>,
}

