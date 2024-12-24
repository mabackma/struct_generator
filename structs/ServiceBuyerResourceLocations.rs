#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerResourceLocationsType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "ResourceLocations", skip_serializing_if = "Option::is_none")]
    pub resource_locations: Option<ResourceLocationsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationsType {
    #[serde(rename = "ResourceLocation")]
    pub resource_location: Vec<ResourceLocationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocationType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "LocationTimestamp")]
    pub location_timestamp: BdtTimeStampType,
    #[serde(rename = "Location")]
    pub location: GdtPointGeometryType,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
}

