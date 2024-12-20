#[derive(Debug, Serialize, Deserialize)]
pub struct Maximum {
    #[serde(flatten)]
    pub maximum: MaximumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(flatten)]
    pub location: LocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Normal {
    #[serde(flatten)]
    pub normal: NormalType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    #[serde(flatten)]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBeta {
    #[serde(flatten)]
    pub shape_beta: ShapeBetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JohnsonSB {
    #[serde(flatten)]
    pub johnson_s_b: JohnsonSBType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minimum {
    #[serde(flatten)]
    pub minimum: MinimumType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGamma {
    #[serde(flatten)]
    pub shape_gamma: ShapeGammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfa {
    #[serde(flatten)]
    pub shape_alfa: ShapeAlfaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamma {
    #[serde(flatten)]
    pub gamma: GammaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variance {
    #[serde(flatten)]
    pub variance: VarianceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mean {
    #[serde(flatten)]
    pub mean: MeanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDelta {
    #[serde(flatten)]
    pub shape_delta: ShapeDeltaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMass {
    #[serde(flatten)]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistribution {
    #[serde(flatten)]
    pub cumulative_point_distribution: CumulativePointDistributionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Beta {
    #[serde(flatten)]
    pub beta: BetaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    #[serde(flatten)]
    pub shape: ShapeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weibull {
    #[serde(flatten)]
    pub weibull: WeibullType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePoint {
    #[serde(flatten)]
    pub cumulative_point: CumulativePointType,
}

