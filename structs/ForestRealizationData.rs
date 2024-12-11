#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectsType {
    #[serde(rename = "GeometryObject")]
    pub geometry_object: Vec<GeometryObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectsType {
    #[serde(rename = "ParentObject")]
    pub parent_object: Vec<ParentObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectType {
    #[serde(rename = "@id")]
    pub id: IdStringNotEmptyType,
    #[serde(rename = "@type")]
    pub r#type: ObjectTypeType,
    #[serde(rename = "ObjectGeometry", skip_serializing_if = "Option::is_none")]
    pub object_geometry: Option<ObjectGeometryType>,
    #[serde(rename = "ParentObjects", skip_serializing_if = "Option::is_none")]
    pub parent_objects: Option<ParentObjectsType>,
    #[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
    pub stand_basic_data: Option<StandBasicDataType>,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub tree_stand_data: Option<String>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub special_features: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestRealizationDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: String2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: String,
    #[serde(rename = "GeometryObjects")]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(rename = "ParentObjectType")]
    pub parent_object_type: ObjectTypeType,
    #[serde(rename = "ParentObjectId")]
    pub parent_object_id: IdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometryType {
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<AreaType>,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaType>,
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub alternative_geometries_group: String,
}

