use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct Face {
    #[serde(flatten)]
    pub face: GmlFaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct topoVolumeProperty {
    #[serde(flatten)]
    pub topo_volume_property: GmlTopoVolumePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct directedTopoSolid {
    #[serde(flatten)]
    pub directed_topo_solid: GmlDirectedTopoSolidPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct topoPrimitiveMember {
    #[serde(flatten)]
    pub topo_primitive_member: GmlTopoPrimitiveMemberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct superComplex {
    #[serde(flatten)]
    pub super_complex: GmlTopoComplexMemberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct container {
    #[serde(flatten)]
    pub container: GmlContainerPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct directedNode {
    #[serde(flatten)]
    pub directed_node: GmlDirectedNodePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct topoCurveProperty {
    #[serde(flatten)]
    pub topo_curve_property: GmlTopoCurvePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct directedFace {
    #[serde(flatten)]
    pub directed_face: GmlDirectedFacePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct topoComplexProperty {
    #[serde(flatten)]
    pub topo_complex_property: GmlTopoComplexMemberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopoComplex {
    #[serde(flatten)]
    pub topo_complex: GmlTopoComplexType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct topoPointProperty {
    #[serde(flatten)]
    pub topo_point_property: GmlTopoPointPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct isolated {
    #[serde(flatten)]
    pub isolated: GmlIsolatedPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopoSolid {
    #[serde(flatten)]
    pub topo_solid: GmlTopoSolidType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct maximalComplex {
    #[serde(flatten)]
    pub maximal_complex: GmlTopoComplexMemberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct directedEdge {
    #[serde(flatten)]
    pub directed_edge: GmlDirectedEdgePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopoSurface {
    #[serde(flatten)]
    pub topo_surface: GmlTopoSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _Topology {
    #[serde(flatten)]
    pub __topology: GmlAbstractTopologyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _TopoPrimitive {
    #[serde(flatten)]
    pub __topo_primitive: GmlAbstractTopoPrimitiveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopoVolume {
    #[serde(flatten)]
    pub topo_volume: GmlTopoVolumeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopoCurve {
    #[serde(flatten)]
    pub topo_curve: GmlTopoCurveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(flatten)]
    pub node: GmlNodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct subComplex {
    #[serde(flatten)]
    pub sub_complex: GmlTopoComplexMemberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    #[serde(flatten)]
    pub edge: GmlEdgeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct topoPrimitiveMembers {
    #[serde(flatten)]
    pub topo_primitive_members: GmlTopoPrimitiveArrayAssociationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopoPoint {
    #[serde(flatten)]
    pub topo_point: GmlTopoPointType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct topoSurfaceProperty {
    #[serde(flatten)]
    pub topo_surface_property: GmlTopoSurfacePropertyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoVolumePropertyType {
    #[serde(rename = "TopoVolume")]
    pub gml_topo_volume: GmlTopoVolume,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoVolumeType {
    #[serde(flatten)]
    pub base: GmlAbstractTopologyType,
    #[serde(rename = "directedTopoSolid")]
    pub gmldirected_topo_solid: Vec<GmldirectedTopoSolid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoPrimitiveArrayAssociationType {
    #[serde(rename = "_TopoPrimitive")]
    pub gml__topo_primitive: Gml_TopoPrimitive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoPointPropertyType {
    #[serde(rename = "TopoPoint")]
    pub gml_topo_point: GmlTopoPoint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoSurfaceType {
    #[serde(flatten)]
    pub base: GmlAbstractTopologyType,
    #[serde(rename = "directedFace")]
    pub gmldirected_face: Vec<GmldirectedFace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoCurveType {
    #[serde(flatten)]
    pub base: GmlAbstractTopologyType,
    #[serde(rename = "directedEdge")]
    pub gmldirected_edge: Vec<GmldirectedEdge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectedEdgePropertyType {
    #[serde(rename = "Edge")]
    pub gml_edge: GmlEdge,
    #[serde(rename = "orientation")]
    pub orientation: SignType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoPointType {
    #[serde(flatten)]
    pub base: GmlAbstractTopologyType,
    #[serde(rename = "directedNode")]
    pub gmldirected_node: GmldirectedNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoComplexType {
    #[serde(flatten)]
    pub base: GmlAbstractTopologyType,
    #[serde(rename = "maximalComplex")]
    pub gmlmaximal_complex: GmlmaximalComplex,
    #[serde(rename = "superComplex", skip_serializing_if = "Option::is_none")]
    pub gmlsuper_complex: Option<Vec<GmlsuperComplex>>,
    #[serde(rename = "subComplex", skip_serializing_if = "Option::is_none")]
    pub gmlsub_complex: Option<Vec<GmlsubComplex>>,
    #[serde(rename = "topoPrimitiveMember", skip_serializing_if = "Option::is_none")]
    pub gmltopo_primitive_member: Option<Vec<GmltopoPrimitiveMember>>,
    #[serde(rename = "topoPrimitiveMembers", skip_serializing_if = "Option::is_none")]
    pub gmltopo_primitive_members: Option<GmltopoPrimitiveMembers>,
    #[serde(rename = "isMaximal")]
    pub is_maximal: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerPropertyType {
    #[serde(rename = "Face")]
    pub gml_face: GmlFace,
    #[serde(rename = "TopoSolid")]
    pub gml_topo_solid: GmlTopoSolid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoPrimitiveMemberType {
    #[serde(rename = "_TopoPrimitive", skip_serializing_if = "Option::is_none")]
    pub gml__topo_primitive: Option<Gml_TopoPrimitive>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeType {
    #[serde(flatten)]
    pub base: GmlAbstractTopoPrimitiveType,
    #[serde(rename = "directedNode")]
    pub gmldirected_node: Vec<GmldirectedNode>,
    #[serde(rename = "directedFace", skip_serializing_if = "Option::is_none")]
    pub gmldirected_face: Option<Vec<GmldirectedFace>>,
    #[serde(rename = "curveProperty", skip_serializing_if = "Option::is_none")]
    pub gmlcurve_property: Option<GmlcurveProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeType {
    #[serde(flatten)]
    pub base: GmlAbstractTopoPrimitiveType,
    #[serde(rename = "directedEdge", skip_serializing_if = "Option::is_none")]
    pub gmldirected_edge: Option<Vec<GmldirectedEdge>>,
    #[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
    pub gmlpoint_property: Option<GmlpointProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectedFacePropertyType {
    #[serde(rename = "Face")]
    pub gml_face: GmlFace,
    #[serde(rename = "orientation")]
    pub orientation: SignType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoSolidType {
    #[serde(flatten)]
    pub base: GmlAbstractTopoPrimitiveType,
    #[serde(rename = "directedFace")]
    pub gmldirected_face: Vec<GmldirectedFace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractTopologyType {
    #[serde(flatten)]
    pub base: GmlAbstractGMLType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractTopoPrimitiveType {
    #[serde(flatten)]
    pub base: GmlAbstractTopologyType,
    #[serde(rename = "isolated", skip_serializing_if = "Option::is_none")]
    pub gmlisolated: Option<Vec<Gmlisolated>>,
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub gmlcontainer: Option<Gmlcontainer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectedTopoSolidPropertyType {
    #[serde(rename = "TopoSolid")]
    pub gml_topo_solid: GmlTopoSolid,
    #[serde(rename = "orientation")]
    pub orientation: SignType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoSurfacePropertyType {
    #[serde(rename = "TopoSurface")]
    pub gml_topo_surface: GmlTopoSurface,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaceType {
    #[serde(flatten)]
    pub base: GmlAbstractTopoPrimitiveType,
    #[serde(rename = "directedEdge")]
    pub gmldirected_edge: Vec<GmldirectedEdge>,
    #[serde(rename = "directedTopoSolid", skip_serializing_if = "Option::is_none")]
    pub gmldirected_topo_solid: Option<Vec<GmldirectedTopoSolid>>,
    #[serde(rename = "surfaceProperty", skip_serializing_if = "Option::is_none")]
    pub gmlsurface_property: Option<GmlsurfaceProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectedNodePropertyType {
    #[serde(rename = "Node")]
    pub gml_node: GmlNode,
    #[serde(rename = "orientation")]
    pub orientation: SignType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoComplexMemberType {
    #[serde(rename = "TopoComplex", skip_serializing_if = "Option::is_none")]
    pub gml_topo_complex: Option<GmlTopoComplex>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsolatedPropertyType {
    #[serde(rename = "Node")]
    pub gml_node: GmlNode,
    #[serde(rename = "Edge")]
    pub gml_edge: GmlEdge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoCurvePropertyType {
    #[serde(rename = "TopoCurve")]
    pub gml_topo_curve: GmlTopoCurve,
}

