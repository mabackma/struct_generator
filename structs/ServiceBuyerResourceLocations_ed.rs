#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocations {
    #[serde(flatten)]
    pub resource_locations: ResourceLocationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationTimestamp {
    #[serde(flatten)]
    pub location_timestamp: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocation {
    #[serde(flatten)]
    pub resource_location: ResourceLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(flatten)]
    pub location: PointGeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocations {
    #[serde(flatten)]
    pub service_buyer_resource_locations: ServiceBuyerResourceLocationsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

