use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDefinitionObjectGeographicalReportType {
    #[serde(flatten)]
    pub base: ObjectDefinitionMachineType,
    #[serde(rename = "TextToMachine", skip_serializing_if = "Option::is_none")]
    pub text_to_machine: Option<String>,
    #[serde(rename = "GeographicDirective")]
    pub geographic_directive: GeographicDirectiveObjectGeographicalReportType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISFileModifiedType {
    #[serde(flatten)]
    pub base: Xsdstring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeographicDirectiveObjectGeographicalReportType {
    #[serde(rename = "AlarmCoordinates", skip_serializing_if = "Option::is_none")]
    pub alarm_coordinates: Option<CoordinatesType>,
    #[serde(rename = "GISCoordinateReferenceSystem")]
    pub g_i_s_coordinate_reference_system: String,
    #[serde(rename = "GISLayer")]
    pub g_i_s_layer: Vec<GISLayerObjectGeographicalReportType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GISLayerObjectGeographicalReportType {
    #[serde(flatten)]
    pub base: GISLayerType,
    #[serde(rename = "GISFileModificationDate")]
    pub g_i_s_file_modification_date: ModificationDateType,
    #[serde(rename = "GISFileModified")]
    pub g_i_s_file_modified: GISFileModifiedType,
    #[serde(rename = "TrackingFileCategory")]
    pub tracking_file_category: TrackingCategoryType,
    #[serde(rename = "GISDatabaseFileSpecification")]
    pub g_i_s_database_file_specification: GISDatabaseFileSpecificationType,
    #[serde(rename = "GISFileFormatPoint", skip_serializing_if = "Option::is_none")]
    pub g_i_s_file_format_point: Option<Vec<GISFileFormatPointType>>,
    #[serde(rename = "GISFileFormatLine", skip_serializing_if = "Option::is_none")]
    pub g_i_s_file_format_line: Option<Vec<GISFileFormatLineType>>,
    #[serde(rename = "GISFileFormatPolygon", skip_serializing_if = "Option::is_none")]
    pub g_i_s_file_format_polygon: Option<Vec<GISFileFormatPolygonType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineObjectGeographicalReportType {
    #[serde(flatten)]
    pub base: MachineWithHeadType,
    #[serde(rename = "ObjectDefinition")]
    pub object_definition: Vec<ObjectDefinitionObjectGeographicalReportType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGeographicalReportType {
    #[serde(rename = "ObjectGeographicalReportHeader")]
    pub object_geographical_report_header: MessageHeaderType,
    #[serde(rename = "Machine")]
    pub machine: Vec<MachineObjectGeographicalReportType>,
    #[serde(rename = "Extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<ExtensionType>,
    #[serde(rename = "messageType")]
    pub message_type: MessageCategoryType,
}

