use serde::{Serialize, Deserialize};
use crate::custom_deserializers::{deserialize_point, deserialize_polygon, deserialize_optional_point, deserialize_optional_polygon, deserialize_multipolygon};
use geo::{Point, Polygon, MultiPolygon};
use chrono;

#[derive(Serialize, Deserialize, Debug)]
pub struct SenderEmail {
    #[serde(flatten)]
    pub sender_email: EmailAddressType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringData {
    #[serde(flatten)]
    pub self_monitoring_data: ForestCentreSelfMonitoringDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestDataUpdate {
    #[serde(flatten)]
    pub forest_data_update: ForestDataUpdateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreMessage {
    #[serde(flatten)]
    pub forest_centre_message: ForestCentreMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessageType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@schemaPackageVersion")]
    pub schema_package_version: ForestDataStandardSchemaPackageVersionType,
    #[serde(rename = "@schemaPackageSubversion")]
    pub schema_package_subversion: ForestDataStandardSchemaPackageSubversionType,
    #[serde(rename = "@schemaPackageVersionDate")]
    pub schema_package_version_date: DateType,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: TimeStampType,
    #[serde(rename = "Message")]
    pub message: MessageType,
    #[serde(rename = "SenderEmail", skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<EmailAddressType>,
    #[serde(rename = "ForestUseDeclaration")]
    pub forest_use_declaration: ForestUseDeclarationType,
}

