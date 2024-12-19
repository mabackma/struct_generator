#[derive(Debug, Serialize, Deserialize)]
pub struct DelinationObjectOrderId {
    #[serde(flatten)]
    pub delination_object_order_id: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesProposalForestHaulageDistances {
    #[serde(flatten)]
    pub storages_proposal_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageProposals {
    #[serde(flatten)]
    pub storage_proposals: StoragesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentID {
    #[serde(flatten)]
    pub assortment_i_d: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageClass {
    #[serde(flatten)]
    pub storage_class: StorageDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalUpdate {
    #[serde(flatten)]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadPaymentReference {
    #[serde(flatten)]
    pub load_payment_reference: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageCount {
    #[serde(flatten)]
    pub image_count: PositiveInteger2digitsType,
}

