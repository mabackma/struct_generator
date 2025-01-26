use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct crsRef {
    #[serde(flatten)]
    pub crs_ref: GmlCRSRefType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct temporalExtent {
    #[serde(flatten)]
    pub temporal_extent: GmlTimePeriodType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct validArea {
    #[serde(flatten)]
    pub valid_area: GmlExtentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct srsName {
    #[serde(flatten)]
    pub srs_name: GmlCodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _ReferenceSystem {
    #[serde(flatten)]
    pub __reference_system: GmlAbstractReferenceSystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct boundingBox {
    #[serde(flatten)]
    pub bounding_box: GmlEnvelopeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct remarks {
    #[serde(flatten)]
    pub remarks: GmlStringOrRefType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _CRS {
    #[serde(flatten)]
    pub __c_r_s: GmlAbstractReferenceSystemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct version {
    #[serde(flatten)]
    pub version: string,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct scope {
    #[serde(flatten)]
    pub scope: string,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct srsID {
    #[serde(flatten)]
    pub srs_i_d: GmlIdentifierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct verticalExtent {
    #[serde(flatten)]
    pub vertical_extent: GmlEnvelopeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct boundingPolygon {
    #[serde(flatten)]
    pub bounding_polygon: GmlPolygonType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct referenceSystemRef {
    #[serde(flatten)]
    pub reference_system_ref: GmlReferenceSystemRefType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CRSRefType {
    #[serde(rename = "_CRS")]
    pub gml__c_r_s: Gml_CRS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceSystemRefType {
    #[serde(rename = "_ReferenceSystem")]
    pub gml__reference_system: Gml_ReferenceSystem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractReferenceSystemType {
    #[serde(flatten)]
    pub base: GmlAbstractReferenceSystemBaseType,
    #[serde(rename = "srsID", skip_serializing_if = "Option::is_none")]
    pub gmlsrs_i_d: Option<Vec<GmlsrsID>>,
    #[serde(rename = "remarks", skip_serializing_if = "Option::is_none")]
    pub gmlremarks: Option<Gmlremarks>,
    #[serde(rename = "validArea", skip_serializing_if = "Option::is_none")]
    pub gmlvalid_area: Option<GmlvalidArea>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub gmlscope: Option<Gmlscope>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(rename = "name")]
    pub gmlname: Gmlname,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub gmlversion: Option<Gmlversion>,
    #[serde(rename = "remarks", skip_serializing_if = "Option::is_none")]
    pub gmlremarks: Option<Gmlremarks>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtentType {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub gmldescription: Option<Gmldescription>,
    #[serde(rename = "boundingBox", skip_serializing_if = "Option::is_none")]
    pub gmlbounding_box: Option<Vec<GmlboundingBox>>,
    #[serde(rename = "boundingPolygon", skip_serializing_if = "Option::is_none")]
    pub gmlbounding_polygon: Option<Vec<GmlboundingPolygon>>,
    #[serde(rename = "verticalExtent", skip_serializing_if = "Option::is_none")]
    pub gmlvertical_extent: Option<Vec<GmlverticalExtent>>,
    #[serde(rename = "temporalExtent", skip_serializing_if = "Option::is_none")]
    pub gmltemporal_extent: Option<Vec<GmltemporalExtent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractReferenceSystemBaseType {
    #[serde(flatten)]
    pub base: GmlDefinitionType,
    #[serde(rename = "metaDataProperty", skip_serializing_if = "Option::is_none")]
    pub gmlmeta_data_property: Option<Vec<GmlmetaDataProperty>>,
    #[serde(rename = "srsName")]
    pub gmlsrs_name: GmlsrsName,
    #[serde(rename = "id")]
    pub gmlid: Gmlid,
}

