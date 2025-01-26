use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct element {
    #[serde(flatten)]
    pub element: GmlGeometricPrimitivePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositeSurface {
    #[serde(flatten)]
    pub composite_surface: GmlCompositeSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometricComplex {
    #[serde(flatten)]
    pub geometric_complex: GmlGeometricComplexType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositeSolid {
    #[serde(flatten)]
    pub composite_solid: GmlCompositeSolidType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositeCurve {
    #[serde(flatten)]
    pub composite_curve: GmlCompositeCurveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeCurvePropertyType {
    #[serde(rename = "CompositeCurve")]
    pub gml_composite_curve: GmlCompositeCurve,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeCurveType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveType,
    #[serde(rename = "curveMember")]
    pub gmlcurve_member: Vec<GmlcurveMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometricComplexType {
    #[serde(flatten)]
    pub base: GmlAbstractGeometryType,
    #[serde(rename = "element")]
    pub element: Vec<GeometricPrimitivePropertyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometricComplexPropertyType {
    #[serde(rename = "GeometricComplex")]
    pub gml_geometric_complex: GmlGeometricComplex,
    #[serde(rename = "CompositeCurve")]
    pub gml_composite_curve: GmlCompositeCurve,
    #[serde(rename = "CompositeSurface")]
    pub gml_composite_surface: GmlCompositeSurface,
    #[serde(rename = "CompositeSolid")]
    pub gml_composite_solid: GmlCompositeSolid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeSurfacePropertyType {
    #[serde(rename = "CompositeSurface")]
    pub gml_composite_surface: GmlCompositeSurface,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeSurfaceType {
    #[serde(flatten)]
    pub base: GmlAbstractSurfaceType,
    #[serde(rename = "surfaceMember")]
    pub gmlsurface_member: Vec<GmlsurfaceMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeSolidType {
    #[serde(flatten)]
    pub base: GmlAbstractSolidType,
    #[serde(rename = "solidMember")]
    pub gmlsolid_member: Vec<GmlsolidMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeSolidPropertyType {
    #[serde(rename = "CompositeSolid")]
    pub gml_composite_solid: GmlCompositeSolid,
}

