#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Group {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineGeometriesGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "@dimension")]
    pub dimension: f64,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "pointProperty")]
    pub point_property: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMultiPolygonGeometryType {
    #[serde(rename = "MultiPolygon")]
    pub multi_polygon: String,
    #[serde(rename = "MultiSurface")]
    pub multi_surface: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygon2Type {
    #[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
    pub point_property: Option<String>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLineAndPolygonGeometriesGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometryType {
    #[serde(rename = "MultiPolygon")]
    pub multi_polygon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygonType {
    #[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
    pub point_property: Option<String>,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Type {
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: f64,
    #[serde(rename = "@location")]
    pub location: PointLocationType,
    #[serde(rename = "pointProperty")]
    pub point_property: String,
    #[serde(rename = "lineStringProperty")]
    pub line_string_property: String,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesType {
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub simple_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "PolygonGeometryGroup")]
    pub polygon_geometry_group: String,
}

