#[derive(Debug, Serialize, Deserialize)]
pub struct LoadType {
    #[serde(rename = "LoadNumber")]
    pub load_number: u32,
    #[serde(rename = "ForwardingDistance")]
    pub forwarding_distance: u32,
    #[serde(rename = "MeasurementDate")]
    pub measurement_date: BdtTimeStampType,
    #[serde(rename = "PartitialLoad")]
    pub partitial_load: Vec<PartitialLoadType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardedProductionType {
    #[serde(rename = "ServiceBuyerId")]
    pub service_buyer_id: BdtString20Type,
    #[serde(rename = "WorkingSiteId")]
    pub working_site_id: WctERPIdType,
    #[serde(rename = "StandId", skip_serializing_if = "Option::is_none")]
    pub stand_id: Option<BdtString20Type>,
    #[serde(rename = "StorageId", skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<WctERPIdType>,
    #[serde(rename = "ContractorId")]
    pub contractor_id: BdtString20Type,
    #[serde(rename = "ResourceId")]
    pub resource_id: BdtString20Type,
    #[serde(rename = "ForestOwner")]
    pub forest_owner: BdtString50Type,
    #[serde(rename = "StartDate")]
    pub start_date: BdtTimeStampType,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<BdtTimeStampType>,
    #[serde(rename = "SendDate")]
    pub send_date: BdtTimeStampType,
    #[serde(rename = "MachineApplicationVersion")]
    pub machine_application_version: BdtString100Type,
    #[serde(rename = "FileName")]
    pub file_name: BdtString100Type,
    #[serde(rename = "Bytes")]
    pub bytes: Vec<u8>,
    #[serde(rename = "LoadCount")]
    pub load_count: u32,
    #[serde(rename = "Load")]
    pub load: Vec<LoadType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitialLoadType {
    #[serde(rename = "PartitialLoadId")]
    pub partitial_load_id: u32,
    #[serde(rename = "Assortment")]
    pub assortment: BdtString50Type,
    #[serde(rename = "StorageId")]
    pub storage_id: WctERPIdType,
    #[serde(rename = "LoadVolume", skip_serializing_if = "Option::is_none")]
    pub load_volume: Option<BdtDecimal3FractionDigitsType>,
    #[serde(rename = "LoadGreenMass")]
    pub load_green_mass: BdtDecimal3FractionDigitsType,
}

