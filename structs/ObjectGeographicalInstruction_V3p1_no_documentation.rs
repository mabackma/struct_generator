use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackingCategory {
    #[serde(flatten)]
    pub tracking_category: TrackingCategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectGeographicalInstructionHeader {
    #[serde(flatten)]
    pub object_geographical_instruction_header: MessageHeaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GISFileMAXScale {
    #[serde(flatten)]
    pub g_i_s_file_m_a_x_scale: XsdnonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlarmCoordinateDescription {
    #[serde(flatten)]
    pub alarm_coordinate_description: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectGeographicalInstruction {
    #[serde(flatten)]
    pub object_geographical_instruction: ObjectGeographicalInstructionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackingFile {
    #[serde(flatten)]
    pub tracking_file: Xsdboolean,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GISFileLayerDescription {
    #[serde(flatten)]
    pub g_i_s_file_layer_description: Xsdstring,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GISFileMINScale {
    #[serde(flatten)]
    pub g_i_s_file_m_i_n_scale: XsdnonNegativeInteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDefinitionObjectGeographicalInstructionType {
    #[serde(flatten)]
    pub base: ObjectDefinitionType,
    #[serde(rename = "GeographicDirective")]
    pub geographic_directive: GeographicDirectiveObjectGeographicalInstructionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeographicDirectiveObjectGeographicalInstructionType {
    #[serde(rename = "AlarmCoordinates", skip_serializing_if = "Option::is_none")]
    pub alarm_coordinates: Option<CoordinatesType>,
    #[serde(rename = "AlarmCoordinateDescription", skip_serializing_if = "Option::is_none")]
    pub alarm_coordinate_description: Option<String>,
    #[serde(rename = "TrackingCategory")]
    pub tracking_category: TrackingCategoryType,
    #[serde(rename = "GISCoordinateReferenceSystem")]
    pub g_i_s_coordinate_reference_system: String,
    #[serde(rename = "GISLayer")]
    pub g_i_s_layer: Vec<GISLayerObjectGeographicalInstructionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISLayerObjectGeographicalInstructionType {
    #[serde(flatten)]
    pub base: GISLayerType,
    #[serde(rename = "TrackingFile", skip_serializing_if = "Option::is_none")]
    pub tracking_file: Option<bool>,
    #[serde(rename = "GISFileLayerDescription")]
    pub g_i_s_file_layer_description: String,
    #[serde(rename = "GISFileMAXScale")]
    pub g_i_s_file_m_a_x_scale: u32,
    #[serde(rename = "GISFileMINScale")]
    pub g_i_s_file_m_i_n_scale: u32,
    #[serde(rename = "GISDatabaseFileSpecification", skip_serializing_if = "Option::is_none")]
    pub g_i_s_database_file_specification: Option<GISDatabaseFileSpecificationType>,
    #[serde(rename = "GISFileFormatPoint", skip_serializing_if = "Option::is_none")]
    pub g_i_s_file_format_point: Option<Vec<GISFileFormatPointType>>,
    #[serde(rename = "GISFileFormatLine", skip_serializing_if = "Option::is_none")]
    pub g_i_s_file_format_line: Option<Vec<GISFileFormatLineType>>,
    #[serde(rename = "GISFileFormatPolygon", skip_serializing_if = "Option::is_none")]
    pub g_i_s_file_format_polygon: Option<Vec<GISFileFormatPolygonType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeographicalInstructionType {
    #[serde(rename = "ObjectGeographicalInstructionHeader")]
    pub object_geographical_instruction_header: MessageHeaderType,
    #[serde(rename = "ObjectDefinition")]
    pub object_definition: ObjectDefinitionObjectGeographicalInstructionType,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

