#[derive(Debug, Serialize, Deserialize)]
pub struct HelpGeometryType {
    #[serde(flatten)]
    pub help_geometry_type: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGeometry {
    #[serde(flatten)]
    pub line_geometry: LineGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonGeometry {
    #[serde(flatten)]
    pub polygon_geometry: PolygonGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    #[serde(flatten)]
    pub id: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointGeometry {
    #[serde(flatten)]
    pub point_geometry: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub status: ChangeStateType,
}

