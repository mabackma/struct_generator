#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationData {
    #[serde(flatten)]
    pub objects_realization_data: ObjectsRealizationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealization {
    #[serde(flatten)]
    pub object_realization: ObjectRealizationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealizationType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "TsTreeStandDataDate")]
    pub ts_tree_stand_data_date: Vec<TreeStandDataDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationDataType {
    #[serde(rename = "ObjectRealization")]
    pub object_realization: Vec<ObjectRealizationType>,
}

