use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct beginPosition {
    #[serde(flatten)]
    pub begin_position: GmlTimePositionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct timePosition {
    #[serde(flatten)]
    pub time_position: GmlTimePositionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _TimePrimitive {
    #[serde(flatten)]
    pub __time_primitive: GmlAbstractTimePrimitiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct endPosition {
    #[serde(flatten)]
    pub end_position: GmlTimePositionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimePeriod {
    #[serde(flatten)]
    pub time_period: GmlTimePeriodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeInstant {
    #[serde(flatten)]
    pub time_instant: GmlTimeInstantType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _TimeComplex {
    #[serde(flatten)]
    pub __time_complex: GmlAbstractTimeComplexType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct validTime {
    #[serde(flatten)]
    pub valid_time: GmlTimePrimitivePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct relatedTime {
    #[serde(flatten)]
    pub related_time: GmlRelatedTimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _TimeObject {
    #[serde(flatten)]
    pub __time_object: GmlAbstractTimeObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct end {
    #[serde(flatten)]
    pub end: GmlTimeInstantPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct timeInterval {
    #[serde(flatten)]
    pub time_interval: GmlTimeIntervalLengthType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct duration {
    #[serde(flatten)]
    pub duration: duration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct begin {
    #[serde(flatten)]
    pub begin: GmlTimeInstantPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _TimeGeometricPrimitive {
    #[serde(flatten)]
    pub __time_geometric_primitive: GmlAbstractTimeGeometricPrimitiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeIndeterminateValueType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractTimeGeometricPrimitiveType {
    #[serde(flatten)]
    pub base: GmlAbstractTimePrimitiveType,
    #[serde(rename = "frame")]
    pub frame: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractTimeObjectType {
    #[serde(flatten)]
    pub base: GmlAbstractGMLType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimePeriodPropertyType {
    #[serde(rename = "TimePeriod")]
    pub gml_time_period: GmlTimePeriod,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractTimeComplexType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeInstantType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeGeometricPrimitiveType,
    #[serde(rename = "timePosition")]
    pub gmltime_position: GmltimePosition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeIntervalLengthType {
    pub base: f64,
    #[serde(rename = "unit")]
    pub unit: TimeUnitType,
    #[serde(rename = "radix")]
    pub radix: u32,
    #[serde(rename = "factor")]
    pub factor: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimePrimitivePropertyType {
    #[serde(rename = "_TimePrimitive")]
    pub gml__time_primitive: Gml_TimePrimitive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimePositionUnion {
    #[serde(rename = "CalDate")]
    pub gml_cal_date: GmlCalDate,
    #[serde(rename = "time")]
    pub time: NaiveTime,
    #[serde(rename = "dateTime")]
    pub date_time: NaiveDateTime,
    #[serde(rename = "anyURI")]
    pub any_u_r_i: String,
    #[serde(rename = "decimal")]
    pub decimal: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalDate {
    #[serde(rename = "date")]
    pub date: NaiveDate,
    #[serde(rename = "gYearMonth")]
    pub g_year_month: NaiveDate,
    #[serde(rename = "gYear")]
    pub g_year: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeGeometricPrimitivePropertyType {
    #[serde(rename = "_TimeGeometricPrimitive")]
    pub gml__time_geometric_primitive: Gml_TimeGeometricPrimitive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeInstantPropertyType {
    #[serde(rename = "TimeInstant")]
    pub gml_time_instant: GmlTimeInstant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimePeriodType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeGeometricPrimitiveType,
    #[serde(rename = "beginPosition")]
    pub begin_position: TimePositionType,
    #[serde(rename = "begin")]
    pub begin: TimeInstantPropertyType,
    #[serde(rename = "endPosition")]
    pub end_position: TimePositionType,
    #[serde(rename = "end")]
    pub end: TimeInstantPropertyType,
    #[serde(rename = "timeLength", skip_serializing_if = "Option::is_none")]
    pub gmltime_length: Option<GmltimeLength>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedTimeType {
    #[serde(flatten)]
    pub base: GmlTimePrimitivePropertyType,
    #[serde(rename = "relativePosition")]
    pub relative_position: relativePosition,
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimePositionType {
    #[serde(flatten)]
    pub base: GmlTimePositionUnion,
    #[serde(rename = "frame")]
    pub frame: String,
    #[serde(rename = "calendarEraName")]
    pub calendar_era_name: String,
    #[serde(rename = "indeterminatePosition")]
    pub indeterminate_position: TimeIndeterminateValueType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractTimePrimitiveType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeObjectType,
    #[serde(rename = "relatedTime", skip_serializing_if = "Option::is_none")]
    pub related_time: Option<Vec<RelatedTimeType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeUnitType {
    pub base: String,
}

