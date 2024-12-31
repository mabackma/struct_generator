use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Debug, Serialize, Deserialize)]
pub struct base64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "base64_binary.base")]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "hex_binary.base")]
    pub base: Vec<u8>,
}

