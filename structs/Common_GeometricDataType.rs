#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygon2Type {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<String>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Type {
    #[serde(rename = "AlternativeGeometries2Group")]
    pub alternative_geometries2_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesGroup {
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineGeometriesGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLineAndPolygonGeometriesGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedMultiPolygonGeometryType {
    #[serde(rename = "GmlMultiPolygon")]
    pub gml_multi_polygon: String,
    #[serde(rename = "GmlMultiSurface")]
    pub gml_multi_surface: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointAndLineOrPolygonType {
    #[serde(rename = "@dimension")]
    pub dimension: Xsdecimal,
    #[serde(rename = "@location")]
    pub location: CoPointLocationType,
    #[serde(rename = "GmlpointProperty")]
    pub gmlpoint_property: String,
    #[serde(rename = "GmllineStringProperty")]
    pub gmlline_string_property: String,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometries2Group {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygonGeometryType {
    #[serde(rename = "GmlMultiPolygon")]
    pub gml_multi_polygon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAlternativeGeometriesType {
    #[serde(rename = "SimpleAlternativeGeometriesGroup")]
    pub simple_alternative_geometries_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesGroup {
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: MultiPolygonGeometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryGroup {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonOrMultiPolygonType {
    #[serde(rename = "GmlpointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<String>,
    #[serde(rename = "GmlpolygonProperty")]
    pub gmlpolygon_property: String,
    #[serde(rename = "MultiPolygonGeometry")]
    pub multi_polygon_geometry: ExtendedMultiPolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "@dimension")]
    pub dimension: Xsdecimal,
    #[serde(rename = "@location")]
    pub location: CoPointLocationType,
    #[serde(rename = "GmlpointProperty")]
    pub gmlpoint_property: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "PolygonGeometryGroup")]
    pub polygon_geometry_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternativeGeometriesType {
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: String,
}

