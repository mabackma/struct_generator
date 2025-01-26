use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct nextEdge {
    #[serde(flatten)]
    pub next_edge: GmlTimeEdgePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct primitive {
    #[serde(flatten)]
    pub primitive: GmlTimeTopologyPrimitivePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeNode {
    #[serde(flatten)]
    pub time_node: GmlTimeNodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeEdge {
    #[serde(flatten)]
    pub time_edge: GmlTimeEdgeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _TimeTopologyPrimitive {
    #[serde(flatten)]
    pub __time_topology_primitive: GmlAbstractTimeTopologyPrimitiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeTopologyComplex {
    #[serde(flatten)]
    pub time_topology_complex: GmlTimeTopologyComplexType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct position {
    #[serde(flatten)]
    pub position: GmlTimeInstantPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct previousEdge {
    #[serde(flatten)]
    pub previous_edge: GmlTimeEdgePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct complex {
    #[serde(flatten)]
    pub complex: GmlReferenceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTopologyComplexPropertyType {
    #[serde(rename = "TimeTopologyComplex")]
    pub gml_time_topology_complex: GmlTimeTopologyComplex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeNodePropertyType {
    #[serde(rename = "TimeNode")]
    pub gml_time_node: GmlTimeNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEdgePropertyType {
    #[serde(rename = "TimeEdge")]
    pub gml_time_edge: GmlTimeEdge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTopologyComplexType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeComplexType,
    #[serde(rename = "primitive")]
    pub primitive: Vec<TimeTopologyPrimitivePropertyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEdgeType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeTopologyPrimitiveType,
    #[serde(rename = "start")]
    pub start: TimeNodePropertyType,
    #[serde(rename = "end")]
    pub end: TimeNodePropertyType,
    #[serde(rename = "extent", skip_serializing_if = "Option::is_none")]
    pub extent: Option<TimePeriodPropertyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractTimeTopologyPrimitiveType {
    #[serde(flatten)]
    pub base: GmlAbstractTimePrimitiveType,
    #[serde(rename = "complex", skip_serializing_if = "Option::is_none")]
    pub complex: Option<ReferenceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessionType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeNodeType {
    #[serde(flatten)]
    pub base: GmlAbstractTimeTopologyPrimitiveType,
    #[serde(rename = "previousEdge", skip_serializing_if = "Option::is_none")]
    pub previous_edge: Option<Vec<TimeEdgePropertyType>>,
    #[serde(rename = "nextEdge", skip_serializing_if = "Option::is_none")]
    pub next_edge: Option<Vec<TimeEdgePropertyType>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<TimeInstantPropertyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTopologyPrimitivePropertyType {
    #[serde(rename = "_TimeTopologyPrimitive")]
    pub gml__time_topology_primitive: Gml_TimeTopologyPrimitive,
}

