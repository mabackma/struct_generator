#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeSummary {
    #[serde(flatten)]
    pub age_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandRootDamagesPercentage {
    #[serde(flatten)]
    pub stand_root_damages_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_diameter_other_tree_species: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaOtherTreeSpecies {
    #[serde(flatten)]
    pub basal_area_other_tree_species: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSilverBirch {
    #[serde(flatten)]
    pub stem_count_silver_birch: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSpruce {
    #[serde(flatten)]
    pub basal_area_spruce: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSummary {
    #[serde(flatten)]
    pub mean_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: MeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurer {
    #[serde(flatten)]
    pub measurer: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSummary {
    #[serde(flatten)]
    pub mean_height_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathTooDeep {
    #[serde(flatten)]
    pub vehicle_path_too_deep: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestType {
    #[serde(flatten)]
    pub forest_type: FertilityClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountHardWood {
    #[serde(flatten)]
    pub stem_count_hard_wood: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    #[serde(flatten)]
    pub user_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaPine {
    #[serde(flatten)]
    pub basal_area_pine: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningTooExcessive {
    #[serde(flatten)]
    pub thinning_too_excessive: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSilverBirch {
    #[serde(flatten)]
    pub basal_area_silver_birch: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVehiclePathTooDeepPercentage {
    #[serde(flatten)]
    pub stand_vehicle_path_too_deep_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    #[serde(flatten)]
    pub district: ThinningDistrictType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathDistance {
    #[serde(flatten)]
    pub vehicle_path_distance: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSpruce {
    #[serde(flatten)]
    pub stem_count_spruce: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemDamages {
    #[serde(flatten)]
    pub stem_damages: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlCutting {
    #[serde(flatten)]
    pub working_site_quality_control_cutting: WorkingSiteQualityControlCuttingType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterPine {
    #[serde(flatten)]
    pub mean_diameter_pine: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgHeightSummary {
    #[serde(flatten)]
    pub stand_avg_height_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummary {
    #[serde(flatten)]
    pub tree_summary: TreeSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountPine {
    #[serde(flatten)]
    pub stem_count_pine: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDamages {
    #[serde(flatten)]
    pub root_damages: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandTooHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_too_height_stumps_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeight {
    #[serde(flatten)]
    pub mean_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasalAreaSummary {
    #[serde(flatten)]
    pub stand_basal_area_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVolumeSummary {
    #[serde(flatten)]
    pub stand_volume_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherTreeSpecies {
    #[serde(flatten)]
    pub other_tree_species: OtherTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgStemCountSummary {
    #[serde(flatten)]
    pub stand_avg_stem_count_summary: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandStemDamagesPercentage {
    #[serde(flatten)]
    pub stand_stem_damages_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgVehiclePathDistance {
    #[serde(flatten)]
    pub stand_avg_vehicle_path_distance: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaHardWood {
    #[serde(flatten)]
    pub basal_area_hard_wood: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandCorrectHeightStumpsPercentage {
    #[serde(flatten)]
    pub stand_correct_height_stumps_percentage: PercentWithFraction1Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDiameterSummary {
    #[serde(flatten)]
    pub stand_avg_diameter_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSilverBirch {
    #[serde(flatten)]
    pub mean_height_silver_birch: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityControlDate {
    #[serde(flatten)]
    pub quality_control_date: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightHardWood {
    #[serde(flatten)]
    pub mean_height_hard_wood: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DominantHeight {
    #[serde(flatten)]
    pub dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManipulationMethod {
    #[serde(flatten)]
    pub manipulation_method: WorkCodeQualifierType2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandVehiclePathWidth {
    #[serde(flatten)]
    pub stand_vehicle_path_width: PositiveDecimalMax4IntegralPartMax2FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountOtherTreeSpecies {
    #[serde(flatten)]
    pub stem_count_other_tree_species: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightOtherTreeSpecies {
    #[serde(flatten)]
    pub mean_height_other_tree_species: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightSpruce {
    #[serde(flatten)]
    pub mean_height_spruce: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSpruce {
    #[serde(flatten)]
    pub mean_diameter_spruce: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanHeightPine {
    #[serde(flatten)]
    pub mean_height_pine: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterSilverBirch {
    #[serde(flatten)]
    pub mean_diameter_silver_birch: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearCutting {
    #[serde(flatten)]
    pub clear_cutting: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountSummary {
    #[serde(flatten)]
    pub stem_count_summary: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasalAreaSummary {
    #[serde(flatten)]
    pub basal_area_summary: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeSummary {
    #[serde(flatten)]
    pub volume_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgAgeSummary {
    #[serde(flatten)]
    pub stand_avg_age_summary: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameter {
    #[serde(flatten)]
    pub mean_diameter: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemCountTarget {
    #[serde(flatten)]
    pub stem_count_target: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radius {
    #[serde(flatten)]
    pub radius: Decimal2FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanDiameterHardWood {
    #[serde(flatten)]
    pub mean_diameter_hard_wood: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePathWidth {
    #[serde(flatten)]
    pub vehicle_path_width: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandAvgDominantHeight {
    #[serde(flatten)]
    pub stand_avg_dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsSummaries {
    #[serde(flatten)]
    pub sample_plots_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storey {
    #[serde(flatten)]
    pub storey: StoreyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThinningTooExcessiveCount {
    #[serde(flatten)]
    pub thinning_too_excessive_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorrectHeightStumps {
    #[serde(flatten)]
    pub correct_height_stumps: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TooHeightStumps {
    #[serde(flatten)]
    pub too_height_stumps: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSummaries {
    #[serde(flatten)]
    pub tree_summaries: TreeSummariesType,
}

