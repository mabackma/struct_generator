#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Description")]
    pub description: CoString2000Type,
    #[serde(rename = "Sender")]
    pub sender: CiContactInformationType,
    #[serde(rename = "Objects")]
    pub objects: ForestObjectDataObjectsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestObjectDataObjectsType {
    #[serde(rename = "Object")]
    pub object: Vec<CodForestObjectDataObjectType>,
}

