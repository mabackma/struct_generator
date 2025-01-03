#[derive(Debug, Serialize, Deserialize)]
pub struct ActingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibleActorType {
    #[serde(flatten)]
    pub base: CiContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationsType {
    #[serde(rename = "Operation")]
    pub operation: Vec<OperationDefType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainTypeType {
    #[serde(flatten)]
    pub base: CoMainTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationDefType {
    #[serde(rename = "@parentId")]
    pub parent_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@mainType")]
    pub main_type: MainTypeType,
    #[serde(rename = "CoTimeStamp")]
    pub co_time_stamp: TimeStamp,
    #[serde(rename = "OperationType")]
    pub operation_type: OperationTypeType,
    #[serde(rename = "OperationStatus")]
    pub operation_status: CoOperationStatusType,
    #[serde(rename = "ActingDate")]
    pub acting_date: ActingDateType,
    #[serde(rename = "ResponsibleActor", skip_serializing_if = "Option::is_none")]
    pub responsible_actor: Option<ResponsibleActorType>,
}

