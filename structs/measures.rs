use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct degrees {
    #[serde(flatten)]
    pub degrees: GmlDegreesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct minutes {
    #[serde(flatten)]
    pub minutes: GmlArcMinutesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct angle {
    #[serde(flatten)]
    pub angle: GmlMeasureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct measure {
    #[serde(flatten)]
    pub measure: GmlMeasureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct dmsAngle {
    #[serde(flatten)]
    pub dms_angle: GmlDMSAngleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct decimalMinutes {
    #[serde(flatten)]
    pub decimal_minutes: GmlDecimalMinutesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct seconds {
    #[serde(flatten)]
    pub seconds: GmlArcSecondsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridLengthType {
    #[serde(flatten)]
    pub base: GmlMeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AngleChoiceType {
    #[serde(rename = "angle")]
    pub gmlangle: Gmlangle,
    #[serde(rename = "dmsAngle")]
    pub gmldms_angle: GmldmsAngle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreeValueType {
    pub base: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DegreesType {
    #[serde(flatten)]
    pub base: GmlDegreeValueType,
    #[serde(rename = "direction")]
    pub direction: direction,
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeType {
    #[serde(flatten)]
    pub base: GmlMeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedType {
    #[serde(flatten)]
    pub base: GmlMeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DMSAngleType {
    #[serde(rename = "degrees")]
    pub gmldegrees: Gmldegrees,
    #[serde(rename = "decimalMinutes")]
    pub gmldecimal_minutes: GmldecimalMinutes,
    #[serde(rename = "minutes")]
    pub gmlminutes: Gmlminutes,
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub gmlseconds: Option<Gmlseconds>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeType {
    #[serde(flatten)]
    pub base: GmlMeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleType {
    #[serde(flatten)]
    pub base: GmlMeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcMinutesType {
    pub base: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcSecondsType {
    pub base: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaType {
    #[serde(flatten)]
    pub base: GmlMeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthType {
    #[serde(flatten)]
    pub base: GmlMeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AngleType {
    #[serde(flatten)]
    pub base: GmlMeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecimalMinutesType {
    pub base: f64,
}

