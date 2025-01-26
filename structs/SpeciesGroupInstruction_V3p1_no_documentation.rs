use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesGroupInstructionType {
    #[serde(rename = "SpeciesGroupInstructionHeader")]
    pub species_group_instruction_header: MessageHeaderType,
    #[serde(rename = "SpeciesGroupDefinition")]
    pub species_group_definition: Vec<SpeciesGroupDefinitionSpeciesGroupInstructionType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesGroupDefinitionSpeciesGroupInstructionType {
    #[serde(flatten)]
    pub base: SpeciesGroupDefinitionWithDetailsType,
    #[serde(rename = "SpeciesDefinition", skip_serializing_if = "Option::is_none")]
    pub species_definition: Option<Vec<SpeciesDefinitionType>>,
}

