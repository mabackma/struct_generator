use serde::{Serialize, Deserialize};
use chrono;
use geo::{Point, Polygon, MultiPolygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationReference {
    #[serde(flatten)]
    pub forest_use_declaration_reference: ForestUseDeclarationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptanceDate {
    #[serde(flatten)]
    pub acceptance_date: AcceptanceDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationReference {
    #[serde(flatten)]
    pub declaration_reference: DeclarationReferenceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalText {
    #[serde(flatten)]
    pub additional_text: AdditionalTextType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationReferences {
    #[serde(flatten)]
    pub forest_use_declaration_references: ForestUseDeclarationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialPermission {
    #[serde(flatten)]
    pub special_permission: SpecialPermissionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalXmlFile {
    #[serde(flatten)]
    pub original_xml_file: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "DeclarationReference")]
    pub declaration_reference: DeclarationReferenceType,
    #[serde(rename = "AcceptanceDate")]
    pub acceptance_date: AcceptanceDateType,
    #[serde(rename = "SpecialPermission")]
    pub special_permission: SpecialPermissionType,
    #[serde(rename = "AdditionalText", skip_serializing_if = "Option::is_none")]
    pub additional_text: Option<AdditionalTextType>,
    #[serde(rename = "OriginalXmlFile", skip_serializing_if = "Option::is_none")]
    pub original_xml_file: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationsType {
    #[serde(rename = "ForestUseDeclarationReference")]
    pub forest_use_declaration_reference: Vec<ForestUseDeclarationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDateType {
    #[serde(flatten)]
    pub base: DateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermissionType {
    #[serde(flatten)]
    pub base: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalTextType {
    #[serde(rename = "additional_text_type.base")]
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationReferenceType {
    #[serde(rename = "declaration_reference_type.base")]
    pub base: String,
}

