use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsibleActor {
    #[serde(flatten)]
    pub responsible_actor: ResponsibleActorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActingDate {
    #[serde(flatten)]
    pub acting_date: ActingDateType,
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
    #[serde(rename = "TimeStamp")]
    pub co_time_stamp: CoTimeStamp,
    #[serde(rename = "OperationType")]
    pub operation_type: OperationTypeType,
    #[serde(rename = "OperationStatus")]
    pub operation_status: CoOperationStatusType,
    #[serde(rename = "ActingDate")]
    pub acting_date: ActingDateType,
    #[serde(rename = "ResponsibleActor", skip_serializing_if = "Option::is_none")]
    pub responsible_actor: Option<ResponsibleActorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationTypeType {
    #[serde(rename = "CuttingTypeType")]
    pub co_cutting_type_type: CoCuttingTypeType,
    #[serde(rename = "SilvicultureTypeType")]
    pub co_silviculture_type_type: CoSilvicultureTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActingDateType {
    #[serde(flatten)]
    pub base: CoDateType,
}

