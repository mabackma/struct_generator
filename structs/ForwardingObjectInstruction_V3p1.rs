use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationDefinitionType {
    #[serde(flatten)]
    pub base: CommonLocationDefinitionType,
    #[serde(rename = "ModificationDate")]
    pub modification_date: ModificationDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDefinitionForwardingObjectInstructionType {
    #[serde(flatten)]
    pub base: ObjectDefinitionType,
    #[serde(rename = "DeliveryUserID")]
    pub delivery_user_i_d: Vec<UserIDType>,
    #[serde(rename = "LocationUserID")]
    pub location_user_i_d: Vec<UserIDType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingObjectInstructionType {
    #[serde(rename = "ForwardingObjectInstructionHeader")]
    pub forwarding_object_instruction_header: MessageHeaderType,
    #[serde(rename = "ObjectDefinition")]
    pub object_definition: ObjectDefinitionForwardingObjectInstructionType,
    #[serde(rename = "LocationDefinition")]
    pub location_definition: Vec<LocationDefinitionType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

