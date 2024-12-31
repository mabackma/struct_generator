use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct PointGeometry {
    #[serde(flatten)]
    pub point_geometry: PointGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonGeometry {
    #[serde(flatten)]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiPolygonGeometry {
    #[serde(flatten)]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Group {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygonType {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<Point<f64>>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: Polygon<f64>,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometryType {
    #[serde(rename = "GmlMultiPolygon")]
    pub gml_multi_polygon: MultiPolygon<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "@dimension")]
    pub dimension: f64,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "GmlpointProperty")]
    pub gmlpoint_property: Point<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "PolygonGeometryGroup")]
    pub polygon_geometry_group: PolygonGeometryGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Type {
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: AlternativeGeometries2Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygon2Type {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<Point<f64>>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: Polygon<f64>,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMultiPolygonGeometryType {
    #[serde(rename = "GmlMultiPolygon")]
    pub gml_multi_polygon: MultiPolygon<f64>,
    #[serde(rename = "GmlMultiSurface")]
    pub gml_multi_surface: MultiSurface,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesType {
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub simple_alternative_geometries_group: SimpleAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: f64,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "GmlpointProperty")]
    pub gmlpoint_property: Point<f64>,
    #[serde(rename = "GmllineStringProperty")]
    pub gmlline_string_property: lineStringProperty,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: Polygon<f64>,
}

