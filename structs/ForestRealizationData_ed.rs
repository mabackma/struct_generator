#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(flatten)]
    pub parent_object_type: ObjectTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjects {
    #[serde(flatten)]
    pub parent_objects: ParentObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectId {
    #[serde(flatten)]
    pub parent_object_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(flatten)]
    pub area: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDetails {
    #[serde(flatten)]
    pub additional_details: String2000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestRealizationData {
    #[serde(flatten)]
    pub forest_realization_data: ForestRealizationDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandBasicData {
    #[serde(flatten)]
    pub stand_basic_data: StandBasicDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjects {
    #[serde(flatten)]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometry {
    #[serde(flatten)]
    pub object_geometry: ObjectGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCase {
    #[serde(flatten)]
    pub use_case: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    #[serde(flatten)]
    pub sender: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObject {
    #[serde(flatten)]
    pub geometry_object: GeometryObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDecrease {
    #[serde(flatten)]
    pub area_decrease: AreaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObject {
    #[serde(flatten)]
    pub parent_object: ParentObjectType,
}

