#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectData {
    #[serde(flatten)]
    pub forest_object_data: ForestObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    #[serde(flatten)]
    pub description: CoString2000Type,
}

