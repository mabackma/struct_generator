#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "GmlPolygon")]
    pub gml_polygon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "GmlLineString")]
    pub gml_line_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometryType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CoChangeStateType>,
    #[serde(rename = "HelpGeometryType", skip_serializing_if = "Option::is_none")]
    pub help_geometry_type: Option<string>,
    #[serde(rename = "GmlPoint")]
    pub gml_point: String,
}

