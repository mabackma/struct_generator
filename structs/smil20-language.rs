use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct animateType {
    #[serde(flatten)]
    pub base: Smil20animatePrototype,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct animateMotionType {
    #[serde(flatten)]
    pub base: Smil20animateMotionPrototype,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct animateColorType {
    #[serde(flatten)]
    pub base: Smil20animateColorPrototype,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct setType {
    #[serde(flatten)]
    pub base: Smil20setPrototype,
}

