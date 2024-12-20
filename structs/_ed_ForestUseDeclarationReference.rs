#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationReferences {
    #[serde(flatten)]
    pub forest_use_declaration_references: ForestUseDeclarationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDate {
    #[serde(flatten)]
    pub acceptance_date: AcceptanceDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalXmlFile {
    #[serde(flatten)]
    pub original_xml_file: Xmimebase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalText {
    #[serde(flatten)]
    pub additional_text: AdditionalTextType,
}

