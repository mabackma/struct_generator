use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectInstruction {
    #[serde(flatten)]
    pub object_instruction: ObjectInstructionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectInstructionHeader {
    #[serde(flatten)]
    pub object_instruction_header: MessageHeaderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterSectionDefinitionObjectInstructionType {
    #[serde(flatten)]
    pub base: DiameterSectionDefinitionType,
    #[serde(rename = "SpeciesGroupUserID")]
    pub species_group_user_i_d: UserIDType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubObjectDefinitionObjectInstructionType {
    #[serde(flatten)]
    pub base: SubObjectDefinitionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionMethodType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDefinitionObjectInstructionType {
    #[serde(flatten)]
    pub base: ObjectDefinitionType,
    #[serde(rename = "ProductUserID")]
    pub product_user_i_d: Vec<UserIDType>,
    #[serde(rename = "DiameterSectionUserID", skip_serializing_if = "Option::is_none")]
    pub diameter_section_user_i_d: Option<Vec<UserIDType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectInstructionType {
    #[serde(rename = "ObjectInstructionHeader")]
    pub object_instruction_header: MessageHeaderType,
    #[serde(rename = "ObjectDefinition")]
    pub object_definition: ObjectDefinitionObjectInstructionType,
    #[serde(rename = "DiameterSectionDefinition", skip_serializing_if = "Option::is_none")]
    pub diameter_section_definition: Option<Vec<DiameterSectionDefinitionObjectInstructionType>>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

