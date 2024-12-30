use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct base64Binary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "base")]
    pub base: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hexBinary {
    #[serde(rename = "@XmimecontentType")]
    pub xmimecontent_type: contentType,
    #[serde(rename = "base")]
    pub base: Vec<u8>,
}

