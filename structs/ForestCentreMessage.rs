use serde::{Serialize, Deserialize};
use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SenderEmail {
    #[serde(flatten)]
    pub sender_email: CiEmailAddressType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfMonitoringData {
    #[serde(flatten)]
    pub self_monitoring_data: CodForestCentreSelfMonitoringDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestCentreMessage {
    #[serde(flatten)]
    pub forest_centre_message: ForestCentreMessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestDataUpdate {
    #[serde(flatten)]
    pub forest_data_update: CodForestDataUpdateType,
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
    pub time_stamp: CoTimeStampType,
    #[serde(rename = "Message")]
    pub message: CoMessageType,
    #[serde(rename = "SenderEmail", skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<CiEmailAddressType>,
    #[serde(rename = "ForestUseDeclaration")]
    pub forest_use_declaration: FudForestUseDeclarationType,
}

