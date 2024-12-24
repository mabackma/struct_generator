#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessage {
    #[serde(flatten)]
    pub forest_centre_message: ForestCentreMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringData {
    #[serde(flatten)]
    pub self_monitoring_data: CodForestCentreSelfMonitoringDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderEmail {
    #[serde(flatten)]
    pub sender_email: CiEmailAddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdate {
    #[serde(flatten)]
    pub forest_data_update: CodForestDataUpdateType,
}

