#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    #[serde(flatten)]
    pub object: ForestCentreControlObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectData {
    #[serde(flatten)]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Objects {
    #[serde(flatten)]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreData {
    #[serde(flatten)]
    pub forest_centre_data: ForestCentreControlDataType,
}

