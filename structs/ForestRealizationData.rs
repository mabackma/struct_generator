use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestRealizationData {
    #[serde(flatten)]
    pub forest_realization_data: ForestRealizationDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryObjects {
    #[serde(flatten)]
    pub geometry_objects: GeometryObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObjectId {
    #[serde(flatten)]
    pub parent_object_id: CoIdStringNotEmptyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObjectType {
    #[serde(flatten)]
    pub parent_object_type: ObjectTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryObject {
    #[serde(flatten)]
    pub geometry_object: GeometryObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObjects {
    #[serde(flatten)]
    pub parent_objects: ParentObjectsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentObject {
    #[serde(flatten)]
    pub parent_object: ParentObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestRealizationDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "UseCase")]
    pub use_case: String,
    #[serde(rename = "GeometryObjects")]
    pub geometry_objects: GeometryObjectsType,
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
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<TsTreeStandData>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<OpOperations>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<StSpecialFeatures>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeometryType {
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<CoChangeState>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<CoChangeTime>,
    #[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
    pub area: Option<AreaType>,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub area_decrease: Option<AreaType>,
    #[serde(rename = "AlternativeGeometriesGroup")]
    pub gdt_alternative_geometries_group: GdtAlternativeGeometriesGroup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectsType {
    #[serde(rename = "ParentObject")]
    pub parent_object: Vec<ParentObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentObjectType {
    #[serde(rename = "ParentObjectType")]
    pub parent_object_type: ObjectTypeType,
    #[serde(rename = "ParentObjectId")]
    pub parent_object_id: CoIdStringNotEmptyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeType {
    pub base: String,
}

