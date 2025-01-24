use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalXmlFile {
    #[serde(flatten)]
    pub original_xml_file: Xmimebase64Binary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestUseDeclarationReferences {
    #[serde(flatten)]
    pub forest_use_declaration_references: ForestUseDeclarationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptanceDate {
    #[serde(flatten)]
    pub acceptance_date: AcceptanceDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalText {
    #[serde(flatten)]
    pub additional_text: AdditionalTextType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalTextType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: CoTimeStamp,
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
pub struct DeclarationReferenceType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclarationsType {
    #[serde(rename = "ForestUseDeclarationReference")]
    pub forest_use_declaration_reference: Vec<ForestUseDeclarationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPermissionType {
    #[serde(flatten)]
    pub base: CoYesNoType,
}

