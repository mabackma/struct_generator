#[derive(Debug, Serialize, Deserialize)]
pub struct typeType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcroleType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct actuateType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct fromType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extended {
    #[serde(rename = "extendedModel", skip_serializing_if = "Option::is_none")]
    pub extended_model: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct roleType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resourceModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcModel {
    #[serde(rename = "xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink:title: Option<Vec<xlink:title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct hrefType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct toType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleAttrType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorModel {
    #[serde(rename = "xlink:title", skip_serializing_if = "Option::is_none")]
    pub xlink:title: Option<Vec<xlink:title>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleEltType {
    #[serde(rename = "titleModel")]
    pub title_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct simpleModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct extendedModel {
    #[serde(rename = "xlink:title")]
    pub xlink:title: xlink:title,
    #[serde(rename = "xlink:arc")]
    pub xlink:arc: xlink:arc,
    #[serde(rename = "xlink:resource")]
    pub xlink:resource: xlink:resource,
    #[serde(rename = "xlink:locator")]
    pub xlink:locator: xlink:locator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct locatorType {
    #[serde(rename = "locatorModel")]
    pub locator_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct resourceType {
    #[serde(rename = "resourceModel")]
    pub resource_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct simple {
    #[serde(rename = "simpleModel")]
    pub simple_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct showType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct titleModel {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct labelType {
    #[serde(flatten)]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct arcType {
    #[serde(rename = "arcModel")]
    pub arc_model: String,
}
