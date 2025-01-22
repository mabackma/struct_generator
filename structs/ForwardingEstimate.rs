use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    #[serde(flatten)]
    pub day: BdtDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardingEstimate {
    #[serde(flatten)]
    pub forwarding_estimate: ForwardingEstimateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssortmentId {
    #[serde(flatten)]
    pub assortment_id: WctERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Loads {
    #[serde(flatten)]
    pub loads: BdtPositiveInteger3digitsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: BdtString20Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: WctERPIdType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    #[serde(flatten)]
    pub code: BdtString50Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartTime {
    #[serde(flatten)]
    pub start_time: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: BdtWorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEstimateType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "StorageId")]
    pub storage_id: ERPIdType,
    #[serde(rename = "StartTime")]
    pub start_time: TimeStampType,
    #[serde(rename = "Assortments")]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentsType {
    #[serde(rename = "Assortment")]
    pub assortment: Vec<AssortmentDataType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentDataType {
    #[serde(rename = "AssortmentId", skip_serializing_if = "Option::is_none")]
    pub assortment_id: Option<ERPIdType>,
    #[serde(rename = "DestinationStorage")]
    pub destination_storage: String20Type,
    #[serde(rename = "Code")]
    pub code: String50Type,
    #[serde(rename = "Volume")]
    pub volume: PositiveInteger4digitsType,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<WorkCodeUnitType>,
    #[serde(rename = "Loads")]
    pub loads: PositiveInteger3digitsType,
    #[serde(rename = "Day")]
    pub day: DateType,
}

