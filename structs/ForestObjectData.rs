use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestObjectData {
    #[serde(flatten)]
    pub forest_object_data: ForestObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Description")]
    pub description: String2000Type,
    #[serde(rename = "Sender")]
    pub sender: ContactInformationType,
    #[serde(rename = "Objects")]
    pub objects: ForestObjectDataObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestObjectDataObjectType>,
}

