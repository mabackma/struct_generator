use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericMetaData {
    #[serde(flatten)]
    pub generic_meta_data: GmlGenericMetaDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Array {
    #[serde(flatten)]
    pub array: GmlArrayType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct description {
    #[serde(flatten)]
    pub description: GmlStringOrRefType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _strictAssociation {
    #[serde(flatten)]
    pub _strict_association: GmlAssociationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct metaDataProperty {
    #[serde(flatten)]
    pub meta_data_property: GmlMetaDataPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _Object {
    #[serde(flatten)]
    pub __object: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _MetaData {
    #[serde(flatten)]
    pub __meta_data: GmlAbstractMetaDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _reference {
    #[serde(flatten)]
    pub _reference: GmlReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _association {
    #[serde(flatten)]
    pub _association: GmlAssociationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct members {
    #[serde(flatten)]
    pub members: GmlArrayAssociationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct name {
    #[serde(flatten)]
    pub name: GmlCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _GML {
    #[serde(flatten)]
    pub __g_m_l: GmlAbstractGMLType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bag {
    #[serde(flatten)]
    pub bag: GmlBagType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayType {
    #[serde(flatten)]
    pub base: GmlAbstractGMLType,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub gmlmembers: Option<Gmlmembers>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayAssociationType {
    #[serde(rename = "_Object", skip_serializing_if = "Option::is_none")]
    pub gml__object: Option<Vec<Gml_Object>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractGMLType {
    #[serde(rename = "StandardObjectProperties")]
    pub gml_standard_object_properties: GmlStandardObjectProperties,
    #[serde(rename = "id")]
    pub gmlid: Gmlid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationType {
    #[serde(rename = "_Object")]
    pub gml__object: Gml_Object,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BagType {
    #[serde(flatten)]
    pub base: GmlAbstractGMLType,
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub gmlmember: Option<Vec<Gmlmember>>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub gmlmembers: Option<Gmlmembers>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericMetaDataType {
    #[serde(flatten)]
    pub base: GmlAbstractMetaDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDataPropertyType {
    #[serde(rename = "about")]
    pub about: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractMetaDataType {
    #[serde(rename = "id")]
    pub gmlid: Gmlid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringOrRefType {
    pub base: String,
}

