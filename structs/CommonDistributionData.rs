#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    #[serde(flatten)]
    pub shape: ShapeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistribution {
    #[serde(flatten)]
    pub cumulative_point_distribution: CumulativePointDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maximum {
    #[serde(flatten)]
    pub maximum: MaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGamma {
    #[serde(flatten)]
    pub shape_gamma: ShapeGammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMass {
    #[serde(flatten)]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Beta {
    #[serde(flatten)]
    pub beta: BetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mean {
    #[serde(flatten)]
    pub mean: MeanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePoint {
    #[serde(flatten)]
    pub cumulative_point: CumulativePointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Normal {
    #[serde(flatten)]
    pub normal: NormalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBeta {
    #[serde(flatten)]
    pub shape_beta: ShapeBetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfa {
    #[serde(flatten)]
    pub shape_alfa: ShapeAlfaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variance {
    #[serde(flatten)]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDelta {
    #[serde(flatten)]
    pub shape_delta: ShapeDeltaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weibull {
    #[serde(flatten)]
    pub weibull: WeibullType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minimum {
    #[serde(flatten)]
    pub minimum: MinimumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    #[serde(flatten)]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamma {
    #[serde(flatten)]
    pub gamma: GammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSB {
    #[serde(flatten)]
    pub johnson_s_b: JohnsonSBType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGammaType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaximumType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMassType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfaType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBetaType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VarianceType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeibullType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GammaType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDeltaType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetaType {
    #[serde(rename = "ShapeAlfa")]
    pub shape_alfa: ShapeAlfaType,
    #[serde(rename = "ShapeBeta")]
    pub shape_beta: ShapeBetaType,
    #[serde(rename = "Minimum")]
    pub minimum: MinimumType,
    #[serde(rename = "Maximum")]
    pub maximum: MaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointType {
    #[serde(rename = "Diameter")]
    pub diameter: DiameterType,
    #[serde(rename = "CumulativeMass")]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionModelGroup {
    #[serde(rename = "Weibull")]
    pub weibull: Weibull,
    #[serde(rename = "JohnsonSB")]
    pub johnson_s_b: JohnsonSB,
    #[serde(rename = "Normal")]
    pub normal: Normal,
    #[serde(rename = "Beta")]
    pub beta: Beta,
    #[serde(rename = "Gamma")]
    pub gamma: Gamma,
    #[serde(rename = "CumulativePointDistribution")]
    pub cumulative_point_distribution: CumulativePointDistribution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistributionType {
    #[serde(rename = "CumulativePoint")]
    pub cumulative_point: Vec<CumulativePointType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: DistributionModelGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalType {
    #[serde(rename = "Mean")]
    pub mean: MeanType,
    #[serde(rename = "Variance")]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimumType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeType {
    #[serde(rename = "base")]
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSBType {
    #[serde(rename = "ShapeGamma")]
    pub shape_gamma: ShapeGammaType,
    #[serde(rename = "ShapeDelta")]
    pub shape_delta: ShapeDeltaType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
    #[serde(rename = "Location")]
    pub location: LocationType,
}

