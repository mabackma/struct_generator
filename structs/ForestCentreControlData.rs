use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlObjectData {
    #[serde(flatten)]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlDataType {
    #[serde(flatten)]
    pub base: ForestCentreDataType,
    #[serde(rename = "ControlObjectData")]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "CodAdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub cod_additional_details: Option<AdditionalDetails>,
    #[serde(rename = "Objects")]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreControlObjectType>,
}

