use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct referenceDate {
    #[serde(flatten)]
    pub reference_date: date,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct start {
    #[serde(flatten)]
    pub start: GmlTimeNodePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct referenceTime {
    #[serde(flatten)]
    pub reference_time: time,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeCalendarEra {
    #[serde(flatten)]
    pub time_calendar_era: GmlTimeCalendarEraType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct member {
    #[serde(flatten)]
    pub member: GmlTimeOrdinalEraPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _TimeReferenceSystem {
    #[serde(flatten)]
    pub __time_reference_system: GmlAbstractTimeReferenceSystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct julianReference {
    #[serde(flatten)]
    pub julian_reference: decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeCoordinateSystem {
    #[serde(flatten)]
    pub time_coordinate_system: GmlTimeCoordinateSystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeClock {
    #[serde(flatten)]
    pub time_clock: GmlTimeClockType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct group {
    #[serde(flatten)]
    pub group: GmlReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct extent {
    #[serde(flatten)]
    pub extent: GmlTimePeriodPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct component {
    #[serde(flatten)]
    pub component: GmlTimeOrdinalEraPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeOrdinalEra {
    #[serde(flatten)]
    pub time_ordinal_era: GmlTimeOrdinalEraType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeOrdinalReferenceSystem {
    #[serde(flatten)]
    pub time_ordinal_reference_system: GmlTimeOrdinalReferenceSystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct referenceFrame {
    #[serde(flatten)]
    pub reference_frame: GmlTimeCalendarEraPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct epochOfUse {
    #[serde(flatten)]
    pub epoch_of_use: GmlTimePeriodPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct origin {
    #[serde(flatten)]
    pub origin: GmlTimeInstantPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct referenceEvent {
    #[serde(flatten)]
    pub reference_event: GmlStringOrRefType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct dateBasis {
    #[serde(flatten)]
    pub date_basis: GmlTimeCalendarPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct utcReference {
    #[serde(flatten)]
    pub utc_reference: time,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct originPosition {
    #[serde(flatten)]
    pub origin_position: GmlTimePositionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeCalendar {
    #[serde(flatten)]
    pub time_calendar: GmlTimeCalendarType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct interval {
    #[serde(flatten)]
    pub interval: GmlTimeIntervalLengthType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct domainOfValidity {
    #[serde(flatten)]
    pub domain_of_validity: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeClockType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeReferenceSystemType,
    #[serde(rename = "referenceEvent")]
    pub reference_event: StringOrRefType,
    #[serde(rename = "referenceTime")]
    pub reference_time: NaiveTime,
    #[serde(rename = "utcReference")]
    pub utc_reference: NaiveTime,
    #[serde(rename = "dateBasis", skip_serializing_if = "Option::is_none")]
    pub date_basis: Option<Vec<TimeCalendarPropertyType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeCalendarType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeReferenceSystemType,
    #[serde(rename = "referenceFrame")]
    pub reference_frame: Vec<TimeCalendarEraPropertyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeCoordinateSystemType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeReferenceSystemType,
    #[serde(rename = "originPosition")]
    pub origin_position: TimePositionType,
    #[serde(rename = "origin")]
    pub origin: TimeInstantPropertyType,
    #[serde(rename = "interval")]
    pub interval: TimeIntervalLengthType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractTimeReferenceSystemType {
    #[serde(flatten)]
    pub base: GmlDefinitionType,
    #[serde(rename = "domainOfValidity", skip_serializing_if = "Option::is_none")]
    pub domain_of_validity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeOrdinalEraPropertyType {
    #[serde(rename = "TimeOrdinalEra")]
    pub gml_time_ordinal_era: GmlTimeOrdinalEra,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeCalendarEraPropertyType {
    #[serde(rename = "TimeCalendarEra")]
    pub gml_time_calendar_era: GmlTimeCalendarEra,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeCalendarPropertyType {
    #[serde(rename = "TimeCalendar")]
    pub gml_time_calendar: GmlTimeCalendar,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeClockPropertyType {
    #[serde(rename = "TimeClock")]
    pub gml_time_clock: GmlTimeClock,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeOrdinalReferenceSystemType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeReferenceSystemType,
    #[serde(rename = "component")]
    pub component: Vec<TimeOrdinalEraPropertyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeOrdinalEraType {
    #[serde(flatten)]
    pub base: GmlDefinitionType,
    #[serde(rename = "relatedTime", skip_serializing_if = "Option::is_none")]
    pub related_time: Option<Vec<RelatedTimeType>>,
    #[serde(rename = "start")]
    pub start: TimeNodePropertyType,
    #[serde(rename = "end")]
    pub end: TimeNodePropertyType,
    #[serde(rename = "extent", skip_serializing_if = "Option::is_none")]
    pub extent: Option<TimePeriodPropertyType>,
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub member: Option<Vec<TimeOrdinalEraPropertyType>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<ReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeCalendarEraType {
    #[serde(flatten)]
    pub base: GmlDefinitionType,
    #[serde(rename = "referenceEvent")]
    pub reference_event: StringOrRefType,
    #[serde(rename = "referenceDate", skip_serializing_if = "Option::is_none")]
    pub reference_date: Option<NaiveDate>,
    #[serde(rename = "julianReference")]
    pub julian_reference: f64,
    #[serde(rename = "epochOfUse")]
    pub epoch_of_use: TimePeriodPropertyType,
}

