#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReferenceType {
    #[serde(flatten)]
    pub base: string,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationsType {
    #[serde(rename = "ForestUseDeclarationReference")]
    pub forest_use_declaration_reference: Vec<ForestUseDeclarationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermissionType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationType {
    #[serde(rename = "@parentId")]
    pub parent_id: string,
    #[serde(rename = "@id")]
    pub id: string,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "DeclarationReference")]
    pub declaration_reference: DeclarationReferenceType,
    #[serde(rename = "AcceptanceDate")]
    pub acceptance_date: AcceptanceDateType,
    #[serde(rename = "SpecialPermission")]
    pub special_permission: SpecialPermissionType,
    #[serde(rename = "AdditionalText", skip_serializing_if = "Option::is_none")]
    pub additional_text: Option<AdditionalTextType>,
    #[serde(rename = "OriginalXmlFile", skip_serializing_if = "Option::is_none")]
    pub original_xml_file: Option<base64Binary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalTextType {
    #[serde(flatten)]
    pub base: string,
}

