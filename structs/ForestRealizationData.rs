use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryObjects {
    #[serde(flatten)]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryObject {
    #[serde(flatten)]
    pub geometry_object: GeometryObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestRealizationData {
    #[serde(flatten)]
    pub forest_realization_data: ForestRealizationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObject {
    #[serde(flatten)]
    pub parent_object: ParentObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObjectId {
    #[serde(flatten)]
    pub parent_object_id: IdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObjectType {
    #[serde(flatten)]
    pub parent_object_type: ObjectTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObjects {
    #[serde(flatten)]
    pub parent_objects: ParentObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeType {
    #[serde(rename = "object_type_type.base")]
    pub base: String,
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
pub struct ObjectGeometryType {
    #[serde(rename = "CoChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<ChangeState>,
    #[serde(rename = "CoChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<ChangeTime>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<AreaType>,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaType>,
    #[serde(rename = "GdtAlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: AlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectsType {
    #[serde(rename = "ParentObject")]
    pub parent_object: Vec<ParentObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryObjectsType {
    #[serde(rename = "GeometryObject")]
    pub geometry_object: Vec<GeometryObjectType>,
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
    #[serde(rename = "TsTreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<TreeStandData>,
    #[serde(rename = "OpOperations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<Operations>,
    #[serde(rename = "StSpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<SpecialFeatures>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(rename = "ParentObjectType")]
    pub parent_object_type: ObjectTypeType,
    #[serde(rename = "ParentObjectId")]
    pub parent_object_id: IdStringNotEmptyType,
}

