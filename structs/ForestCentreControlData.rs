#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectDataType {
    #[serde(rename = "@id")]
    pub id: CoIdStringType,
    #[serde(rename = "CodAdditionalDetails", skip_serializing_if = "Option::is_none")]
    pub cod_additional_details: Option<String>,
    #[serde(rename = "Objects")]
    pub objects: ControlObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreControlDataType {
    #[serde(flatten)]
    pub base: FccForestCentreDataType,
    #[serde(rename = "ControlObjectData")]
    pub control_object_data: ControlObjectDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<CodForestCentreControlObjectType>,
}

