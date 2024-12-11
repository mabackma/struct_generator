#[derive(Debug, Serialize, Deserialize)]
pub struct TaxNumber {
    #[serde(flatten)]
    pub tax_number: TaxNumberType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummary {
    #[serde(flatten)]
    pub sample_plot_measurement_summary: SamplePlotMeasurementSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlots {
    #[serde(flatten)]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FertileType {
    #[serde(flatten)]
    pub fertile_type: MaterialCodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummary {
    #[serde(flatten)]
    pub sample_plot_summary: SamplePlotSummaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotInfoText {
    #[serde(flatten)]
    pub sample_plot_info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(flatten)]
    pub images: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct isGPSlocation {
    #[serde(flatten)]
    pub is_g_p_slocation: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(flatten)]
    pub hopper_type: HopperTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AverageVolume {
    #[serde(flatten)]
    pub average_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PilotName {
    #[serde(flatten)]
    pub pilot_name: String50Type,
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
pub struct Measurable {
    #[serde(flatten)]
    pub measurable: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaries {
    #[serde(flatten)]
    pub sample_plot_summaries: SamplePlotSummariesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryReal {
    #[serde(flatten)]
    pub geometry_real: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperNumber {
    #[serde(flatten)]
    pub hopper_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilization {
    #[serde(flatten)]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerId {
    #[serde(flatten)]
    pub measurer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hopper {
    #[serde(flatten)]
    pub hopper: HopperType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hoppers {
    #[serde(flatten)]
    pub hoppers: HoppersType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperLocationFromGPS {
    #[serde(flatten)]
    pub hopper_location_from_g_p_s: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerName {
    #[serde(flatten)]
    pub measurer_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanVolume {
    #[serde(flatten)]
    pub mean_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurerType {
    #[serde(flatten)]
    pub measurer_type: MeasurerTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasureDate {
    #[serde(flatten)]
    pub measure_date: DateType,
}

