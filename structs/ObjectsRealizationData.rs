#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsRealizationDataType {
    #[serde(rename = "ObjectRealization")]
    pub object_realization: Vec<ObjectRealizationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRealizationType {
    #[serde(rename = "@id")]
    pub id: Xsstring,
    #[serde(rename = "@parentId")]
    pub parent_id: Xsstring,
    #[serde(rename = "TsTreeStandDataDate")]
    pub ts_tree_stand_data_date: Vec<String>,
}

