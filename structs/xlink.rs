use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct title {
    #[serde(flatten)]
    pub title: XlinktitleEltType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct locator {
    #[serde(flatten)]
    pub locator: XlinklocatorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct resource {
    #[serde(flatten)]
    pub resource: XlinkresourceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct arc {
    #[serde(flatten)]
    pub arc: XlinkarcType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcroleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcModel {
    #[serde(rename = "xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink:title: Option<Vec<xlink:title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorType {
    #[serde(rename = "locatorModel")]
    pub xlinklocator_model: XlinklocatorModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct actuateType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct roleType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resourceType {
    #[serde(rename = "resourceModel")]
    pub xlinkresource_model: XlinkresourceModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorModel {
    #[serde(rename = "xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink:title: Option<Vec<xlink:title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct showType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fromType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct simple {
    #[serde(rename = "simpleModel")]
    pub xlinksimple_model: XlinksimpleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct toType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct typeType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleAttrType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hrefType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleEltType {
    #[serde(rename = "titleModel")]
    pub xlinktitle_model: XlinktitleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extendedModel {
    #[serde(rename = "xlink:title")]
    pub xlink:title: xlink:title,
    #[serde(rename = "xlink:resource")]
    pub xlink:resource: xlink:resource,
    #[serde(rename = "xlink:locator")]
    pub xlink:locator: xlink:locator,
    #[serde(rename = "xlink:arc")]
    pub xlink:arc: xlink:arc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extended {
    #[serde(rename = "extendedModel", skip_serializing_if = "Option::is_none")]
    pub xlinkextended_model: Option<Vec<XlinkextendedModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcType {
    #[serde(rename = "arcModel")]
    pub xlinkarc_model: XlinkarcModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct labelType {
    pub base: String,
}

