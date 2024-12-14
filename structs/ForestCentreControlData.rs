#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<ForestCentreControlObjectType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectDataType {
    #[serde(rename = "@id")]
    pub id: IdStringType,
    #[serde(rename = "AdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String2000Type>,
    #[serde(rename = "Objects")]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlDataType {
    #[serde(flatten)]
    pub base: ForestCentreDataType,
    #[serde(rename = "ControlObjectData")]
    pub control_object_data: ControlObjectDataType,
}
