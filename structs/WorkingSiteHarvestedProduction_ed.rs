#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LengthClass {
    #[serde(flatten)]
    pub length_class: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentName {
    #[serde(flatten)]
    pub assortment_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineApplicationVersion {
    #[serde(flatten)]
    pub machine_application_version: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunningMeters {
    #[serde(flatten)]
    pub running_meters: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBH {
    #[serde(flatten)]
    pub dbh: PositiveInteger3digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumes {
    #[serde(flatten)]
    pub assortment_volumes: AssortmentVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolume {
    #[serde(flatten)]
    pub assortment_matrix_volume: AssortmentMatrixVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: StanfordTreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductGroupName {
    #[serde(flatten)]
    pub product_group_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendDate {
    #[serde(flatten)]
    pub send_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterClass {
    #[serde(flatten)]
    pub diameter_class: PositiveInteger4digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: HarvestingStemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    #[serde(flatten)]
    pub resource_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytes {
    #[serde(flatten)]
    pub bytes: base64Binary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestOwner {
    #[serde(flatten)]
    pub forest_owner: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolume {
    #[serde(flatten)]
    pub stem_type_volume: StemTypeVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteHarvestedProduction {
    #[serde(flatten)]
    pub working_site_harvested_production: WorkingSiteHarvestedProductionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileName {
    #[serde(flatten)]
    pub file_name: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolume {
    #[serde(flatten)]
    pub assortment_volume: AssortmentVolumeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserId {
    #[serde(flatten)]
    pub product_user_id: String100Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumeUnclassified {
    #[serde(flatten)]
    pub assortment_volume_unclassified: AssortmentVolumeUnclassifiedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndDate {
    #[serde(flatten)]
    pub end_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    #[serde(flatten)]
    pub count: PositiveInteger6digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModificationDate {
    #[serde(flatten)]
    pub modification_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemTypeVolumes {
    #[serde(flatten)]
    pub stem_type_volumes: StemTypeVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentVolumesUnclassified {
    #[serde(flatten)]
    pub assortment_volumes_unclassified: AssortmentVolumesUnclassifiedType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandId {
    #[serde(flatten)]
    pub stand_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentMatrixVolumes {
    #[serde(flatten)]
    pub assortment_matrix_volumes: AssortmentMatrixVolumesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartDate {
    #[serde(flatten)]
    pub start_date: TimeStampType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: String5Type,
}

