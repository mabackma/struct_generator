#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementDate {
    #[serde(flatten)]
    pub measurement_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitialLoad {
    #[serde(flatten)]
    pub partitial_load: PartitialLoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
    #[serde(flatten)]
    pub load: LoadType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteForwardedProduction {
    #[serde(flatten)]
    pub working_site_forwarded_production: WorkingSiteForwardedProductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: BdtString50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineApplicationVersion {
    #[serde(flatten)]
    pub machine_application_version: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendDate {
    #[serde(flatten)]
    pub send_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: WctERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadCount {
    #[serde(flatten)]
    pub load_count: XsnonNegativeInteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: BdtTimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadNumber {
    #[serde(flatten)]
    pub load_number: XsnonNegativeInteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitialLoadId {
    #[serde(flatten)]
    pub partitial_load_id: XsnonNegativeInteger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: Xsbase64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: BdtString20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadGreenMass {
    #[serde(flatten)]
    pub load_green_mass: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadVolume {
    #[serde(flatten)]
    pub load_volume: BdtDecimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: BdtString100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingDistance {
    #[serde(flatten)]
    pub forwarding_distance: XsnonNegativeInteger,
}

