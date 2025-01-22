use serde::{Serialize, Deserialize};
use chrono::*;

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceLocation {
    #[serde(flatten)]
    pub resource_location: ResourceLocationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationTimestamp {
    #[serde(flatten)]
    pub location_timestamp: BdtTimeStampType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceLocations {
    #[serde(flatten)]
    pub resource_locations: ResourceLocationsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBuyerResourceLocations {
    #[serde(flatten)]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: String20Type,
    #[serde(rename = "LocationTimestamp")]
    pub location_timestamp: TimeStampType,
    #[serde(rename = "Location")]
    pub location: PointGeometryType,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<ERPIdType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocationsType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ResourceLocations", skip_serializing_if = "Option::is_none")]
    pub resource_locations: Option<ResourceLocationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationsType {
    #[serde(rename = "ResourceLocation")]
    pub resource_location: Vec<ResourceLocationType>,
}

