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

