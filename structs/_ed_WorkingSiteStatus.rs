#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatus {
    #[serde(flatten)]
    pub working_site_status: WorkingSiteStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(flatten)]
    pub status: BdtWorkingSiteStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteNumber {
    #[serde(flatten)]
    pub working_site_number: WctWorkingSiteNumberType,
}

