#[derive(Debug, Serialize, Deserialize)]
pub struct SecondStoreyTrees {
    #[serde(flatten)]
    pub second_storey_trees: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status4 {
    #[serde(flatten)]
    pub status4: CoChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNorthingCoordinate {
    #[serde(flatten)]
    pub sample_plot_northing_coordinate: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackWidth {
    #[serde(flatten)]
    pub sample_plot_track_width: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanHeight {
    #[serde(flatten)]
    pub sample_plot_mean_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasalArea {
    #[serde(flatten)]
    pub sample_plot_basal_area: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDepth {
    #[serde(flatten)]
    pub sample_plot_track_depth: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotDominantHeight {
    #[serde(flatten)]
    pub sample_plot_dominant_height: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotEastingCoordinate {
    #[serde(flatten)]
    pub sample_plot_easting_coordinate: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDistance {
    #[serde(flatten)]
    pub sample_plot_track_distance: Xsinteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotStubDiameter {
    #[serde(flatten)]
    pub sample_plot_stub_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanDiameter {
    #[serde(flatten)]
    pub sample_plot_mean_diameter: CoPositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

