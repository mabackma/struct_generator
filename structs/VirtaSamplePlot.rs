use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotBasalArea {
    #[serde(flatten)]
    pub sample_plot_basal_area: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotEastingCoordinate {
    #[serde(flatten)]
    pub sample_plot_easting_coordinate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: VirtaEvaluationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status4 {
    #[serde(flatten)]
    pub status4: ChangeStateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotTrackDepth {
    #[serde(flatten)]
    pub sample_plot_track_depth: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotMeanHeight {
    #[serde(flatten)]
    pub sample_plot_mean_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotDominantHeight {
    #[serde(flatten)]
    pub sample_plot_dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotStubDiameter {
    #[serde(flatten)]
    pub sample_plot_stub_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotNorthingCoordinate {
    #[serde(flatten)]
    pub sample_plot_northing_coordinate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecondStoreyTrees {
    #[serde(flatten)]
    pub second_storey_trees: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotTrackWidth {
    #[serde(flatten)]
    pub sample_plot_track_width: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotMeanDiameter {
    #[serde(flatten)]
    pub sample_plot_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SamplePlotTrackDistance {
    #[serde(flatten)]
    pub sample_plot_track_distance: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreesType {
    #[serde(rename = "TrTree")]
    pub tr_tree: Vec<Tree>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaEvaluationType {
    #[serde(flatten)]
    pub base: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "Status4", skip_serializing_if = "Option::is_none")]
    pub status4: Option<ChangeStateType>,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: String,
    #[serde(rename = "SamplePlotEastingCoordinate", skip_serializing_if = "Option::is_none")]
    pub sample_plot_easting_coordinate: Option<String>,
    #[serde(rename = "SamplePlotNorthingCoordinate", skip_serializing_if = "Option::is_none")]
    pub sample_plot_northing_coordinate: Option<String>,
    #[serde(rename = "SamplePlotSize", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size: Option<i32>,
    #[serde(rename = "SamplePlotMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub sample_plot_mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotStubDiameter", skip_serializing_if = "Option::is_none")]
    pub sample_plot_stub_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotMeanHeight", skip_serializing_if = "Option::is_none")]
    pub sample_plot_mean_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotDominantHeight", skip_serializing_if = "Option::is_none")]
    pub sample_plot_dominant_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotBasalArea", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basal_area: Option<i32>,
    #[serde(rename = "SoilModificationEstimate", skip_serializing_if = "Option::is_none")]
    pub soil_modification_estimate: Option<VirtaEvaluationType>,
    #[serde(rename = "SamplePlotTrackDistance", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_distance: Option<i32>,
    #[serde(rename = "SamplePlotTrackWidth", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_width: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotTrackDepth", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_depth: Option<i32>,
    #[serde(rename = "SecondStoreyTrees", skip_serializing_if = "Option::is_none")]
    pub second_storey_trees: Option<i32>,
    #[serde(rename = "Trees", skip_serializing_if = "Option::is_none")]
    pub trees: Option<TreesType>,
}

