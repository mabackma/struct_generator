use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct low {
    #[serde(flatten)]
    pub low: GmlintegerList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct limits {
    #[serde(flatten)]
    pub limits: GmlGridLimitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct offsetVector {
    #[serde(flatten)]
    pub offset_vector: GmlVectorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grid {
    #[serde(flatten)]
    pub grid: GmlGridType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RectifiedGrid {
    #[serde(flatten)]
    pub rectified_grid: GmlRectifiedGridType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridEnvelope {
    #[serde(flatten)]
    pub grid_envelope: GmlGridEnvelopeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _ImplicitGeometry {
    #[serde(flatten)]
    pub __implicit_geometry: GmlAbstractGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct axisName {
    #[serde(flatten)]
    pub axis_name: string,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct high {
    #[serde(flatten)]
    pub high: GmlintegerList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RectifiedGridType {
    #[serde(flatten)]
    pub base: GmlGridType,
    #[serde(rename = "origin")]
    pub origin: PointPropertyType,
    #[serde(rename = "offsetVector")]
    pub offset_vector: Vec<VectorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridLimitsType {
    #[serde(rename = "GridEnvelope")]
    pub grid_envelope: GridEnvelopeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridType {
    #[serde(flatten)]
    pub base: GmlAbstractGeometryType,
    #[serde(rename = "limits")]
    pub limits: GridLimitsType,
    #[serde(rename = "axisName")]
    pub axis_name: Vec<String>,
    #[serde(rename = "dimension")]
    pub dimension: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridEnvelopeType {
    #[serde(rename = "low")]
    pub low: integerList,
    #[serde(rename = "high")]
    pub high: integerList,
}

