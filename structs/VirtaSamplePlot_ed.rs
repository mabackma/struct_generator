#[derive(Debug, Serialize, Deserialize)]
pub struct Status4 {
    #[serde(flatten)]
    pub status4: ChangeStateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotEastingCoordinate {
    #[serde(flatten)]
    pub sample_plot_easting_coordinate: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDepth {
    #[serde(flatten)]
    pub sample_plot_track_depth: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNumber {
    #[serde(flatten)]
    pub sample_plot_number: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanHeight {
    #[serde(flatten)]
    pub sample_plot_mean_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trees {
    #[serde(flatten)]
    pub trees: TreesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackWidth {
    #[serde(flatten)]
    pub sample_plot_track_width: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondStoreyTrees {
    #[serde(flatten)]
    pub second_storey_trees: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotTrackDistance {
    #[serde(flatten)]
    pub sample_plot_track_distance: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotNorthingCoordinate {
    #[serde(flatten)]
    pub sample_plot_northing_coordinate: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilModificationEstimate {
    #[serde(flatten)]
    pub soil_modification_estimate: VirtaEvaluationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotStubDiameter {
    #[serde(flatten)]
    pub sample_plot_stub_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotMeanDiameter {
    #[serde(flatten)]
    pub sample_plot_mean_diameter: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotSize {
    #[serde(flatten)]
    pub sample_plot_size: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlot {
    #[serde(flatten)]
    pub sample_plot: SamplePlotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotBasalArea {
    #[serde(flatten)]
    pub sample_plot_basal_area: integer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplePlotDominantHeight {
    #[serde(flatten)]
    pub sample_plot_dominant_height: PositiveDecimalMax2IntegralPartMax1FractionalPartType,
}

