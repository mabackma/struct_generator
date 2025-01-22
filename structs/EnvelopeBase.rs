use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeBaseType {
    #[serde(rename = "Header")]
    pub header: HeaderType,
}

