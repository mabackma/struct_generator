#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationDataType {
    #[serde(rename = "ObjectRealization")]
    pub object_realization: Vec<ObjectRealizationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealizationType {
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<String>,
}

