use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct using {
    #[serde(flatten)]
    pub using: GmlFeaturePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct target {
    #[serde(flatten)]
    pub target: GmlTargetPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct subject {
    #[serde(flatten)]
    pub subject: GmlTargetPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct resultOf {
    #[serde(flatten)]
    pub result_of: GmlAssociationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct distance {
    #[serde(flatten)]
    pub distance: GmlMeasureType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Observation {
    #[serde(flatten)]
    pub observation: GmlObservationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectedObservation {
    #[serde(flatten)]
    pub directed_observation: GmlDirectedObservationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectedObservationAtDistance {
    #[serde(flatten)]
    pub directed_observation_at_distance: GmlDirectedObservationAtDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationType {
    #[serde(flatten)]
    pub base: GmlAbstractFeatureType,
    #[serde(rename = "validTime")]
    pub gmlvalid_time: GmlvalidTime,
    #[serde(rename = "using", skip_serializing_if = "Option::is_none")]
    pub gmlusing: Option<Gmlusing>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub gmltarget: Option<Gmltarget>,
    #[serde(rename = "resultOf")]
    pub gmlresult_of: GmlresultOf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectedObservationAtDistanceType {
    #[serde(flatten)]
    pub base: GmlDirectedObservationType,
    #[serde(rename = "distance")]
    pub distance: MeasureType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectedObservationType {
    #[serde(flatten)]
    pub base: GmlObservationType,
    #[serde(rename = "direction")]
    pub gmldirection: Gmldirection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPropertyType {
    #[serde(rename = "_Feature")]
    pub gml__feature: Gml_Feature,
    #[serde(rename = "_Geometry")]
    pub gml__geometry: Gml_Geometry,
}

