#[derive(Debug, Serialize, Deserialize)]
pub struct hrefType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct typeType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcModel {
    #[serde(rename = "xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink:title: Option<Vec<xlink:title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fromType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct toType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resourceType {
    #[serde(rename = "XlinkresourceModel")]
    pub xlinkresource_model: resourceModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct roleType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct labelType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct simple {
    #[serde(rename = "XlinksimpleModel")]
    pub xlinksimple_model: simpleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extendedModel {
    #[serde(rename = "xlink:locator")]
    pub xlink:locator: xlink:locator,
    #[serde(rename = "xlink:arc")]
    pub xlink:arc: xlink:arc,
    #[serde(rename = "xlink:resource")]
    pub xlink:resource: xlink:resource,
    #[serde(rename = "xlink:title")]
    pub xlink:title: xlink:title,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleAttrType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct actuateType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct showType {
    #[serde(rename = "base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorType {
    #[serde(rename = "XlinklocatorModel")]
    pub xlinklocator_model: locatorModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extended {
    #[serde(rename = "XlinkextendedModel", skip_serializing_if = "Option::is_none")]
    pub xlinkextended_model: Option<Vec<extendedModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorModel {
    #[serde(rename = "xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink:title: Option<Vec<xlink:title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleEltType {
    #[serde(rename = "XlinktitleModel")]
    pub xlinktitle_model: titleModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcType {
    #[serde(rename = "XlinkarcModel")]
    pub xlinkarc_model: arcModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcroleType {
    #[serde(rename = "base")]
    pub base: String,
}

