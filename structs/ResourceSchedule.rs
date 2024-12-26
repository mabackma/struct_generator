#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceScheduleType {
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "Resources")]
    pub resources: ResourcesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceDataType {
    #[serde(rename = "StanfordResourceId", skip_serializing_if = "Option::is_none")]
    pub stanford_resource_id: Option<BdtString100Type>,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ServiceBuyerResourceId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_resource_id: Option<BdtString20Type>,
    #[serde(rename = "ResourceType")]
    pub resource_type: BdtResourceTypeType,
    #[serde(rename = "WorkingSites")]
    pub working_sites: WorkingSitesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitesType {
    #[serde(rename = "WorkingSite", skip_serializing_if = "Option::is_none")]
    pub working_site: Option<Vec<WorkingSiteType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteType {
    #[serde(rename = "ServiceBuyerId", skip_serializing_if = "Option::is_none")]
    pub service_buyer_id: Option<BdtString20Type>,
    #[serde(rename = "WorkingSiteId", skip_serializing_if = "Option::is_none")]
    pub working_site_id: Option<WctERPIdType>,
    #[serde(rename = "WorkingSiteNumber", skip_serializing_if = "Option::is_none")]
    pub working_site_number: Option<WctWorkingSiteNumberType>,
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<BdtString50Type>,
    #[serde(rename = "StartDate")]
    pub start_date: BdtDateType,
    #[serde(rename = "EndDate")]
    pub end_date: BdtDateType,
    #[serde(rename = "ForwarderDelay", skip_serializing_if = "Option::is_none")]
    pub forwarder_delay: Option<BdtPositiveInteger2digitsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesType {
    #[serde(rename = "Resource")]
    pub resource: Vec<ResourceDataType>,
}

