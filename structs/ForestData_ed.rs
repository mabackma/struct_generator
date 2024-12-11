#[derive(Debug, Serialize, Deserialize)]
pub struct ForestPropertyData {
    #[serde(flatten)]
    pub forest_property_data: ForestPropertyDataType,
}

