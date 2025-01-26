use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct syncBehaviorType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct animateMotionPrototype {
    #[serde(rename = "origin")]
    pub origin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fillDefaultType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct nonNegativeDecimalType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct syncBehaviorDefaultType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct restartDefaultType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct restartTimingType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fillTimingAttrsType {
    pub base: String,
}

