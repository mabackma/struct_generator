use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct HelpGeometryType {
    #[serde(flatten)]
    pub help_geometry_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineGeometry {
    #[serde(flatten)]
    pub line_geometry: LineGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<String>,
    #[serde(rename = "GmlLineString")]
    pub gml_line_string: LineString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<String>,
    #[serde(rename = "GmlPoint")]
    pub gml_point: Point<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<String>,
    #[serde(rename = "GmlPolygon")]
    pub gml_polygon: Polygon<f64>,
}

