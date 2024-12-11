#[derive(Debug, Serialize, Deserialize)]
pub struct SelfMonitoringData {
    #[serde(flatten)]
    pub self_monitoring_data: ForestCentreSelfMonitoringDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestDataUpdate {
    #[serde(flatten)]
    pub forest_data_update: ForestDataUpdateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub message: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooseDamageDeclaration {
    #[serde(flatten)]
    pub moose_damage_declaration: MooseDamageDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderEmail {
    #[serde(flatten)]
    pub sender_email: EmailAddressType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActApplication {
    #[serde(flatten)]
    pub financing_act_application: FinancingActApplicationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancingActCompletionDeclaration {
    #[serde(flatten)]
    pub financing_act_completion_declaration: FinancingActCompletionDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestUseDeclaration {
    #[serde(flatten)]
    pub forest_use_declaration: ForestUseDeclarationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestCentreMessage {
    #[serde(flatten)]
    pub forest_centre_message: ForestCentreMessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStamp {
    #[serde(flatten)]
    pub time_stamp: TimeStampType,
}

