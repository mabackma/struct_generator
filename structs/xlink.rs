use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct arc {
    #[serde(flatten)]
    pub arc: arcType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct title {
    #[serde(flatten)]
    pub title: titleEltType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct locator {
    #[serde(flatten)]
    pub locator: locatorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct resource {
    #[serde(flatten)]
    pub resource: resourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcModel {
    #[serde(rename = "xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink:title: Option<Vec<xlink:title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct labelType {
    #[serde(rename = "label_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct typeType {
    #[serde(rename = "type_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct showType {
    #[serde(rename = "show_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extended {
    #[serde(rename = "XlinkextendedModel", skip_serializing_if = "Option::is_none")]
    pub xlinkextended_model: Option<Vec<extendedModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct roleType {
    #[serde(rename = "role_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct actuateType {
    #[serde(rename = "actuate_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resourceType {
    #[serde(rename = "XlinkresourceModel")]
    pub xlinkresource_model: resourceModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorType {
    #[serde(rename = "XlinklocatorModel")]
    pub xlinklocator_model: locatorModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcType {
    #[serde(rename = "XlinkarcModel")]
    pub xlinkarc_model: arcModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct simple {
    #[serde(rename = "XlinksimpleModel")]
    pub xlinksimple_model: simpleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fromType {
    #[serde(rename = "from_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleAttrType {
    #[serde(rename = "title_attr_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hrefType {
    #[serde(rename = "href_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleEltType {
    #[serde(rename = "XlinktitleModel")]
    pub xlinktitle_model: titleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcroleType {
    #[serde(rename = "arcrole_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extendedModel {
    #[serde(rename = "xlink:resource")]
    pub xlink:resource: xlink:resource,
    #[serde(rename = "xlink:title")]
    pub xlink:title: xlink:title,
    #[serde(rename = "xlink:arc")]
    pub xlink:arc: xlink:arc,
    #[serde(rename = "xlink:locator")]
    pub xlink:locator: xlink:locator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct toType {
    #[serde(rename = "to_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorModel {
    #[serde(rename = "xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink:title: Option<Vec<xlink:title>>,
}

