use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceType {
    #[serde(flatten)]
    pub resource_type: BdtResourceTypeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSites {
    #[serde(flatten)]
    pub working_sites: WorkingSitesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    #[serde(flatten)]
    pub resource: ResourceDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerResourceId {
    #[serde(flatten)]
    pub service_buyer_resource_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwarderDelay {
    #[serde(flatten)]
    pub forwarder_delay: BdtPositiveInteger2digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceSchedule {
    #[serde(flatten)]
    pub resource_schedule: ResourceScheduleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    #[serde(flatten)]
    pub resources: ResourcesType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WctWorkingSiteNumberType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StanfordResourceId {
    #[serde(flatten)]
    pub stanford_resource_id: BdtString100Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamName {
    #[serde(flatten)]
    pub team_name: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSite {
    #[serde(flatten)]
    pub working_site: WorkingSiteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesType {
    #[serde(rename = "Resource")]
    pub resource: Vec<ResourceDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceScheduleType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitesType {
    #[serde(rename = "WorkingSite", skip_serializing_if = "Option::is_none")]
    pub working_site: Option<Vec<WorkingSiteType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceDataType {
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<String100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<String20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceTypeType,
    #[serde(rename = "WorkingSites")]
    pub working_sites: WorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: DateType,
    #[serde(rename = "EndDate")]
    pub end_date: DateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<PositiveInteger2digitsType>,
}

