use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderType {
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(rename = "header_type.base")]
    pub base: String,
}

