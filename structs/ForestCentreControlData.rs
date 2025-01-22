use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlObjectData {
    #[serde(flatten)]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreData {
    #[serde(flatten)]
    pub forest_centre_data: ForestCentreControlDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreControlObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub cod_additional_details: Option<CodAdditionalDetails>,
    #[serde(rename = "Objects")]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ControlObjectData")]
    pub control_object_data: ControlObjectDataType,
}

