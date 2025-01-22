use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct CumulativePointDistribution {
    #[serde(flatten)]
    pub cumulative_point_distribution: CumulativePointDistributionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mean {
    #[serde(flatten)]
    pub mean: MeanType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeGamma {
    #[serde(flatten)]
    pub shape_gamma: ShapeGammaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shape {
    #[serde(flatten)]
    pub shape: ShapeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beta {
    #[serde(flatten)]
    pub beta: BetaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CumulativePoint {
    #[serde(flatten)]
    pub cumulative_point: CumulativePointType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeBeta {
    #[serde(flatten)]
    pub shape_beta: ShapeBetaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Normal {
    #[serde(flatten)]
    pub normal: NormalType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Minimum {
    #[serde(flatten)]
    pub minimum: MinimumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CumulativeMass {
    #[serde(flatten)]
    pub cumulative_mass: CumulativeMassType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variance {
    #[serde(flatten)]
    pub variance: VarianceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scale {
    #[serde(flatten)]
    pub scale: ScaleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Maximum {
    #[serde(flatten)]
    pub maximum: MaximumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JohnsonSB {
    #[serde(flatten)]
    pub johnson_s_b: JohnsonSBType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gamma {
    #[serde(flatten)]
    pub gamma: GammaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weibull {
    #[serde(flatten)]
    pub weibull: WeibullType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeDelta {
    #[serde(flatten)]
    pub shape_delta: ShapeDeltaType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShapeAlfa {
    #[serde(flatten)]
    pub shape_alfa: ShapeAlfaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GammaType {
    #[serde(rename = "Shape")]
    pub shape: ShapeType,
    #[serde(rename = "Scale")]
    pub scale: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativePointDistributionType {
    #[serde(rename = "CumulativePoint")]
    pub cumulative_point: Vec<CumulativePointType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeBetaType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimumType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeanType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeAlfaType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionType {
    #[serde(rename = "DistributionModelGroup")]
    pub distribution_model_group: DistributionModelGroup,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct VarianceType {
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
pub struct ShapeDeltaType {
    pub base: f32,
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
    #[serde(rename = "Normal")]
    pub normal: Normal,
    #[serde(rename = "Weibull")]
    pub weibull: Weibull,
    #[serde(rename = "Gamma")]
    pub gamma: Gamma,
    #[serde(rename = "Beta")]
    pub beta: Beta,
    #[serde(rename = "CumulativePointDistribution")]
    pub cumulative_point_distribution: CumulativePointDistribution,
    #[serde(rename = "JohnsonSB")]
    pub johnson_s_b: JohnsonSB,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CumulativeMassType {
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
pub struct LocationType {
    pub base: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeGammaType {
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
pub struct MaximumType {
    pub base: f32,
}

