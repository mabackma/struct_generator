#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "@id")]
    pub id: BdtIdStringNotEmptyType,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: BdtPositiveInteger3digitsType,
    #[serde(rename = "StandNumber")]
    pub stand_number: BdtString20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<BdtString20Type>,
    #[serde(rename = "MeasureDate")]
    pub measure_date: BdtDateType,
    #[serde(rename = "Measurable")]
    pub measurable: BdtYesNoType,
    #[serde(rename = "MeasurerId")]
    pub measurer_id: BdtString20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<WctTaxNumberType>,
    #[serde(rename = "MeasurerName")]
    pub measurer_name: BdtString50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "GeometryReal")]
    pub geometry_real: GdtPointGeometryType,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: BdtYesNoType,
    #[serde(rename = "PilotName", skip_serializing_if = "Option::is_none")]
    pub pilot_name: Option<BdtString50Type>,
    #[serde(rename = "FertileType", skip_serializing_if = "Option::is_none")]
    pub fertile_type: Option<BdtMaterialCodeType>,
    #[serde(rename = "MeanVolume", skip_serializing_if = "Option::is_none")]
    pub mean_volume: Option<BdtDecimal3FractionDigitsType>,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Hoppers")]
    pub hoppers: HoppersType,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<BdtString1000Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(rename = "HopperNumber")]
    pub hopper_number: BdtString20Type,
    #[serde(rename = "HopperType")]
    pub hopper_type: WctHopperTypeType,
    #[serde(rename = "Volume")]
    pub volume: BdtDecimal3FractionDigitsType,
    #[serde(rename = "Geometry")]
    pub geometry: GdtPointGeometryType,
    #[serde(rename = "HopperLocationFromGPS")]
    pub hopper_location_from_g_p_s: BdtYesNoType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<BdtString100Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: BdtString20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilizationType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<BdtPositiveInteger2digitsType>,
    #[serde(rename = "SamplePlotSummaries")]
    pub sample_plot_summaries: SamplePlotSummariesType,
    #[serde(rename = "SamplePlots")]
    pub sample_plots: SamplePlotsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoppersType {
    #[serde(rename = "Hopper", skip_serializing_if = "Option::is_none")]
    pub hopper: Option<Vec<HopperType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: BdtMeasurerTypeType,
    #[serde(rename = "AverageVolume")]
    pub average_volume: BdtDecimal3FractionDigitsType,
}

