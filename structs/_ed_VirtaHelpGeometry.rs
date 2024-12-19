#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometryType {
    #[serde(flatten)]
    pub help_geometry_type: Xsstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometry {
    #[serde(flatten)]
    pub line_geometry: LineGeometryType,
}

