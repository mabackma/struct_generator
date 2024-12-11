#[derive(Debug, Serialize, Deserialize)]
pub struct TreesType {
    #[serde(rename = "Tree")]
    pub tree: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotType {
    #[serde(rename = "Status4", skip_serializing_if = "Option::is_none")]
    pub status4: Option<ChangeStateType>,
    #[serde(rename = "SamplePlotNumber")]
    pub sample_plot_number: String,
    #[serde(rename = "SamplePlotEastingCoordinate", skip_serializing_if = "Option::is_none")]
    pub sample_plot_easting_coordinate: Option<string>,
    #[serde(rename = "SamplePlotNorthingCoordinate", skip_serializing_if = "Option::is_none")]
    pub sample_plot_northing_coordinate: Option<string>,
    #[serde(rename = "SamplePlotSize", skip_serializing_if = "Option::is_none")]
    pub sample_plot_size: Option<integer>,
    #[serde(rename = "SamplePlotMeanDiameter", skip_serializing_if = "Option::is_none")]
    pub sample_plot_mean_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotStubDiameter", skip_serializing_if = "Option::is_none")]
    pub sample_plot_stub_diameter: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotMeanHeight", skip_serializing_if = "Option::is_none")]
    pub sample_plot_mean_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotDominantHeight", skip_serializing_if = "Option::is_none")]
    pub sample_plot_dominant_height: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotBasalArea", skip_serializing_if = "Option::is_none")]
    pub sample_plot_basal_area: Option<integer>,
    #[serde(rename = "SoilModificationEstimate", skip_serializing_if = "Option::is_none")]
    pub soil_modification_estimate: Option<VirtaEvaluationType>,
    #[serde(rename = "SamplePlotTrackDistance", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_distance: Option<integer>,
    #[serde(rename = "SamplePlotTrackWidth", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_width: Option<PositiveDecimalMax2IntegralPartMax1FractionalPartType>,
    #[serde(rename = "SamplePlotTrackDepth", skip_serializing_if = "Option::is_none")]
    pub sample_plot_track_depth: Option<integer>,
    #[serde(rename = "SecondStoreyTrees", skip_serializing_if = "Option::is_none")]
    pub second_storey_trees: Option<integer>,
    #[serde(rename = "Trees", skip_serializing_if = "Option::is_none")]
    pub trees: Option<TreesType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtaEvaluationType {
    #[serde(flatten)]
    pub base: VirtaEvaluationType,
}

