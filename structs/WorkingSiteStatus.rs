#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteStatusType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: String20Type,
    #[serde(rename = "ContractorId")]
    pub contractor_id: String20Type,
    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String20Type>,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: ERPIdType,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WorkingSiteNumberType>,
    #[serde(rename = "ServiceType")]
    pub service_type: ServiceTypeType,
    #[serde(rename = "Status")]
    pub status: WorkingSiteStatusType,
}

