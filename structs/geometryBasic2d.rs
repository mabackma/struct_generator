use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct _Ring {
    #[serde(flatten)]
    pub __ring: GmlAbstractRingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct exterior {
    #[serde(flatten)]
    pub exterior: GmlAbstractRingPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct polygonProperty {
    #[serde(flatten)]
    pub polygon_property: GmlPolygonPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct outerBoundaryIs {
    #[serde(flatten)]
    pub outer_boundary_is: GmlAbstractRingPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Polygon {
    #[serde(flatten)]
    pub polygon: GmlPolygonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct innerBoundaryIs {
    #[serde(flatten)]
    pub inner_boundary_is: GmlAbstractRingPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct surfaceArrayProperty {
    #[serde(flatten)]
    pub surface_array_property: GmlSurfaceArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _Surface {
    #[serde(flatten)]
    pub __surface: GmlAbstractSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinearRing {
    #[serde(flatten)]
    pub linear_ring: GmlLinearRingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct surfaceProperty {
    #[serde(flatten)]
    pub surface_property: GmlSurfacePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct interior {
    #[serde(flatten)]
    pub interior: GmlAbstractRingPropertyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceArrayPropertyType {
    #[serde(rename = "_Surface", skip_serializing_if = "Option::is_none")]
    pub gml__surface: Option<Vec<Gml_Surface>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractRingType {
    #[serde(flatten)]
    pub base: GmlAbstractGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractSurfaceType {
    #[serde(flatten)]
    pub base: GmlAbstractGeometricPrimitiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonType {
    #[serde(flatten)]
    pub base: GmlAbstractSurfaceType,
    #[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
    pub gmlexterior: Option<Gmlexterior>,
    #[serde(rename = "interior", skip_serializing_if = "Option::is_none")]
    pub gmlinterior: Option<Vec<Gmlinterior>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearRingPropertyType {
    #[serde(rename = "LinearRing")]
    pub gml_linear_ring: GmlLinearRing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonPropertyType {
    #[serde(rename = "Polygon")]
    pub gml_polygon: GmlPolygon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfacePropertyType {
    #[serde(rename = "_Surface")]
    pub gml__surface: Gml_Surface,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractRingPropertyType {
    #[serde(rename = "_Ring")]
    pub gml__ring: Gml_Ring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearRingType {
    #[serde(flatten)]
    pub base: GmlAbstractRingType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "coord")]
    pub gmlcoord: Vec<Gmlcoord>,
}

