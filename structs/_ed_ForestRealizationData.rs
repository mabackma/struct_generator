#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(flatten)]
    pub parent_object_type: ObjectTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObject {
    #[serde(flatten)]
    pub geometry_object: GeometryObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectId {
    #[serde(flatten)]
    pub parent_object_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestRealizationData {
    #[serde(flatten)]
    pub forest_realization_data: ForestRealizationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjects {
    #[serde(flatten)]
    pub parent_objects: ParentObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObject {
    #[serde(flatten)]
    pub parent_object: ParentObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjects {
    #[serde(flatten)]
    pub geometry_objects: GeometryObjectsType,
}

