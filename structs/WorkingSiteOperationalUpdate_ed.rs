#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub volume: Decimal3FractionDigitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub storage: StorageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractorId {
    #[serde(flatten)]
    pub contractor_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForestHaulageDistance {
    #[serde(flatten)]
    pub forest_haulage_distance: PositiveInteger5digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanModify {
    #[serde(flatten)]
    pub can_modify: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortments {
    #[serde(flatten)]
    pub assortments: AssortmentsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementMethod {
    #[serde(flatten)]
    pub measurement_method: MeasurementMethodType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteId {
    #[serde(flatten)]
    pub working_site_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageAdditionalRemarks {
    #[serde(flatten)]
    pub storage_additional_remarks: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageForestHaulageDistance {
    #[serde(flatten)]
    pub storage_forest_haulage_distance: StorageForestHaulageDistanceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assortment {
    #[serde(flatten)]
    pub assortment: AssortmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageId {
    #[serde(flatten)]
    pub storage_id: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningOperation {
    #[serde(flatten)]
    pub working_site_planning_operation: WorkingSitePlanningOperationStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    #[serde(flatten)]
    pub quality: String5Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageCount {
    #[serde(flatten)]
    pub image_count: PositiveInteger2digitsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMax {
    #[serde(flatten)]
    pub diameter_max: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingEmail {
    #[serde(flatten)]
    pub plowing_email: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGroup {
    #[serde(flatten)]
    pub code_group: AssortmentGroupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageName {
    #[serde(flatten)]
    pub storage_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadNumber {
    #[serde(flatten)]
    pub load_number: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSafetyInfo {
    #[serde(flatten)]
    pub working_safety_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPlace {
    #[serde(flatten)]
    pub measurement_place: MeasurementPlaceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    #[serde(flatten)]
    pub code: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageClass {
    #[serde(flatten)]
    pub storage_class: StorageDryingClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUserIds {
    #[serde(flatten)]
    pub product_user_ids: ProductUserIdsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseContractExtraInfo {
    #[serde(flatten)]
    pub purchase_contract_extra_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesForestHaulageDistances {
    #[serde(flatten)]
    pub storages_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiameterMin {
    #[serde(flatten)]
    pub diameter_min: PositiveIntegerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    #[serde(flatten)]
    pub unit: WorkCodeUnitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeSpecies {
    #[serde(flatten)]
    pub tree_species: TreeSpeciesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMin {
    #[serde(flatten)]
    pub height_min: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentInfo {
    #[serde(flatten)]
    pub assortment_info: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingTelephone {
    #[serde(flatten)]
    pub plowing_telephone: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageAddress {
    #[serde(flatten)]
    pub storage_address: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanCultivateInAutumn {
    #[serde(flatten)]
    pub can_cultivate_in_autumn: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryModificationAllowed {
    #[serde(flatten)]
    pub geometry_modification_allowed: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingArranged {
    #[serde(flatten)]
    pub plowing_arranged: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageProposals {
    #[serde(flatten)]
    pub storage_proposals: StoragesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemType {
    #[serde(flatten)]
    pub stem_type: StemTypeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryUserId {
    #[serde(flatten)]
    pub delivery_user_id: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelinationObjectOrderId {
    #[serde(flatten)]
    pub delination_object_order_id: String200Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceBuyerId {
    #[serde(flatten)]
    pub service_buyer_id: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSiteOperationalUpdate {
    #[serde(flatten)]
    pub working_site_operational_update: WorkingSiteOperationalUpdateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningStatus {
    #[serde(flatten)]
    pub working_site_planning_status: WorkingSitePlanningStatusType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    #[serde(flatten)]
    pub accessibility: TransportAccessibilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingName {
    #[serde(flatten)]
    pub plowing_name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    #[serde(flatten)]
    pub storage_info: String500Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwner {
    #[serde(flatten)]
    pub storage_land_owner: StorageLandOwnerType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragesProposalForestHaulageDistances {
    #[serde(flatten)]
    pub storages_proposal_forest_haulage_distances: StoragesForestHaulageDistancesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storages {
    #[serde(flatten)]
    pub storages: StoragesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationStorage {
    #[serde(flatten)]
    pub destination_storage: String20Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssortmentID {
    #[serde(flatten)]
    pub assortment_i_d: ERPIdType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoText {
    #[serde(flatten)]
    pub info_text: String1000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLinkedToWorkingSite {
    #[serde(flatten)]
    pub storage_linked_to_working_site: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingSitePlanningInfo {
    #[serde(flatten)]
    pub working_site_planning_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightMax {
    #[serde(flatten)]
    pub height_max: Decimal1FractionDigitType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadPaymentReference {
    #[serde(flatten)]
    pub load_payment_reference: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(flatten)]
    pub geometry: PointAndLineOrPolygonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryRestriction {
    #[serde(flatten)]
    pub delivery_restriction: YesNoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    #[serde(flatten)]
    pub name: String50Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingsiteInfo {
    #[serde(flatten)]
    pub workingsite_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLandOwnerInformation {
    #[serde(flatten)]
    pub storage_land_owner_information: ContactInformationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurningPointClass {
    #[serde(flatten)]
    pub turning_point_class: TurningPointClassType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentalObjectInfo {
    #[serde(flatten)]
    pub environmental_object_info: String3000Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlowingDate {
    #[serde(flatten)]
    pub plowing_date: DateType,
}

