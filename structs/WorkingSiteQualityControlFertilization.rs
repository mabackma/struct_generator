use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct HopperLocationFromGPS {
    #[serde(flatten)]
    pub hopper_location_from_g_p_s: BdtYesNoType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PilotName {
    #[serde(flatten)]
    pub pilot_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteQualityControlFertilization {
    #[serde(flatten)]
    pub working_site_quality_control_fertilization: WorkingSiteQualityControlFertilizationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurerId {
    #[serde(flatten)]
    pub measurer_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hopper {
    #[serde(flatten)]
    pub hopper: HopperType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeanVolume {
    #[serde(flatten)]
    pub mean_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FertileType {
    #[serde(flatten)]
    pub fertile_type: BdtMaterialCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HopperNumber {
    #[serde(flatten)]
    pub hopper_number: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeasurerName {
    #[serde(flatten)]
    pub measurer_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hoppers {
    #[serde(flatten)]
    pub hoppers: HoppersType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HopperType {
    #[serde(flatten)]
    pub hopper_type: WctHopperTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AverageVolume {
    #[serde(flatten)]
    pub average_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteQualityControlFertilizationType {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct HoppersType {
    #[serde(rename = "Hopper", skip_serializing_if = "Option::is_none")]
    pub hopper: Option<Vec<HopperType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotsType {
    #[serde(rename = "SamplePlot")]
    pub sample_plot: Vec<SamplePlotType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummariesType {
    #[serde(rename = "SamplePlotSummary")]
    pub sample_plot_summary: Vec<SamplePlotSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSummaryType {
    #[serde(rename = "StandNumber")]
    pub stand_number: String20Type,
    #[serde(rename = "SamplePlotMeasurementSummary")]
    pub sample_plot_measurement_summary: Vec<SamplePlotMeasurementSummaryType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeasurementSummaryType {
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "AverageVolume")]
    pub average_volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopperType {
    #[serde(rename = "HopperNumber")]
    pub hopper_number: String20Type,
    #[serde(rename = "HopperType")]
    pub hopper_type: HopperTypeType,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Geometry")]
    pub geometry: PointGeometryType,
    #[serde(rename = "HopperLocationFromGPS")]
    pub hopper_location_from_g_p_s: YesNoType,
    #[serde(rename = "InfoText", skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String100Type>,
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
    #[serde(rename = "MeasurerId")]
    pub measurer_id: String20Type,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<TaxNumberType>,
    #[serde(rename = "MeasurerName")]
    pub measurer_name: String50Type,
    #[serde(rename = "MeasurerType")]
    pub measurer_type: MeasurerTypeType,
    #[serde(rename = "GeometryReal")]
    pub geometry_real: PointGeometryType,
    #[serde(rename = "isGPSlocation")]
    pub is_g_p_slocation: YesNoType,
    #[serde(rename = "PilotName", skip_serializing_if = "Option::is_none")]
    pub pilot_name: Option<String50Type>,
    #[serde(rename = "FertileType", skip_serializing_if = "Option::is_none")]
    pub fertile_type: Option<MaterialCodeType>,
    #[serde(rename = "MeanVolume", skip_serializing_if = "Option::is_none")]
    pub mean_volume: Option<Decimal3FractionDigitsType>,
    #[serde(rename = "Volume")]
    pub volume: Decimal3FractionDigitsType,
    #[serde(rename = "Hoppers")]
    pub hoppers: HoppersType,
    #[serde(rename = "SamplePlotInfoText", skip_serializing_if = "Option::is_none")]
    pub sample_plot_info_text: Option<String1000Type>,
}

