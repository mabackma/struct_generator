#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectData {
    #[serde(flatten)]
    pub forest_object_data: ForestObjectDataType,
}

